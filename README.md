# Beyond10x Agent Plugins

Curated marketplace identity: `beyond10x`.

The repository deliberately contains five focused plugins:

- `beyond10x`: marketplace navigation, public resource discovery, and portable plugin creation.
- `aep-planning`: governed planning, decomposition, plan review, and reverse engineering.
- `adp`: wave coordination, story scoping, implementation, and adversarial review.
- `ess-schema`: ESS validation and deterministic schema/OpenAPI projection guidance.
- `workspace-hygiene`: safe creation, leases, publication checks, and cleanup for Git worktrees.

`beyond10x` is the front door, not a catch-all. It routes a task to the smallest specialist and
keeps plugin-creation workflows portable by making shared skills the canonical implementation for
Codex and Claude Code. It does not copy or replace the specialists' instructions.

Codex marketplace metadata lives at `.agents/plugins/marketplace.json`; Claude plugin marketplace
metadata lives at `.claude-plugin/marketplace.json`. Each plugin owns its own manifest and only the
skills or agents in its stated scope.

Run `task check` before publishing. The gate fails on missing focused content, mismatched plugin
names, a marketplace identity other than `beyond10x`, or plugin versions that disagree with the
workspace release. Run `task site-build` for the public documentation under `website/`.

The adopter guide is published at <https://beyond10x.github.io/agentplugins/>. This repository
contains no credential or bot-token delivery machinery; release mutations are performed through
the private organization tooling outside this tree.

<!-- b10x-docs:start -->
## Documentation

[Agent Plugins documentation](https://beyond10x.github.io/docs/agentplugins/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
