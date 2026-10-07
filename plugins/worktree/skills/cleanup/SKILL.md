---
name: cleanup
description: Review and safely clean up managed Git worktrees, as /worktree:cleanup or when the operator asks an agent to clean them up. Hands off to worktree:managing-worktrees for the procedure.
argument-hint: "[--repo <primary>] [--id <id>...]"
---

# Clean up worktrees

Read `worktree:managing-worktrees` in full, then follow its *Finish and clean up* and *Audit and
recovery* sections for the trees named in `$ARGUMENTS`, or for the current repository if none are.

The operator's request authorizes cleanup in that scope: this command, or the same request in words
that an agent acts on. With it, review the dry-run results, apply the exact eligible ids, and report
removals and retention reasons without asking for another approval. Without such a request, stop
after the dry-run and report what it found.
