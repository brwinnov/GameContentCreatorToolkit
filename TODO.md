# TODO

Living checklist. Move finished items to CHANGELOG.md, don't just delete them.

## Now
- [x] Diff `initial/download_steam_trailers.ps1` vs
      `scripts/pwsh/download_steam_trailers.ps1` — **confirmed identical**
      (matching MD5), no reconciliation needed. `initial/` can be archived/
      deleted locally once you've confirmed nothing else in that folder is
      needed.
- [x] Build and package Windows release set for 0.1.0: portable EXE +
      MSI + NSIS install bundle in `workspace/release/ggc-app-0.1.0`.
- [x] Release target decision for the next milestone: Windows + Linux
      `.deb` / `.rpm` distribution only; macOS packaging is deferred.
- [x] Hide ffmpeg/ffprobe console windows on Windows while keeping live in-app
      progress and captured stderr handling; this avoids black cmd popups in a
      desktop release.
- [x] Add Steam game-name lookup fallback: users can type a game name and
      pick a matching app from Steam search results before fetching trailers.
- [ ] Verify `.gitignore` actually keeps local test output (`trailers/`,
      `images/`, `NMS_trailers/`, `dist/`) and any `.env` out of git
- [ ] `git add` / commit / push this scaffold from the `workspace/` folder to
      GitHub (repo is currently public — flip back to private after pushing
      if that's the intent)
- [ ] Delete/archive the old local `initial/` folder once everything useful
      from it is merged in, to avoid drifting duplicate copies

## Next
- [x] `cd app && npm install && npm run tauri dev` — first real compile of the
      Tauri app scaffold. Compiled clean on the first try, no fixes needed
      (2026-08-01, after a reboot to pick up newly-installed Rust toolchain).
- [x] Test the Steam tab end-to-end with a real App ID (2424010 / Parcel
      Simulator) — fetch, select, download, ffmpeg detection all worked.
      Along the way, filtered a benign ffmpeg DASH stderr line that was
      appearing on some downloads despite success (see CHANGELOG), and fixed
      `build_toolkit.ps1` to actually sync `toolkit/` from source before
      zipping instead of silently re-packaging a stale copy.
- [x] Steam tab polish pass (2026-08-01): live 0–100% download progress bar
      (real `ffprobe`/ffmpeg progress, not fake), a "Show history" panel
      logging every search + download attempt with status badges (persisted
      across restarts), and an uppercase pane title. Verified live against
      App ID 2201940 (Ship Graveyard Simulator 2).
- [x] Release hardening pass (2026-08-16): hide child-process console windows,
      create shareable Windows test builds, and add Steam game-name search.
- [ ] Pick first Phase 1 hardening item (batch mode vs. metadata sidecar vs.
      screenshot/art pulling) and implement — **next up**

## Later
- [ ] Design the common "downloader module" interface so Steam, YouTube,
      TikTok, Facebook, and Instagram all plug into the same UI shape
- [ ] Settings screen: download folder override, quality prefs
- [ ] yt-dlp integration for the social platforms (Phase 3)

## Someday
- [ ] Pester tests + CI
- [ ] Steam Workshop support
- [ ] Resolve bin/timeline export
