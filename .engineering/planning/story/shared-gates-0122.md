---
format: aep.planning-md/3
id: story:shared-gates-0122
kind: story
status: implemented
title: Pin the shared gates workflow at gates 0.1.22
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:58Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

`.github/workflows/shared-gates.yml:16` pins the gates `common.yml` at 1a5ede1 (0.1.16); gates 0.1.22
is `f504f163dc04f5a003df6599e4d36002d11bd07e`.

## Acceptance

`.github/workflows/shared-gates.yml` pins `f504f163dc04f5a003df6599e4d36002d11bd07e` with its version
comment form unchanged; the pull request's `common / Security and privacy` check passes;
`agentplugins-check upstream` reports no gates drift.

## Scope

`.github/workflows/shared-gates.yml`, `CHANGELOG.md` (Unreleased).
