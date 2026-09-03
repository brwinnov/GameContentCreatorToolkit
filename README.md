# Game Content Creator Toolkit

A Tauri desktop app for game-review content creators who want to pull official
Steam trailer media into local MP4 files without hunting through browser DevTools
or copy-pasting ephemeral blob URLs.

The current app is a working desktop workflow for Steam trailer downloads, with
ffmpeg management and persistent user settings built into the UI. The 0.1.6
release includes the desktop app plus the current Linux DEB/RPM build path, while
Windows remains the primary validation target for day-to-day installer testing.

## What the app does now

- Download official Steam trailer listings from Steam's public API
- Paste a Steam URL or App ID directly into the desktop app
- Save trailers as local MP4 files in a chosen output folder
- Detect and manage ffmpeg/ffprobe, including a verified managed install path
  on Windows
- Store recent Steam history and output-location preferences locally
- Provide a Settings view for ffmpeg status, install/update actions, and theme
  customisation
- Keep the desktop UI aligned with the current app version and release metadata

This project is not only a script anymore; the main experience is now the
Tauri desktop application in the `app/` folder.

## Current status

Current release: `0.1.6`

The shipped desktop experience includes:

- Steam trailer search and download flow
- Steam history and durable local settings
- ffmpeg install, repair, and update handling in Settings
- Windows-friendly media-engine detection and fallback behavior
- Tauri desktop layout with Home, Steam, and Settings views

Planned areas for later milestones include additional media sources such as
screenshots/key art and other social platforms, but the core Steam trailer
workflow is the primary implemented feature today.

## Quick start

### Prerequisites

- Node.js LTS
- Rust + Cargo
- On Windows: WebView2 and the Visual Studio C++ build tools if your machine is
  missing the default dependencies for Tauri

### Run the desktop app in development

From the `app/` directory:

```powershell
npm install
npm run tauri dev
```

This launches the real desktop app with the Rust backend wired up.

### Build a desktop bundle

```powershell
cd app
npm run tauri build
```

Outputs are generated under the Tauri bundle directories for the selected target.
The project currently ships the Windows MSI and validates the Linux DEB/RPM
packages as part of the same release set, while the Tauri app remains the
authoritative project path.

## Project structure

```text
.
├── app/
│   ├── package.json
│   ├── src/
│   └── src-tauri/
├── scripts/
│   ├── bash/
│   └── pwsh/
├── docs/
├── CHANGELOG.md
├── AGENTS.md
├── CLAUDE.md
├── README.md
├── SECURITYAUDIT.md
├── PLAN.md
├── TODO.md
└── LICENSE
```

## Documentation

- `CHANGELOG.md` — release notes and shipped changes
- `docs/PLAN.md` — roadmap and future milestones
- `docs/TODO.md` — current active work
- `docs/SECURITYAUDIT.md` — security notes and audit findings
- `docs/ai-journal/` — project handoff notes and AI session context
- `app/README.md` — app-level setup and development notes

## Release notes and packaging

This repository includes a tag-driven release flow for shipping the desktop app
artifacts. The current app version is kept in sync across the Tauri project and
frontend metadata, and each release is published with GitHub release notes.

Releases are unsigned by policy — see
[`docs/CODE_SIGNING_POLICY.md`](docs/CODE_SIGNING_POLICY.md). The Linux
DEB/RPM release path remains in active testing as part of the current release
set, while the project continues to validate the Windows 0.1.6 installer and
the published desktop workflow.

## License

This project is licensed under the [MIT License](LICENSE).
