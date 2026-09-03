# How to Cut a Release

Public releases are built from version tags by GitHub Actions. Do not manually
attach binaries built from another branch or commit.

## Release Types

- Patch: `0.1.0` → `0.1.1`
- Minor: `0.1.0` → `0.2.0`
- Major: `0.1.0` → `1.0.0`

## Version Sources

Update these files to the same semantic version:

- `app/package.json`
- `app/package-lock.json`
- `app/src-tauri/Cargo.toml`
- `app/src-tauri/Cargo.lock` (`ggt-app` package entry)
- `app/src-tauri/tauri.conf.json`

Also add the release to `CHANGELOG.md`. The CI release gate rejects a tag when
its value does not match the npm, lockfile, Cargo, and Tauri versions.

## Local Validation

From the repository root:

```powershell
Set-Location app
npm ci
node --check src/main.js
cargo check --manifest-path src-tauri/Cargo.toml
npx tauri build --no-bundle
```

For a complete local Windows package check:

```powershell
npx tauri build --bundles msi
```

Linux packages are built and validated by the Ubuntu GitHub Actions job.

## Commit and Tag

Commit the fully validated release candidate, then create an annotated tag on
that exact commit:

```powershell
git add --all
git commit -m "Release 0.1.3"
git tag -a v0.1.3 -m "Release 0.1.3"
git push origin main
git push origin v0.1.3
```

Never move or reuse an already-published release tag. Increment the patch
version for a correction.

## Automated GitHub Flow

The `Tauri release build` workflow runs separate hosted builds:

- Windows: MSI installer
- Linux: Debian `.deb` and RPM packages

MSI is the sole supported Windows installer format. Keep the WiX `upgradeCode`
in `tauri.conf.json` unchanged across releases so Windows upgrades the existing
installation instead of creating a second uninstall entry. CI inspects the
built MSI and rejects version or UpgradeCode drift before artifact upload.

The tag build embeds the GitHub run number as the app build number. After both
jobs pass, `Publish Tauri release artifacts` downloads artifacts from that exact
workflow run and creates the GitHub Release for the tag.

CI publishes the MSI unsigned, with an explicit unsigned warning in the release
notes and a verified `SHA256SUMS` checksum manifest. The project has no active
code-signing plan — see [`CODE_SIGNING_POLICY.md`](CODE_SIGNING_POLICY.md) for
why (SignPath Foundation declined the free OSS application, and a commercial
SignPath subscription is not free).

The publisher generates and verifies `SHA256SUMS` for the MSI, DEB, and RPM
before creating the release. Linux package signatures are not implemented and
must not be claimed until configured and verified.

Normal pushes to `main` build validation artifacts but do not publish a release.

## Verification

Before sharing a release, confirm:

1. The tag points to the release commit.
2. Windows and Linux jobs completed successfully.
3. The publish workflow completed successfully.
4. The release notes carry the unsigned warning.
5. The release contains one current MSI, `.deb`, `.rpm`, and `SHA256SUMS`.
6. `sha256sum --check SHA256SUMS` passes for every release artifact.
7. Package filenames and displayed application version match the tag.
8. Release notes accurately describe working and placeholder features.
9. The repository worktree is clean and synchronised with `origin/main`.

## Auto-Update Future Work

Signed in-app updating is not enabled yet. Its proposed key management,
manifest, channel, CI, and runtime design is documented in `repo-update.md`.
