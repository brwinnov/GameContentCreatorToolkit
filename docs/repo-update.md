# Repository Update Plan

Plan for adding secure, user-approved application updates to GCCtoolkit.
This document reflects the current project architecture and Tauri v2 updater
recommendations as of 2026-08-28.

## Decision

Use Tauri's official updater plugin with a signed static update manifest stored
as an asset on GitHub Releases.

The installed app will make an HTTPS request to:

```text
https://github.com/brwinnov/GameContentCreatorToolkit/releases/latest/download/latest.json
```

`latest.json` will identify the current release and provide a signed download
URL for each supported updater platform. The app will not scrape GitHub pages,
execute arbitrary download links, or install an unsigned update.

Initial scope:

- Windows MSI installation: full in-app update support
- Windows portable ZIP: update notification and download guidance only
- Linux `.deb` and `.rpm`: update notification and package download guidance
- macOS: deferred with the rest of macOS support

Tauri's Linux self-updater is designed around AppImage artifacts. The current
`.deb` and `.rpm` packages should not overwrite files managed by a system
package manager. Full Linux in-app updating can be reconsidered if an AppImage
distribution is added later.

## Desired User Experience

The behavior should feel similar to a normal per-user VS Code installation:

1. The app starts normally and performs a quiet update check after the main
   window is usable.
2. If no update exists, no notification is shown.
3. If an update exists, a non-blocking banner shows the version, summary, and
   an **Update** button. The user can choose **Later** or open the release notes.
4. Clicking **Update** starts a background download with visible progress. The
   app remains usable while downloading.
5. Once verified and ready, the user chooses **Restart and update**. The app
   saves transient state, exits, installs the update, and relaunches.
6. A manual **Check for updates** command remains available in Settings/About.
7. Update failures leave the currently installed version working and show a
   useful retry or manual-download option.

Do not download a large package before the user approves the update. A small
manifest check may happen automatically; downloading and installation require
clear consent.

## Installation Model

Keep MSI as the sole supported Windows installer and preserve the original WiX
UpgradeCode in every release. Existing installations are machine-wide under:

```text
%PROGRAMFILES%\AckrosGaming\Game Content Creator Toolkit\
```

Updates may require Windows elevation. Use Tauri's passive update mode so this
is visible to the user. Do not introduce NSIS as a second installation lineage;
switching installer technologies can leave duplicate uninstall entries or fail
to remove the existing MSI-managed application.

Mutable state must remain outside the installation directory:

```text
%LOCALAPPDATA%\com.ackrosgaming.gcc\
```

Settings, logs, update staging files, and future independently updated tools
belong there. User downloads continue to use the configured download folder.

## Update Architecture

### Application components

Add the official Tauri v2 plugins:

- Rust: `tauri-plugin-updater`
- JavaScript: `@tauri-apps/plugin-updater`
- JavaScript/Rust process plugin for controlled relaunch

Grant only the required capabilities:

- updater check
- updater download/install
- process relaunch

The current project has the updater disabled in `tauri.conf.json`; implementation
will replace that placeholder with a public key and HTTPS endpoint configuration.

### Tauri configuration shape

The implementation should follow this structure, with the real public key:

```json
{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      "pubkey": "TAURI_PUBLIC_KEY_CONTENT",
      "endpoints": [
        "https://github.com/brwinnov/GameContentCreatorToolkit/releases/latest/download/latest.json"
      ],
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

Production endpoints must use HTTPS. Do not enable
`dangerousInsecureTransportProtocol`.

`passive` is the preferred Windows update mode because it provides progress
without requiring installer interaction. `quiet` is not recommended because it
hides progress and cannot elevate itself.

### Static manifest

Each stable GitHub Release will contain `latest.json`. Its essential structure
is:

```json
{
  "version": "0.2.0",
  "notes": "Release summary",
  "pub_date": "2026-09-01T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "CONTENTS_OF_THE_GENERATED_SIG_FILE",
      "url": "https://github.com/brwinnov/GameContentCreatorToolkit/releases/download/v0.2.0/GCCtoolkit_0.2.0_x64_en-US.msi"
    }
  }
}
```

The signature value is the content of the generated `.sig` file, not a path to
that file. Tauri validates the signature before installation and does not allow
signature verification to be disabled.

## Signing and Trust

Tauri update signing and Windows Authenticode signing solve different problems:

- **Tauri updater signature:** proves an update was authorized by this project
  before an installed copy accepts it.
- **Authenticode signature:** establishes publisher identity to Windows and
  reduces SmartScreen warnings.

Both are recommended before broad public distribution.

Generate a dedicated Tauri updater key pair:

```powershell
npm run tauri signer generate -- -w "$HOME\.tauri\gcctoolkit.key"
```

Key rules:

- Commit the public key in `tauri.conf.json`.
- Never commit the private key or its password.
- Store the private key and password as GitHub Actions secrets:
  `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
