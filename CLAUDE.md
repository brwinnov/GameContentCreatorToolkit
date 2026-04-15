# GGT — Grab Game Trailer

## Project purpose
PowerShell 7 tool to download Steam game trailers as local MP4 files.
Queries the Steam API directly — no browser DevTools, no manual URL hunting.

## Key files
- `download_steam_trailers.ps1` — primary script (PowerShell 7, use this)
- `download_steam_trailers.sh` — bash version (kept for reference, has Windows PATH quirks)
- `README.md` — full usage documentation
- `STEAM_TRAILER_DOWNLOAD.md` — background notes on Steam's CDN and the blob URL problem

## How to run
```powershell
.\download_steam_trailers.ps1 -AppId <STEAM_APP_ID> -OutputDir .\output
```

## ffmpeg location on this machine
`F:\ffmpeg\bin\ffmpeg.exe` — already hardcoded in the candidate probe list.
yt-dlp is also at `F:\ffmpeg\bin\yt-dlp.exe`.

## Steam API behaviour
- Endpoint: `https://store.steampowered.com/api/appdetails?appids=<ID>&cc=us&l=english`
- Age gate: bypassed via `birthtime=757382401; lastagecheckage=1-0-1994; mature_content=1` cookies
- Trailers returned under `data.movies[]`, always **newest-first**
- Each trailer has `dash_h264` (DASH manifest URL) — Steam no longer serves direct .webm/.mp4
- ffmpeg reassembles DASH segments into a single MP4 with `-c copy` (no re-encode)

## Why NOT to scrape the store page HTML
The `<video src="blob:https://...">` URLs in DevTools are ephemeral browser Blob URLs.
They cannot be downloaded externally. Always use the Steam API instead.

## Known gotcha — bash script on Windows
When `bash` is invoked from PowerShell, it does not always inherit the full
Windows PATH. The `.sh` script works around this with `cmd //c where ffmpeg`
but the `.ps1` script has no such issue — use it instead.

## Script parameters
| Flag         | Default      | Purpose                                              |
|--------------|--------------|------------------------------------------------------|
| `-AppId`     | required     | Steam App ID from store URL                          |
| `-OutputDir` | `.\trailers` | Where to save MP4s                                   |
| `-Latest N`  | 0 (all)      | Download N most recent trailers                      |
| `-Oldest N`  | 0 (all)      | Download N oldest trailers                           |
| `-ListOnly`  | off          | Print numbered list, no download                     |

No flags = interactive mode: lists trailers then prompts to pick All / Latest / by number.
