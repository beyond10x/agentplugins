---
format: aep.planning-md/3
id: migration-plan:aep-runtime-extraction
kind: migration-plan
status: implemented
title: Move concrete AEP execution above the foundation
revision: 4
transitions:
- {from: "draft", to: "active", at: "2026-09-09T13:42:07Z", actor: "human:timo", revision: 2, imported: true}
- {from: "active", to: "implemented", at: "2026-09-09T14:11:36Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}, imported: true}
---
## Decision
The operator approved the runtime extraction and caller migration on 2026-09-09 (Atlas ADR 0047).

## Repository scope
Change the governed drive entry, wave/evaluation guidance, planning example and public documentation to invoke `metaharness aep drive`. Require the selected AEP planning executable and preserve the saved budget and plugin configuration on resume. Keep plugin versions unchanged.

## Repository acceptance
`task check`, `task site-build`, the three changed skills' validators and `claude plugin validate` for aep-plan and aep-drive pass. These are source and manifest checks; they do not claim a new installed-host run or paid evaluation.

## Coordination
Integrate after the AEP neutral host and Metaharness adapter. Atlas's migration-plan:aep-runtime-extraction and task:foundation-composition-evidence own the final dependency reconciliation, foundation composition receipt and ER planning evidence. This record's implemented status describes the caller source implementation; the coordinated migration remains active in Atlas until its final evidence is recorded.
