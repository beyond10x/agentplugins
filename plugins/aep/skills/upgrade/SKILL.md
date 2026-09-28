---
name: upgrade
description: Check whether the AEP plugin and the `aep` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether AEP is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `aep`, or when a `aep` command behaves differently from what a AEP skill describes or names the planning store's `aep.project` version.
---

# Upgrade AEP

```bash
b10x upgrade aep --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It compares the installed `aep` plugin with what the marketplace serves and the `aep` on `PATH`
with the newest release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "AEP is current" with the versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill aep:<skill>` prints the new text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## An older planning store

The current `aep` reads `aep.project/5` (`store: {git: {}}`). Read `version` in the repository's
`.engineering/project.yaml`, tell the user which it is, and after a clear yes run the steps for it.
Each migration refuses a dirty `.engineering`, so commit what is there first.

**`aep.project/1`, or `.engineering/planning/` with no `project.yaml`.** The current release
migrates it; add `--protocols <source> --profile <profile>` when there is no `project.yaml`
(values in `aep:planning` § 5). `--dry-run` first shows what it would write.

```bash
aep plan store migrate git --verify
git add .engineering && git commit    # message names the migration
```

**`aep.project/2`, `/3` or `/4`.** The current release refuses these stores. Migrate with the pinned
build that still reads them, then return to the current release:

```bash
cargo install --git https://github.com/beyond10x/aep --rev 9c0f1da44429ff935fa0b2d743457945d51e1c51 aep-cli
aep plan store migrate git --verify
git add .engineering && git commit    # message names the migration
b10x install aep --method cargo       # replaces the pinned build in ~/.cargo/bin with the newest release
```

Done when `aep --version` prints the newest release, `version` reads `aep.project/5`, and
`aep plan artifact validate` exits 0. A `--verify` difference exits non-zero: relay it and stop.

## Next

- Continue the work that prompted the check; `aep:init` lists the skills.
