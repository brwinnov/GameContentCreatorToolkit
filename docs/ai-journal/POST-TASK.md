# Post-Task Journal

Add completed tasks directly below this introduction, newest first. Keep each
entry concise enough for another AI assistant to understand what changed and
what remains without reading a chat transcript.

## 2026-08-28 - Documentation Layout

### Request

Move nonstandard root Markdown into `docs/`, retain standard GitHub repository
files at root, and identify worthwhile missing community files.

### Decisions

- Keep `README.md`, `CHANGELOG.md`, tool discovery files, and `LICENSE` at root.
- Keep package and configuration documentation beside the files it explains.
- Add useful security and contribution guidance rather than empty placeholders.

### Changed

- Moved project plans, policies, audits, release guidance, and download
  documentation into `docs/` and repaired all active references.
- Archived dated AI records under `docs/ai-journal/archive/`.
- Added root `SECURITY.md` and `CONTRIBUTING.md` community files.

### Validation

- All 26 repository-owned Markdown files passed `markdownlint-cli2`.
- Local-link validation passed all 26 Markdown files after relocation.
- `git diff --check` passed with repository line endings governed by
  `.gitattributes`.

### Remaining Work

- Consider `CODE_OF_CONDUCT.md` and `SUPPORT.md` when external community demand
  makes their policies useful; do not add empty placeholders.

## 2026-08-28 - SignPath Repository Preparation

### Request

Create all missing repository artifacts needed to pursue SignPath Foundation's
free open-source code-signing method.

### Decisions

- Submit and deep-sign the Windows MSI through SignPath from the same
  GitHub-hosted job that builds it, with manual approval and fail-closed signer
  and timestamp verification.
- Keep SignPath dashboard configuration as reviewed repository templates because
  SignPath does not import artifact configurations directly from source control.
- Prepare RPM signing separately; SignPath documents it for Open Source Code
  Signing, while embedded DEB signing is Advanced-only.
- Never expose the unsigned SignPath input artifact to the release publisher.

### Changed

- Added application answers, onboarding instructions, and schema-valid MSI/RPM
  artifact configuration templates under `.signpath/`.
- Added `DOWNLOADS.md` with the required SignPath notice, verification, system
  changes, and uninstallation guidance, plus CODEOWNERS for signing surfaces.
- Added tagged MSI submission through SignPath's SHA-pinned GitHub action,
  manual-approval wait, signer/timestamp verification, and signed-only upload.
- Restricted the publisher to named signed Windows/Linux artifacts and added a
  generated, self-verified `SHA256SUMS` release asset.
- Updated signing, release, audit, planning, and handoff documentation and
  ignored local `.firecrawl/` research output.

### Validation

- Both artifact configuration templates passed SignPath's live v1 XSD.
- All 24 tracked and newly added Markdown files passed `markdownlint-cli2`.
- VS Code workflow validation found no defects; it reports only the expected
  absent `SIGNPATH_API_TOKEN` secret and `SIGNPATH_ORGANIZATION_ID` variable.
- `actionlint` 1.7.12 passed both release workflows with no findings.
- The release checksum commands passed with representative MSI, DEB, and RPM
  filenames; a credential-pattern scan found no committed secrets.

### Remaining Work

- Add credible public reputation evidence and the SignPath account email to the
  application, then submit it and await Foundation acceptance.
- Complete the SignPath dashboard, GitHub App, MFA, repository secret/variable,
  and manual approver setup documented in `.signpath/README.md`.
- Run a test tag and verify the first signed MSI end to end before public release.
- Activate RPM signing if SignPath supplies a GPG policy; choose a separate
  supported trust mechanism for DEB and sign the checksum manifest.

## 2026-08-28 - MIT License and SignPath Policy

### Request

Drop the Azure signing direction and add the missing license needed to pursue
free SignPath Foundation signing.

### Decisions

- License project-owned source under the OSI-approved MIT License, with Barry
  Reilly as the copyright holder.
- Prepare the repository for SignPath Foundation rather than claim acceptance
  or signed releases before its application and workflow setup are complete.
- Publish SignPath's required roles, manual approval rule, and privacy statement
  alongside the license and package metadata.

### Changed

- Added `LICENSE`, `CODE_SIGNING_POLICY.md`, and `PRIVACY.md`.
- Declared `MIT` in npm, npm lockfile, and Cargo package metadata.
- Linked the policies from `README.md`, completed `SEC-01` and `SEC-04` in
  `FEATURES.md`, and updated the `SEC-002` remediation direction.
- Refreshed `PRE-TASK.md` with the licensing and signing decision.

### Validation

- All 21 tracked and newly added Markdown files passed `markdownlint-cli2`.
- npm and Cargo metadata both resolved the project license as `MIT`.

