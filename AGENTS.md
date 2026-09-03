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
- Current packaging: Windows MSI plus Linux DEB/RPM are built; macOS remains deferred
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

## AI task handoff

All AI assistants, regardless of vendor or model, must follow
docs/ai-journal/README.md and read `docs/ai-journal/PRE-TASK.md` before
editing. After validation, add a concise POST-TASK journal entry and refresh the
pre-task snapshot whenever status, priorities, constraints, or the next action
changes.

## How to run

From the app directory:

```powershell
cd app
npm install
npm run tauri dev
```

## Steam API behaviour

- Endpoint: `https://store.steampowered.com/api/appdetails?appids=<ID>&cc=us&l=english`
- Age-gate bypass cookie string: `birthtime=757382401; lastagecheckage=1-0-1994; mature_content=1`
- Trailers arrive under `data.movies[]` and are newest-first
- Each trailer exposes a `dash_h264` manifest; the app remuxes the stream to MP4 with `ffmpeg -c copy`
- Do not scrape the store page HTML or ephemeral browser Blob URLs; those blob URLs are not reusable outside the browser context

## ffmpeg

- Managed install location: `%LOCALAPPDATA%\com.ackrosgaming.gcc\tools\ffmpeg\bin`
- `settings.json` `ffmpegPath` means a user-chosen custom path only
- `managedArchiveSha256` drives the managed-install update check
- Local Windows probe candidate: `F:\ffmpeg\bin\ffmpeg.exe`

## Validation baseline

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `node --check app/src/main.js`
- `npx markdownlint-cli2 $(git ls-files '*.md')`

## Language

All user-facing text (UI labels, status messages, errors, docs) uses English
(Ireland) spelling: `colour`, `customise`, `initialise`, `organise`, `centre`.
Code identifiers, CSS properties, and third-party API names keep their original
spelling.
