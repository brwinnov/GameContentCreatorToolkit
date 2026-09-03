# Code Signing Policy

Game Content Creator Toolkit does not Authenticode-sign its Windows releases
and has no active code-signing plan. The project applied to SignPath
Foundation's free open-source code-signing programme; the application was
declined because the project did not meet SignPath's minimum public-reputation
and visibility threshold. A commercial SignPath subscription is not free, and
the project is not paying for one, so SignPath is not a route this project
will use.

Every tagged release is published as an explicitly unsigned build, with
release notes stating that status and a `SHA256SUMS` manifest for integrity
verification. Windows may display an Unknown Publisher warning when installing
the MSI; this is expected and does not indicate a corrupted download.

## Verification

Compare the SHA-256 digest of a downloaded file with its entry in
`SHA256SUMS` (see [`DOWNLOADS.md`](DOWNLOADS.md) for the exact command). A
checksum match confirms the file matches this repository's release; it is not
a publisher signature and does not authenticate the publisher the way
Authenticode would.

## Scope

Only binaries built from source owned and maintained in this repository are
published as project releases. Third-party binaries (ffmpeg, ffprobe) are
never represented as signed or endorsed by this project.

## Privacy

This program will not transfer any information to other networked systems
unless specifically requested by the user or the person installing or
operating it. See the [privacy policy](PRIVACY.md) for the user-triggered Steam
and GitHub requests and applicable third-party policies.
