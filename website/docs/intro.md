---
sidebar_position: 1
slug: /
title: Agent Plugins
description: The curated b10x marketplace for focused engineering-agent guidance.
---

# Focused guidance, explicit scope

The `b10x` marketplace publishes a front door and four specialist plugins. Install the front
door when you want routing, ecosystem resources, or portable plugin creation. Install a specialist
directly when the work is already clear.

| Plugin | Use it for | Includes |
|---|---|---|
| [`b10x`](./plugins/b10x.md) | Setup, upgrades and navigation | setup skill and `b10x` binary, drift check, router, portable plugin creator |
| [`aep-plan`](./plugins/aep-plan.md) | Governed plans and artifact stores | planning skill, decomposer, plan reviewer, reverse engineer |
| [`aep-drive`](./plugins/aep-drive.md) | Governed development delivery | wave skill, story scoper, implementor, adversary |
| [`ess`](./plugins/ess.md) | Executable System Specifications | specify, retrofit, coverage skills; author, conformance, retrofitter agents |
| [`worktree`](./plugins/worktree.md) | Git workspaces | managed worktrees, leases, recovery proof, and safe cleanup |

`ess` and `worktree` ship from their own repositories at the version of the binary they describe;
this marketplace points at them and names no version.

The marketplace contains instructions, not credentials. A plugin does not acquire authority to
write a repository, contact a service, or bypass an approval boundary merely because it is
installed.

[Set up with one sentence](./install.md), [start with the front door](./plugins/b10x.md), [choose a specialist](./choose-a-plugin.md),
or go directly to [installation](./install.md).
