---
name: managing-worktrees
description: Safely create, inspect, finish, reconcile, recover, and garbage-collect managed Git worktrees. Use whenever an agent needs an isolated checkout for repository changes, must hand off a worktree, or needs to audit, recover, or clean linked worktrees.
---

# Managing worktrees

Use the `worktree` CLI as the sole owner of linked-worktree lifecycle. It keeps trees outside primary checkout collections and refuses cleanup without current recovery proof.

## Start repository work

1. From a primary checkout run `worktree create --purpose <short-purpose>`. Add `--repo <path>`, `--base <revision>`, or `--id <stable-id>` when needed.
2. Treat the printed path as the task checkout and do all changes there. The tree starts on a detached HEAD; run `git switch -c <branch>` in it before the first commit, so the work has a branch to push.
3. If already inside a managed tree, reuse it; do not nest another worktree.
4. For automation, add `--json` and consume the versioned output.

## Maintain the lease

Acquire a lease before changing the tree: run `worktree hook session-start --path <tree> --session <session-id>`. Use a stable id unique to this session. If the host does not demonstrably run lifecycle hooks, run these commands yourself; loading this skill does not install hooks.

Run `worktree hook heartbeat --path <tree> --session <session-id>` periodically during long work, before the configured lease expiry, including while builds or external checks are running. Check each result. A live lease blocks cleanup; a missing or expired lease does not prove that another session has stopped.

Release only your own lease with `worktree hook session-end --path <tree> --session <session-id>` when leaving or immediately before finish. Never clear another session's lease to make cleanup pass.

## Bound disposable storage

Before a large build, inspect free space and `worktree inspect --repo <primary> --id <id>`. Keep compiler caches and dependencies separate from source and retained evidence. Prefer a repository-supported cache location and bounded build settings; do not force a shared target directory across incompatible build configurations.

After verification, preserve the small logs, reports, or deliverables needed for review in their intended durable location, never only below an ignored build directory such as `target/`. Delete build output with `worktree discard-cache [<tree>]`; run it with `--dry-run` first to see the classification. It deletes only ignored directories it recognises as cache by their structure: Cargo profiles inside a tagged target, `node_modules` at or below a tracked lockfile, a virtual environment beside a tracked Python manifest, and tagged `.pytest_cache`, `.mypy_cache` and `.ruff_cache`. Every other ignored entry is kept and named. It refuses while a lease is live or another process uses the tree (`worktree-in-use`, naming the processes). You do not need to establish who built the cache: the structure decides. Ignored files can contain valuable work: never blanket-delete them, delete a build directory by name, or use `git clean -fdx`.

## Finish and clean up

An explicit cleanup request authorizes removal of eligible trees in the requested scope. Inspect
that scope, review the dry-run results yourself, and proceed to exact-id apply without another
approval prompt. A request only to inspect, review, or dry-run ends with the findings. Retain trees
with unresolved ownership, recovery, lease, lock, or work-state blockers and report the reason;
cleanup authorization does not waive those checks or authorize publishing someone else's work.

1. Commit and publish every wanted change. A local-only commit is deliberately not cleanup-safe. Work merged as rebased or cherry-picked copies also qualifies when an advertised ref carries every unique commit's exact patch; GC reports that proof as `patch-equivalent`.
   When work must not be published, run `worktree archive <tree>` instead. It never modifies the tree; it writes `commits.bundle` (every commit no advertised ref holds), `dirty.patch` (tracked, untracked and ignored changes over HEAD) and a `worktree.archive/1` `manifest.json` below the state directory's `worktree/archives/<repository>/<id>/`, and verifies them. GC then accepts that archive as `archive` proof while HEAD and every file still match it exactly; any later commit or edit is refused as `archive-stale` until `worktree archive --replace <tree>` writes a new one. `--replace` moves the old archive aside and never deletes it. Every ignored file is archived, so discard the build cache first; `worktree finish --discard-cache --archive` in step 2 does both in that order; submodules, nested repositories, special files and paths containing a newline are refused as `archive-unsupported-entry`. GC apply returns an archived dirty tree to HEAD, removes it without force, and keeps the archive.
2. Preserve required evidence. Release your own lease, then run `worktree finish --discard-cache --archive <tree>`. It deletes the recognised build cache, archives whatever the tree still holds that no advertised ref recovers (uncommitted or ignored files, commits that are not published), and finishes, so nothing that is not cache is lost. Without `--archive` it refuses a tree that still differs from HEAD and names what was kept. Every form refuses locked, unmanaged, live, or mid-operation Git worktrees.
3. Run `worktree gc --repo <primary> --dry-run --id <id>` and inspect every result. Always pass `--id`. Without exact ids, `--repo` selects the activated workspace profile, not just the repository: the assessment covers records under that profile's `workspace_root`, including other repositories.
4. Run `worktree gc --repo <primary> --apply --id <reviewed-id>` with repeated `--id` values only for the exact results intended for removal. The command refreshes remote advertisements, fetches required objects, and revalidates immediately before non-forced removal. Check the result before reporting storage reclaimed.
5. End with either verified cleanup or an explicit handoff: tree id and path, published branch/commit, related work-item references, retained evidence, remaining blockers, next owner and next action. Never leave a tree silently active or label work complete merely from its age or Git state.

