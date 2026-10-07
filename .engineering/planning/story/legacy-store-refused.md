---
format: aep.planning-md/3
id: story:legacy-store-refused
kind: story
status: implemented
title: AEP skills say the newest aep refuses an aep.project/1 store and name its migration
summary: planning and implementing stop at a project/1 store and name aep plan store migrate git; tools checks the claim against the newest aep
refs:
- provider: github
  reference: https://github.com/beyond10x/agentplugins/issues/54
scope:
- confidence: cited
  path: crates/agentplugins-check/src/tools.rs
- confidence: cited
  path: plugins/aep/skills/implementing/SKILL.md
- confidence: cited
  path: plugins/aep/skills/planning/SKILL.md
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-07T08:06:23Z", actor: "human:timo", revision: 5}
- {from: "proposed", to: "active", at: "2026-10-07T08:06:23Z", actor: "human:timo", revision: 6}
- {from: "active", to: "implemented", at: "2026-10-07T09:18:25Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"verification":1}}}
---
## Context

https://github.com/beyond10x/agentplugins/issues/54. `plugins/aep/skills/planning/SKILL.md`
("The store's version, before the first write") and `plugins/aep/skills/implementing/SKILL.md`
("The store's version, first") say an `aep.project/1` store, or a `.engineering/planning/` with no
`project.yaml`, still works with every verb and the agent may carry on. `aep` 0.68.0 refuses it.

Observed 2026-10-07 with `aep 0.68.0` on a scratch `aep.project/1` project:

- `aep plan artifact list --kind story --format json`, `aep plan artifact lifecycle story` and
  `aep plan artifact new story probe --title probe` each exit 1 with
  `[unsupported_protocol_version] project.version: this store is `aep.project/1`, the Markdown
  journal layout, which AEP no longer opens — migrate it to `aep.project/5` with
  `aep plan store migrate git --verify``.
- With no `project.yaml` and a `journal.jsonl`, `aep plan artifact list --store .engineering/planning`
  exits 1 naming the same migration.
- `aep plan store migrate git --help` lists `--dry-run`, `--verify`, `--protocols`, `--profile`,
  `--protocol`, `--engineering`; it refuses a dirty `.engineering`.

## Acceptance

The store-version tables in `plugins/aep/skills/planning/SKILL.md` and
`plugins/aep/skills/implementing/SKILL.md` say that the newest `aep` refuses an `aep.project/1`
store (and a journal-layout plan with no `project.yaml`) with `[unsupported_protocol_version]`
before any write, that `aep plan store migrate git` is the one verb that opens it, and that store
work stops until the user has approved and reviewed `aep plan store migrate git --dry-run`, then
`--verify`, and committed the result; neither skill tells an agent to carry on, to use an older
executable or to edit artifact state by hand. `agentplugins-check tools` runs the newest `aep`
against a generated `aep.project/1` fixture and reports a problem when a skill's `aep.project/1`
row disagrees with what that release does (accepts versus refuses), with an offline unit test for
the row reader. `task check` passes.

## Scope

Confirmed by the implementor's table and the merged diff (`2684ef1`):

Cited, held: plugins/aep/skills/planning/SKILL.md, plugins/aep/skills/implementing/SKILL.md
(store-version sections only), crates/agentplugins-check/src/tools.rs.

Inferred, held with no change: plugins/aep/skills/upgrade/SKILL.md already says the current
release migrates `/1`; its code block has no `--dry-run` line, while its prose names `--dry-run`
first.
