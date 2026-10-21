---
format: aep.planning-md/3
id: epic:release-0-24-0
kind: epic
status: active
title: 'Release 0.24.0: newest CLIs, gates 0.1.22, neutral sample keys, Keep a Changelog'
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 3}
---
## Context

On 2026-10-21 the scheduled Tools check on `main` (d4926a4) has been red since 2026-10-19: ess 0.66.0,
metaharness 0.10.1 and connectors v0.55.0 are newer than `verified.json`, and the connectors skills
still spell `adapters list/describe/status`, which v0.55.0 no longer has. The shared gates workflow pins
gates 0.1.16, the newest is 0.1.22. Thirteen sample ticket keys of another organization remain in four
files (open PR #87 removes them but is red and based on 0.23.2). `CHANGELOG.md` does not follow
Keep a Changelog.

## Outcome

Release 0.24.0 of Agentplugins, published by the bot, with `agentplugins-check tools` exit 0 on the
tagged commit, every tracked CLI at its newest release in `verified.json`, the shared gates at 0.1.22,
no sample ticket key of another organization in the tree, `CHANGELOG.md` in Keep a Changelog 1.1.0
form and PR #87 settled.

## Out of scope

Deleting the unmerged remote branches; Website source locks or consumer pins; registry publishing.
