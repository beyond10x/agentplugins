# Hand-over: agentplugins-improvements, 2026-10-06

Dispatch DSP-20261006-12 (decision DEC-20261006-06) closes this session.

## Shipped

| What | Where |
|---|---|
| Release 0.20.1: worktree skill ends trees with `worktree finish --discard-cache --archive`; sub-agents finish their trees or return their id; AEP wave teardown archives unpublished work; `verified.json` worktree 0.9.0 | PR #61, main `efa67af`, tag `0.20.1`, GitHub Release by b10x-bot[bot] with 6 assets |
| Planning record | `task:worktree-finish-discard-cache` stays draft; neither its progress nor lifecycle records were written |

## State left behind

| Kind | Items |
|---|---|
| Branches | `fix/worktree-finish-discard-cache` on origin, merged into main; `handoff/2026-10-06-agentplugins-improvements` for this file |
| Managed worktrees | none open; `agentplugins-finish-discard-20261006` retired through `finish --discard-cache --archive` and gc |
| Unpushed commits | none |
| Open PRs | the PR carrying this file, until it merges |
| Open dispatches | none |

## Next steps

1. `verified.json` pins worktree 0.9.0, and worktree 0.10.0 (`worktree sweep`) was released the same day. `agentplugins-check tools` will report it. Re-verify with the `worktree-onboarding` trial and move the pin.
2. `verified.json` pins connectors v0.28.0 while v0.30.0 is out. Because of that, the non-required "Skills match the newest CLI releases" check fails on main. Re-verify the Connectors skills, run an ESS trial round, then move the pin.
3. The `worktree-onboarding` trial agent quoted three instructions as confusing:
   - `worktree:init`: "If it answers, go to step 2: `b10x:init` just installed it, or it was already there". It had not run `b10x:init`.
   - `worktree:init`: "`--help` calls it a "committed" template, but any local file works". It was unclear whether the template belongs in a repository.
   - `worktree:managing-worktrees`: "Without exact ids, `--repo` selects the activated workspace profile, not just the repository".
4. Record `task:worktree-finish-discard-cache` progress (the release facts above) and move it to implemented once its lifecycle records are written.

## Notes for the operator's machine

- All five plugins are installed from the local checkout through `B10X_MARKETPLACE`, in both Claude Code and Codex. A plain `b10x setup apply` without that variable reports "not converged" and would switch the marketplace back to GitHub.
- Twelve beyond10x repositories built into a shared `~/.cache/b10x-target`. DEC-20261006-02 moves the nine non-archived ones to in-tree builds through their owners.
