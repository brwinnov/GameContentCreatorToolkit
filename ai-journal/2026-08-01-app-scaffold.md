# 2026-08-01 — Desktop app scaffold (Tauri v2)

## Context

Repo scaffold + roadmap were already in place (see the earlier journal entry
today). Local `initial/` script confirmed identical to the repo version, no
merge needed. Moved on to actually building the app: real UI with a Steam
tab (wired to working logic) plus placeholder tabs for YouTube, TikTok, and
Instagram.

## Decisions made

- **Design direction**: dark, monospace-leaning UI deliberately modeled on
  the user's own PowerShell terminal output (cyan/green/amber/red status
  colors, `[DOWN]`/`[OK]`/`[SKIP]`/`[FAIL]` console lines) rather than a
  generic SaaS dashboard look — grounded in how the user already interacts
  with this tool daily.
- **Steam API logic ported from PowerShell to Rust**, not shelled out to
  `pwsh`. Reasoning: the app's whole point is cross-platform (Win/macOS/
  Linux), and depending on PowerShell 7 being installed on every OS adds an
  unnecessary runtime dependency when the logic itself (HTTP GET + JSON
  parse + spawn ffmpeg) is simple enough to port directly.
- **Folder picker uses the Tauri dialog plugin's JS API directly**
  (`window.__TAURI__.dialog.open({ directory: true })`) rather than a custom
  Rust command wrapping it — simpler, and it's exactly what the plugin is
  for.
- Verified Tauri v2 API shapes (command/event syntax, `tauri.conf.json`
  schema, dialog plugin usage) against current docs at v2.tauri.app before
  writing code, since v1→v2 changed significantly and stale v1 syntax would
  not have compiled.

## What changed

- Added `app/` — full Tauri v2 project scaffold (`src/` frontend: plain
  HTML/CSS/JS, no bundler; `src-tauri/` Rust backend)
- Frontend also works standalone in a browser (double-click `src/index.html`)
  via a mock Tauri API layer — lets the UI be previewed/reviewed without
  installing Rust
- Placeholder icons generated (png set + `.ico`); `.icns` for macOS not yet
  generated — needs `tauri icon` run against a real logo later

## Open questions / follow-ups

- **The Rust code has not been compiled.** No Rust toolchain was available
  in the session that wrote it — this is unverified beyond matching current
  Tauri v2 documentation. First real next step is `npm install && npm run
  tauri dev` in `app/` and working through whatever build errors surface.
- Icon set is a crude placeholder (solid "G" mark) — fine for development,
  needs a real logo before any distribution.
- No batching/retry/metadata-sidecar features yet (Phase 1 items )— the
  Rust port is a straight 1:1 of the current PowerShell script's behavior,
  nothing more.
