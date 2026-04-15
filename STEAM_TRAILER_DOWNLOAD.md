# Downloading Steam Trailers Locally

## What You Need
- The **Steam App ID** — found in the store page URL:
  `https://store.steampowered.com/app/3041230/Windrose/`
  → App ID is `3041230`
- `ffmpeg` installed and on your PATH
- `curl` (comes with Windows 10+)

---

## Step 1 — Get the trailer URLs from the Steam API

Run this in a terminal, replacing `3041230` with your App ID:

```bash
curl -s "https://store.steampowered.com/api/appdetails?appids=3041230" | node -e "
const chunks = [];
process.stdin.on('data', d => chunks.push(d));
process.stdin.on('end', () => {
  const movies = JSON.parse(chunks.join(''))[3041230].data.movies || [];
  movies.forEach((m, i) => console.log(i+1 + '.', m.name, '\n  ', m.dash_h264, '\n'));
});
"
```

This prints each trailer's name and its DASH streaming URL. Example output:

```
1. Windrose: Gameplay Trailer
   https://video.akamai.steamstatic.com/store_trailers/3041230/.../dash_h264.mpd?t=...

2. Windrose: Early Access Release Date Reveal Trailer
   https://video.akamai.steamstatic.com/store_trailers/3041230/.../dash_h264.mpd?t=...
```

---

## Step 2 — Download a trailer with ffmpeg

```bash
ffmpeg -i "<paste dash_h264 URL here>" -c copy "trailer_name.mp4" -y
```

### Real example (Windrose):

```bash
# Trailer 1 — Gameplay Trailer
ffmpeg -i "https://video.akamai.steamstatic.com/store_trailers/3041230/2012344070/6b0b773c076c6f25728af99a7af10b14639ec6bb/1776076852/dash_h264.mpd?t=1776152381" -c copy "images/gameplay_trailer.mp4" -y

# Trailer 2 — Early Access Reveal Trailer
ffmpeg -i "https://video.akamai.steamstatic.com/store_trailers/3041230/1708336673/38110294840935b7135c2dd1bc9f1cdf8ed5149b/1776077130/dash_h264.mpd?t=1776152383" -c copy "images/early_access_reveal_trailer.mp4" -y
```

`-c copy` means no re-encoding — fast download, original quality preserved (1080p60 H.264).

---

## Why NOT to use the page HTML source

The `<video src="blob:https://...">` URLs you see in browser DevTools are **ephemeral Blob URLs** — temporary in-memory references created by the browser. They:
- Cannot be downloaded externally
- Die when the browser tab closes
- Are not real file paths

Always use the **Steam API** method above instead.

---

## Notes

- The `?t=` timestamp in the URL is a cache-busting parameter — it changes when Steam updates the trailer. Re-run Step 1 to get a fresh URL if a download fails.
- The trailing HTTP 404 ffmpeg logs at the end of a download is normal — it's ffmpeg probing past the last segment. The file is complete.
- Output is `.mp4` (H.264 + AAC), which is compatible with all editors and platforms.
