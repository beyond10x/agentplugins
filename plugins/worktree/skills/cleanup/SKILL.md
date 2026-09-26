---
name: cleanup
description: Review and safely clean up managed Git worktrees, started by the operator as /worktree:cleanup. Hands off to worktree:managing-worktrees for the procedure.
disable-model-invocation: true
argument-hint: "[--repo <primary>] [--id <id>...]"
---

# Clean up worktrees

Read `worktree:managing-worktrees` in full, then follow its *Finish and clean up* and *Audit and
recovery* sections for the trees named in `$ARGUMENTS`, or for the current repository if none are.

1. `worktree inspect --repo <primary>` (add `--id <id>` per tree) and read every retention blocker.
2. For a tree whose work must not be published, `worktree archive <tree>` instead of publishing.
3. Release only your own lease, then `worktree finish <tree>` for each tree that is done.
4. `worktree gc --repo <primary> --dry-run --id <id>` for each tree, and show the results as a table.
5. `worktree gc --repo <primary> --apply --id <reviewed-id>` only for the ids the operator approved
   from that table.

Never force a removal, delete a tree by hand, or clear another session's lease. End with every tree
that was kept and the reason the dry-run or inspect gave for keeping it.
