---
name: upgrade
description: Check every installed Beyond10x plugin and CLI against what is newest, and upgrade them with the user's confirmation. Use when the user asks whether Beyond10x is up to date or to upgrade or update it, or when a session-start line starting with `b10x:` reports drift, a legacy plugin or an old check.
---

# Upgrade Beyond10x

```bash
b10x upgrade --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It checks each installed product — plugin against the marketplace, CLI on `PATH` against the newest
release — plus earlier installs under retired names, and prints each difference with the action that
fixes it. Nothing is changed yet. One product only: `b10x upgrade ess --host claude --out …`. A CLI the repository pins
in `b10x.toml` stays at its pin: the plan names a newer release but does not install it
(`b10x unpin <cli>` follows the newest again, unless a local pin holds that CLI).

A CLI held back by a local pin stays at its pin the same way, in every directory; the plan says
which pin holds it and why. A session-start line saying a CLI "is held back by a local pin" is not
drift: leave that CLI as it is, and do not remove the pin or install a newer release over it. The
hold is the user's.

## Hold a CLI back on this machine

The local pin file is `~/.config/b10x/pins.toml` (`$XDG_CONFIG_HOME/b10x/pins.toml` when that is
set; `$B10X_PINS` names another file by its absolute path). Its `[pins]` table takes the same versions as `b10x.toml`,
with an optional reason:

```toml
[pins]
aep = { version = "0.71.3", reason = "0.72.0 refuses this machine's planning stores" }
ess = "0.59"
```

For the same CLI, the local pin wins over a repository's `b10x.toml`. The user edits this file;
removing a line follows the newest release again. A file that does not parse makes `b10x check`
print one warning line and makes the upgrade plan refuse until it is fixed.

- Nothing to change: say "Beyond10x is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- New plugin versions load in a new session; until then `b10x skill <plugin>:<skill>` prints the
  new text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Add a product: `/b10x:init`.
