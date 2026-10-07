---
name: init
description: Start with worktree in this project — make sure the `worktree` CLI is available and take the first step. worktree is isolated Git worktrees with leases, recovery proof and safe cleanup. Use when the user wants isolated checkouts for agent work, asks to set up or install worktree, or when `worktree:managing-worktrees` reports that `worktree` is missing. Installs CLIs only after the user confirms the plan.
---

# Start with worktree

## 1. Have the CLI

Run `worktree --version`. If it prints a version, the CLI is installed (by `b10x:init` or earlier); go to step 2, and use `worktree:upgrade` later for newer releases. If it is missing, install it with `b10x`.
Plan for the host you run in (`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init worktree --host claude --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: the prebuilt, checksummed release archive
by default, or `cargo` (a source build) when the user asks for it and a Rust toolchain is on `PATH`.
When both are possible, say which is planned and offer the other; re-run with `--method cargo` if
they choose it, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first; its step 1 asks the user before it installs `b10x`.

## 2. First step here

Activate a workspace profile once, then check the setup. The profile is a small template; the
worktree repository publishes a default one:

```bash
curl -fsSL --create-dirs -o ~/.config/worktree/default.toml \
  https://raw.githubusercontent.com/beyond10x/worktree/main/profiles/default.toml
worktree activate --profile ~/.config/worktree/default.toml --workspace <workspace-root>
worktree doctor --check
```

`activate` copies the profile into `~/.config/worktree/config.toml`. The downloaded file stays a
local file under `~/.config/worktree/`; do not commit it into the user's repository. Ask the user
for the workspace root (an absolute path to the directory that holds their repositories) rather
than guessing it. With nobody to ask (a headless run), use the parent directory of the current
repository's root (for `/src/app`, that is `/src`) and say so in the report.

`worktree doctor --check` fails with `no active profile` until `activate` has run.

Add `--install-agent-guidance` to `activate` only when the user asks for it: it writes a managed
guidance block, pointing at `worktree:managing-worktrees`, into `~/.claude/CLAUDE.md` and
`~/.codex/AGENTS.md`, and replaces only the text between its own markers.

**Cleanup needs recovery proof.** Publish commits to a remote, or preserve unpublished work with
`worktree archive <tree>`. A repository without a remote can use a verified archive while its tree
still matches it. Follow `worktree:managing-worktrees` to end owned leases, finish the tree and
review exact GC IDs; a local merge alone is not recovery proof.

## 3. Pick the work

| the task | skill |
|---|---|
| create, lease, finish, inspect or clean a worktree | `worktree:managing-worktrees` |

A skill that is not loaded yet in this session prints with `b10x skill worktree:<skill>`.

## Next

- Start work in an isolated checkout: `worktree:managing-worktrees`.
- Later, `worktree:upgrade` checks for a newer plugin and CLI.
