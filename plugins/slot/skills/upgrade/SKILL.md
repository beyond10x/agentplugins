---
name: upgrade
description: Check whether the slot plugin and the `slot` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether slot is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `slot`, or when a `slot` command behaves differently from what a slot skill describes.
---

# Upgrade slot

```bash
b10x upgrade slot --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It compares the installed `slot` plugin with what the marketplace serves and the `slot` on `PATH`
with the newest release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "slot is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill slot:<skill>` prints the new text.

## The running daemon

A replaced binary does not change the daemon that is already running; it keeps the old one until it
restarts. A restart cancels every queued and running build of every session, so restart only when
no build is active: `curl -fsS http://127.0.0.1:7411/api/state` shows an empty `active` list. Then,
with the user's yes, `systemctl --user restart slot.service`, and check the same `curl` line answers.
A build server's daemon is upgraded the same way on that machine.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check; `slot:init` lists the skills.
