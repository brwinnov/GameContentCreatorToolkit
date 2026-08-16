# Feature Backlog

Reviewable list of product and engineering ideas for GCCtoolkit. Use this file
to choose what to build next, add new ideas, and record decisions without
turning `TODO.md` into a long-term wishlist.

## How to Use This List

- **Status:** `shipped`, `ready`, `planned`, `idea`, `deferred`, or `rejected`
- **Priority:** `P0` release blocker, `P1` high value, `P2` useful, `P3` later
- **Effort:** rough relative estimate: `S`, `M`, `L`, or `XL`
- Move selected work into `TODO.md` with acceptance criteria.
- Keep architecture details in focused plans such as `repo-update.md`.
- Mark delivered behavior in `CHANGELOG.md`.

## Suggested Next Choices

These are the strongest candidates for the next focused milestone:

| ID | Feature | Why now |
|---|---|---|
| REL-01 | Clean beta release baseline | Fixes version, provenance, dependency, and public-distribution gaps before adding scope. |
| DEP-01 | Bundle ffmpeg/ffprobe on Windows | Makes the working Steam feature usable without manual dependency setup. |
| STM-03 | Screenshots and key art | High creator value using data already available from Steam. |
| STM-02 | Metadata sidecars | Small, useful foundation for organization and Resolve workflows. |
| APP-03 | Download queue and batch mode | Turns the app from a one-game tool into a repeatable production utility. |

Choose one primary feature and one small supporting item per milestone. Avoid
starting auto-update, social downloaders, and a major UI rewrite together.

## Shipped Foundation

| ID | Feature | Status | Notes |
|---|---|---|---|
| STM-00 | Steam trailer discovery by App ID or URL | shipped | Uses Steam `appdetails`; handles age-gated titles. |
| STM-00A | Steam game-name search | shipped | User selects the correct Steam match. |
| STM-00B | Trailer selection and MP4 download | shipped | ffmpeg remuxes DASH without re-encoding. |
| APP-00 | Tauri desktop application | shipped | Windows and Linux package builds are operational. |
| APP-00A | Native download-folder picker | shipped | Defaults to the OS Downloads folder. |
| APP-00B | Live download progress | shipped | Based on ffprobe duration and ffmpeg progress. |
| APP-00C | Search/download history | shipped | Local browser storage, capped at 200 entries. |
| REL-00 | Windows and Linux CI packages | shipped | MSI, NSIS, `.deb`, and `.rpm` build successfully. |

## Release and Distribution

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| REL-01 | Clean beta release baseline | ready | P0 | M | Align tag and artifact commit; commit npm lockfile; remove duplicate assets; update release notes; mark beta. |
| REL-02 | Per-user Windows NSIS install | planned | P1 | S | Change install mode to `currentUser`; validate install/update/uninstall under `%LOCALAPPDATA%`. |
| REL-03 | Portable Windows ZIP | planned | P1 | M | Package app plus required tools and notices; portable builds receive update notifications but do not overwrite themselves. |
| REL-04 | Signed application auto-update | planned | P1 | L | Detailed in `repo-update.md`; requires per-user install, signing keys, tag-driven releases, and update UI. |
| REL-05 | Windows Authenticode signing | planned | P1 | M | Requires a code-signing certificate and secured CI signing process. |
| REL-06 | Stable and beta update channels | idea | P2 | M | Separate signed manifests; stable installations must ignore prereleases. |
| REL-07 | Linux AppImage | idea | P2 | M | Enables Tauri-style Linux self-update; retain `.deb`/`.rpm` for package-manager users. |
| REL-08 | Linux package runtime test matrix | ready | P0 | M | Test Ubuntu/Debian and Fedora clean installs, ffmpeg discovery, picker, download, uninstall. |
| REL-09 | macOS app/DMG support | deferred | P3 | L | Requires Apple hardware/runner, signing, notarization, icons, and runtime validation. |
| REL-10 | Checksums and software bill of materials | planned | P1 | M | Publish SHA-256 and SBOM/provenance with each release. |

