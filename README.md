# Beyond10x Agent Plugins

Curated marketplace identity: `beyond10x`.

The repository deliberately contains three focused plugins:

- `aep-planning`: governed planning, decomposition, plan review, and reverse engineering.
- `adp`: wave coordination, story scoping, implementation, and adversarial review.
- `ess-schema`: ESS validation and deterministic schema/OpenAPI projection guidance.

Codex marketplace metadata lives at `.agents/plugins/marketplace.json`; Claude plugin marketplace
metadata lives at `.claude-plugin/marketplace.json`. Each plugin owns its own manifest and only the
skills or agents in its stated scope.

Run `task check` before publishing. The gate fails on missing focused content, mismatched plugin
names, a marketplace identity other than `beyond10x`, or any retired marketplace/repository name.
