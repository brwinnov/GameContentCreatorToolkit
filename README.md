# Game Content Creator Toolkit

A toolkit for game-review content creators: pull official media assets
(trailers, screenshots, key art) straight from a game's store/social pages,
for use as press-kit source material alongside your own gameplay recording.

**Status:** BETA — the Tauri desktop Steam downloader and legacy PowerShell
script work; additional media sources remain planned. See
[`docs/PLAN.md`](docs/PLAN.md) for the full roadmap and
[`docs/TODO.md`](docs/TODO.md) for the
active checklist.

**Current release target:** one Windows MSI plus Linux `.deb` and `.rpm`
packages. macOS is intentionally deferred while Windows/Linux stabilize.

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

Full detail in [`docs/PLAN.md`](docs/PLAN.md), short version:

1. Harden the Steam script (batch mode, screenshots/art, metadata sidecars)
2. Complete packaged Windows/Linux smoke tests and Windows code signing
3. Add screenshots/key art, metadata sidecars, and batch workflows
4. Add YouTube, TikTok, Instagram, and Facebook downloader modules via yt-dlp
5. Revisit macOS packaging in a later milestone once the Windows/Linux release
   path is stabilized

## Release workflow

The repo includes GitHub Actions workflows that build release artifacts for the
current supported platforms:

- Windows: MSI installer
- Linux: `.deb` and `.rpm` packages

See:

- [`.github/workflows/tauri-release.yml`](.github/workflows/tauri-release.yml)
- [`.github/workflows/tauri-release-publish.yml`](.github/workflows/tauri-release-publish.yml)

The build runs in GitHub-hosted runners, so the Linux packages are produced in an
Ubuntu environment instead of a local WSL2 setup.

## Project docs

- [`docs/DOWNLOADS.md`](docs/DOWNLOADS.md) — downloads and verification
- [`.signpath/README.md`](.signpath/README.md) — SignPath setup and templates
- [`docs/PLAN.md`](docs/PLAN.md) — full roadmap
- [`docs/FEATURES.md`](docs/FEATURES.md) — feature backlog and next-step ideas
- [`docs/TODO.md`](docs/TODO.md) — active checklist
- [`docs/SECURITYAUDIT.md`](docs/SECURITYAUDIT.md) — security audit and tasks
- [`CHANGELOG.md`](CHANGELOG.md) — what's shipped
- [`docs/repo-update.md`](docs/repo-update.md) — application auto-update plan
- [`docs/PRIVACY.md`](docs/PRIVACY.md) — data and network-request policy
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — contribution workflow
- [`SECURITY.md`](SECURITY.md) — private vulnerability reporting policy
- [`AGENTS.md`](AGENTS.md) — model-neutral repository instructions
- [`docs/ai-journal/`](docs/ai-journal/) — canonical pre-task context and
   post-task AI handoff journal
- [`ai-journal/`](ai-journal/) — historical AI-assisted work-session records

## Code signing policy

The project intends to use SignPath Foundation after its application and build
configuration are accepted. Free code signing provided by
[SignPath.io](https://about.signpath.io/), certificate by
[SignPath Foundation](https://signpath.org/). Releases are built from this
repository and each signing request requires manual approval.

- Committer and reviewer: [Barry Reilly (@brwinnov)](https://github.com/brwinnov)
- Approver: [Barry Reilly (@brwinnov)](https://github.com/brwinnov)
- Full policy: [`docs/CODE_SIGNING_POLICY.md`](docs/CODE_SIGNING_POLICY.md)
- Privacy policy: [`docs/PRIVACY.md`](docs/PRIVACY.md)

## License

Game Content Creator Toolkit is licensed under the [MIT License](LICENSE).
