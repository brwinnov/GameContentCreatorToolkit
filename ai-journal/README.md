# Historical AI Journal

The canonical current handoff workflow is now
[`../docs/ai-journal/`](../docs/ai-journal/). Read its `PRE-TASK.md` before new
work and update its `POST-TASK.md` after validation. The dated entries in this
directory remain historical decision records.

A running log of AI-assisted work sessions on this project — what was asked,
what was decided, what was built, and why. This is context for future-you
(and future AI sessions, including Claude Code reading this repo cold) to
pick up the thread without re-deriving decisions from scratch.

## Format

One file per session (or per meaningful chunk of work), named:

```text
YYYY-MM-DD-short-slug.md
```

Each entry should briefly cover:

- **Context**: what prompted the session
- **Decisions made**: especially anything that forecloses other options
  (e.g. "chose Tauri over Electron because...")
- **What changed**: files/structure touched, at a summary level (git history
  has the details — this is the *why*, not the *diff*)
- **Open questions / follow-ups**: things intentionally left for later

Keep entries honest and short. This is a decision log, not a transcript.
