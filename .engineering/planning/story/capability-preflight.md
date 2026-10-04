---
format: aep.planning-md/3
id: story:capability-preflight
kind: story
status: active
title: Current capability guidance and repository upgrade offers
scope:
- confidence: cited
  path: plugins
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 4}
---
Implement shared AEP and ESS repository preflights reachable from all entry points; distinguish formats/backends/effective toolchains; once-per-session offers honor authorization, declines, offline and pinned states. Correct all five plugins against released contracts, including Connectors and host delegation. Preserve active CLIs during isolated legacy migration. Verify entry-point reachability and released examples.

Source: operator-approved Agentplugins refinement plan, 2026-10-04. Baseline 4f529f1; existing ignored website/build caused 2 source-gate failures in previous review.

## Scope
Cited: plugins. Source changes only; repository is installation/documentation tooling, no new product domain.
