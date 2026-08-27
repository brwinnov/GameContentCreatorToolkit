# Game Content Creator Toolkit

A toolkit for game-review content creators: pull official media assets
(trailers, screenshots, key art) straight from a game's store/social pages,
for use as press-kit source material alongside your own gameplay recording.

**Status:** BETA — one working piece so far, more planned. See
[`PLAN.md`](PLAN.md) for the full roadmap and [`TODO.md`](TODO.md) for the
active checklist.

**Current release target:** Windows desktop builds plus Linux `.deb` and `.rpm`
packages. macOS is intentionally deferred for now while we stabilize the first
productive Windows/Linux release set.

---

## What works today: Steam Trailer Downloader

Downloads all trailers for any Steam game as local MP4 files, straight from
Steam's API — no browser, no DevTools, no manual URL hunting.

**Requires:** PowerShell 7+, ffmpeg

### Setup

1. **ffmpeg** — install via `winget install ffmpeg` / `scoop install ffmpeg` /
   `choco install ffmpeg` / `brew install ffmpeg`, or the script will look in
   a few common install paths automatically.
2. **Execution policy** (first run only, if PowerShell blocks the script):

   ```powershell
   Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
   ```

3. **Find the Steam App ID** — it's the number in the store URL:
   `https://store.steampowered.com/app/3041230/Windrose/` → App ID `3041230`

### Usage

```powershell
.\scripts\pwsh\download_steam_trailers.ps1 -AppId <ID> [options]
```

| Parameter    | Type   | Default      | Description                                |
|--------------|--------|--------------|----------------------------------------------|
| `-AppId`     | string | *(required)* | Steam App ID from the store URL             |
| `-OutputDir` | string | `.\trailers` | Folder to save MP4 files into               |
| `-Latest`    | int    | 0 (all)      | Download only the N most recent trailers    |
| `-Oldest`    | int    | 0 (all)      | Download only the N oldest trailers         |
| `-ListOnly`  | switch | off          | Print the trailer list and exit — no download |

No `-Latest`/`-Oldest` flag → interactive mode: lists trailers, then prompts
`[A]` all / `[1]` latest / `[#]` pick one.

Examples:

```powershell
# See what's available first
.\scripts\pwsh\download_steam_trailers.ps1 -AppId 275850 -ListOnly

# Grab everything
.\scripts\pwsh\download_steam_trailers.ps1 -AppId 3041230 -OutputDir .\images

# Just the newest one
.\scripts\pwsh\download_steam_trailers.ps1 -AppId 275850 -Latest 1 -OutputDir .\NMS_trailers
```

Already-downloaded files are skipped automatically — safe to re-run after an
interruption.

### How it works (short version)

Steam no longer serves direct `.mp4`/`.webm` trailer files — everything's a
DASH stream now. The script queries the public `appdetails` API (with
age-gate cookies so 18+ titles work without a Steam login), gets each
trailer's `dash_h264` manifest, and has ffmpeg reassemble the chunks into a
single MP4 with `-c copy` (no re-encoding, so it's fast and lossless).

---

## What's next

Full detail in [`PLAN.md`](PLAN.md), short version:

1. Harden the Steam script (batch mode, screenshots/art, metadata sidecars)
2. Wrap it in a real desktop app for the current release targets: Tauri on
   Windows + Linux (`.deb` / `.rpm`) packages
3. Paste-a-URL UI: detect available media, choose All/Latest/pick-some
4. Downloads default to the OS Downloads folder, overridable in Settings
5. Add YouTube, TikTok, Instagram, and Facebook downloader modules (via yt-dlp)
6. Revisit macOS packaging in a later milestone once the Windows/Linux release
   path is stabilized

## Release workflow

The repo includes GitHub Actions workflows that build release artifacts for the
current supported platforms:

- Windows: MSI and NSIS installer bundles
- Linux: `.deb` and `.rpm` packages

See:

- [`.github/workflows/tauri-release.yml`](.github/workflows/tauri-release.yml)
- [`.github/workflows/tauri-release-publish.yml`](.github/workflows/tauri-release-publish.yml)

The build runs in GitHub-hosted runners, so the Linux packages are produced in an
Ubuntu environment instead of a local WSL2 setup.

## Project docs

- [`PLAN.md`](PLAN.md) — full roadmap
- [`FEATURES.md`](FEATURES.md) — reviewable feature backlog and next-step ideas
- [`TODO.md`](TODO.md) — active checklist
- [`SECURITYAUDIT.md`](SECURITYAUDIT.md) — what data/credentials this project touches
- [`CHANGELOG.md`](CHANGELOG.md) — what's shipped
- [`repo-update.md`](repo-update.md) — signed application auto-update plan
- [`CLAUDE.md`](CLAUDE.md) — project context for AI coding assistants
- [`ai-journal/`](ai-journal/) — running log of AI-assisted work sessions
