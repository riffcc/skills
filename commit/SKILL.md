---
name: riff-commit
description: Create atomic git commits by reviewing uncommitted changes, grouping them by logical concern, and committing each group with concise why-focused messages.
---

# riff-commit

Use this skill to turn a dirty git worktree into one or more clean commits.

## Workflow

1. Inspect the repo state.
   - `git status --short`
   - `git diff`
   - `git diff --cached`
2. Group changes by logical concern.
3. Stage only the relevant files or hunks.
4. Commit each group with a concise message that explains why the change exists.
5. Summarize what landed.

## Rules

- Keep implementation and its tests together.
- Split unrelated work into separate commits.
- If boundaries are unclear, ask instead of guessing.
- Do not commit changes you do not understand.
- Do not revert or rewrite user work unless explicitly requested.
- Prefer several small commits over one muddy commit.
