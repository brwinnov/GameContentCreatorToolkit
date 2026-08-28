# Downloads

Official releases of Game Content Creator Toolkit are published on the
[GitHub Releases page](https://github.com/brwinnov/GameContentCreatorToolkit/releases).
Each release provides a Windows MSI and Linux DEB/RPM packages together with a
`SHA256SUMS` manifest.

## Code Signing Policy

The project uses free code signing provided by
[SignPath.io](https://about.signpath.io/), certificate by
[SignPath Foundation](https://signpath.org/). Each signed release is built by
GitHub Actions from this repository and manually approved under the project's
[code signing policy](CODE_SIGNING_POLICY.md).

Until the SignPath Foundation application and integration are accepted, a
release without a valid signature must be treated as unsigned. The release
workflow is configured to prevent new tagged releases when Windows signing is
unavailable.

## Verification

Compare the SHA-256 digest of a downloaded file with its entry in
`SHA256SUMS`. On PowerShell:

```powershell
Get-FileHash -Algorithm SHA256 .\GCCtoolkit_*.msi
```

For signed Windows releases, open the MSI's **Digital Signatures** properties
or run Windows SDK `signtool verify /pa /all /v <installer.msi>`. Require a
valid timestamped signature issued to SignPath Foundation.

## System Changes and Uninstallation

The Windows MSI installs Game Content Creator Toolkit machine-wide and may
request elevation. The application changes ffmpeg settings or installs its
managed ffmpeg/ffprobe copy only after explicit user action; managed tools and
application settings are stored in the user's local application-data directory.

Uninstall the Windows package from **Settings > Apps > Installed apps**. Remove
Linux packages through the same graphical package manager or package-management
tool used to install the DEB or RPM. Normal uninstallation may preserve local
settings, history, managed tools, and downloaded media. Delete those files
manually only when their contents are no longer needed.
