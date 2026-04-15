#!/usr/bin/env bash
# download_steam_trailers.sh
# Downloads all trailers for a Steam game as local MP4 files.
#
# Usage:
#   ./download_steam_trailers.sh <APP_ID> [output_dir]
#
# Examples:
#   ./download_steam_trailers.sh 3041230
#   ./download_steam_trailers.sh 3041230 ./images

set -euo pipefail

# ── Dependency check ──────────────────────────────────────────────────────────
# Find ffmpeg: check bash PATH, then fall back to Windows 'where' (which sees
# the full Windows PATH including drives like F:\ffmpeg\bin).

FFMPEG_CMD=""

if command -v ffmpeg &>/dev/null; then
  FFMPEG_CMD="ffmpeg"
else
  # Ask Windows where.exe — it resolves the real Windows PATH correctly
  WIN_PATH=$(cmd //c where ffmpeg 2>/dev/null | head -1 | tr -d '\r\n') || true
  if [ -n "$WIN_PATH" ]; then
    FFMPEG_CMD="$WIN_PATH"
  fi
fi

if [ -z "$FFMPEG_CMD" ]; then
  echo ""
  echo "ERROR: ffmpeg not found."
  echo ""
  echo "ffmpeg must be installed and on your Windows PATH."
  echo "Your ffmpeg is in F:\\ffmpeg\\bin — add that to System Environment Variables > PATH."
  echo ""
  echo "  Or prefix the command:"
  echo "    FFMPEG_CMD=/f/ffmpeg/bin/ffmpeg.exe bash download_steam_trailers.sh <APP_ID>"
  exit 1
fi

# ── Arguments ────────────────────────────────────────────────────────────────

APP_ID="${1:?Error: Steam App ID required.  Usage: $0 <APP_ID> [output_dir]}"
OUTPUT_DIR="${2:-./trailers}"

# ── Age-gate bypass cookies ───────────────────────────────────────────────────
# Steam sets these when a user confirms their age on the store page.
# Passing them here prevents the API from returning a gated/empty response
# for mature-rated titles.

COOKIES="birthtime=757382401; lastagecheckage=1-0-1994; mature_content=1"

# ── Fetch app details ─────────────────────────────────────────────────────────

echo ""
echo "Steam Trailer Downloader"
echo "────────────────────────"
echo "App ID     : $APP_ID"
echo "Output dir : $OUTPUT_DIR"
echo ""
echo "Querying Steam API..."

API_RESPONSE=$(curl -s \
  --fail \
  -H "Cookie: $COOKIES" \
  -A "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" \
  "https://store.steampowered.com/api/appdetails?appids=${APP_ID}&cc=us&l=english") || {
    echo "Error: Could not reach Steam API. Check your internet connection."
    exit 1
  }

# ── Parse trailer list ────────────────────────────────────────────────────────

MOVIE_LIST=$(echo "$API_RESPONSE" | node -e "
const chunks = [];
process.stdin.on('data', d => chunks.push(d));
process.stdin.on('end', () => {
  let data;
  try { data = JSON.parse(chunks.join('')); }
  catch (e) { process.stderr.write('Error: Invalid API response (not JSON)\n'); process.exit(1); }

  const app = data['${APP_ID}'];

  if (!app) {
    process.stderr.write('Error: App ID ${APP_ID} not found in API response\n');
    process.exit(1);
  }

  if (!app.success) {
    process.stderr.write('Error: Steam returned success=false for App ID ${APP_ID}\n');
    process.stderr.write('This can happen for unreleased apps or region-locked titles.\n');
    process.exit(1);
  }

  const movies = app.data.movies || [];

  if (!movies.length) {
    process.stderr.write('No trailers found for App ID ${APP_ID}\n');
    process.exit(1);
  }

  movies.forEach(m => {
    // Sanitise name to a safe filename
    const safeName = m.name
      .replace(/[^\w\s-]/g, '')   // strip non-word chars except spaces/hyphens
      .replace(/\s+/g, '_')       // spaces → underscores
      .replace(/_+/g, '_')        // collapse multiple underscores
      .toLowerCase()
      .replace(/^_|_$/, '');      // trim leading/trailing underscores

    const url = m.dash_h264 || '';
    if (!url) {
      process.stderr.write('Warning: No dash_h264 URL for \"' + m.name + '\" — skipping\n');
      return;
    }

    console.log(safeName + '|' + m.name + '|' + url);
  });
});
") || exit 1

if [ -z "$MOVIE_LIST" ]; then
  echo "No downloadable trailers found."
  exit 1
fi

# ── Summary ───────────────────────────────────────────────────────────────────

TRAILER_COUNT=$(echo "$MOVIE_LIST" | wc -l | tr -d ' ')
echo "Found $TRAILER_COUNT trailer(s):"
echo "$MOVIE_LIST" | while IFS='|' read -r safe_name display_name url; do
  echo "  • $display_name  →  ${OUTPUT_DIR}/${safe_name}.mp4"
done
echo ""

# ── Download ──────────────────────────────────────────────────────────────────

mkdir -p "$OUTPUT_DIR"

SUCCESS=0
SKIPPED=0
FAILED=0

echo "$MOVIE_LIST" | while IFS='|' read -r safe_name display_name url; do
  OUT="${OUTPUT_DIR}/${safe_name}.mp4"

  if [ -f "$OUT" ]; then
    echo "[SKIP] $display_name (file already exists)"
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  echo "[DOWN] $display_name"
  echo "       $url"

  if "$FFMPEG_CMD" -i "$url" -c copy "$OUT" -y \
      -loglevel error \
      -stats \
      2>&1; then
    SIZE=$(du -sh "$OUT" 2>/dev/null | cut -f1)
    echo "[OK]   Saved → $OUT  ($SIZE)"
    SUCCESS=$((SUCCESS + 1))
  else
    echo "[FAIL] Download failed for: $display_name"
    rm -f "$OUT"   # remove partial file
    FAILED=$((FAILED + 1))
  fi

  echo ""
done

echo "────────────────────────"
echo "Done."
echo ""
