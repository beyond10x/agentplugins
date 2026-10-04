---
name: upgrade
description: Check whether the Connectors plugin and the `connectors` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether Connectors is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `connectors`, or when a `connectors` command behaves differently from what a Connectors skill describes.
---

# Upgrade Connectors

```bash
b10x upgrade connectors --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It compares the installed `connectors` plugin with what the marketplace serves and the `connectors` on `PATH`
with the newest stable release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "Connectors is current" with the versions, and stop.
- Otherwise show the actions in one list and reuse existing authorization covering them. Ask once
  only for a missing decision; after authorization:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill connectors:<skill>` prints the new text.

Use the installed receipt or source pin when this CLI has no version flag. Verify command help
and used operation contracts after the Cargo installation; release assets are not assumed.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check; `connectors:init` lists the skills.
