---
name: hardening
description: >-
  Harden an ESS specification and the implementation held to it once the conformance suite is green — ask the questions a passing suite cannot, with a catalogue of eight techniques from mutation to a design review. Use when the user asks how to harden a specification, whether a green suite can be trusted, what to do after coverage is raised, or asks for mutation testing, random command sequences or a model-based check, replay of a real caller, determinism checks, metamorphic relations, guard analysis, classifying a specification diff as breaking, or reviewing a design document against a specification. Not for raising the number of scenarios that execute, which is the `ess:testing-conformance` skill; not for writing the specification, which is `ess:specifying`.
---

# Hardening a specification

A green conformance suite answers one question: do the scenarios somebody wrote pass. Each
technique here asks a different one, and each runs only after the suite is green — a red suite is
the work, and `ess:testing-conformance` is where it is done.

## The rule that makes any of this count

**A check nobody has seen fail is not evidence. Plant a defect first.** Before a technique's green
result is reported, break the implementation (or the specification) in the way the technique claims
to catch, run it, and record the named failure. Then restore and run it green. A technique that
cannot be made to fail on a planted defect has measured nothing, whatever it prints.

Record both runs: the planted defect, the command, the failure it produced, and the green run after
restoring.

## The catalogue

| # | technique | the question | what it catches | needs from the IR |
|---|---|---|---|---|
| 1 | mutation audit | would the suite notice if a declared rule broke? | declared behaviour no scenario pins down: a dropped `from` state, a moved guard boundary, a dropped `sets` entry | lifecycles, guards, `sets`, payloads |
| 2 | random command sequences against a reference model | does the implementation agree with the spec along paths nobody wrote? | engine defects that only show later in a sequence: a view that caps its rows, a refusal that still writes, an identity reused, a state not re-enterable; invariants that hold only through undeclared defaults | everything the [reference model](references/reference-model.md) reads: lifecycles, guards, `sets`, payloads, views, invariants, `wrong_state` |
| 3 | replay of the real caller | does the code that *uses* the implementation send only what the spec accepts? | a caller relying on behaviour the spec does not declare; refusal outcomes the caller never triggers | the reference model, plus actors (who may send which command) |
| 4 | determinism | is the implementation deterministic where the spec assumes it? | clock reads, randomness, iteration order leaking into outcomes | none beyond technique 2's runner |
| 5 | metamorphic relations | do promises that span two runs hold? | a command allowed where the design forbids it ("a pause changes nothing but the pause commands") | the reference model; the relation comes from the design |
| 6 | exhaustive guard analysis | are guards dead, overlapping, gap-leaving, or weak enough to admit an invariant break? | a `when:` that can never match, two that match together, an input no outcome answers | guards, field types, invariants |
| 7 | spec diff in the gate | is a change breaking, and was that acknowledged? | a narrowing released as if it were additive | two compiled revisions |
| 8 | design review by an agent | what does the design say that the spec omits or contradicts? | rules the design states and the spec never declares ("one entry per player", "the count always goes up") | the spec files and the design document; no IR |

Procedures, with the defect to plant for each: [references/techniques.md](references/techniques.md).

Formal model checking (a TLA+ or Alloy export) is not in the catalogue. On a spec with 31 commands,
random sequences reached every declared outcome, 59 of 59; reach for a model checker only when
technique 2 reports declared outcomes it never reaches.

## The order

Cheapest first, with one exception: **the design review runs early, because it finds the most.** On
one adopter's spec (3 domains, 7 entities, 31 commands, a 121-scenario suite that passed) it
produced 26 findings, the most of any technique. What it finds changes the spec, and
every later technique is then run against the spec that will stay.

1. **Design review** (8) — no tooling, one agent, the [brief](references/design-review.md).
2. **Spec diff in the gate** (7) — one command and a [classification](references/spec-diff.md); it
   protects everything after it.
3. **Mutation audit** (1) — reuses the suite you already have.
4. **Guard analysis** (6) — reads the IR only; no implementation runs.
5. **Reference model and random sequences** (2) — the one piece of code the catalogue needs; build it
   once.
6. **Determinism** (4), **metamorphic relations** (5) and **caller replay** (3) — each reuses the
   runner from step 5.

Stop where the cost exceeds what the spec is worth, and say which techniques were not run.

## What the IR is for

Techniques 1–6 read the canonical IR, not the YAML:

```console
ess specify compile --path <specification> --format json --out <ir.json>
```

The IR is the resolved model: every name qualified, every guard in structured form, every default
applied. A technique that reads the YAML re-implements resolution and disagrees with the compiler
somewhere nobody looked.

## The reference model

Techniques 2, 3 and 5 rest on one small interpreter over the IR — about 150 lines — that answers,
for any command in any state, what the spec says happens. `ess` does not ship it yet;
[beyond10x/ess#114](https://github.com/beyond10x/ess/issues/114) proposes shipping the mutation audit
and the sequence runner as `ess` features, and carries a draft for the TypeScript target. Until it
ships, build it from the pattern: [references/reference-model.md](references/reference-model.md).

## Reporting

Per technique run: the planted defect and the failure it produced, the green result after restoring,
the command, and the findings — each one a spec gap, an implementation defect or a false positive of
the technique, said which. Name the techniques not run. A finding that belongs to `ess` itself (a
generator or validate gap) is an issue on beyond10x/ess, not a workaround here.

## Next

- A finding changes the specification: `ess:specifying`.
- A finding is a missing scenario or a skipped one: `ess:testing-conformance`.
