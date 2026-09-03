# Pre-Task Context

Last refreshed: 2026-09-03

Read this file before starting repository work. Verify its claims against the
current worktree because source code and Git history remain authoritative.

## Project Snapshot

- GCCtoolkit is a Tauri v2 desktop application for game-content creators.
- The working feature downloads official Steam trailers through Steam's API and
  remuxes DASH media to MP4 with ffmpeg.
- Windows MSI and Linux DEB/RPM packages are released through tag-driven
  GitHub Actions. Version `0.1.6` is the current published release; the 0.1.5
  tag was created on the wrong commit and was never shipped.
- The frontend is plain HTML, CSS, and JavaScript under `app/src/`; the backend
  is Rust under `app/src-tauri/src/`.
- The legacy PowerShell and Bash downloader paths were retired from the active
  project state; the desktop-first Tauri workflow is the authoritative path.
- ffmpeg and ffprobe are external dependencies. Windows users can install a
  verified managed copy under user-local app data; Linux remains user-managed.
- Steam trailer downloads are grouped beneath the selected root in a safe
  `<SteamID> <Safe_Game_Name>` folder. Existing same-ID folders are reused.
- Steam history is capped at 200 entries in `steam-history.json`. Windows
  `0.1.3` performs a one-time recovery from the legacy `com.brwinnov.ggt`
  WebView profile; two existing entries were recovered in local runtime testing.
- Settings shows the media engine as `MediaEngineStatus` (notFound / ready /
  broken, source managed / system / custom, display version). Windows can
  select an existing ffmpeg/ffprobe pair or install the verified BtbN
  `n8.1-latest` LGPL pair without elevation, with progress, cancel, staged
  swap, and a manifest-hash update check (`Install latest` / `Up to date`).
- Settings provides editable Default and NavyWhite1 themes plus user-created
  themes. Explicit updates, Save As, rename/delete, colour customisations, and
  startup choice persist in WebView storage; Default cannot be renamed/deleted.

## Current Planning State

- `docs/FEATURES.md` is the long-term backlog; `docs/TODO.md` is active work;
  `docs/PLAN.md` is the phased roadmap; `CHANGELOG.md` records shipped behavior.
- Project documentation lives under `docs/`; standard GitHub community files,
  `AGENTS.md`, `CLAUDE.md`, and `CHANGELOG.md` remain at repository root.
- `STM-10`, a user-triggered clipboard `PASTE` action, is shipped.
- Recently reviewed creative-media ideas are technically feasible and tracked
  as `MED-01` through `MED-04` and `PKT-01` through `PKT-05`.
- Recommended creative-media order: import/export and project inventory first,
  then background-removal prototypes, still-image canvas editing, and finally
  animated composition and controlled font installation.
- Signed auto-update architecture is planned in `docs/repo-update.md` but is not
  implemented.
- The 2026-08-28 source security audit found 0 critical, 2 high, 5 medium, and
  2 low issues. See `docs/SECURITYAUDIT.md` for evidence and remediation tasks.
- The repository is MIT-licensed. SignPath Foundation declined the free OSS
  code-signing application; the project is not paying for a commercial
  SignPath subscription and has no active code-signing plan. See
  `docs/CODE_SIGNING_POLICY.md`. All CI SignPath steps and the `.signpath/`
  templates were removed.
- `SEC-001` (Bash App ID code-injection path) is resolved by removal.
  `SEC-002` (unsigned installers) remains open with no viable remediation
  route; releases stay explicitly unsigned by policy.
- After security work, the next product choice is metadata sidecars (`STM-02`),
  screenshots/key art (`STM-03`), or batch input (`STM-04`).

## Constraints and Decisions

- Current release targets are Windows and Linux; macOS is deferred.
- MSI is the sole Windows installer format. Its pinned WiX UpgradeCode must
  remain unchanged so upgrades replace the existing installation.
- Steam data must come from supported APIs, not ephemeral browser Blob URLs.
- Public-media workflows are in scope; DRM bypass and private-account credential
  capture are out of scope.
