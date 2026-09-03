# SignPath Setup

This directory contains the repository-controlled templates for the project's
SignPath Foundation open-source signing application. Artifact configurations
must be created in SignPath's dashboard from these XML templates; SignPath does
not load them directly from the repository.

## Requested SignPath Resources

Use these stable names when creating the SignPath project:

| Resource | Value |
| --- | --- |
| Project slug | `game-content-creator-toolkit` |
| Windows artifact configuration slug | `windows-msi` |
| Windows signing policy slug | `release-signing` |
| RPM artifact configuration slug | `linux-rpm` |
| RPM signing policy slug | `rpm-release-signing` |

The Windows workflow uses the first three slugs directly. The RPM template is
ready for later activation if SignPath Foundation provides a GPG signing policy.
SignPath's documentation lists RPM signing for Open Source Code Signing but DEB
signing as Advanced-only, so no unsupported DEB configuration is included.

## Dashboard Setup

This directory remains a future-activation template for SignPath, not a current
signed-release path. The initial SignPath Foundation application was declined,
so the project is continuing with explicit unsigned releases while the project
builds public reputation and visibility.

1. Keep the application answers in [`APPLICATION.md`](APPLICATION.md) as a
   reapplication template for a future SignPath Foundation attempt.
2. After acceptance, add the predefined `GitHub.com` trusted build system to
   the SignPath organization and project.
3. Install the [SignPath GitHub App](https://github.com/apps/signpath) for this
   repository.
4. Create the `windows-msi` artifact configuration from
   [`artifact-configurations/windows-msi.xml`](artifact-configurations/windows-msi.xml).
5. Create the `release-signing` policy with the SignPath Foundation certificate,
   Barry Reilly as approver, and manual approval required.
6. Create a SignPath API token for a submitter account restricted to this
   project and policy.
7. Add the organization ID as repository variable
   `SIGNPATH_ORGANIZATION_ID` and the token as repository secret
   `SIGNPATH_API_TOKEN`.
8. Set repository variable `SIGNPATH_ENABLED` to `true`.
9. Confirm GitHub-hosted runners, artifact origin verification, manual approval,
   signed MSI download, and CI signature verification with a test tag.

While `SIGNPATH_ENABLED` is not `true`, tagged builds publish an explicitly
labeled unsigned MSI with checksums. Once enabled, tagged Windows builds fail
closed unless SignPath returns a valid timestamped MSI. Ordinary `main` builds
remain unsigned validation artifacts and are never published as releases.

## Human-Controlled Requirements

Repository files cannot complete these SignPath conditions:

- SignPath Foundation must accept the application and project reputation.
- The project must have sufficient public visibility and community trust.
- Every maintainer must enable MFA for GitHub and SignPath.
- The approver must manually approve every signing request.
- SignPath must issue and control the Foundation certificate.
- GitHub repository settings must grant the SignPath App access.
- GitHub repository rules must enforce any desired CODEOWNERS review policy.
