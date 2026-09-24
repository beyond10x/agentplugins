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
| `worktree:upgrade` | check the plugin and CLI, offer the upgrade |

```text
/plugin install worktree@b10x
```

Codex: `codex plugin add worktree@b10x`. Then `/worktree:init` installs the CLI — with `cargo`, or
the prebuilt, checksummed archive from the [worktree release](https://github.com/beyond10x/worktree/releases) —
and runs:

```bash
worktree activate --profile <profile.toml> --workspace <workspace-root>
worktree doctor --check
```

The skill teaches agents to create trees outside the primary repository collection, maintain live
session leases, and publish wanted commits before finishing. Cleanup is review-bound: inspect
`worktree gc --repo <primary> --dry-run`, then pass only ids from that review to
`worktree gc --repo <primary> --apply --id <reviewed-id>`. Dirty, locked, live, local-only,
offline, unmanaged, and out-of-policy trees are retained. Work merged as rebased or cherry-picked
copies is recoverable when an advertised ref carries every unique commit's exact patch.

`worktree inspect --repo <primary>` reports actual Git state, storage, ignored files, leases and
retention blockers. It defaults to one repository; use `--workspace` to inspect the wider profile.
Add `--refresh` for current remote recovery evidence. Inspection does not infer story completion
or authorize removal.

Use `worktree reconcile --repo <primary> --dry-run` for interrupted provisioning, adopted legacy
paths, finished external trees, and already-missing records. Apply only exact reviewed ids.
External retirement additionally requires the explicit `--allow-external-retirement`
acknowledgement.

The plugin contains no cleanup script and no independent policy copy; `agentplugins-check tools`
checks every command it spells against the newest `worktree` release.