- Project-owned source is distributed under MIT. Third-party components retain
  their own licenses and must not be signed as project-owned binaries.
- Tagged Windows releases are explicitly labeled unsigned; this project has no
  active code-signing plan (see `docs/CODE_SIGNING_POLICY.md`). Do not claim
  signed Windows or Linux packages.
- Clipboard reads must occur only after a user clicks `PASTE` and should request
  only Tauri's text-read capability.
- Steam history and ffmpeg settings live under the stable Tauri identifier's
  local app-data folder, outside the MSI installation directory.
- On Windows that folder is `%LOCALAPPDATA%\com.ackrosgaming.gcc`; managed
  tools are under `tools\ffmpeg\bin`. Normal MSI update/uninstall preserves
  this data, but deleting the folder intentionally removes it.
- Windows ffmpeg installation downloads only after user approval, verifies the
  ZIP against BtbN's release checksum manifest, applies size limits, extracts
  only `ffmpeg.exe` and `ffprobe.exe` into a staging folder, and replaces
  `tools\ffmpeg\bin` only after the new binary passes `-version`.
- `settings.json` `ffmpegPath` means a user-chosen custom path only; the
  managed copy is recognised by location. `managedArchiveSha256` records the
  installed archive hash for the update check.
- Installer progress is emitted on `media-engine-progress`; the Steam
  `download-progress` event is unchanged.
- Remote media import requires URL, redirect, timeout, size, and content-type
  validation.
- Font installation should be explicit, license-aware, per-user, and never
  silently elevated.
- Preserve unrelated worktree changes and keep generated files generated.
- User-facing text uses English (Ireland) spelling: `colour`, `customise`,
  `initialise`, `organise`.

## Validation Baseline

- Markdown: pass tracked Markdown paths from `git ls-files '*.md'` to
  `markdownlint-cli2` so dependency files under `node_modules` are excluded.
- Rust backend: from `app/src-tauri/`, run `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo test`.
- Frontend: run `node --check app/src/main.js`.
- Desktop app: use the npm scripts documented in `app/README.md`.
- Release claims require verification against the tag, originating commit, CI
  run, and published assets.

## Current Release

- Version `0.1.6` is the current repository release state in source metadata,
  verified live against GitHub on 2026-09-03.
- Tag `v0.1.6` points at commit `bc7c412` ("Release 0.1.6"), the commit that
  built the published artifacts. The tag briefly drifted to a later commit
  after a housekeeping push retriggered the tagged workflow; it was moved back
  to `bc7c412` and force-pushed, which retriggered a rebuild from that same
  commit and republished the release assets (new hashes, same source).
- The GitHub release is marked `Latest`, is not a draft or prerelease, and
  contains exactly one MSI, one `.deb`, one `.rpm`, and `SHA256SUMS`. The
  downloaded MSI's SHA-256 matches its `SHA256SUMS` entry
  (`1e208cdf...ec2637`), and its `ProductVersion` (`0.1.6`) and `UpgradeCode`
  (`{1DDF37BF-062F-547C-A167-DAB5D7867081}`) match source.
- The 0.1.5 tag was created on the wrong commit and was never published; it
  and its GitHub release no longer exist (already removed before this
  housekeeping pass). The shipped work was carried forward into 0.1.6.
- Releases are unsigned by policy with no active code-signing plan — see
  `docs/CODE_SIGNING_POLICY.md`. All SignPath CI steps and the `.signpath/`
  directory were removed; the repository variable `SIGNPATH_ENABLED` and any
  related secrets are unused and can be deleted from repository settings.
- Dependency audits currently report 0 known npm and Rust vulnerabilities.
  RustSec reports 18 non-vulnerability warnings, including a Linux GTK
  unsoundness advisory; these are tracked under `SEC-007`.

## Start-of-Task Checklist

- [ ] Read this file and `AGENTS.md`.
- [ ] Run `git status --short`.
- [ ] Identify the owning source file or planning document.
- [ ] Check the nearest relevant test, call site, or prior journal entry.
- [ ] State a narrow hypothesis and a check that can disprove it before editing.