## Dependencies and Tooling

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| DEP-01 | Bundle ffmpeg and ffprobe on Windows | ready | P1 | M | Pin a redistributable build; verify SHA-256 in CI; include exact license/build notices. |
| DEP-02 | Detect and explain missing Linux ffmpeg | planned | P1 | S | Show distro-appropriate commands and retry detection; do not silently install system packages. |
| DEP-03 | Tool version display and diagnostics | planned | P2 | S | Show app, ffmpeg, ffprobe, and future yt-dlp versions in Settings/About. |
| DEP-04 | Verified yt-dlp baseline bundle | planned | P1 | M | Implement only when the first social downloader ships; keep shipped copy as fallback. |
| DEP-05 | Independent verified yt-dlp updater | idea | P2 | L | Official source only, checksum/signature validation, staged activation, health check, rollback. |
| DEP-06 | Dependency lockfiles and reproducible CI | ready | P0 | S | Commit `package-lock.json`; use `npm ci`; keep Cargo lockfile; pin critical Actions. |
| DEP-07 | Third-party notices screen/file | ready | P0 | S | Required before redistributing ffmpeg or yt-dlp binaries. |

## Steam Features

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| STM-01 | Trailer quality selection | planned | P2 | M | Parse DASH variants and expose quality preferences. |
| STM-02 | JSON metadata sidecar | ready | P1 | S | Include game/app ID, trailer name, source URL, fetch time, dimensions, duration, and file hash. |
| STM-03 | Screenshot and key-art downloader | ready | P1 | M | Use screenshots, header, capsule, background, and logo data already available from Steam. |
| STM-04 | Batch game input | ready | P1 | M | Accept pasted lists, CSV, or multiple App IDs; feeds the shared queue. |
| STM-05 | Retry with exponential backoff | planned | P1 | M | Cover Steam API, manifest, and segment failures with bounded retries and cancellation. |
| STM-06 | Preview before download | idea | P2 | M | Play trailer or show thumbnails before selecting assets. |
| STM-07 | Language and region selection | idea | P3 | M | Make Steam `cc` and `l` configurable. |
| STM-08 | Steam Workshop asset support | idea | P3 | XL | Separate legal/API and content-type investigation required. |
| STM-09 | Store description and press facts export | idea | P2 | M | Export supported text metadata for review notes without copying user/private content. |

## Download Workflow

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| APP-01 | Common downloader-module interface | planned | P1 | L | Normalize source detection, item metadata, selection, progress, cancellation, and results. |
| APP-02 | Persistent settings | planned | P1 | M | Download folder, quality, naming, tool paths, channel, and update preferences. |
| APP-03 | Download queue and batch mode | ready | P1 | L | Add queue states, concurrency limits, pause/cancel/retry, and session recovery. |
| APP-04 | Concurrent downloads | idea | P2 | M | Requires limits and bandwidth/error behavior; start conservatively. |
| APP-05 | Resume interrupted downloads | idea | P2 | L | Depends on source/range support and safe partial-file tracking. |
| APP-06 | Duplicate detection across runs | planned | P1 | M | Use source identity plus optional hashes; distinguish skip, replace, and versioned copy. |
| APP-07 | Naming templates | idea | P2 | M | Tokens such as game, asset type, date, resolution, and source. |
| APP-08 | Automatic folder organization | planned | P1 | M | `GameName/Trailers`, `Screenshots`, `Art`, `Metadata`; configurable template. |
| APP-09 | Disk-space and destination validation | planned | P1 | S | Check writable destination and estimated space before starting. |
| APP-10 | In-app logs and diagnostics export | idea | P2 | M | Redact usernames/paths where possible; make support reports user-controlled. |
| APP-11 | Native notifications | idea | P3 | S | Completion/failure notifications, configurable and non-spammy. |
| APP-12 | Light/dark/system themes | idea | P3 | M | Preserve current visual identity; accessibility takes priority. |
| APP-13 | Keyboard and screen-reader accessibility | planned | P1 | M | Focus order, labels, contrast, reduced motion, and keyboard-complete workflows. |

## Additional Platforms

These should use yt-dlp behind the shared downloader interface rather than
custom page scrapers. Authentication bypasses and DRM circumvention are out of
scope.

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| SRC-01 | YouTube downloader | planned | P1 | L | First yt-dlp module; quality/format selection, metadata, subtitles, playlists policy. |
| SRC-02 | TikTok downloader | planned | P2 | M | Public URLs only initially; test frequent upstream changes. |
| SRC-03 | Instagram downloader | planned | P2 | L | Public content initially; authentication and cookies need a separate security design. |
| SRC-04 | Facebook downloader | planned | P2 | L | Public content initially; expect upstream variability. |
| SRC-05 | Generic yt-dlp URL handler | idea | P2 | M | Powerful but harder to support; expose detected extractor and clear compatibility status. |
| SRC-06 | Subtitle/caption download | idea | P2 | M | Useful for transcription and review research where available. |
| SRC-07 | Playlist/channel batch import | idea | P3 | L | Requires safeguards against unexpectedly large jobs. |

