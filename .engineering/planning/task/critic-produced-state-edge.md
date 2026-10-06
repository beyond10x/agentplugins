---
format: aep.planning-md/3
id: task:critic-produced-state-edge
kind: task
status: active
title: Design and parallel-safety critics flag a read of produced state without depends_on
summary: An acceptance line that reads state another item in the set produces needs a depends_on edge; two critics now raise it
relations:
- informed_by: story:plan-time-critic-panel
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-06T23:26:23Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-06T23:26:24Z", actor: "human:timo", revision: 3}
---
## Context

A four-critic panel approved a decomposition in which one story's acceptance line read a record
store that a sibling story created, and no `depends_on` edge recorded the order. The defect was
found by hand after the panel. The design critic's "hidden dependency" row asked whether an
outcome names another item's internals; it did not ask what each acceptance line reads. The
parallel-safety critic compared file surfaces only, so a reader and its producer that touch
different files looked independent, and `aep plan artifact waves` (which orders by `depends_on`
and `scope`) would place them in one wave.

## Acceptance

`plugins/aep/skills/planning/references/plan-critic-design.md` and
`plan-critic-parallel-safety.md` each instruct the critic to list what every acceptance line
reads (a file, a store or table, a record, a configuration key, a command's output), to find the
item in the set that creates or writes it, and to raise a finding when no `depends_on` edge runs
from the reader to that producer. Each gives a finding-line example for the case and says how many
reads it traced in its report. `task check` passes.

## Scope

Cited: plugins/aep/skills/planning/references/plan-critic-design.md,
plugins/aep/skills/planning/references/plan-critic-parallel-safety.md.
