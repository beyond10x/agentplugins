---
format: aep.planning-md/3
id: story:upgrade-trials
kind: story
status: active
title: Observe upgrade behavior in Codex and Claude
scope:
- confidence: cited
  path: trials
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T03:38:32Z", actor: "human:timo", revision: 4}
---
Add and run isolated positive-control trials for offers, authorization, compatible continuation, required generation and failures. Missing or undecidable required evidence fails. Verify current/outdated/declined/authorized/offline/pinned/dirty/future states and repeated calls. Record real evidence rather than counting static instructions as observed behavior.

Source: operator-approved Agentplugins refinement plan, 2026-10-04. Baseline 4f529f1; existing ignored website/build caused 2 source-gate failures in previous review.

## Scope
Cited: trials. Source changes only; repository is installation/documentation tooling, no new product domain.
