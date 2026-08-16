# download_steam_trailers.ps1
# Downloads all trailers for a Steam game as local MP4 files.
# Requires: PowerShell 7+, ffmpeg on PATH (or in a common location)
#
# Usage:
#   .\download_steam_trailers.ps1 -AppId 3041230
#   .\download_steam_trailers.ps1 -AppId 3041230 -OutputDir .\images
#   .\download_steam_trailers.ps1 -AppId 275850 -OutputDir .\NoMansSky_trailers -Limit 3

#Requires -Version 7.0

param(
    [Parameter(Mandatory, Position = 0, HelpMessage = "Steam App ID (from the store page URL)")]
    [string]$AppId,

    [Parameter(Position = 1)]
    [string]$OutputDir = ".\trailers",

    [Parameter(HelpMessage = "Only download the N most recent trailers (Steam API is newest-first)")]
    [int]$Latest = 0,

    [Parameter(HelpMessage = "Only download the N oldest trailers")]
    [int]$Oldest = 0,

    [Parameter(HelpMessage = "List trailers without downloading")]
    [switch]$ListOnly
)

# ── ffmpeg detection ──────────────────────────────────────────────────────────

$ffmpeg = Get-Command ffmpeg -ErrorAction SilentlyContinue |
          Select-Object -ExpandProperty Source

if (-not $ffmpeg) {
    # Probe common install locations not always on PATH
    $candidates = @(
        "F:\ffmpeg\bin\ffmpeg.exe"
        "C:\ffmpeg\bin\ffmpeg.exe"
        "D:\ffmpeg\bin\ffmpeg.exe"
        "E:\ffmpeg\bin\ffmpeg.exe"
        "C:\Program Files\ffmpeg\bin\ffmpeg.exe"
        "C:\tools\ffmpeg\bin\ffmpeg.exe"
    )
    $ffmpeg = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
}

if (-not $ffmpeg) {
    Write-Host ""
    Write-Host "ERROR: ffmpeg not found." -ForegroundColor Red
    Write-Host ""
    Write-Host "Install ffmpeg and ensure its \bin folder is on your PATH, or place it in:"
    Write-Host "  F:\ffmpeg\bin\ffmpeg.exe"
    Write-Host ""
    Write-Host "Install options:"
    Write-Host "  winget install ffmpeg"
    Write-Host "  scoop install ffmpeg"
    Write-Host "  choco install ffmpeg"
    exit 1
}

# ── Header ────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "Steam Trailer Downloader" -ForegroundColor Cyan
Write-Host "────────────────────────"
Write-Host "App ID     : $AppId"
Write-Host "Output dir : $OutputDir"
Write-Host "ffmpeg     : $ffmpeg"
if ($Latest -gt 0) { Write-Host "Filter     : $Latest most recent" }
if ($Oldest -gt 0) { Write-Host "Filter     : $Oldest oldest" }
if ($ListOnly)     { Write-Host "Mode       : list only (no download)" -ForegroundColor Yellow }
Write-Host ""

# ── Steam API query ───────────────────────────────────────────────────────────
# The birthtime cookie bypasses the age gate for mature-rated titles.

Write-Host "Querying Steam API..." -ForegroundColor Gray

$headers = @{
    Cookie     = "birthtime=757382401; lastagecheckage=1-0-1994; mature_content=1"
    "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
}

