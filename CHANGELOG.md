# Changelog

All notable changes to this project. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- Repo scaffold reorganized around `workspace/` as the VS Code project root
- `PLAN.md`, `TODO.md`, `SECURITYAUDIT.md`, `CHANGELOG.md` at repo root
- `CLAUDE.md` moved from `docs/` to repo root (AI-assistant project context)
- `ai-journal/` — running log of AI-assisted work sessions
- `logs/` — local runtime logs (gitignored, folder tracked via `.gitkeep`)
- `.gitignore` and `.env.example` added
- `SECURITYAUDIT.md` establishing that no credentials/secrets are currently
  handled by the project

### Changed
- Consolidated `docs/README.md` and `docs/PLAN.md` (now superseded by root
  `README.md` and `PLAN.md`) to avoid duplicate/drifting copies

## [0.1.0-BETA] — 2026-04-15
### Added
- Initial `download_steam_trailers.ps1` (PowerShell 7): queries the Steam
  `appdetails` API, resolves `dash_h264` DASH manifests, reassembles via
  `ffmpeg -c copy` into MP4. Supports `-Latest`, `-Oldest`, `-ListOnly`, and
  interactive All/Latest/pick-one selection.
- `build_toolkit.ps1` packaging script → `toolkit/` distributable copy
- Initial README, usage docs, and development plan