A tree belongs to the session that created it. A sub-agent that creates a tree runs step 2 on it before it returns, or, when the tree must outlive it, returns the tree id and path in its report. A parent finishes every tree its sub-agents return before its own work ends: their sessions are gone, and nobody else knows the trees exist.

## Audit and recovery

- Run `worktree inspect --repo <path>` for actual Git state, separate ignored-file counts, storage, leases, and retention blockers. It defaults to that repository; add `--workspace` to expand to its profile and repeat `--id` to narrow the selection. Sizes are bounded observations, not promised reclaimable bytes. Add `--refresh` for fresh remote recovery evidence (which may fetch objects). Inspection never changes lifecycle or infers owner abandonment or story completion; review GC separately before removal.
- Run `worktree status` for durable lifecycle state. It accepts no filter and reports every record in every profile, so read `repository_root` on each one before acting.
- Run `worktree repo list --repo <path>` to distinguish managed, unmanaged, primary, and linked checkouts.
- Run `worktree reconcile --repo <path> --dry-run` to assess interrupted provisioning, adopted legacy paths, finished external trees, and missing records.
- Apply reconciliation only to ids copied from that immediately preceding dry-run with `worktree reconcile --repo <path> --apply --id <reviewed-id>` and repeated `--id` arguments when needed.
- If that dry-run explicitly proposes `retire-external`, confirm that destructive action separately by adding `--allow-external-retirement`; never add it for an unrelated migration or missing-record repair.
- A finished external legacy tree may supersede a stale migration intent only when the dry-run itself proposes `retire-external`; never reinterpret or bypass a cross-device or ambiguous-relocation refusal.
- If removal is interrupted while the path still exists, rerun GC dry-run and exact-id apply; it finishes a removal Git already unlinked only when every remaining file is the commit's tracked content, and otherwise retains the tree as `removal-residue-unproven`. If the path is already absent, use reconciliation dry-run and exact-id apply; its durable removal intent can safely finish the recorded transition.
- A missing Active record without matching durable removal intent stays refused while its work may still exist. Preserve and investigate its registry evidence; never edit the registry by hand, delete related state, or fabricate recovery proof. If its recorded commit still exists anywhere, publish it and rerun the dry-run.
- Only once you have established that such a record's recorded commit is gone for good, abandon it with `worktree reconcile --repo <path> --apply --id <reviewed-id> --acknowledge-unrecoverable <recorded-commit>`. That acknowledgement asserts one exact commit named by the immediately preceding dry-run; the command still checks it and refuses while any local branch, tag, remote-tracking ref, or remote advertisement contains it. It deletes nothing from disk or from Git, and records the tombstone with no recovery proof, because there is none to record.
- A record whose repository was deleted (its root is gone, or has no `.git`) is reported as `repository-missing`, naming the repository, the tree path and the recorded commit. Git cannot check anything for it, so the dry-run's own `--acknowledge-unrecoverable <recorded-commit>` apply, run from any live repository of the same workspace as `--repo`, is the only way to retire it; select it with `--id`. It is refused as `worktree-path-exists` while the tree path, or a relocation or removal intent's path, still exists: deal with that tree yourself first. Never recreate the repository just to make reconciliation run.
- An archive outlives the tree it retired. Restore it from a `--no-checkout` clone that has the advertised refs: first write `* -text -eol -filter -ident -working-tree-encoding` to `.git/info/attributes` so that attributes cannot rewrite the archived bytes, then `git fetch <archive>/commits.bundle refs/worktree-archive/head:refs/heads/<name>`, and run both `switch <name>` and, when the archive has one, `apply --binary --whitespace=nowarn <archive>/dirty.patch` as `git -c core.autocrlf=false -c core.fileMode=true -c core.symlinks=true …`. Never use `--attr-source` for this: Git 2.55 `apply` crashes with it. Never delete an archive to make GC pass; `archive-digest-mismatch`, `archive-incomplete`, `archive-bundle-invalid` and `archive-invalid` mean it no longer proves recovery, and `archive-stale` means the tree changed after it was written.
- `worktree-hidden-state` (assume-unchanged or skip-worktree entries, staged content only the index holds, a nested `.git`) and `worktree-local-refs` (refs under `refs/worktree/`, `refs/bisect/`, `refs/rewritten/`) retain a tree whether or not it is archived, because Git status does not show that state and removal would destroy it. Resolve the named state yourself; never clear it just to make GC pass.
- Run `worktree doctor --check` for prerequisites and configuration. It exits non-zero and names each failure, including `no active profile` when no workspace profile is activated.
- Only after a human explicitly decides an existing linked tree should become manager-owned, run `worktree repo adopt --repo <primary> --path <linked-tree> --id <stable-id> --purpose <purpose>`. Then review `reconcile --dry-run` and use exact-id apply only if migration is intended.

Never run `git worktree remove --force`, recursively delete a linked tree, place managed trees below the primary workspace, or clean up a tree merely because it looks old.

## Next

- `worktree` missing or behind: `worktree:init`, later `worktree:upgrade`.
