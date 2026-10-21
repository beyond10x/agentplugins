---
format: aep.planning-md/3
id: task:refresh-aep-075-redact
kind: task
status: active
title: 'Track AEP 0.75.0: redact an immutable record'
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T06:54:23Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T06:54:23Z", actor: "human:timo", revision: 3}
---
## Context

AEP 0.75.0 adds `aep plan artifact redact <id> --literal <text> --with <replacement> --reason <text>`,
which removes a literal from an artifact's title, summary and body and writes one evidence record
per redaction (the literal's SHA-256, never the literal; the replacement, the count, the reason and
the instant). `validate` admits a `review-result` that differs from its first committed version only
by the redactions its evidence records. The planning skill's guardrail 2 says `new --from` is the
only way an immutable record's body arrives, and the store conventions do not list `redact`.

## Acceptance

Guardrail 2 of `plugins/aep/skills/planning/SKILL.md` and the mutation table of
`references/store-conventions.md` name `redact` as the only way to change an immutable record's text,
`verified.json` names aep 0.75.0, `agentplugins-check tools` reports no problem for aep 0.75.0, and
the pull request's required checks pass.

## Scope

`plugins/aep/skills/planning/SKILL.md`; `plugins/aep/skills/planning/references/store-conventions.md`;
`verified.json`; `CHANGELOG.md`.

## Classification

| Change | Resource | Decision |
|---|---|---|
| aep 0.75.0: `aep plan artifact redact` | `aep:planning` guardrail 2, store conventions | named as the only way to change an immutable record's text |
| aep 0.75.0: `validate` replays redaction records for a `review-result` | `aep:planning` guardrail 2, store conventions | described |
| aep 0.75.0: ESS pin 0.62.0 in the AEP repository | none | internal to the AEP build |

## Verification

- `agentplugins-check tools` with `verified.json` at aep 0.75.0: aep 0.75.0 225 spelled commands checked, 0 problems for aep; the run exits 1 on 7 problems already on `main` (ess 0.66.0, metaharness 0.10.1, connectors v0.55.0 and its `adapters` verbs), none from this change.
- `task check`: exit 0 (118, 2 and 77 tests passed; 22 eval cases; marketplace valid).
