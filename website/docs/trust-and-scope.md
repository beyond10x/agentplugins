---
title: Trust and scope
---

# What installation does—and does not—mean

Plugins are instruction bundles. They shape how an agent approaches a task, but they do not carry
credentials and cannot enlarge the authority granted by the host, operator, or repository.

Every published plugin is intentionally narrow:

- `b10x` sets up the other plugins and their command-line tools, routes work, links public
  resources, and creates portable plugins without copying the specialist workflows.
- `aep` governs planning artifacts and lifecycle-aware planning work, and coordinates accepted
  development work.
- `ess` specifies, validates and projects executable system contracts, and holds implementations
  to them.
- `worktree` creates, leases and safely cleans isolated Git worktrees.
- `connectors` sets up providers and invokes governed integrations; credentials stay with the
  Connector.

The repository gate checks that the two marketplace formats agree in their declared order, plugin
and directory names match, required skills and agents exist, and no retired marketplace or
source-repository identity reappears. Release tags also pin every plugin manifest to the workspace
version.

Review the source before installation and pin a release tag when repeatability matters.
