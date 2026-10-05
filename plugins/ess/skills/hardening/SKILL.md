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

For communicating finite-state peers, current ESS also has experimental `ess-protospec/1`
validation, simulation, replay and bounded exploration. Read the
[protocol example](../specifying/references/current-features.md) when transport ordering, timers
or flush/close boundaries are the question. Model traces are not implementation evidence;
missing observations and exhausted bounds stay inconclusive.

## The order

Cheapest first, with one exception: **the design review runs early, because it finds the most.** On
one adopter's spec (3 domains, 7 entities, 31 commands, a 121-scenario suite that passed) it
produced 26 findings, the most of any technique. What it finds changes the spec, and
every later technique is then run against the spec that will stay.

1. **Design review** (8) — no tooling, one agent, the [brief](references/design-review.md).
2. **Spec diff in the gate** (7) — one command and a [classification](references/spec-diff.md); it
   protects everything after it.
3. **Mutation audit** (1) — `ess verify conform mutate`, run through your own suite runner.
4. **Guard analysis** (6) — reads the IR only; no implementation runs.
5. **Random sequences** (2) — the explorer in the generated Go or TypeScript package; a reference
   model of your own only where the explorer excludes what you need.
6. **Determinism** (4) — the explorer twice with one seed. **Metamorphic relations** (5) and
   **caller replay** (3) — a reference model you drive with your own traces.

Stop where the cost exceeds what the spec is worth, and say which techniques were not run.

## What the IR is for

Anything you build for techniques 2–6 reads the canonical IR, not the YAML (`ess` and the explorer
already do):

```console
ess specify compile --path <specification> --format json --out <ir.json>
```

The IR is the resolved model: every name qualified, every guard in structured form, every default
applied. A technique that reads the YAML re-implements resolution and disagrees with the compiler
somewhere nobody looked.

## What `ess` ships for techniques 1 and 2

- **Mutation audit:** `ess verify conform mutate` mutates the specification in named
  classes (including `sets-drop`, `outcome-order-flip`, comparison flips and `emit-swap`) and writes `ess-mutation-report/1`. Against your own implementation, `--emit DIR` writes
  the baseline's and every mutant's suite, your runner writes `report.json` beside each, and
  `--collect DIR` scores them. [references/techniques.md](references/techniques.md) § 1.
- **Reference model and random sequences:** the Go and TypeScript packages
  `ess verify conform synthesize --target go|typescript` writes carry a seeded explorer
  (`Explore`/`explore`) that walks your `Target` against a model interpreted from the IR, shrinks a
  failure and fails on an unreached outcome. Use it for technique 2. Techniques 3 and 5 still need
  a model you drive yourself, and so does a construct the explorer lists as `excluded`: build it from
  [references/reference-model.md](references/reference-model.md).
- **Several clients at once:** the same packages carry `ExploreConcurrent`/`exploreConcurrent`,
  which drive 2–4 clients on a seeded clock, write `history-<seed>.json`, and call
  `ess verify conform check-history --history <file>` to search for an order the specification's
  model accepts. Exit 0 is linearizable, 1 a violation (with a shrunk history), 3 the search budget
  ran out, which is never a pass. With `Inject`/`inject` it adds only the faults the specification
  declares (`delivery: at_least_once`, `replays:`, `external:` branches). Use it for technique 2
  wherever the service takes concurrent calls.

## Reporting

Per technique run: the planted defect and the failure it produced, the green result after restoring,
the command, and the findings — each one a spec gap, an implementation defect or a false positive of
the technique, said which. Name the techniques not run. A finding that belongs to `ess` itself (a
generator or validate gap) is an issue on beyond10x/ess, not a workaround here.

## Next

- A finding changes the specification: `ess:specifying`.
- A finding is a missing scenario or a skipped one: `ess:testing-conformance`.

Current hardening details: `--component` scopes mutation emit/collect; declared
`ess-known-failures/1` scenarios are counted separately instead of silently skipped. The explorer
can draw Optional inputs and commands selected by stored state, follow `.count` boundaries and
`example` values, and restart its target. Review exclusions from the actual generated runner.
Compatibility is built into `ess verify diff --compatibility --fail-on breaking-or-unknown`;
[spec-diff.md](references/spec-diff.md) specifies its exit-status and acknowledgement gate.
