---
name: init
description: Start with worktree in this project — make sure the `worktree` CLI is available and take the first step. worktree is isolated Git worktrees with leases, recovery proof and safe cleanup. Use when the user wants isolated checkouts for agent work, asks to set up or install worktree, or when `worktree:managing-worktrees` reports that `worktree` is missing.
---

# Start with worktree

## 1. Have the CLI

Run `worktree --version`. If it answers, go to step 2: `b10x:init` just installed it, or it was already there (`worktree:upgrade` handles newer releases). If it is missing, install it with `b10x`:

```bash
b10x init worktree --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: `cargo` when a Rust toolchain is on
`PATH`, otherwise a prebuilt, checksummed release archive. Ask the user which method they want if
both are possible, re-run with `--method cargo` or `--method prebuilt` to match, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## 2. First step here

Activate a workspace profile once, then check the setup. The profile is a small template; the
worktree repository publishes a default one:

```bash
mkdir -p ~/.config/worktree
curl -fsSL -o ~/.config/worktree/default.toml \
  https://raw.githubusercontent.com/beyond10x/worktree/main/profiles/default.toml
worktree activate --profile ~/.config/worktree/default.toml --workspace <workspace-root>
worktree doctor --check
```

`activate` copies the profile into `~/.config/worktree/config.toml`, so the template file may stay
where it is or be committed into a repository the user shares with others. Ask the user for the
workspace root (an absolute path to the directory that holds their repositories) rather than
guessing it.

`worktree doctor --check` exits 0 even when no profile is active; read its `profiles=` line and
treat `profiles=0` as not set up.

**A repository needs a remote before its trees can be cleaned up.** Cleanup proves that each
commit reached a remote, so in a repository with no remote (`git remote` prints nothing) `create`
works and every later `gc` refuses. Tell the user before they start work there.

## 3. Pick the work

| the task | skill |
|---|---|
| create, lease, finish, inspect or clean a worktree | `worktree:managing-worktrees` |

A skill that is not loaded yet in this session prints with `b10x skill worktree:<skill>`.

## Next

- Start work in an isolated checkout: `worktree:managing-worktrees`.
- Later, `worktree:upgrade` checks for a newer plugin and CLI.
