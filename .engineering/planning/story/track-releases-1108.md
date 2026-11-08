---
format: aep.planning-md/3
id: story:track-releases-1108
kind: story
status: draft
title: Track ESS 0.90.0, Worktree 0.19.4, Metaharness 0.30.0 and Connectors v0.69.0
summary: Move verified.json and the eval pins to the newest releases and re-verify the skills
revision: 1
---
## Context

The scheduled Tools runs on main failed every day from 2026-11-04 (last 42477778419, 2026-11-08)
on four releases newer than `verified.json`: ESS 0.90.0 (0.83.0), Worktree 0.19.4 (0.19.0),
Metaharness 0.30.0 (0.26.2) and Connectors v0.69.0 (v0.65.0). Every spelled command and the
tutorial passed against those releases; only the version check failed. ESS 0.85.0 adds `ess/34`
(a compensation that deletes its row; a stricter row snapshot) and grant mutation classes, 0.89.0
adds `ess/35` (`repeated: last_wins`), the `ess-http/1` bare body, literal-segment precedence and
`ess-output-state/4` (breaking for older readers); the rest is `ess-ui/1` and synthesis checks.
Metaharness 0.27.0 links AEP 0.79.0 and takes `aep drive resume --evidence`. Connectors v0.66.0 to
v0.69.0 change the MCP revision binding and the owner's bounded read. Worktree 0.19.1 to 0.19.4
are performance and recovery fixes; 0.19.1 changes how `gc --apply` reports a stale reviewed id.

## Acceptance

The ESS, Worktree, Connectors and AEP drive skills describe each consumer-visible change of the
releases or this record says why none applies; `verified.json` names ess 0.90.0, worktree 0.19.4,
metaharness 0.30.0 and connectors v0.69.0; `.github/workflows/eval.yml` names metaharness 0.30.0
with eval AEP 0.79.0, ess 0.90.0, connectors v0.69.0 and worktree 0.19.4; the ESS tutorial set
moves to 0.90.0 and `ess/35` together; `agentplugins-check tools` exits 0; the `ess-tutorial`,
`ess-full-package` and `worktree-onboarding` trials measure no worse than `trials/baseline.json`;
`task check` and `task site-build` exit 0.

## Scope

`plugins/{ess,worktree,connectors,aep}/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`CHANGELOG.md`, `fixtures/library-reservations-drafted`, `trials/{ess-tutorial,aep-tutorial}`,
`website/docs/{tutorials,plugins}`.
