# 2026-08-16 — release hardening and game-name search

## Context

The app was working in a test build but had two obvious release issues: Windows ffmpeg/ffprobe launches were flashing black console windows, and the Steam search flow only accepted a raw App ID or store URL. We wanted a shareable Windows build that behaved like a proper desktop app and also supported search by game name.

## Decisions made

- Keep the app in a Tauri GUI architecture; do not move to a shell-based launcher or a different app framework.
- Hide the child ffmpeg/ffprobe console windows on Windows while still capturing their stdout/stderr for progress and diagnostics.
- Preserve the existing App ID / full URL workflow, but add a fallback that searches Steam by game name when the input is not a direct app reference.
- Treat release packaging as a repeatable artifact flow: keep both a portable EXE and installable MSI/NSIS bundles in a versioned share folder.

## What changed

- Updated the Rust subprocess launch logic so Windows launches do not open visible console windows.
- Kept in-app progress reporting and error capture in the desktop UI rather than relying on terminal output.
- Added a Steam store search command that resolves candidate games by name and lets the user choose the matching app.
- Updated the UI to show a small match list beneath the Steam input before the trailer fetch runs.
- Created a versioned release folder containing the portable EXE and installer copies for 0.1.0.
- Documented the release/build status in `CHANGELOG.md` and `TODO.md`.

## Open questions / follow-ups

- Need a real external smoke test in Windows Sandbox or on a clean Windows 11 machine to validate the installer + app launch path end-to-end.
- Need to decide whether release naming should remain `ggc-app-0.1.0` or whether we want a more conventional `GameContentCreatorToolkit-0.1.0-win64` pattern.
- Next likely hardening phase: batch download defaults, metadata sidecars, or screenshot/export helpers.
