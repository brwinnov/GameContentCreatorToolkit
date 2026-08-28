# Game Content Creator Toolkit — desktop app

Windows + Linux desktop wrapper around the Steam trailer downloader, with
placeholder tabs for YouTube, TikTok, and Instagram (Phase 3 — routed through
yt-dlp, not yet implemented). macOS packaging is intentionally deferred for the
current release milestone.

## Preview the UI right now (no install needed)

Double-click `src/index.html` to open it in a browser. It runs in a mocked
mode (fake trailer data, simulated download log) so you can click through the
whole interface before building the real app. Look for the
`[preview mode]` warning in the browser console — that's expected.

## Run the real app (first-time setup)

### 1. Install prerequisites

- **Rust**: <https://rustup.rs> (installs `rustc` + `cargo`)
- **Node.js** (LTS): <https://nodejs.org>
- **Windows only** — Tauri needs the WebView2 runtime (already preinstalled
  on most Windows 10/11 machines) and the "Desktop development with C++"
  workload from Visual Studio Build Tools:
  <https://visualstudio.microsoft.com/visual-cpp-build-tools/>

### 2. Install dependencies

From this folder (`app/`):

```powershell
npm install
```

### 3. Run in dev mode

```powershell
npm run tauri dev
```

This opens the real desktop window with the Rust backend wired up — Steam
lookups and downloads will work for real. ffmpeg is still required, but Windows
users can install a verified user-local copy from Home or Settings; users can
also select an existing ffmpeg/ffprobe pair.

### 4. Build an installer

```powershell
npm run tauri build
```

Output lands in `src-tauri/target/release/bundle/`. Public Windows releases use
one MSI installer; Linux releases use `.deb` and `.rpm` bundles. macOS
`.app`/DMG is not part of the active roadmap.

## Icons

Placeholder icons are included so the project builds out of the box. Once
there's a real logo, regenerate the full icon set (including macOS `.icns`,
which isn't included yet) with:

```powershell
npm run tauri icon path/to/logo.png
```

## Project layout

```text
app/
├── package.json
├── src/                  ← frontend (plain HTML/CSS/JS, no bundler)
│   ├── index.html
│   ├── style.css
│   └── main.js
└── src-tauri/             ← Rust backend
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/default.json
    ├── icons/
    └── src/
        ├── main.rs
        └── lib.rs          ← Steam API + ffmpeg download logic
```

## What's wired up vs. placeholder

- **Steam tab**: fully functional. `fetch_steam_trailers` calls the same
  Steam `appdetails` API + age-gate cookies as the PowerShell script;
  `download_trailers` runs ffmpeg the same way (`-c copy`, DASH → MP4). The UI
  includes explicit clipboard paste and durable user-local history.
- **YouTube / TikTok / Instagram / Facebook tabs**: UI placeholders only, matching the
  visual style so they're easy to fill in later. See `docs/PLAN.md` Phase 3.
- **Settings tab**: shows the default download folder plus detected ffmpeg path
  and version. It can select an existing ffmpeg/ffprobe pair or install a
  checksum-verified Windows copy under user-local app data. Folder override
  uses the native OS picker (Tauri's dialog plugin).
