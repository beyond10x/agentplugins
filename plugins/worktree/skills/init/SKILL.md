---
name: init
description: Start with worktree in this project — make sure the `worktree` CLI is available and take the first step. worktree is isolated Git worktrees with leases, recovery proof and safe cleanup. Use when the user wants isolated checkouts for agent work, asks to set up or install worktree, or when `worktree:managing-worktrees` reports that `worktree` is missing.
---

# Start with worktree

## 1. Have the CLI

Run `worktree --version`. If it is missing or `b10x` says it is behind, install it with `b10x`:

```bash
b10x init worktree --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: `cargo` when a Rust toolchain is on
`PATH`, otherwise a prebuilt, checksummed release archive. Ask the user which method they want if
both are possible, re-run with `--method cargo` or `--method prebuilt` to match, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## 2. First step here

Activate a workspace profile once, then check the setup:

```bash
worktree activate --profile <profile.toml> --workspace <workspace-root>
worktree doctor --check
```

`worktree doctor --check` names anything missing. Ask the user for the workspace root (the directory
that holds their repositories) rather than guessing it.

## 3. Pick the work

| the task | skill |
|---|---|
| create, lease, finish, inspect or clean a worktree | `worktree:managing-worktrees` |

A skill that is not loaded yet in this session prints with `b10x skill worktree:<skill>`.

## Next

- Start work in an isolated checkout: `worktree:managing-worktrees`.
- Later, `worktree:upgrade` checks for a newer plugin and CLI.
