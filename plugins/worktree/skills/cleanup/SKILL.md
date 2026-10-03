---
name: cleanup
description: Review and safely clean up managed Git worktrees, started by the operator as /worktree:cleanup. Hands off to worktree:managing-worktrees for the procedure.
disable-model-invocation: true
argument-hint: "[--repo <primary>] [--id <id>...]"
---

# Clean up worktrees

Read `worktree:managing-worktrees` in full, then follow its *Finish and clean up* and *Audit and
recovery* sections for the trees named in `$ARGUMENTS`, or for the current repository if none are.

Invoking this command authorizes cleanup in that scope. Review the dry-run results, apply the exact
eligible ids, and report removals and retention reasons without asking for another approval.
