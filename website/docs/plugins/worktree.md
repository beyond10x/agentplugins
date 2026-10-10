---
sidebar_position: 5
title: Worktree
---

# `worktree`

Use this plugin whenever an agent needs an isolated Git checkout, hands work to another session,
or audits old linked worktrees.

| skill | for |
|---|---|
| `worktree:init` | install the `worktree` CLI, activate a workspace, check it |
| `worktree:managing-worktrees` | create, lease, finish, inspect and clean worktrees |
| `worktree:cleanup` | command: `/worktree:cleanup`, or an agent acting on your request, reviews the managed trees, garbage-collects the eligible ids by exact id, and reports what it kept and why; without a request it stops after the dry-run |
| `worktree:upgrade` | check the plugin and CLI, offer the upgrade |

```text
/plugin install worktree@b10x
```

Codex: `codex plugin add worktree@b10x`. Then `/worktree:init` installs the CLI — the prebuilt,
checksummed archive from the [worktree release](https://github.com/beyond10x/worktree/releases), or with `cargo` on request —
and runs:

```bash
worktree activate --profile <profile.toml> --workspace <workspace-root>
worktree doctor --check
```

The skill teaches agents to create trees outside the primary repository collection, maintain live
session leases, and publish wanted commits before finishing. Cleanup is review-bound: inspect
`worktree gc --repo <primary> --dry-run`, then pass only ids from that review to
`worktree gc --repo <primary> --apply --id <reviewed-id>`. Locked, live, offline, unmanaged, and
out-of-policy trees are retained, and so are dirty and local-only trees unless `worktree archive`
holds their exact current state. Work merged as rebased or cherry-picked copies is recoverable when
an advertised ref carries every unique commit's exact patch.

Agents end their work with `worktree finish --discard-cache --archive <tree>` (worktree 0.9.0 or
later). It deletes only ignored build cache recognised by structure (Cargo profiles in a tagged
target and its `tmp/` test scratch, `node_modules` below a tracked lockfile, a virtual environment
beside a tracked Python manifest, tagged tool caches), archives everything else the tree holds that
no remote ref recovers, and finishes. Since worktree 0.12.1 an untagged `target/` beside a tracked
`Cargo.toml` counts as tagged once a profile in it holds `.fingerprint/` and `deps/`. A directory's
name never makes it cache: records written into `target/` are kept.
`worktree discard-cache --dry-run` shows the classification without deleting anything.

Since worktree 0.12.0 an archive also images each nested Git repository in the tree, such as a
test fixture in an ignored directory, as `nested-<n>.tar` (manifest `worktree.archive/2`), restored
with `tar -xpf`. Submodules, linked worktrees and nested repositories that share state with another
repository are still refused as `archive-unsupported-entry`. Plain `worktree finish` refuses a
dirty tree unless its archive holds exactly the current state.

`worktree sweep --all-profiles` (worktree 0.10.0 or later) does the same for trees nobody
finished. Run daily from a timer, it discards the recognised build cache of every tree idle for a
day or more without a live lease, and archives what an expired or finished tree still holds. It
never changes lifecycle, removes a tree or applies GC: a swept tree shows as eligible in
`worktree gc --dry-run`, and removal stays an exact-id apply. `--dry-run` shows what it would do.

Since worktree 0.13.0, `worktree gc` without `--id` assesses only the current repository's trees;
`--scope profile` assesses every repository in the profile. `finish`, `discard-cache`, `archive`,
`gc --id` and `reconcile --id` all accept a tree's id, its path or its unique directory name, and
`finish` prints `finished <id> <path>`, either of which `gc --id` takes.

Since worktree 0.14.0, `worktree prune-archives --repo <primary> --dry-run` lists each archive
with its bytes and verdict (`--scope profile` for every archive). Only a `Removable` archive is
deleted, with `worktree prune-archives --repo <primary> --apply --id <reviewed-directory>`: its
tree is gone, it holds no `dirty.patch` and no nested image, and HEAD and every unique commit it
records are ancestors of a ref a remote freshly advertises. Every other verdict is kept and no flag
forces a removal; deleting such an archive by hand loses the only copy of its work.

Since worktree 0.15.0, `worktree archive` (and `finish --archive`, and `sweep`) leaves cargo's
build layout below a recognised target out of `dirty.patch` and records it as `build_output` in a
`worktree.archive/3` manifest; other files under `target/` stay archived. An older archive kept as
`UncommittedState` only because its patch holds build output shrinks with
`worktree prune-archives --strip-build-output --repo <primary> --dry-run`, then `--apply --id
<reviewed-directory>`; it rewrites the patch, verifies it over HEAD and never deletes an archive.

`/worktree:cleanup [--repo <primary>] [--id <id>…]` starts that review by name. It is a command:
you start it, or an agent starts it when you ask, and it hands off to `worktree:managing-worktrees`
for every step — `inspect`, `archive` for work that must not be published, `finish`,
`gc --dry-run --id`, then `gc --apply --id` for the eligible ids. Your request authorizes that
apply; without one it stops after the dry-run. It never forces a removal.

`worktree inspect --repo <primary>` reports actual Git state, storage, ignored files, leases and
retention blockers. It defaults to one repository; use `--workspace` to inspect the wider profile.
Add `--refresh` for current remote recovery evidence. Inspection does not infer story completion
or authorize removal.

Use `worktree reconcile --repo <primary> --dry-run` for interrupted provisioning, adopted legacy
paths, finished external trees, and already-missing records. Apply only exact reviewed ids.
External retirement additionally requires the explicit `--allow-external-retirement`
acknowledgement. A missing record whose recorded commit is gone for good, including one whose
repository was deleted (`repository-missing`), is retired only with
`--acknowledge-unrecoverable <recorded-commit>`, which the command refuses while any ref still
contains that commit.

The plugin contains no cleanup script and no independent policy copy; `agentplugins-check tools`
checks every command it spells against the newest `worktree` release.