try {
    $response = Invoke-RestMethod `
        -Uri "https://store.steampowered.com/api/appdetails?appids=$AppId&cc=us&l=english" `
        -Headers $headers `
        -ErrorAction Stop
} catch {
    Write-Host "ERROR: Could not reach Steam API. Check your internet connection." -ForegroundColor Red
    Write-Host $_.Exception.Message
    exit 1
}

$appData = $response.$AppId

if (-not $appData.success) {
    Write-Host "ERROR: Steam returned no data for App ID $AppId." -ForegroundColor Red
    Write-Host "The app may not exist, be region-locked, or not yet released."
    exit 1
}

$movies = $appData.data.movies
if (-not $movies -or $movies.Count -eq 0) {
    Write-Host "No trailers found for App ID $AppId." -ForegroundColor Yellow
    exit 0
}

# Apply ordering filter — Steam API is always newest-first
if ($Oldest -gt 0) {
    $movies = $movies | Select-Object -Last $Oldest
} elseif ($Latest -gt 0) {
    $movies = $movies | Select-Object -First $Latest
}

# ── Trailer list ──────────────────────────────────────────────────────────────

Write-Host "Found $($movies.Count) trailer(s) — newest first:" -ForegroundColor Green
$i = 1
foreach ($m in $movies) {
    Write-Host ("  {0,2}. {1}" -f $i, $m.name)
    $i++
}
Write-Host ""

if ($ListOnly) { exit 0 }

# ── Interactive selection (when no filter flags were passed) ──────────────────

if ($Latest -eq 0 -and $Oldest -eq 0) {
    Write-Host "What would you like to download?" -ForegroundColor Yellow
    Write-Host "  [A]  All $($movies.Count) trailers"
    Write-Host "  [1]  Latest only  (#1 — $($movies[0].name))"
    Write-Host "  [#]  Enter a number from the list above (e.g. 3)"
    Write-Host ""

    do {
        $choice = (Read-Host "Your choice").Trim()

        if ($choice -eq 'A' -or $choice -eq 'a') {
            # keep all — no filter needed
            break
        } elseif ($choice -eq '1') {
            $movies = @($movies[0])
            break
        } elseif ($choice -match '^\d+$') {
            $idx = [int]$choice
            if ($idx -ge 1 -and $idx -le $movies.Count) {
                $movies = @($movies[$idx - 1])
                break
            } else {
                Write-Host "  Please enter a number between 1 and $($movies.Count)." -ForegroundColor Red
            }
        } else {
            Write-Host "  Invalid input. Type A, 1, or a number from the list." -ForegroundColor Red
        }
    } while ($true)

    Write-Host ""
    Write-Host "Downloading $($movies.Count) trailer(s)..." -ForegroundColor Cyan
    Write-Host ""
}

# ── Download ──────────────────────────────────────────────────────────────────

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$success = 0
$skipped = 0
$failed  = 0

foreach ($movie in $movies) {
    $url = $movie.dash_h264

    if (-not $url) {
        Write-Host "[SKIP] $($movie.name) — no dash_h264 URL available" -ForegroundColor Yellow
        $skipped++
        continue
    }

    # Build a safe filename from the trailer name
    $safeName = $movie.name `
        -replace '[^\w\s-]', '' `
        -replace '\s+', '_' `
        -replace '_+', '_'
    $safeName = $safeName.ToLower().Trim('_')
    $outFile  = Join-Path $OutputDir "$safeName.mp4"

    if (Test-Path $outFile) {
        Write-Host "[SKIP] $($movie.name) — already exists" -ForegroundColor DarkGray
        $skipped++
        continue
    }

    Write-Host "[DOWN] $($movie.name)" -ForegroundColor Cyan
    Write-Host "       $url" -ForegroundColor DarkGray

    $ffmpegArgs = @("-i", $url, "-c", "copy", $outFile, "-y", "-loglevel", "error", "-stats")

    try {
        # ffmpeg's DASH demuxer logs a benign "Error when loading first fragment
        # of playlist" line on some segment retries even when the download
        # completes fine (exit code 0, valid output file) — filter just that
        # known-harmless line so it doesn't read as a failure. Anything else on
        # stderr still prints, and $LASTEXITCODE still gates success/failure below.
        & $ffmpeg @ffmpegArgs 2>&1 | ForEach-Object {
            $line = $_.ToString()
            if ($line -notmatch 'Error when loading first fragment of playlist') {
                Write-Host $line
            }
        }
        if ($LASTEXITCODE -eq 0) {
            $size = (Get-Item $outFile).Length / 1MB
            Write-Host "[OK]   Saved -> $outFile  ($([math]::Round($size, 1)) MB)" -ForegroundColor Green
            $success++
        } else {
            throw "ffmpeg exited with code $LASTEXITCODE"
        }
    } catch {
        Write-Host "[FAIL] $($movie.name) — $($_.Exception.Message)" -ForegroundColor Red
        Remove-Item $outFile -ErrorAction SilentlyContinue
        $failed++
    }

    Write-Host ""
}

# ── Summary ───────────────────────────────────────────────────────────────────

Write-Host "────────────────────────"
Write-Host "Downloaded : $success"
if ($skipped -gt 0) { Write-Host "Skipped    : $skipped" -ForegroundColor DarkGray }
if ($failed  -gt 0) { Write-Host "Failed     : $failed"  -ForegroundColor Red }
Write-Host ""
