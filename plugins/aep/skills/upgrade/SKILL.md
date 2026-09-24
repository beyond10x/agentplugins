---
name: upgrade
description: Check whether the AEP plugin and the `aep` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether AEP is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `aep`, or when a `aep` command behaves differently from what a AEP skill describes.
---

# Upgrade AEP

```bash
b10x upgrade aep --out ~/.local/state/b10x/plan.json
```

It compares the installed `aep` plugin with what the marketplace serves and the `aep` on `PATH`
with the newest release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "AEP is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill aep:<skill>` prints the new text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check; `aep:init` lists the skills.
