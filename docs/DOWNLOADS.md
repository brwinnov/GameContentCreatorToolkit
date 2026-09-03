# Downloads

Official releases of Game Content Creator Toolkit are published on the
[GitHub Releases page](https://github.com/brwinnov/GameContentCreatorToolkit/releases).
Each release provides a Windows MSI and Linux DEB/RPM packages together with a
`SHA256SUMS` manifest.

## Code Signing Policy

The project uses free code signing provided by
[SignPath.io](https://about.signpath.io/), certificate by
[SignPath Foundation](https://signpath.org/), when that service becomes
available to this project. The initial Foundation application was declined
because the project does not yet meet SignPath's minimum public-reputation and
visibility threshold.

At this time, releases remain explicitly unsigned. The project is not paying
for a commercial SignPath subscription to bypass the Foundation gate, and no
signed Windows installer is presently being claimed. Windows may display an
Unknown Publisher warning. Download only from this repository and verify the
published checksum before running an unsigned installer.

## Verification

Compare the SHA-256 digest of a downloaded file with its entry in
`SHA256SUMS`. On PowerShell:

```powershell
Get-FileHash -Algorithm SHA256 .\GCCtoolkit_*.msi
```

For signed Windows releases, open the MSI's **Digital Signatures** properties
or run Windows SDK `signtool verify /pa /all /v <installer.msi>`. Require a
valid timestamped signature issued to SignPath Foundation.

For an explicitly unsigned preview release, confirm that its release notes say
unsigned and verify `SHA256SUMS`. A checksum detects download corruption and
supports comparison with this repository's release, but it is not a publisher
signature and does not provide the same protection as Authenticode.

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
