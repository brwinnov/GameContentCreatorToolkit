# AI Task Handoff

This directory is the model-neutral handoff point for GitHub Copilot, Claude
Code, and other AI coding assistants. It complements Git history and the older
dated records in [`../../ai-journal/`](../../ai-journal/); it is not a chat
transcript archive.

## Required Workflow

### Before a Task

1. Read [`PRE-TASK.md`](PRE-TASK.md).
2. Read the repository root [`AGENTS.md`](../../AGENTS.md), plus any
   tool-specific instruction file that applies.
3. Check `git status` and preserve existing user changes.
4. Read only the plans, source files, tests, and latest historical journal entry
   relevant to the requested task.
5. Update `PRE-TASK.md` when its durable project snapshot is stale.

### After a Task

1. Validate the changed behavior or documentation.
2. Update roadmap, TODO, changelog, or security documentation when relevant.
3. Add a concise newest-first entry to [`POST-TASK.md`](POST-TASK.md).
4. Refresh `PRE-TASK.md` if current status, priorities, constraints, or the next
   recommended action changed.
5. Record facts only: request, decisions, files changed, validation, remaining
   work, and commit/release identifiers when available.

Never place credentials, tokens, private user data, or full chat transcripts in
these files. Prefer paths and commit IDs over copied code or terminal output.
