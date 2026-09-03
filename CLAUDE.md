# GGT — Grab Game Trailer

## Project purpose

This repository is the source for the Tauri v2 desktop toolkit used to fetch
official Steam trailers and save them locally as MP4 files for game-review and
press-kit workflows.

The desktop app is the active workflow. The legacy PowerShell helper has been
retired; the project is now documented and maintained around the desktop-first
app in `app/`.

## Current status

- Release: `0.1.6`
- Main workflow: Steam trailer lookup + download in the Tauri desktop app
- Active UI: Home, Steam, and Settings views under `app/src/`
- Current packaging: Windows and Linux release targets; macOS remains deferred
- Active docs: `README.md`, `CHANGELOG.md`, `docs/PLAN.md`, and `app/README.md`

## Key files

- `app/` — active Tauri desktop application source
- `app/src/index.html` — main UI layout
- `app/src/main.js` — app runtime and mock/desktop command wiring
- `app/src/style.css` — app styling and layout
- `app/src-tauri/` — Rust backend and Tauri configuration
- `README.md` — GitHub landing page and current product overview
- `CHANGELOG.md` — shipped features and release notes
- `docs/PLAN.md` — roadmap for future milestones
- `docs/TODO.md` — active checklist and in-flight items
- `docs/SECURITYAUDIT.md` — security notes and dependency review
- `docs/ai-journal/` — handoff and historical journal notes

## How to run

From the app directory:

```powershell
cd app
npm install
npm run tauri dev
```

## Desktop app behaviour

- Steam URLs and App IDs are accepted directly in the desktop UI
- Trailer search uses the public Steam API and Steam metadata
- Downloaded media is assembled to MP4 with ffmpeg
- ffmpeg installation and repair flows are managed inside the app on Windows
- Local history and settings persist outside the install directory

## Relevant roadmap

The project is still in active feature evolution, but the shipped path is now the
Tauri desktop app, not the retired standalone PowerShell script.

Future work is tracked in:

- `docs/PLAN.md`
- `docs/TODO.md`
- `docs/FEATURES.md`
- `CHANGELOG.md`

## Documentation standards

Public-facing documentation should remain aligned with the current desktop app,
not with retired helper scripts or earlier beta-era assumptions.