### Remaining Work

- Apply to SignPath Foundation and await project acceptance.
- Configure SignPath artifact metadata, GitHub integration, manual release
  approval, and post-signing verification before representing releases as
  signed.
- Complete the remaining `SEC-002` Linux signatures and checksum-manifest work.

## 2026-08-28 - Source Security Audit

### Request

Run a security audit and append findings and remediation tasks to the security
audit file.

### Decisions

- Treat the deprecated Bash script's App ID interpolation into `node -e` as a
  high-priority code-injection issue while the script remains shipped.
- Treat unsigned public installers as high severity before broad distribution.
- Record dependency warnings separately from known vulnerabilities: npm and
  RustSec found no known vulnerabilities in the locked dependency graphs.

### Changed

- Appended a dated, severity-ranked report to `SECURITYAUDIT.md` covering the
  Tauri boundary, rendering, network/process limits, managed ffmpeg, local
  persistence, legacy scripts, dependency locks, and release workflows.
- Added actionable checklists for all 9 findings and corrected the stale
  description of Tauri auto-update as a future GUI concern.
- Promoted security remediation ahead of feature work in `PRE-TASK.md`.

### Validation

- `npm audit --package-lock-only --audit-level=low`: 0 vulnerabilities.
- `cargo audit`: 0 known vulnerabilities and 18 warnings.
- Current tracked-file and Git-history scans found no common credential or
  private-key patterns and no unexpectedly tracked sensitive artifacts.
- `SECURITYAUDIT.md` passed `markdownlint-cli2` after the report was appended.

### Remaining Work

- Address `SEC-001` first, then `SEC-002`; the complete ordered remediation
  checklist is in `SECURITYAUDIT.md`.
- Re-run both dependency audits and targeted tests after remediation.

## 2026-08-28 - Finalize Model-Neutral Handoff

### Request

Bring all journals and Markdown up to date for switching to Claude Code,
Claude CLI, another coding app, or a different model, then commit and push.

### Decisions

- Keep `PRE-TASK.md` as the canonical incoming snapshot and `POST-TASK.md` as
  newest-first history; older entries remain unchanged historical records.
- Make the handoff self-contained with current architecture, paths, release
  artifact identity, passed checks, known gaps, and the recommended next task.
- Keep local MSI artifacts out of Git while documenting their location/hash
  for another session on this machine.

### Changed

- Corrected stale active claims in root/app READMEs, roadmap, feature backlog,
  updater plan, release guide, and model-specific/model-neutral instructions.
- Expanded the canonical handoff for direct use by any replacement assistant.

### Validation

- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test`: 6 passed.
- `node --check app/src/main.js`: passed.
- Release version gate for `v0.1.3`: passed across npm, lockfile, Tauri,
  Cargo, and Cargo.lock.
- Markdown lint: 0 issues across all 19 tracked Markdown files.
- Local Markdown links: passed across all 19 tracked Markdown files.
- VS Code diagnostics and targeted diff security scan: passed.

### Remaining Work

- Runtime-test managed ffmpeg installation and the packaged `0.1.3` app on a
  clean Windows environment; test Linux packages separately.
- Choose the next Steam hardening item, with metadata sidecars as the smallest
  foundation-oriented candidate.

## 2026-08-28 - Paste, Durable History, and ffmpeg Setup

### Request

Add the missing Steam clipboard action, retain history across upgrades and
clean installs, and make ffmpeg setup manageable from the app without admin.

### Decisions

- Read clipboard text only after `PASTE` is clicked.
- Persist history as JSON under the stable local app-data identifier and copy
  the legacy `com.brwinnov.ggt` WebView local storage before startup once.
- Keep ffmpeg outside the MSI under user-local app data. Download BtbN's latest
  LGPL Windows ZIP only after approval and verify its release SHA-256 manifest.
- Allow an existing ffmpeg/ffprobe pair to be selected in Settings.

### Changed

- Added Steam `PASTE`, first-steps warnings, and ffmpeg Install/Change controls.
- Added detected ffmpeg version to the Home status and Settings details.
- Added backend history load/save, legacy profile migration, persistent ffmpeg
  settings, managed discovery, bounded download, verification, and extraction.

### Validation

- Rust formatting and strict Clippy passed.
- Six Rust tests passed, including checksum-manifest and version-line parsing.
- Frontend syntax and workspace diagnostics passed.
- Real Tauri dev startup accepted the plugin, capability, and new commands.

### Remaining Work

- Complete a clean Windows runtime test of managed ffmpeg installation.
- A clean install cannot recover history if the user manually deletes the
  stable local app-data directory; normal MSI update/uninstall leaves it intact.

## 2026-08-28 - Standardize Windows Installer Format

### Request

Assess the risk of shipping MSI and NSIS together and select one Windows
installer format.

### Decisions

- Use MSI as the sole Windows installer because the installed `0.1.0` lineage
  is MSI and `0.1.2` already preserves its verified UpgradeCode.
- Do not migrate to NSIS for planned auto-updates; a second installer lineage
  risks duplicate installations and uninstall records.
- Accept elevation for machine-wide MSI updates and keep updater interaction
  visible through passive mode.

### Changed

- Removed NSIS from Tauri configuration, CI builds, artifact publishing, and
  active release documentation.
- Revised the updater plan around signed MSI upgrades and a permanent
  UpgradeCode assertion.
- Added a Windows CI check that opens the built MSI and rejects version or
  UpgradeCode drift before artifact upload.

### Validation

- Tauri reported the expected UpgradeCode override and built one MSI
  successfully.
- Installed `0.1.0` and rebuilt `0.1.2` both report machine-wide scope and
  UpgradeCode `{1DDF37BF-062F-547C-A167-DAB5D7867081}`; the new MSI has a
  distinct ProductCode and an upgrade-detection table entry.
- The local `release/gcc-app-0.1.2` folder contains only the validated MSI.
- Workflow and Tauri configuration diagnostics reported no errors.
- All tracked Markdown files passed `markdownlint-cli2`.

### Remaining Work

- Authenticode-sign public installers before broad distribution.

## 2026-08-28 - Safe Per-Game Trailer Folders

### Request

Group each game's Steam trailer downloads in a safe folder combining its Steam
ID and game name, reuse prior folders, prioritize the fix, and report the next
TODO item.

### Decisions

- Use the authoritative game name from Steam's `appdetails` response.
- Name new folders `<SteamID> <Safe_Game_Name>` using ASCII letters, digits,
  and underscores, with a 100-character title limit.
- Reuse an existing directory whose name begins with the Steam ID followed by
  a space, so a later Steam title change does not split one game's downloads.
- Keep broader asset-type subfolders tracked separately as `APP-08`.

### Changed

- Extended the Steam response and frontend download request with `gameName`.
- Added backend folder sanitization, numeric ID validation, same-ID lookup,
  directory creation, and the resolved-folder progress message.
- Marked `STM-11` shipped across feature, roadmap, TODO, and changelog docs.
- Bumped all application version sources to `0.1.2` for a Windows upgrade
  installer that can replace the existing `0.1.1` installation.
- Pinned the installed `0.1.0` MSI's original UpgradeCode after package
  inspection showed the generated `0.1.2` identity would install side-by-side.

### Validation

- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test`: 4 passed, 0 failed.
- `node --check app/src/main.js`: passed.

### Remaining Work

- Implement `STM-10`, the Steam clipboard `PASTE` button, next.
- Runtime-test folder creation with a real download before the next release.

## 2026-08-28 - Clipboard Idea and Feasibility Review

### Request

Add a Steam-tab clipboard paste idea, review TODO and feature planning, assess
whether the newly proposed creative-media ideas are achievable, and establish a
model-neutral pre-task/post-task handoff in `docs/`.

### Decisions

- Track the paste action as `STM-10`, a small near-term feature using Tauri's
  text-read-only clipboard permission after an explicit user click.
- Keep background removal and PressKit Maker in a later phased workspace.
- Treat still-image import/export and folder inventory as straightforward;
  prototype segmentation and animation before selecting production engines.
- Use this directory as the canonical current AI handoff while retaining older
  dated records in `../../ai-journal/` as historical context.

### Changed

- Updated `FEATURES.md`, `PLAN.md`, and `TODO.md` with `STM-10`, feasibility
  verdicts, implementation order, and corrected stale project statuses.
- Added this pre-task/post-task handoff workflow under `docs/ai-journal/`.
- Linked repository AI instructions and the legacy journal index to this
  canonical workflow.

### Validation

- All 19 Markdown files pass `markdownlint-cli2` with zero issues.
- All local Markdown links resolve, and TODO feature IDs resolve to
  `FEATURES.md`.
- No application code was changed in this task.

### Remaining Work

- Implement and package-test `STM-10` on Windows and Linux.
- Choose prototype engines before promoting `MED-02`, `PKT-02`, or `PKT-03` to
  ready status.

## Entry Template

```markdown
## YYYY-MM-DD - Short Task Name

### Request

One or two sentences describing the user's goal.

### Decisions

- Important technical or product decisions and why they were made.

### Changed

- Files or behavior changed at a summary level.

### Validation

- Commands or checks run and their result.

### Remaining Work

- Known follow-ups, blockers, or intentionally deferred work.
```
