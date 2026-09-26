# The eight techniques, step by step

Each procedure ends the same way: plant the named defect, watch the technique fail on it, restore,
run it green. The figures in each section are from one adopter's spec (3 domains, 7 entities, 31
commands, a JavaScript implementation, a generated suite of 121 scenarios that all passed); they say
what a technique is worth on a real spec, not what yours will find.

Every technique that reads the model reads the compiled IR:

```console
ess specify compile --path <specification> --format json --out <ir.json>
```

## 1. Mutation audit

**Question:** would the suite notice if a declared rule broke?

1. From the IR, list one mutant per declared rule. The classes that find gaps:

   | class | example |
   |---|---|
   | drop a `from` state | `close: from [Open, Held]` → `[Open]` |
   | change a transition's `to` | `B → C` becomes `B → B` |
   | move a guard boundary | `items >= 0` → `items > 0` |
   | drop a `sets` entry | `colour: input.colour` removed |
   | invert or weaken a guard | `A && B` → `A \|\| B` |
   | wrong error, event or order | `desc` → `asc` |

2. Apply each mutant — to the implementation's rule table, or to a copy of the specification whose
   suite is re-synthesized and run against the unchanged implementation — one at a time.
3. Record, per mutant, the scenarios that failed. A mutant no scenario kills is a declared rule the
   suite does not pin down.
4. For each survivor, decide: a missing scenario (author one, then re-run that mutant and see it
   killed), a view that does not expose the field (see `ess:testing-conformance`, "does a green run
   mean anything?"), or a generator gap (an issue on beyond10x/ess).

**Planted defect:** the audit is its own plant — but confirm the harness first with one mutant you
know a scenario kills. If it survives, the harness is not applying mutants.

**Found:** 8 of 35 single-rule mutants passed every scenario; 6 were declared behaviour. After 6
authored scenarios, 1 survived, and it rested on a value the spec did not declare.

## 2. Random command sequences against a reference model

**Question:** does the implementation agree with the spec along paths nobody wrote?

1. Build the [reference model](reference-model.md) over the IR.
2. Drive a seeded random walk: at each step pick an actor, a command and an input (valid, boundary
   and invalid), send it to the implementation through the suite's own `Target` interface
   (`executeCommand`, `queryView`), and to the model.
3. After every step compare: the outcome, the error, the events and their payloads, every view's rows
   and their order, identity uniqueness, and every invariant.
4. On a disagreement, shrink the trace (drop steps while it still disagrees) and report the shortest.
5. Fail the run when a declared outcome is never reached across all seeds. An unreached outcome is
   untested, not passed.
6. Where the model cannot say what a view shows because no command sets the field, report it: the
   invariant holds only through an undeclared default.

**Planted defect:** one engine defect the suite passes — a view returning at most 5 rows, or a
`wrong_state` answer that still applies `sets`. The walk must catch it and shrink it.

**Found:** 300 walks of 80 steps in about 1.3 s caught 4 of 4 planted engine defects that the
127-scenario suite passed, and surfaced 3 invariants that held only through undeclared defaults.

## 3. Replay of the real caller

**Question:** does the code that uses the implementation send only what the spec accepts?

1. Record the caller's command stream: the application's own traffic, from a log, a test run or a
   recorded session. Strip any personal data before it leaves the environment it was recorded in.
2. Replay it through the reference model, from the same initial state.
3. Report every command the model refuses or answers differently from what the caller expected, and
   every refusal outcome the caller never triggers (a candidate for a caller-side test, or for
   deletion from the spec if it is fiction).
4. Check the actor of each command against the actors' grants.

**Planted defect:** edit one recorded command so it sends a field the spec does not accept, or runs
from a state the transition does not start from. The replay must name it.

**Found:** 0 disagreements over 25,980 commands; 25 refusal outcomes the caller never triggers.

## 4. Determinism

**Question:** is the implementation deterministic where the spec assumes it?

1. Run technique 2 twice with the same seed and compare the two command streams and every result,
   byte for byte.
2. Scan the implementation's source for clock reads, randomness and unordered iteration in code a
   command reaches (`Date.now`, `Math.random`, `time.Now`, `rand.`, iteration over a map or set).
   Each hit is either injected (a seed, a clock the target controls) or a finding.

**Planted defect:** add a `Math.random()` (or the language's equivalent) to one outcome. The two
runs must diverge, and the report must name the first divergent command.

**Found:** passed; the planted `Math.random()` diverged at command 235.

## 5. Metamorphic relations

**Question:** do promises that span two runs hold?

1. Read the relations from the design, not the spec: "a pause changes nothing but the pause
   commands", "cancelling and re-creating equals never creating", "the order two independent
   commands arrive in does not matter".
2. For each, generate a base trace with technique 2's walker, derive the paired trace (insert the
   pause, swap the independent pair), run both, and compare what the relation says must be equal.

**Planted defect:** make one command succeed while paused. The pause relation must fail on it.

**Found:** 1 real defect: an action allowed while paused that the design forbids.

## 6. Exhaustive guard analysis

**Question:** are guards dead, overlapping, gap-leaving, or weak enough to admit an invariant break?

1. For each command, collect its outcomes' `when:` guards from the IR.
2. Build each guard's boundary domain from the field types and every literal the guards compare
   against: each literal, one either side, the type's extremes, and for an enum every variant.
3. Evaluate every guard of the command over the product of those domains and report: a guard no
   input satisfies (dead), an input two guards both satisfy (overlap), an input no outcome answers
   (gap), and an input a guard admits that leaves an invariant false after the outcome's `sets`.
4. Optionally cross-check each finding, and each claimed absence, with `z3`.

**Planted defect:** in a copy of the spec, where one outcome's guard is `>= 0` and its sibling's
is `< 0`, change the sibling to `<= 0`. The analysis must report the overlap at `0`. If
`ess specify validate` already refuses the planted copy, the compiler covers that class: plant a gap
instead (`< 0` → `< -1`) and record which class the compiler caught.

**Found:** 11 guards, 374 combinations, 0 findings — a result that counted only because the planted
overlap was reported first.

## 7. Spec diff in the gate

**Question:** is a change breaking, and was that acknowledged?

Follow [spec-diff.md](spec-diff.md).

**Planted defect:** remove one enum variant on a branch. The gate must fail until the change is
acknowledged.

**Found:** 20 changes since the release tag, all additive.

## 8. Design review by an agent

**Question:** what does the design say that the spec omits or contradicts?

Dispatch one agent with [design-review.md](design-review.md) as its brief.

**Planted defect:** delete one declared rule from a copy of the spec that the design states. The
review must report it as `missing`, citing both sides.

**Found:** 26 findings, the most of any technique — among them "one entry per player" and "the
count always goes up", neither declared at all.
