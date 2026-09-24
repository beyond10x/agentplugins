---
name: upgrade
description: Check every installed Beyond10x plugin and CLI against what is newest, and upgrade them with the user's confirmation. Use when the user asks whether Beyond10x is up to date or to upgrade or update it, or when a session-start line starting with `b10x:` reports drift, a legacy plugin or an old check.
---

# Upgrade Beyond10x

```bash
b10x upgrade --out ~/.local/state/b10x/plan.json
```

It checks each installed product — plugin against the marketplace, CLI on `PATH` against the newest
release — plus earlier installs under retired names, and prints each difference with the action that
fixes it. Nothing is changed yet. One product only: `b10x upgrade ess`.

- Nothing to change: say "Beyond10x is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- New plugin versions load in a new session; until then `b10x skill <plugin>:<skill>` prints the
  new text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Add a product: `/b10x:init`.
