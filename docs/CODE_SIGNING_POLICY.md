# Code Signing Policy

Game Content Creator Toolkit intends to use SignPath Foundation's free
open-source code-signing service when the project meets the reputation and
visibility requirements for that program. The initial Foundation application was
not accepted: SignPath indicated that the project does not yet have enough public
trust and community signals for a Foundation certificate.

The project therefore continues with a clearly labeled unsigned release policy.
Preview and tagged release artifacts may be published only as explicitly
unsigned builds, with checksums and release notes stating that status. The
repository variable `SIGNPATH_ENABLED` remains `false` unless a future SignPath
setup is accepted and verified end to end. We are not paying for a commercial
SignPath subscription to bypass the Foundation policy gate.

Free code signing provided by [SignPath.io](https://about.signpath.io/),
certificate by [SignPath Foundation](https://signpath.org/), remains an
aspirational path for the future rather than a current release requirement.

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
