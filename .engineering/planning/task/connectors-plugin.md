---
format: aep.planning-md/3
id: task:connectors-plugin
kind: task
status: implemented
title: Make the connectors CLI available as a portable marketplace plugin
relations:
- informed_by: epic:second-adopter-feedback
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-09-05T11:25:27Z", actor: "human:timo", revision: 2, imported: true}
- {from: "proposed", to: "active", at: "2026-09-05T11:25:28Z", actor: "human:timo", revision: 3, imported: true}
- {from: "active", to: "implemented", at: "2026-09-05T11:32:35Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":2}}, imported: true}
---
## Outcome
Add a focused connectors plugin to the beyond10x marketplace for Claude Code and Codex, using one shared skill grounded in connectors 0.6.0 help.

## Scope
Both marketplace entries and host manifests, shared CLI instructions, front-door routing, public installation documentation, and the focused-plugin gate. The plugin supplies instructions, not a daemon or credentials. No new domain model or CLI changes.

## Acceptance
Both hosts validate and install the same skill. Diagnostics and search/describe/invoke use the shipped grouped CLI commands, fresh description references, existing grants and secret-safe onboarding. Run skill/plugin validators, task check, task site-build, and read-only host discovery and CLI smoke checks. Record any unavailable live service coverage honestly.

## Authorization
Interactive user request to ensure this plugin exists and can be installed in both hosts. No release tag requested.
