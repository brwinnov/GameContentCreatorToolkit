# Steam Trailer Downloader

Downloads all trailers for any Steam game as local MP4 files.
Queries the Steam API directly — no browser, no DevTools, no manual URL hunting.

**Requires:** PowerShell 7+, ffmpeg

---

## Setup

### 1. ffmpeg
The script auto-detects ffmpeg in these locations (in order):
- Anywhere on your Windows `PATH`
- `F:\ffmpeg\bin\ffmpeg.exe`
- `C:\ffmpeg\bin\ffmpeg.exe`
- `D:\ffmpeg\bin\ffmpeg.exe`
- `E:\ffmpeg\bin\ffmpeg.exe`
- `C:\Program Files\ffmpeg\bin\ffmpeg.exe`
- `C:\tools\ffmpeg\bin\ffmpeg.exe`

If ffmpeg isn't found the script will tell you and exit cleanly.

### 2. Execution policy (first time only)
If PowerShell blocks the script, run this once:
```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

### 3. Find the Steam App ID
It's in the store page URL:
```
https://store.steampowered.com/app/3041230/Windrose/
                                   ↑↑↑↑↑↑↑
                                   App ID
```

---

## Usage

```powershell
.\download_steam_trailers.ps1 -AppId <ID> [options]
```

---

## Examples

### See what trailers exist before downloading anything
```powershell
.\download_steam_trailers.ps1 -AppId 275850 -ListOnly
```
Prints a numbered list, newest first. No files downloaded.

```
Found 43 trailer(s) — newest first:
   1. No Man's Sky Xeno Arena Trailer
   2. No Man's Sky Remnant Trailer
   3. No Man's Sky Breach Trailer
   ...
  43. No Man's Sky: The Abyss
```

---

### Interactive mode (default — no filter flags)
```powershell
.\download_steam_trailers.ps1 -AppId 275850 -OutputDir .\NMS_trailers
```
Lists all trailers then prompts:
```
What would you like to download?
  [A]  All 43 trailers
  [1]  Latest only  (#1 — No Man's Sky Xeno Arena Trailer)
  [#]  Enter a number from the list above (e.g. 3)

Your choice: _
```
- Type `A` to download everything
- Type `1` to download only the most recent trailer
- Type any number (e.g. `5`) to download that specific trailer from the list

---

### Download all trailers
```powershell
.\download_steam_trailers.ps1 -AppId 3041230 -OutputDir .\images
```
Downloads every trailer for the app into the specified folder.

---

### Download the N most recent trailers
```powershell
# Most recent 1
.\download_steam_trailers.ps1 -AppId 275850 -Latest 1 -OutputDir .\NMS_trailers

# Most recent 3
.\download_steam_trailers.ps1 -AppId 275850 -Latest 3 -OutputDir .\NMS_trailers
```
Steam's API always returns trailers newest-first, so `-Latest 1` is always the
most recently published trailer.

---

### Download the N oldest trailers
```powershell
.\download_steam_trailers.ps1 -AppId 275850 -Oldest 3 -OutputDir .\NMS_trailers
```

---

### Custom output folder
```powershell
.\download_steam_trailers.ps1 -AppId 3041230 -OutputDir G:\Presskits\Windrose\video
```
The folder is created automatically if it doesn't exist.

---

## Re-running safely

Already-downloaded files are skipped automatically — the script checks for the
output file before starting each download. Safe to re-run if a download was
interrupted.

---

## Parameters

| Parameter    | Type   | Default      | Description                                              |
|--------------|--------|--------------|----------------------------------------------------------|
| `-AppId`     | string | *(required)* | Steam App ID from the store URL                          |
| `-OutputDir` | string | `.\trailers` | Folder to save MP4 files into                            |
| `-Latest`    | int    | 0 (all)      | Download only the N most recent trailers                 |
| `-Oldest`    | int    | 0 (all)      | Download only the N oldest trailers                      |
| `-ListOnly`  | switch | off          | Print the trailer list and exit — no download            |

When neither `-Latest` nor `-Oldest` is set, the script enters **interactive mode**
and prompts for a selection after listing.

---

## How it works

### Why not use the page HTML source?

The `<video src="blob:https://...">` URLs visible in browser DevTools are
**ephemeral Blob URLs** — temporary in-memory references created by the browser.
They cannot be downloaded externally and die when the tab closes.

### Steam API

The script calls:
```
https://store.steampowered.com/api/appdetails?appids=<ID>&cc=us&l=english
```
with age-gate cookies (`birthtime`, `lastagecheckage`, `mature_content`) to
handle 18+ rated titles without requiring a Steam login.

The API returns each trailer's `dash_h264` URL — a DASH streaming manifest
(`.mpd` file) pointing to the video segments on Steam's Akamai CDN.

### Why DASH, not a direct .mp4?

Steam no longer serves direct `.webm` or `.mp4` trailer files. All trailers are
now delivered as **DASH streams** (Dynamic Adaptive Streaming over HTTP), where
the video is split into small chunks served from a manifest. ffmpeg reassembles
these chunks into a single `.mp4` with `-c copy` (no re-encoding).

Output is always **1080p H.264 + AAC**, compatible with all editors and platforms.

### Age-gate

Steam sets browser cookies when a user confirms their age on a store page. The
script sends equivalent cookies with every API request, bypassing the gate for
mature-rated titles without requiring a Steam account or login.

---

## Tested games

| Game | App ID | Trailers |
|------|--------|----------|
| Windrose | 3041230 | 2 |
| No Man's Sky | 275850 | 43 |
