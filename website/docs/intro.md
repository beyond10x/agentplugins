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
| [`b10x`](./plugins/b10x.md) | Setup, upgrades and navigation | `init`, `upgrade`, `routing` and `authoring-plugins` skills, the `b10x` binary, drift check |
| [`aep`](./plugins/aep.md) | Governed planning and delivery | `planning`, `migrating`, `implementing`, `diagnosing` and `investigating` skills; decomposer, plan critics, reverse engineer, story scoper, implementor, adversary, security reviewer |
| [`ess`](./plugins/ess.md) | Executable System Specifications | `specifying`, `retrofitting`, `testing-conformance` and `hardening` skills; author, conformance, retrofitter agents |
| [`worktree`](./plugins/worktree.md) | Git workspaces | managed worktrees, leases, recovery proof, and safe cleanup |
| [`connectors`](./plugins/connectors.md) | Integrations | the `integrating` skill for the `connectors` CLI: set up providers and invoke governed integrations |

Every plugin starts with `/<plugin>:init` and checks itself with `/<plugin>:upgrade`; `/b10x:init`
asks what you want to do and installs the matching plugins and CLIs.

The marketplace contains instructions, not credentials. A plugin does not acquire authority to
write a repository, contact a service, or bypass an approval boundary merely because it is
installed.

New to ESS? [Your first ESS specification](./tutorials/first-ess-specification.md) takes you from an
empty directory to a validated specification and a passing conformance suite, with an agent doing
the writing, and [Your first governed plan](./tutorials/first-governed-plan.md) continues it with an
AEP plan and one reviewed wave. Otherwise [set up with one sentence](./install.md), [start with the front
door](./plugins/b10x.md) or [choose a specialist](./choose-a-plugin.md).
