---
format: aep.planning-md/3
id: story:independent-docs
kind: story
status: active
title: Build and validate Agentplugins documentation independently
scope:
- confidence: cited
  path: website
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 4}
---
Separate docs gate from plugin gate; fix ignored generated-output scan; move governance outside website; preserve public Markdown, assets, routes, anchors and tutorial. Add Rust static builder and exact-artifact bot-controlled independent publication carrying source SHA; coordinator integrates Atlas and Website routing changes only after live project-site verification.

Source: operator-approved Agentplugins refinement plan, 2026-10-04. Baseline 4f529f1; existing ignored website/build caused 2 source-gate failures in previous review.

## Scope
Cited: website. Source changes only; repository is installation/documentation tooling, no new product domain.
