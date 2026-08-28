# 2026-08-01 — Repo scaffold + roadmap session

## Context

Project had a working PowerShell 7 Steam-trailer downloader in the
`brwinnov/GameContentCreatorToolkit` GitHub repo, plus a possibly-newer local
copy in an `initial/` folder that had never been reconciled with the repo.
Goal for this session: set up `G:\6-GameTools\ContentCreatorToolkit\workspace`
as the new VS Code project root synced to the repo, and put real planning/
governance docs in place before building further.

## Decisions made

- **App stack: Tauri** (Rust shell + web-tech UI) for the eventual
  cross-platform desktop app, chosen over Electron and a Python/Qt app mainly
  for native installer size and the built-in "sidecar" mechanism for calling
  external binaries (ffmpeg now, yt-dlp later) as subprocesses.
- **Social platform downloads (Phase 3) will route through yt-dlp** rather
  than hand-rolled scrapers per platform (YouTube/TikTok/Facebook/Instagram).
- **Repo root docs consolidated**: `docs/README.md` and `docs/PLAN.md` were
  duplicates/precursors of what's now at repo root (`README.md`, `PLAN.md`);
  removed to avoid drift. `docs/CLAUDE.md` moved to repo-root `CLAUDE.md`
  (standard location many AI coding tools, including Claude Code, look for
  automatically).

## What changed

- Added root-level `PLAN.md`, `TODO.md`, `SECURITYAUDIT.md`, `CHANGELOG.md`
- Added `ai-journal/` (this folder) and `logs/` (gitignored, `.gitkeep`d)
- Added `.gitignore` and `.env.example` (project had neither before)
- Confirmed `scripts/pwsh/download_steam_trailers.ps1` and
  `toolkit/Steam-Trailer-Downloader.ps1` are intentionally identical
  (the latter is a build output of `build_toolkit.ps1`, not a fork)

## Open questions / follow-ups

- Repo was made public temporarily to allow this session to fetch it (no
  authenticated GitHub access is possible in the AI sandbox). Flip back to
  private if that was the intended default state.

**Update (same day, follow-up):** the `initial/` vs repo script diff was done
— confirmed byte-identical (matching MD5). No merge was needed; `initial/`
can be archived/deleted locally.
