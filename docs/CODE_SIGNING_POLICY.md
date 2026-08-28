# Code Signing Policy

Game Content Creator Toolkit intends to use SignPath Foundation's free
open-source code-signing service after the project and its build configuration
have been accepted. Until that integration is complete, release artifacts must
not be represented as SignPath-signed.

Free code signing provided by [SignPath.io](https://about.signpath.io/),
certificate by [SignPath Foundation](https://signpath.org/).

## Team Roles

- Author and committer: [Barry Reilly (@brwinnov)](https://github.com/brwinnov)
- Reviewer: [Barry Reilly (@brwinnov)](https://github.com/brwinnov)
- Approver: [Barry Reilly (@brwinnov)](https://github.com/brwinnov)

Every signing request requires manual approval by the approver. Signed
artifacts must be produced by the repository's automated release workflow from
the corresponding reviewed source revision. Product name and version metadata
must match the repository release and be enforced in SignPath's artifact
configuration.

## Privacy

This program will not transfer any information to other networked systems
unless specifically requested by the user or the person installing or
operating it. See the [privacy policy](PRIVACY.md) for the user-triggered Steam
and GitHub requests and applicable third-party policies.

## Scope

Only binaries built from source owned and maintained in this repository may be
submitted under this project's signing configuration. Third-party binaries
must not be signed as project binaries.

Repository-controlled SignPath setup templates and the activation checklist
are maintained under [`.signpath/`](../.signpath/README.md).
