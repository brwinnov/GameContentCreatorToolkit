# Security Audit

Living document. Update whenever a new dependency, credential type, or
external call is added. Not a substitute for a real third-party audit if this
project ever gets wide distribution — treat it as a running checklist.

## Current data handled

| Item | Sensitive? | Notes |
| --- | --- | --- |
| Steam age-gate cookies (`birthtime`, `lastagecheckage`, `mature_content`) | No | Fixed, publicly-known constant values used to bypass the 18+ content gate. Not tied to any account, not a login, not a secret. Safe to commit in source. |
| Steam App ID | No | Public identifier, same as in the store URL. |
| Downloaded media files | No | Publicly published marketing assets from the publisher/developer, used under fair-use/press-kit norms. Not redistributed. |
| Local file paths (e.g. ffmpeg location) | No | Machine-specific but not sensitive; may still reveal a username in a path — keep out of anything published publicly (screenshots, issue reports). |

**No credentials, API keys, tokens, passwords, or personal data are currently
required or stored by this project.**

## Things to check before every commit / push

- [ ] No hardcoded absolute paths containing your real Windows username
      (`C:\Users\<name>\...`) in committed files
- [ ] No `.env` file committed (see `.env.example` for the template — real
      values stay local only)
- [ ] No API keys/tokens if a future feature adds one (e.g. a YouTube Data
      API key, a Steam Web API key with account access) — these must go in
      `.env`, never in source, never in `docs/`, `ai-journal/`, or `logs/`
