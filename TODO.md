# TODO

Living checklist. Move finished items to CHANGELOG.md, don't just delete them.

## Now
- [ ] Paste/upload `initial/download_steam_trailers.ps1` for a line-by-line
      diff against `scripts/pwsh/download_steam_trailers.ps1` — confirm which
      is newer and reconcile into one authoritative version
- [ ] Verify `.gitignore` actually keeps local test output (`trailers/`,
      `images/`, `NMS_trailers/`, `dist/`) and any `.env` out of git
- [ ] `git add` / commit / push this scaffold from the `workspace/` folder to
      GitHub (repo is currently public — flip back to private after pushing
      if that's the intent)
- [ ] Delete/archive the old local `initial/` folder once everything useful
      from it is merged in, to avoid drifting duplicate copies

## Next
- [ ] Pick first Phase 1 hardening item (batch mode vs. metadata sidecar vs.
      screenshot/art pulling) and implement
- [ ] Scaffold the Tauri app shell (empty window + "hello world" sidecar call
      to the existing PowerShell script, just to prove the plumbing)

## Later
- [ ] Design the common "downloader module" interface so Steam, YouTube,
      TikTok, Facebook, and Instagram all plug into the same UI shape
- [ ] Settings screen: download folder override, quality prefs
- [ ] yt-dlp integration for the social platforms (Phase 3)

## Someday
- [ ] Pester tests + CI
- [ ] Steam Workshop support
- [ ] Resolve bin/timeline export
