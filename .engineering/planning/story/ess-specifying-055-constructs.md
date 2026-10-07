---
format: aep.planning-md/3
id: story:ess-specifying-055-constructs
kind: story
status: draft
title: ess:specifying documents the 0.55.0 constructs and synthesis limits authors could not find
revision: 1
---
## Outcome

`ess:specifying` documents the ESS 0.55.0 constructs an author reached for and could not find, and
the synthesis limits an author hits after `validate` passes. Found while modelling a published
protocol with no read surface; every statement below was reproduced on `ess` 0.55.0.

| Missing from the skill | Where it now is |
|---|---|
| `naming: {summary: …}` on entities, types, actors and commands; a flat `summary:` there is an unknown field | `references/syntax.md` lending example, `references/later-formats.md` table |
| `refs:` (`provider:key`) on commands and outcomes | same |
| `{response: <field>}` as an event payload source; `sets:` refuses it; `creates:` still takes its identity from an event | `references/later-formats.md` table |
| `one_time_response:` goes on the outcome beside `returns: true`, not on the command | same |
| `now` in `when:` (ess/16) and `when_subject:`/`when_related:` (ess/22); refused in invariants, view filters, selections | same |
| Synthesis needs a view for stored-field guards; seeds are not applied to owned rows; constrained types in a typed response and type invariants without a view get no scenario | `references/later-formats.md`, *What validates and still gets no scenario*; pointer in `SKILL.md` |

## Acceptance

`task check` exits 0. The three yaml blocks of `references/syntax.md` validate with `ess` 0.55.0
and synthesize with 0 refusals (`agentplugins-check tools`).
