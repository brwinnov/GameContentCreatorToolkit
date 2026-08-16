# Changelog

All notable changes to this project. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- `app/` — Tauri v2 desktop app scaffold. Steam tab fully wired to a Rust
  port of the Steam trailer download logic (appdetails API + ffmpeg DASH→MP4);
  YouTube/TikTok/Instagram tabs are styled placeholders. First compile and
  end-to-end Steam tab test (App ID 2424010) passed — see `PLAN.md` Phase 2
  for status.
- Repo scaffold reorganized around `workspace/` as the VS Code project root
- `PLAN.md`, `TODO.md`, `SECURITYAUDIT.md`, `CHANGELOG.md` at repo root
- `CLAUDE.md` moved from `docs/` to repo root (AI-assistant project context)
- `ai-journal/` — running log of AI-assisted work sessions
- `logs/` — local runtime logs (gitignored, folder tracked via `.gitkeep`)
- `.gitignore` and `.env.example` added
- `SECURITYAUDIT.md` establishing that no credentials/secrets are currently
  handled by the project
- Steam tab: "Show history" button (footer, left of "Download selected") opens
  a modal listing every App ID/URL search and every per-trailer download
  attempt, each with a status badge (`SEARCH`/`INVALID` for searches;
  `DOWNLOADED`/`FAILED`/`SKIPPED`/`INVALID` for download attempts). Persisted
  in `localStorage`, capped at the most recent 200 entries, survives app
  restarts.
- Windows release build for 0.1.0: portable EXE + MSI + NSIS installer in
  `workspace/release/ggc-app-0.1.0` for sharing and quick validation.
- Steam name-based lookup: users can type a game name, see matching results,
  and click the correct match to continue the trailer fetch flow.

### Changed
- Consolidated `docs/README.md` and `docs/PLAN.md` (now superseded by root
  `README.md` and `PLAN.md`) to avoid duplicate/drifting copies
- Updated the distribution plan to focus on Windows desktop builds plus Linux
  `.deb` and `.rpm` packages; macOS packaging is intentionally deferred for
  the current milestone.
- Steam tab: added a live 0–100% progress bar in the console header, driven
  by real `ffprobe` duration + ffmpeg `-progress` decode time (not a fake
  animation)
- Steam tab pane title now renders uppercase (`STEAM TRAILER DOWNLOADER`)
- Windows release polish: ffmpeg/ffprobe are launched with hidden console
  behavior for desktop use, while the app continues to capture progress and
  errors in its own UI instead of flashing black cmd windows.
- Added a global `[hidden] { display: none !important; }` rule — a few
  elements (`.console`, `.pane-footer`, the new `.modal-overlay`) pair the
  `hidden` attribute with their own unconditional `display` value, which
  depends on the browser engine weighting `[hidden]` as `!important` to stay
  hidden. Made that explicit instead of relying on it implicitly.

### Fixed
- ffmpeg's DASH demuxer occasionally logs a benign "Error when loading first
  fragment of playlist" line on segment retries even though the download
  completes fine (exit code 0, valid output file). Filtered out of
  `scripts/pwsh/download_steam_trailers.ps1` console output; the Tauri app's
  `download_trailers` command now captures ffmpeg's output instead of
  inheriting stdio, so it never reaches the UI on success and only the
  actual last stderr line is shown on a genuine failure.
- Windows release UX: stopped the black console popups from ffmpeg/ffprobe so
  the desktop app behaves like a proper GUI tool during Steam trailer
  downloads.
- `build_toolkit.ps1` was re-zipping whatever was already in `toolkit/`
  without syncing it from `scripts/pwsh/download_steam_trailers.ps1` first —
  the two copies could silently drift despite CLAUDE.md documenting them as
  always identical. The build script now copies the source in before
  packaging.

## [0.1.0-BETA] — 2026-04-15
### Added
- Initial `download_steam_trailers.ps1` (PowerShell 7): queries the Steam
  `appdetails` API, resolves `dash_h264` DASH manifests, reassembles via
  `ffmpeg -c copy` into MP4. Supports `-Latest`, `-Oldest`, `-ListOnly`, and
  interactive All/Latest/pick-one selection.
- `build_toolkit.ps1` packaging script → `toolkit/` distributable copy
- Initial README, usage docs, and development plan
