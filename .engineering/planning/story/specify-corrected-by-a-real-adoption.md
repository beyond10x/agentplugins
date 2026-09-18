---
format: aep.planning-md/1
id: story:specify-corrected-by-a-real-adoption
kind: story
status: implemented
title: The specify skill is corrected by a real adoption
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: evals/ess-specify-new-entity/case.yaml
- confidence: cited
  path: plugins/ess-specify/skills/coverage/SKILL.md
- confidence: cited
  path: plugins/ess-specify/skills/specify/SKILL.md
revision: 5
---
# Story: The specify skill is corrected by a real adoption

## Outcome

Someone specifying a system with `ess-specify:specify` declares the format their `ess` binary
actually speaks, writes a disjunction the grammar accepts, reaches for an authored scenario when
synthesis cannot arrange one, and knows a built-in target proves nothing about their implementation
— none of which the skill tells them today.

## Context

An adopter specified a 46k-line Rust service with this skill and held it to a conformance suite. Six
defects cost measurable time, and one caused the rest.

The skill writes `format: ess/1` in its worked example and pins its sample output to a named ESS
release. The language supports four majors. The adopter wrote a whole specification at `ess/1` and
produced a thirteen-item list of things "ESS cannot say", of which **three were already solved** and
a fourth was solved on re-baselining. Re-declaring `ess/4` also surfaced a real defect the older
major had hidden: an emitted event field whose source nothing had ever checked.

The other five: the predicate grammar is never described, so an infix `or` — which parses as one
fact path — was read as proof that disjunction does not exist; authored scenarios never appear, so a
quarter of the suite reported no information for most of the work; `run` is described as holding "a
built-in reference implementation", which reads as a result when it returns every scenario
unsupported; `aep plan reverse openapi` is presented as the first move and yielded 197 types, zero
entities and 132 `UNMAPPED` markers on a 54-operation contract; and two of the seven sections address
someone working on ESS rather than someone using it.

Reading handlers rather than the contract also found five fictions in that adopter's own draft — a
refusal no handler performs, an entity that is a request extractor rather than a stored row, two
idempotent deletes modelled as conflicts. The skill points at the OpenAPI document first.

## Acceptance

- The skill's first section is the version question: the adopter establishes what the binary on
  `PATH` accepts and declares the highest major it does, rather than copying a number out of the
  instructions. No ESS or AEP release number is asserted as fact anywhere in the file, including in
  sample output.
- The skill states that a rule over two fields is written in the structured predicate form, and that
  an infix `or` is read as a single fact path. One worked implication appears.
- The skill names authored scenarios as the answer to what synthesis cannot derive, including the
  flag that selects them, and says plainly that synthesis arranges a subject by replaying commands
  and reads no entity relations.
- The skill says a built-in target proves nothing about the adopter's implementation, and that a real
  target is written against the conformance target trait.
- The skill says a declared refusal is checked by running it against the implementation, not by
  reading a contract that documents intent.
- `## Adapter contract` and `## Format changes` are gone; both address someone working on ESS.
- The `coverage` skill gains the precondition case under skip ranking, and the suite-major runner
  trap.
- The six behaviours `evals/ess-specify-new-entity` holds the skill to still hold, and its frontmatter
  `name:` is unchanged.
- The skill validator and `task check` pass.

## Out of Scope

The marketplace descriptions and `website/docs/plugins/ess-specify.md`, which describe only the
`specify` skill and never mention `coverage`. Nothing gates them and they are a separate correction.
New eval cases: `story:plugin-eval-cases` owns the uncovered subjects, `ess-specify:coverage` among
them.

## Open Questions

None. The two shaping decisions — teach the version question rather than re-pin the numbers, and
change both skills rather than one — were taken by the operator before the work started.
