---
name: init
description: Guided onboarding for Beyond10x — ask what the user wants to do (plan and deliver work, write specifications, isolated Git worktrees, integrations), then install the matching plugins and their CLIs in Claude Code or Codex, migrating any earlier Beyond10x install. Use when the user wants to set up, install or onboard Beyond10x, points at github.com/beyond10x/agentplugins, asks which Beyond10x plugins exist, or when a session-start line starting with `b10x:` says nothing is set up.
---

# Set up Beyond10x

The `b10x` CLI decides; you converse. It reads what is installed on this host, which CLIs are on
`PATH`, and what the `b10x` marketplace serves, and prints exact actions. You ask the user two
questions, show one list of changes, and apply it only after they confirm.

Never run `claude plugin …` or `codex plugin …` yourself for this, and never edit plugin settings by
hand: `b10x setup apply` snapshots every file it changes, and `b10x setup undo` restores them.

## 1. Have `b10x`

Run `b10x --version`. Missing: follow
https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md step 1, then come back.

## 2. Ask what the user wants to do

Ask **one** multi-select question: *What do you want to do with Beyond10x?*

| option | installs |
|---|---|
| Plan and deliver work (epics, stories, reviewed implementation waves) | `aep` |
| Write specifications of a system or API (schemas, OpenAPI, conformance) | `ess` |
| Work in isolated Git checkouts that clean up safely | `worktree` |
| Connect external tools and providers | `connectors` |

Preselect what the user already named ("I want to write specs" → specifications). When planning is
chosen, preselect specifications too and say why: AEP planning models every new noun as an ESS
domain before writing stories around it, so `aep` without `ess` stops at the first new entity. Use the host's
question tool when it has one (Claude Code: `AskUserQuestion` with `multiSelect: true`).

## 3. Ask how to install the CLIs

```bash
command -v cargo
```

Ask: *How should the command-line tools be installed?* — **cargo** (builds from the release tag;
offered first when `cargo` is on `PATH`) or **prebuilt** (downloads the release's checksummed
archive; no Rust toolchain needed). Without `cargo`, say prebuilt is used and skip the question.

## 4. Plan and confirm

Plan for the host you run in (`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init <products> --method <cargo|prebuilt> --host claude --out ~/.local/state/b10x/plan.json
```

`<products>` is the answer to step 2, comma separated (`ess`, or `aep,worktree`); `--method` is the
answer to step 3.

It prints a summary and writes the plan to the file. List every change it names in one numbered
list, one line each, and call out: CLIs it installs or replaces (which version, where), earlier
installs it replaces, and anything under `warn` (a shadowed CLI copy, an edit to a committed file).
Ask one yes/no question; anything but a clear yes changes nothing.

## 5. Apply

```bash
b10x setup apply --plan ~/.local/state/b10x/plan.json --yes
```

Pass `--yes` only after the user said yes to this exact list. It refuses a plan whose installed
state changed since it was made; plan again if it does. Relay the snapshot path, how many actions
ran and whether it converged; on a failure, the failing line verbatim and `b10x setup undo`.

## 6. Hand over

New plugins load in a new session (Claude Code: `/reload-plugins` or restart; Codex: a new thread).
Until then `b10x skill <plugin>` lists a plugin's skills and `b10x skill <plugin>:init` prints the
first one; follow it as if it were loaded.

## Next

- Each installed product starts with its own `init`: `/aep:init`, `/ess:init`, `/worktree:init`,
  `/connectors:init`.
- `/b10x:upgrade` checks everything later; the session-start `b10x:` line says when it is due.
