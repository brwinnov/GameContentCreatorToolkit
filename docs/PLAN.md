# PLAN — Game Content Creator Toolkit

## What this is

Game Content Creator Toolkit is a Tauri desktop application for collecting
official trailer media from Steam and organising it into local download folders
for content-creator workflows.

The active product is the desktop app in `app/`, not a standalone PowerShell
helper. The legacy script has been retired from the supported path and removed
from the public-facing documentation.

## Current state (as of 2026-09-03)

- Active product: Tauri desktop application in `app/`
- Working feature: Steam trailer lookup and local MP4 download flow
- Settings: ffmpeg detection, managed install path, repair/update handling, and
  persistent user-local preferences
- Release state: `0.1.6` is the current public release
- Packaging: Windows MSI and Linux DEB/RPM builds are both part of the active
  release set; Linux packages remain unvalidated rather than paused
- macOS remains deferred

## Roadmap

### Phase 1 — Steam workflow hardening

Completed in the current app:

- Steam App ID and URL input in the desktop UI
- Local trailer download output in a safe per-game folder
- Persisted Steam history
- ffmpeg management and diagnostics in Settings
- Windows-managed install path for verified ffmpeg/ffprobe

Further plan items:

- batch mode for multiple App IDs
- richer metadata sidecars for downloaded trailers
- screenshots and art capture from the same source data
- better retry and validation handling for transient network/API errors

### Phase 2 — Desktop app maturity

This is the current focus of the project.

- refine the Home/Steam/Settings UX and release metadata consistency
- tighten installer and package validation
- continue polish around history, output selection, and error handling
- expand Windows/Linux reliability before additional feature work

### Phase 3 — Additional downloader modules

The long-term plan remains to add social-media downloaders behind a common
interface, with yt-dlp as the likely engine when those integrations ship.

- YouTube
- TikTok
- Instagram
- Facebook

### Phase 4 — Creator workflow features

These features are future-facing and should not be mistaken for the current
shipped capability set:

- key art and screenshot collection
- media project organisation for press-kit workflows
- simple editing and export tools for still-image assets
- later integration with external editing workflows

## Explicitly deferred / not current

- no bundling of ffmpeg or yt-dlp in the repository itself
- no macOS release packaging in the active milestone
- no claim that the Linux package set is fully validated beyond the current
  release pass
- no support for any legacy PowerShell-only workflow as the primary path

## Key references

- `README.md` — public project overview
- `CHANGELOG.md` — shipped release notes
- `docs/TODO.md` — active checklist and current tasks
- `docs/FEATURES.md` — roadmap and feature inventory
- `app/README.md` — project-level app setup and run instructions