## Creator Workflow

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| CRT-01 | Contact sheet generation | idea | P1 | M | Create timestamped visual sheets for quickly reviewing trailers/screenshots. |
| CRT-02 | Proxy and thumbnail generation | planned | P2 | L | Presets optimized for fast preview and Resolve import. |
| CRT-03 | DaVinci Resolve bin export | idea | P2 | L | Start with folder/bin metadata before attempting timeline generation. |
| CRT-04 | DaVinci Resolve timeline export | idea | P3 | XL | Requires a clearly defined editorial workflow and interchange format. |
| CRT-05 | Asset favorites and review status | idea | P2 | M | Mark shortlist/rejected/used without modifying source media. |
| CRT-06 | Copy attribution/source details | idea | P2 | S | Copy source URL, game, publisher, and fetch date for notes/descriptions. |
| CRT-07 | Asset manifest export | idea | P1 | M | Machine-readable inventory with relative paths, metadata, and hashes. |
| CRT-08 | Thumbnail composition workspace | idea | P3 | XL | Crop, logo placement, safe areas, and export presets; potentially a separate module. |

## Reliability, Security, and Privacy

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
|---|---|---:|---:|---:|---|
| QLT-01 | Rust unit tests for pure logic | ready | P0 | M | URL/App ID parsing, filename safety, selection, response mapping, and version logic. |
| QLT-02 | Frontend interaction tests | planned | P1 | M | Search states, selection, history, progress, errors, and settings. |
| QLT-03 | Packaged-app smoke tests | planned | P1 | L | Clean Windows and Linux VMs; launch, fetch, download, and uninstall. |
| QLT-04 | CI formatting/lint/test gates | ready | P0 | M | Add Rust formatting/clippy/tests and frontend checks before packaging. |
| SEC-01 | Project license | ready | P0 | S | Choose and add a license before broad source redistribution. |
| SEC-02 | Dependabot and dependency review | ready | P0 | S | Enable alerts; add scheduled dependency updates and PR review. |
| SEC-03 | Release provenance attestation | idea | P1 | M | GitHub artifact attestations/SLSA provenance for public binaries. |
| SEC-04 | Privacy and network-requests page | planned | P1 | S | Explain Steam/GitHub/update checks, local history, paths, and no telemetry. |
| SEC-05 | Content-use responsibility notice | planned | P1 | S | Explain that users are responsible for rights, platform terms, and redistribution. |
| SEC-06 | Secret and binary scanning in CI | idea | P1 | M | Secret scanning, dependency review, and optional malware scan of release binaries. |

## Deliberately Deferred or Out of Scope

| ID | Feature | Status | Reason |
|---|---|---|---|
| OUT-01 | macOS release | deferred | Focus remains Windows and Linux until those paths are stable. |
| OUT-02 | Built-in telemetry | deferred | No analytics unless explicitly designed as opt-in and privacy-preserving. |
| OUT-03 | DRM bypass | rejected | Not part of the product. |
| OUT-04 | Private-account credential capture | rejected | High security and platform-policy risk; public media is the initial scope. |
| OUT-05 | Automatic system-package installation on Linux | rejected | The app may explain commands but should not silently elevate or mutate the OS. |

## Idea Intake Template

Add ideas to the relevant section using this format:

```markdown
| NEW-00 | Short feature name | idea | P2 | M | User value, constraints, and dependencies. |
```

Before promoting an idea to `ready`, answer:

1. Who is it for and what repeated problem does it solve?
2. What is the smallest useful version?
3. What existing feature or architecture does it depend on?
4. What privacy, security, licensing, or platform risks apply?
5. How will we verify it works on the supported operating systems?

## Decisions Needed

- Which license should govern the repository?
- Should the next public version focus on release integrity or a new creator
  feature?
- Should Windows bundle ffmpeg immediately, or first ship improved dependency
  guidance?
- Which Steam enhancement should come first: art, metadata, or batch queues?
- Is YouTube the first social source, or should the shared downloader interface
  be completed and tested before any new source?
- Should Linux add AppImage or remain `.deb`/`.rpm` only?