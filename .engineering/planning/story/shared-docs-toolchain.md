---
format: aep.planning-md/3
id: story:shared-docs-toolchain
kind: story
status: implemented
title: Align the passive producer with the shared contract viewer runtime
scope:
- confidence: cited
  path: .github/workflows/b10x-docs-bundle.yml
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-09-28T13:12:32Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-09-28T13:12:32Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-09-28T13:12:33Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

Use the reviewed shared documentation runtime for this repository's passive source bundle producer as part of the coordinated Mandate contract viewer publication.

## Acceptance

The Atlas-generated producer caller pins Docs System commit 1d4c0262911761118ffdd7037890f541a0688714. Source allowlists, credential-free execution and repository-owned validation remain intact. Atlas reconciliation reports no caller drift. Required source and common checks pass on the exact published commit, and its passive producer yields a valid immutable source bundle.

## Scope

.github/workflows/b10x-docs-bundle.yml and this planning record. The coordinating authority is the shared documentation rollout; this repository's product/runtime contracts do not change.

## Resolution

Delivered by a later pin. `.github/workflows/b10x-docs-bundle.yml` runs
`beyond10x/docs-system/.github/actions/bundle@339b4b8462f19b4c9d3716e6a44ed2a3691eb9d8`
(2026-09-23, "a v5 registry surface is a documentation surface"), and 1d4c026 is an ancestor of
339b4b8 (`git merge-base --is-ancestor`), so the reviewed runtime this story named is in force.
The producer ran credential-free on 44bb0ee and succeeded (run 36406238132, 2026-09-28).
