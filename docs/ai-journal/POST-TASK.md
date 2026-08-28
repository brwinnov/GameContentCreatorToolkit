# Post-Task Journal

Add completed tasks directly below this introduction, newest first. Keep each
entry concise enough for another AI assistant to understand what changed and
what remains without reading a chat transcript.

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
