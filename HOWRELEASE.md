# How to publish the current release to GitHub

This guide explains how to publish the current Windows release for this project so it appears in the GitHub repository's Releases section and shows as the latest release.

## What this release includes

The current Windows build should include all release formats for the same version:

- `ggc-app-0.1.0.exe` — portable standalone app
- `ggc-app-0.1.0-installer.msi` — Microsoft Installer package
- `ggc-app-0.1.0-setup.exe` — setup/installer executable

These files are expected to be in the release folder:

- `release/ggc-app-0.1.0/`

---

## Prerequisites

Before publishing:

- You have a GitHub repository already created for this project
- You are logged in to GitHub with permissions to create releases
- The code is committed and pushed to the `main` branch
- The final build artifacts are present locally on disk

---

## Step 1: Check that your repo is clean and up to date

From the project folder:

```bash
git checkout main
git pull origin main
git status
```

If you see local changes, commit them first before tagging the release.

---

## Step 2: Verify the release artifacts exist

Check that all release files are present:

```bash
Get-ChildItem .\release\ggc-app-0.1.0
```

You should see files like:

- `ggc-app-0.1.0.exe`
- `ggc-app-0.1.0-installer.msi`
- `ggc-app-0.1.0-setup.exe`

---

## Step 3: Create a git tag for the release version

Use a version tag that matches the release number. For this project, use:

```bash
git tag -a v0.1.0 -m "Release 0.1.0"
```

Then push the tag to GitHub:

```bash
git push origin v0.1.0
```

This is the important step that makes GitHub know this is an official release.

---

## Step 4: Create the GitHub release

### Option A: GitHub web UI

1. Open your repository on GitHub
2. Click the `Releases` tab
3. Click `Draft a new release`
4. Choose the tag `v0.1.0`
5. Set the release title to `0.1.0`
6. Add a short changelog summary
7. Upload all three release files
8. Click `Publish release`

### Option B: GitHub CLI (if installed)

```bash
gh release create v0.1.0 \
  --title "0.1.0" \
  --target main \
  --notes "Game Content Creator Toolkit 0.1.0 release. Includes portable EXE and MSI installer packaging." \
  --verify-tag
```

Then attach the files:

```bash
gh release upload v0.1.0 \
  .\release\ggc-app-0.1.0\ggc-app-0.1.0.exe \
  .\release\ggc-app-0.1.0\ggc-app-0.1.0-installer.msi \
  .\release\ggc-app-0.1.0\ggc-app-0.1.0-setup.exe
```

---

## Step 5: Make sure it shows as the latest release

GitHub will show a release as `Latest` when:

- the release is published
- it is not marked as a pre-release
- it is not marked as a draft
- it is based on the newest published non-prerelease tag

Avoid marking the release as a pre-release if you want it to appear as the main current version.

---

## Important notes

- Pushing code alone does not create a GitHub release.
- GitHub Releases are attached to tags.
- The repo’s Releases tab will remain empty until a tag and release are created.
- Including the EXE and MSI/installer options is recommended so users can choose the right install method.

---

## Quick copy-paste version

If you want the shortest working sequence:

```bash
git checkout main
git pull origin main
git tag -a v0.1.0 -m "Release 0.1.0"
git push origin v0.1.0
```

Then in GitHub UI:

1. Create release from tag `v0.1.0`
2. Upload all files from `release/ggc-app-0.1.0/`
3. Publish

---

## Recommended release naming pattern

Use a versioned tag and a versioned folder name together:

- tag: `v0.1.0`
- folder: `release/ggc-app-0.1.0/`
- files: `ggc-app-0.1.0.exe`, `ggc-app-0.1.0-installer.msi`, etc.

This keeps the repo, release metadata, and distributable files aligned.
