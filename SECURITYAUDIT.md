# Security Audit

Living document. Update whenever a new dependency, credential type, or
external call is added. Not a substitute for a real third-party audit if this
project ever gets wide distribution — treat it as a running checklist.

## Current data handled

| Item | Sensitive? | Notes |
| --- | --- | --- |
| Steam age-gate cookies (`birthtime`, `lastagecheckage`, `mature_content`) | No | Fixed, publicly-known constant values used to bypass the 18+ content gate. Not tied to any account, not a login, not a secret. Safe to commit in source. |
| Steam App ID | No | Public identifier, same as in the store URL. |
| Downloaded media files | No | Publicly published marketing assets from the publisher/developer, used under fair-use/press-kit norms. Not redistributed. |
| Local file paths (e.g. ffmpeg location) | No | Machine-specific but not sensitive; may still reveal a username in a path — keep out of anything published publicly (screenshots, issue reports). |

**No credentials, API keys, tokens, passwords, or personal data are currently
required or stored by this project.**

## Things to check before every commit / push

- [ ] No hardcoded absolute paths containing your real Windows username
      (`C:\Users\<name>\...`) in committed files
- [ ] No `.env` file committed (see `.env.example` for the template — real
      values stay local only)
- [ ] No API keys/tokens if a future feature adds one (e.g. a YouTube Data
      API key, a Steam Web API key with account access) — these must go in
      `.env`, never in source, never in `docs/`, `ai-journal/`, or `logs/`
- [ ] `logs/` directory contents are not committed (may contain local paths
      or App IDs of unreleased/private review copies you don't want public)
- [ ] Downloaded media itself is never committed to the repo

## External dependencies / supply-chain notes

- **ffmpeg** — not bundled in the repo. User installs via winget/scoop/choco/
  brew/apt. Script only auto-detects common install paths, never downloads or
  executes an unverified binary on your behalf.
- **yt-dlp** (planned, Phase 3) — same policy: user-installed, not bundled.
  yt-dlp interacts with third-party platforms (YouTube/TikTok/FB/Instagram);
  review its own security advisories periodically since it's under active,
  frequent development.
- **Steam `appdetails` API** — unauthenticated public endpoint, no API key
  required, no login performed.

## Future items to revisit

- If a GUI/desktop app (Phase 2, Tauri) ever adds auto-update, verify update
  packages are signed and fetched over HTTPS from a pinned source.
- If any platform integration ever requires OAuth (e.g. a real Instagram/
  Facebook Graph API instead of yt-dlp scraping), tokens must be stored using
  OS-level secure storage (Windows Credential Manager / macOS Keychain /
  Linux Secret Service), never in plaintext config.
- Review whether review-copy game names/App IDs logged locally in `logs/`
  should be treated as confidential (some publishers include NDA terms on
  early-access review copies) — if so, `logs/` should be gitignored (already
  is) and periodically purged.