- Keep an offline encrypted backup. Losing the private key prevents existing
  installations from accepting future updates.
- Restrict release-environment access and require review for production jobs.
- Document and test a public-key rotation procedure before rotation is needed.

Release workflows must use immutable action versions or pinned commit SHAs where
practical. Enable GitHub dependency review, Dependabot alerts, secret scanning,
and protected release environments.

## Version and Release Rules

An update must always be built from the commit referenced by its version tag.
Never attach a binary built from `main` to a release whose tag points to another
commit.

For each release:

1. Update the version consistently in `app/package.json`,
   `app/src-tauri/Cargo.toml`, and `app/src-tauri/tauri.conf.json`.
2. Commit and validate the release candidate.
3. Create an annotated tag such as `v0.2.0` on that exact commit.
4. Push the tag.
5. Build from `github.sha` for the tag event.
6. Generate normal installers, updater bundles, `.sig` files, checksums, and
   `latest.json` in that same workflow.
7. Create a draft GitHub Release, upload all assets, verify them, then publish
   the release atomically.

The updater compares semantic versions and offers an update only when the feed
version is newer than the installed version. Downgrades remain disabled.

### Release channels

Keep stable and beta channels separate:

- Stable builds use a stable feed and exclude GitHub prereleases.
- Beta builds use a separate `latest-beta.json` endpoint and opt-in setting.
- Never allow a stable installation to consume an accidental prerelease.

GitHub's `/releases/latest/` endpoint is suitable only when the intended stable
release is marked as the latest non-prerelease. A dedicated stable manifest URL
or small update service is preferable if channel logic becomes more complex.

## CI/CD Changes

Replace the current release process with a tag-driven release workflow:

