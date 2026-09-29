---
format: aep.planning-md/3
id: story:investigating-skill
kind: story
status: implemented
title: aep:investigating — evidence-based investigation of a live incident
summary: 'One activity skill with ten technique references for investigating what cannot be re-run: capture before remediation, sourced timeline, onset, peer differential, negative claims, ship state, hypothesis ledger, checking the check, complete reads, post-incident.'
scope:
- confidence: cited
  path: crates/agentplugins-check/src/main.rs
- confidence: cited
  path: plugins/aep/skills/investigating/SKILL.md
- confidence: cited
  path: plugins/aep/skills/investigating/references/techniques.md
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-09-29T09:39:27Z", actor: "human:timo", revision: 4}
- {from: "proposed", to: "active", at: "2026-09-29T09:39:27Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-09-29T09:39:27Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1}}}
---
# Story: aep:investigating

## Outcome

An agent asked to investigate a production incident, an outage or a "what happened / when did it
start / has it shipped" question works from evidence it can cite, preserves state before anyone
remediates, and labels every claim verified or inferred.

## Context

`aep:diagnosing` covers a defect that can be reproduced: it builds a red-capable loop first. A live
incident cannot be re-run, and the evidence disappears on remediation. On 2026-09-29 an operator's
hung-process report was remediated by a restart about four minutes after it was posted; the pod was
gone within a minute and no thread or lock state survived, so the root cause cannot now be
determined. The ten techniques come from failures of that kind recorded in an operator's knowledge
store over 2026-08 and 2026-09, each generalised with no system or customer named.

Placement: R2 permits no plugin without a product and CLI, and R3 names activities in `-ing` form,
so the ten techniques are one activity skill with a reference catalogue, as `ess:hardening` does.

## Acceptance

- `plugins/aep/skills/investigating/SKILL.md` exists, carries the `**Skill version**` line, and
  names ten techniques, each with its procedure in `references/techniques.md`.
- `agentplugins-check`'s file list for `aep` includes the skill and its references; `task check`
  exits 0.
- `aep:diagnosing` and `b10x:routing` route a live incident to `aep:investigating` and a
  reproducible defect to `aep:diagnosing`.
- The README tree, `website/docs/plugins/aep.md`, `website/docs/structure.md`,
  `website/docs/intro.md` and `evals/README.md` name the skill.

## Out of Scope

- An eval case with a recorded transcript. `evals/README.md` lists the skill as uncovered.
- An agent role. The skill runs in the main session.
- Any tooling for a specific platform (Kubernetes, a particular metrics store). The techniques name
  the kind of instrument, with common examples.

## Ambiguities

None.

## Open Questions

None.
