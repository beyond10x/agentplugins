---
name: upgrade
description: Check whether the worktree plugin and the `worktree` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether worktree is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `worktree`, or when a `worktree` command behaves differently from what a worktree skill describes.
---

# Upgrade worktree

```bash
b10x upgrade worktree --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It compares the installed `worktree` plugin with what the marketplace serves and the `worktree` on `PATH`
with the newest release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "worktree is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill worktree:<skill>` prints the new text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check; `worktree:init` lists the skills.
