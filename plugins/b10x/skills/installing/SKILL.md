---
name: installing
description: Install, upgrade, repair or migrate the Beyond10x agent plugins (aep, ess, worktree, connectors) and the binaries they drive, in Claude Code and Codex. Use when the user asks to set up, install, onboard, upgrade or update Beyond10x or any of its plugins, when a session-start line starting with `b10x:` reports drift or a legacy plugin, when a Beyond10x skill says its binary is missing or older than the plugin, or when the user asks which Beyond10x plugins are installed.
---

# Set up Beyond10x

The `b10x` binary decides; you converse. It reads what both hosts have installed, which binaries
are on `PATH`, and what the `b10x` marketplace serves now, and it prints exact actions. You show
the result, ask the user two questions, and apply only what they approved.

Never run `claude plugin …` or `codex plugin …` yourself for this work, and never edit plugin
settings by hand: `b10x setup apply` snapshots every file it changes so `b10x setup undo` can
restore them.

## 1. Make sure `b10x` runs

Run `b10x --version`. If it is missing, install it:

1. Target: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin` or
   `aarch64-apple-darwin`, from `uname -m` and `uname -s`.
2. Download `https://github.com/beyond10x/agentplugins/releases/latest/download/b10x-<target>.tar.gz`
   and `https://github.com/beyond10x/agentplugins/releases/latest/download/SHA256SUMS`.
3. Check the archive against `SHA256SUMS` (`sha256sum --check --ignore-missing`, or
   `shasum -a 256 --check --ignore-missing` on macOS). Stop if it does not match.
4. Extract it and move `b10x` into `~/.local/bin`. If `~/.local/bin` is not on `PATH`, say so and
   use the full path for the rest of this skill.

## 2. Plan

Plan for the host you run in: `--host claude` in Claude Code, `--host codex` in Codex. Add the
other host (`--host all`) only when the user says they use it too.

```bash
mkdir -p ~/.local/state/b10x
b10x setup plan --host claude --out ~/.local/state/b10x/plan.json
```

It prints a readable summary and writes the plan to the file; read the file. `offers` lists the products and which are preselected; `findings` says what was
found, each with a `level`; `actions` is what would run.

## 3. Show the state, then ask which products

Show one short table, one row per product (`offers`): its plugins' state and its binary's state,
taken from the `change`, `ok` and `warn` findings for that product. Under it, list every `warn`
finding in one line each — a shadowed binary copy, an orphan entry in a committed file, a version
that could not be resolved. Do not paste the raw JSON.

Then ask **one** multi-select question: which products to have. Preselect the offers with
`selected: true`; list `connectors` as optional. Use the host's question tool when it has one
(Claude Code: `AskUserQuestion` with `multiSelect: true`); otherwise ask in one line and wait.

If the answer differs from the preselection, plan again with exactly that set:

```bash
b10x setup plan --host claude --products aep,ess --out ~/.local/state/b10x/plan.json
```

`--products none` keeps only the `b10x` plugin. A product left out has its plugins removed from
user scope; say that before asking for confirmation.

## 4. Confirm once

If the plan has no action other than `refresh`, say "Beyond10x is current" with the product table
and stop.

Otherwise list every action with `kind` other than `refresh`, numbered, as one line each: what it
does and why. Call out three kinds separately:

- `install-binary` — which binary, which version, which directory, and that it replaces the copy there.
- `remove-setting` on a `.claude/settings.json` inside a project — a committed file; the user should
  review that diff.
- uninstalling a plugin — that it is being replaced, or removed because its product was not selected.

Ask one yes/no question. Anything but a clear yes means nothing is applied.

## 5. Apply

```bash
b10x setup apply --plan ~/.local/state/b10x/plan.json --yes
```

Pass `--yes` only after the user said yes to this exact list. `apply` refuses a plan whose installed
state changed since it was made; if it does, go back to step 2 and show what changed.

Relay the result in a few lines: the snapshot path, how many actions ran, and whether it
converged. On a failed action, show that action's line and error verbatim, and offer
`b10x setup undo` (it restores the snapshot `apply` printed).

## 6. Finish

- New plugins load in a new session: Claude Code after `/reload-plugins` or a restart, Codex in a
  new thread.
- To use one in this session, `b10x skill <plugin>` lists its skills and agents and
  `b10x skill <plugin>:<skill>` prints one; follow the printed text as if the skill were loaded.
- If a `warn` finding named a shadowed binary copy, repeat its path: it never runs, and removing it
  is the user's call.

## When the session starts with a `b10x:` line

The `b10x` plugin runs `b10x check` at session start. Each line it prints is one drift: a binary
older or newer than the plugin that describes it, or a legacy plugin still installed. When the user
asks about it, or before you use a skill whose binary it names, run this skill from step 2.
