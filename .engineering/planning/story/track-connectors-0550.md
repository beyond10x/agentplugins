---
format: aep.planning-md/3
id: story:track-connectors-0550
kind: story
status: implemented
title: 'Track Connectors v0.55.0: instances, --connector/--instance, connections reconnect'
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:57Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

Connectors v0.54.0 deprecated `--adapter`, `adapters …` and `connections repair`; v0.55.0 (2026-10-20)
removed `adapters list`, so `agentplugins-check tools` reports four missing spellings in
`plugins/connectors/skills/init/SKILL.md` and `integrating/SKILL.md`. The "verified against" lines name
v0.48.0.

## Acceptance

No skill or reference under `plugins/connectors/skills/` spells `adapters list|describe|status`,
`--adapter` or `connections repair`; they spell `instances …`, `--connector`/`--instance` and
`connections reconnect` as `connectors --help` of v0.55.0 does; `connectors:upgrade` names
`connectors setup migrate`; the "verified against" lines, `verified.json` and `.github/workflows/eval.yml`
name v0.55.0; `agentplugins-check tools` reports no connectors problem.

## Scope

`plugins/connectors/skills/{init,integrating,upgrade}/**`, `website/docs/plugins/connectors.md`,
`verified.json`, `.github/workflows/eval.yml`, `CHANGELOG.md` (Unreleased).