1. Checkout the pushed tag with full tag metadata.
2. Confirm the tag version equals all application version fields.
3. Install dependencies with lockfiles (`npm ci` and Cargo's lockfile).
4. Run formatting, linting, unit tests, and a release compile check.
5. Build Windows MSI and Linux `.deb`/`.rpm` artifacts.
6. Generate updater artifacts by setting `createUpdaterArtifacts: true` and
   supplying the private key through the job environment.
7. Generate SHA-256 checksums for every public artifact.
8. Generate and validate `latest.json` from the actual filenames, URLs, and
   signature contents.
9. Upload artifacts to a draft release.
10. Run package smoke tests where possible.
11. Publish only after all required jobs and release checks pass.

Normal pushes to `main` may continue to build test artifacts, but must never
create or alter a public GitHub Release.

## Runtime Update Policy

### Check timing

- Check shortly after startup, without delaying the main window.
- Cache the last successful check time.
- Check no more than once every 6 hours automatically.
- Add small random jitter if the user base grows, avoiding synchronized traffic.
- Always permit a manual check.
- Use a reasonable timeout and handle offline/proxy failures quietly.

### User controls

Settings/About should show:

- installed application version
- update channel
- last update-check time
- **Check for updates** button
- automatic update-check toggle
- release notes link

The initial release should check automatically but require user approval to
download and restart. Fully unattended application updates are not recommended
for this project yet.

### State and shutdown

Before installation on Windows, Tauri exits the application. Use the updater's
before-exit hook to flush settings/history, stop child processes, remove partial
downloads, and release file handles. Never interrupt an active media download
without warning; defer installation until downloads finish or the user confirms
cancellation.

## Portable Windows ZIP

A portable ZIP cannot safely assume installer-style self-updating because the
app may run from a read-only folder, removable drive, or directory without
replacement permissions.

Portable behavior for the initial implementation:

- check the same signed manifest
- notify the user of a newer version
- open the GitHub Release or download the new ZIP to the Downloads folder
- never overwrite the running portable directory automatically

If portable self-update is added later, use a separate signed bootstrap updater
that replaces files only after the app exits and supports rollback.

## Bundled Tool Updates

Application updates and media-tool updates are separate concerns.

### ffmpeg and ffprobe

Ship pinned, verified Windows binaries with each application release. Update
them through normal signed application releases. Record versions, source URLs,
SHA-256 checksums, build configuration, and licenses in third-party notices.

### yt-dlp

Add yt-dlp only when its downloader features are implemented. It changes more
frequently than the application, so a later tool updater may:

1. Query only the official yt-dlp release source.
2. Download to a staging directory under user application data.
3. Verify a trusted checksum/signature and enforce expected file type/size.
4. Atomically activate the new binary while retaining a known-good fallback.
5. Roll back automatically if a health/version check fails.

Do not execute an unverified binary from a generic "latest" URL, and do not let
`yt-dlp --update` mutate files inside the application installation directory.

## Failure Handling and Recovery

- A failed manifest check must not affect application startup.
- A failed or cancelled download deletes its temporary file safely.
- A signature mismatch is a hard failure and must never offer an override.
- Keep the current installation intact until the new package is fully
  downloaded and verified.
- Link to the exact GitHub Release for manual recovery.
- Log update events without credentials, local usernames, or sensitive paths.
- Rate-limit retries and avoid retry loops.
- Publish emergency fixes as a higher semantic version; do not silently replace
  already-published release assets.

Rollback is initially manual: reinstall the previous signed release. Automatic
downgrades remain disabled because they weaken normal version guarantees and
complicate feed security.

## Validation Checklist

Before enabling updates for public users:

- [ ] Test MSI clean install, update from every supported prior version,
      uninstall, and reinstall on Windows 10 and Windows 11.
- [x] Assert the MSI version and UpgradeCode match Tauri configuration before
  artifact upload.
- [ ] Generate, back up, and configure the Tauri updater signing keys.
- [ ] Add updater/process dependencies and least-privilege capabilities.
- [ ] Add `createUpdaterArtifacts: true`, the public key, and the HTTPS endpoint.
- [ ] Commit the npm lockfile and use reproducible CI installs.
- [ ] Make releases tag-driven and prove artifact/tag commit equality.
- [ ] Generate `.sig`, SHA-256 checksum, and valid `latest.json` assets.
- [ ] Sign Windows executables/installers with Authenticode.
- [ ] Test no-update, available-update, offline, timeout, corrupt download,
      invalid signature, cancelled update, and interrupted update paths.
- [ ] Test updating while idle and while a trailer download is active.
- [ ] Confirm settings/history survive an update.
- [ ] Confirm a normal `main` push cannot publish or change a release feed.
- [ ] Document stable/beta channel behavior and recovery instructions.
- [ ] Add privacy wording stating that update checks contact GitHub and disclose
      the user's IP address and standard HTTPS request metadata to GitHub.

## Implementation Phases

### Phase 1: Release integrity

- Lock dependencies.
- Standardize versions and artifact names.
- Build only from tags for public releases.
- Add checksums, signing secrets, and draft-release verification.

### Phase 2: Windows MSI upgrade validation

- Preserve the original MSI UpgradeCode across every version.
- Confirm clean install, elevated update, repair, and uninstall behavior.
- [x] Detect and block unexpected installer identity changes in release CI.

### Phase 3: Update notification

- Add updater/process plugins and permissions.
- Add startup/manual checks and update UI.
- Show release notes and require user approval.

### Phase 4: Download and install

- Add progress, cancellation, state flushing, restart, and error recovery.
- Validate signed updater packages end to end on clean Windows VMs.

### Phase 5: Channels and tool updates

- Add opt-in beta feed.
- Add verified yt-dlp tool updates when that integration ships.
- Evaluate AppImage if in-app Linux updates become a requirement.

## Definition of Done

Auto-update is ready when a clean MSI installation can detect a newer
tagged stable release, obtain explicit user approval, download the correct
platform package in the background, cryptographically verify it, preserve user
state, request elevation when Windows requires it, restart successfully, and
remain usable after every tested failure mode.