- [ ] `logs/` directory contents are not committed (may contain local paths
      or App IDs of unreleased/private review copies you don't want public)
- [ ] Downloaded media itself is never committed to the repo

## External dependencies / supply-chain notes

- **ffmpeg** — not bundled in the repo or MSI. On Windows, the app can download
  the latest BtbN LGPL static ZIP only after an explicit user click, verify its
  SHA-256 against that release's `checksums.sha256`, enforce download/extraction
  size limits, and extract only `ffmpeg.exe` and `ffprobe.exe` under user-local
  app data. Users may instead choose an existing pair in Settings. Linux remains
  user-managed.
- **yt-dlp** (planned, Phase 3) — same policy: user-installed, not bundled.
  yt-dlp interacts with third-party platforms (YouTube/TikTok/FB/Instagram);
  review its own security advisories periodically since it's under active,
  frequent development.
- **Steam `appdetails` API** — unauthenticated public endpoint, no API key
  required, no login performed.

## Future items to revisit

- Before enabling the planned Tauri auto-updater, verify update packages are
  signed and fetched over HTTPS from a pinned source.
- If any platform integration ever requires OAuth (e.g. a real Instagram/
  Facebook Graph API instead of yt-dlp scraping), tokens must be stored using
  OS-level secure storage (Windows Credential Manager / macOS Keychain /
  Linux Secret Service), never in plaintext config.
- Review whether review-copy game names/App IDs logged locally in `logs/`
  should be treated as confidential (some publishers include NDA terms on
  early-access review copies) — if so, `logs/` should be gitignored (already
  is) and periodically purged.

  ## Audit report — 2026-08-28

  ### Scope and method

  Reviewed baseline commit `1430805` across the Tauri command boundary, frontend
  rendering, Steam requests, ffmpeg execution and installation, ZIP extraction,
  local persistence, clipboard/dialog permissions, legacy scripts, dependency
  locks, and GitHub Actions release workflows.

  Automated checks performed:

  - `npm audit --package-lock-only --audit-level=low`: 0 vulnerabilities.
  - `cargo audit`: 0 known vulnerabilities; 18 warnings comprising 16
    unmaintained crates, 1 unsound crate, and 1 yanked crate.
  - Current tracked files and Git history were scanned for common credential and
    private-key patterns: no credential findings.
  - No installers, archives, binaries, private keys, environment files, logs, or
    build/dependency directories are unexpectedly tracked. `.env.example` and
    `logs/.gitkeep` are intentional placeholders.

  This is a source and configuration review, not penetration testing, binary
  analysis, or a third-party assessment.

  ### Summary

  | Severity | Count | Disposition |
  | --- | ---: | --- |
  | Critical | 0 | None found |
  | High | 2 | Remediate before broad distribution |
  | Medium | 5 | Schedule before or with the next release |
  | Low | 2 | Defense-in-depth backlog |

  ### Findings and remediation tasks

  #### SEC-001 — High — Bash App ID can become executable JavaScript

  The deprecated Bash downloader accepts any `APP_ID` and interpolates it into
  the JavaScript source passed to `node -e` in
  `scripts/bash/download_steam_trailers.sh`. Shell quoting prevents a second
  shell evaluation, but it does not prevent a crafted argument from changing the
  JavaScript program that Node executes.

  Existing mitigation: the script is deprecated and normal documented inputs are
  numeric Steam IDs. This is not sufficient while the script remains shipped.

  This issue is resolved by removal in commit `bc6d041` when the legacy Bash and
  PowerShell downloader paths were retired from the supported project state.

  The release workflow publishes MSI, DEB, and RPM files without an application
  signature or detached release signature. HTTPS and GitHub access controls help
  transport integrity, but users cannot independently authenticate the publisher
  or distinguish an official installer from a convincing replacement.

  Remediation direction: the repository is MIT-licensed and publishes the
  code-signing and privacy policies required for a SignPath Foundation OSS
  application. The application has been submitted and dashboard-ready artifact
  configurations are checked in. Foundation acceptance, SignPath resource
  setup, repository credentials, and the first successful signed build remain.

  Temporary risk acceptance: to establish public usage and project reputation,
  the maintainer accepts publishing preview installers before SignPath approval.
  These releases must be labeled unsigned, originate from tagged GitHub Actions
  builds, include `SHA256SUMS`, and warn that checksums are not signatures. This
  does not remediate SEC-002; enable the fail-closed signed path as soon as
  SignPath setup is complete.

  Tasks:

  - [x] Adopt an OSI-approved project license and publish SignPath's required
    code-signing roles and privacy policy.
  - [ ] Authenticode-sign the MSI with a protected code-signing certificate and
    timestamp it before upload.
  - [ ] Verify the MSI signature in CI and fail publication on absence, mismatch,
    or invalid timestamp.
  - [ ] Define signing or detached-signature verification for Linux packages.
  - [x] Generate and verify a `SHA256SUMS` manifest for every release.
  - [ ] Add an independently verifiable signature for the checksum manifest.
  - [x] Keep signing keys outside repository and workflow plaintext; SignPath
    Foundation controls the certificate and CI receives only a submitter token.

  #### SEC-003 — Medium — Managed ffmpeg trust is mutable and same-origin

  The Windows installer fetches both `ffmpeg-master-latest-win64-lgpl.zip` and
  `checksums.sha256` from BtbN's mutable `latest` GitHub release. SHA-256 detects
  corruption and mismatched files, but compromise of the upstream repository or
  release can replace both the executable and checksum together. The current
  size limits, HTTPS, safe ZIP names, and two-file extraction are good controls.

  Tasks:

  - [ ] Pin each application release to an immutable ffmpeg release/tag and an
    expected SHA-256 stored in reviewed source or a signed manifest.
  - [ ] Verify an upstream signature or provenance attestation when available.
  - [ ] Record the selected ffmpeg source/version/hash in release notes and the
    software bill of materials.
  - [ ] Extract into a temporary directory and atomically replace the managed
    pair only after both executables pass integrity and version checks.

  #### SEC-004 — Medium — Renderer-supplied URIs reach ffprobe and ffmpeg

  `download_trailers` accepts `TrailerInfo` values from the webview and passes
  each `dash_url` directly as an ffprobe/ffmpeg input argument. Argument arrays
  prevent shell injection, but ffmpeg supports local files and many network
  protocols. If renderer code is compromised, it could make the backend process
  arbitrary local or remote inputs rather than only URLs returned by Steam.

  Tasks:

  - [ ] Parse and validate trailer URLs in Rust both when received from Steam and
    immediately before process launch: require HTTPS and approved Steam CDN
    hosts/paths.
  - [ ] Reject credentials, fragments, non-default ports, local/file schemes,
    loopback/private/link-local destinations, and unexpected redirects.
  - [ ] Prefer backend-held lookup state or opaque trailer IDs so the renderer
    cannot substitute an arbitrary URL between lookup and download.
  - [ ] Restrict ffmpeg protocols explicitly to the minimum DASH playback set.
  - [ ] Add tests for `file:`, `concat:`, loopback, private-IP, redirect, and
    lookalike-host inputs.

  #### SEC-005 — Medium — Missing network, body, process, and output limits

  Steam API clients, checksum retrieval, ffprobe, and ffmpeg have no explicit
  timeouts. Steam JSON and the checksum manifest are read into memory without
  response-size limits. The ffmpeg archive itself is correctly capped while
  streaming, but trailer output and process duration are unbounded. A stalled or
  malicious endpoint can consume time, memory, disk, or child processes.

  Tasks:

  - [ ] Set connect and total request timeouts and an explicit redirect policy on
    every `reqwest::Client`.
  - [ ] Stream and cap Steam JSON and checksum-manifest response bodies before
    parsing.
  - [ ] Add ffprobe and ffmpeg timeouts/cancellation that terminate child
    processes and remove partial files.
  - [ ] Define a configurable maximum trailer size and enforce free-space/output
    limits while downloading.

  #### SEC-006 — Medium — Renderer hardening is incomplete

  `tauri.conf.json` disables CSP and enables the global Tauri object. The default
  capability grants broad dialog defaults plus clipboard text reads. Most remote
  content is rendered with `textContent` or escaping, but history rendering
  inserts `appId` and status-derived values into `innerHTML` without escaping.
  Locally altered or future imported history could therefore become stored HTML.
  Any renderer injection has elevated impact because custom commands can write
  files and launch a selected executable.

  Tasks:

  - [ ] Define a restrictive Tauri CSP for local scripts/styles and required IPC;
    do not permit arbitrary remote script or frame sources.
  - [ ] Replace history and trailer `innerHTML` templates with DOM construction
    and `textContent`, or escape every interpolated value including `appId`
    and status.
  - [ ] Replace `dialog:default` with only the exact open-dialog permission used.
  - [ ] Migrate away from `withGlobalTauri` when the frontend build/import setup
    supports scoped module APIs.
  - [ ] Add hostile-string rendering tests and a CSP smoke test.

  #### SEC-007 — Medium — RustSec warnings require an explicit policy

  RustSec reports no known vulnerabilities, but the locked graph has one
  unsoundness advisory (`glib 0.18.5`, `RUSTSEC-2024-0429`) through Tauri's Linux
  GTK3 stack, 16 unmaintained transitive crates, and one yanked lockfile entry.
  The application does not directly call the affected `glib::VariantStrIter`
  API, reducing immediate exposure, but this has not been proven unreachable in
  all Tauri/WebKit paths.

  Tasks:

  - [ ] Track the Tauri/WRY/GTK dependency path and upgrade when a compatible
    release removes or patches `glib 0.18.5`.
  - [ ] Determine why the yanked `chacha20 0.10.1` entry remains in Cargo.lock and
    regenerate the lockfile if it is unreachable stale data.
  - [ ] Run `cargo audit` and `npm audit` in CI; fail known vulnerabilities and
    maintain a reviewed, expiring allowlist for unavoidable warnings.
  - [ ] Reassess the Linux release decision if the GTK unsoundness cannot be
    removed before wider distribution.

  #### SEC-008 — Low — Release workflow hardening is incomplete

  Most GitHub Actions are pinned to full commit SHAs. The Rust toolchain action
  still uses the mutable `dtolnay/rust-toolchain@stable` ref, and the build
  workflow does not declare explicit read-only token permissions.

  Tasks:

  - [ ] Pin the Rust toolchain action to a reviewed full commit SHA and use a
    pinned Rust version or dated channel for reproducible releases.
  - [ ] Add top-level `permissions: contents: read` to the build workflow and keep
    `contents: write` scoped only to the publishing job.
  - [ ] Add dependency review, SBOM generation, and provenance attestations to
    the release pipeline.

  #### SEC-009 — Low — Input, persistence, and privacy limits rely on frontend

  Rust commands do not consistently repeat frontend validation: Steam App IDs and
  search lengths are not bounded, search query construction is manual, and
  `save_history` accepts arbitrary untyped JSON without enforcing the 200-entry
  frontend cap or a byte limit. Durable history stores search terms, game names,
  App IDs, and timestamps in plaintext local app data. Browser-preview mock paths
  also contain the developer's real Windows username.

  Tasks:

  - [ ] Enforce numeric App IDs, query length, typed history entries, entry count,
    field lengths, and total serialized size in Rust.
  - [ ] Build Steam URLs with `reqwest` query parameters instead of string
    interpolation or manual space replacement.
  - [ ] Replace real usernames in preview mocks with generic paths.
  - [ ] Document local-history contents and retention, and provide a reliable
    clear/delete path before adding cloud sync or diagnostic exports.

  ### Verified controls

  - Tauri process launches use argument arrays rather than a shell.
  - Game and trailer names are sanitized before becoming filesystem components;
    numeric App IDs are enforced before creating game folders.
  - Managed ffmpeg downloads use HTTPS, SHA-256 comparison, streamed archive-size
    enforcement, per-tool extraction limits, safe enclosed ZIP names, and an
    explicit two-file extraction allowlist.
  - Clipboard reads require a user click and only text-read permission is granted.
  - Durable files stay under the stable user-local application data directory.
  - Release actions other than the Rust toolchain action are full-SHA pinned, and
    publishing permission is job-scoped.
  - Logs, downloaded media, release output, dependency trees, and build output are
    excluded from source control.
