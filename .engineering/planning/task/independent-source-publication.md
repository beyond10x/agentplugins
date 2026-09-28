---
format: aep.planning-md/3
id: task:independent-source-publication
kind: task
status: implemented
title: Document source publication independent of Atlas
relations:
- informed_by: story:asynchronous-source-release
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-09-10T13:54:21Z", actor: "agent:atlas-independent-publication", revision: 2, imported: true}
- {from: "proposed", to: "active", at: "2026-09-10T13:54:21Z", actor: "agent:atlas-independent-publication", revision: 3, imported: true}
- {from: "active", to: "implemented", at: "2026-09-10T13:54:21Z", actor: "agent:atlas-independent-publication", revision: 4, imported: true}
---
## Outcome

Agentplugins owns its correctness checks and release requirements. Ordinary source publication uses standalone bot delivery and requires no Atlas checkout, current Atlas main or dependency admission.

## Scope

AGENTS.md source-publication guidance only. Atlas task:agent-tooling-independent-publication owns the coordinated admission-policy implementation for Harness, Metaharness, Agentplugins and MCP. Preserve the offline gate, skill/plugin validation and release workflow.

## Evidence

The operator requested this migration on 2026-09-10. Read-only GitHub API inspection found no Atlas admission requirement in branch rules or classic protection. The previous AGENTS.md bot paragraph required private Atlas tooling. No gates, tests or scanners run, per the operator's standing instruction. This documentation change does not enroll additional common Gates checks or declare runtime verification.
