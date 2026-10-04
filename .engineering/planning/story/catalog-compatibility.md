---
format: aep.planning-md/3
id: story:catalog-compatibility
kind: story
status: active
title: Verify every catalogued CLI and upgrade behavior
scope:
- confidence: cited
  path: crates/agentplugins-check/src/tools.rs
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 4}
---
Derive verification from catalog, register Connectors Cargo installation, verify flags before subcommands and all six external tools, explicitly report unsupported platforms. Exercise migrations and backend preservation, verify ESS semantic changes and generated outputs. Update live eval tool combination only with observed evidence.

Source: operator-approved Agentplugins refinement plan, 2026-10-04. Baseline 4f529f1; existing ignored website/build caused 2 source-gate failures in previous review.

## Scope
Cited: crates/agentplugins-check/src/tools.rs. Source changes only; repository is installation/documentation tooling, no new product domain.
