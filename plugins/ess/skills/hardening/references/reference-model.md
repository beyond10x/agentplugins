# The reference model: a pattern

Techniques 2, 3 and 5 compare an implementation with what the spec says happens. That needs a
**reference model**: a small interpreter over the compiled IR that, given a state and a command,
returns the outcome the spec declares. On one adopter it was about 150 lines of TypeScript.

**Status.** The Go and TypeScript packages that
`ess verify conform synthesize --target go|typescript` writes carry this model and a seeded
sequence runner over it (`explore`/`Explore`, [techniques.md](techniques.md) § 2), bound to the
suite's `spec_digest` through `ir.json`. Use that for technique 2. Write the model yourself from the
pattern below when the runner's `excluded` list holds what you need, or for techniques 3 and 5,
which drive the model with traces the explorer does not generate.

## Input

The canonical JSON IR, and nothing else:

```console
ess specify compile --path <specification> --format json --out <ir.json>
```

Read the IR, never the YAML: the IR has qualified names, guards in structured form and defaults
applied. Read field names from your own IR file rather than from this page; the pattern is stable,
the exact keys are the compiler's.

## State

```text
instances: map entity -> map identity -> { state, fields }
issued:    set of every identity ever created   (identity uniqueness)
```

Nothing else. A model that tracks anything the IR does not declare is asserting behaviour the spec
does not have.

## Step: one command

For `execute(command, input)`:

1. **Pick the outcome.** Evaluate each outcome's `when:` predicate over the input — comparisons over
   `input_field` and `literal` values, combined with and/or/not — and a `when_subject` predicate
   over the addressed instance's stored fields (with `input.<field>` operands from `ess/15`).
   Exactly one non-`wrong_state` outcome must match; zero or two is a spec defect, report it rather
   than choosing.
2. **Check the lifecycle.** For an outcome that `moves`, `updates` or `deletes` an instance, look it
   up by the outcome's `instance` field. A missing instance answers, in order, the command's
   `unknown_instance:` outcome, its not-found outcome (an `external:` refusal whose `error:` carries
   a field of the identity's type), or its `wrong_state` outcome. A current state not in the
   transition's `from` answers `wrong_state`. Either way **change nothing** — no `sets`, no events.
   (That a `wrong_state` answer still applied `sets` was one of the defects this found.)
3. **Apply.**
   - `creates`: a new identity (refuse to reuse one in `issued`), the `into:` state or else the
     lifecycle's `initial`, the fields the outcome sets.
   - `moves`: set the state to the transition's `to`.
   - `deletes`: remove the instance; its identity stays in `issued`.
   - `updates` / `sets`: write each field from its value expression — `input.<field>`, a literal,
     `{subject: f}` (the value before this step), `{increment: n}`, or `{generated: true}` (unknown
     to the model: see Views).
   - `error`, `preserves`, `accepts: nothing`: change nothing.
4. **Emit.** For each event in `emits`, build its payload from the outcome's `payload` mapping.
5. **Check invariants** of every instance the step touched, over its fields after the step.

Return `{ outcome, error, events, payloads }`.

## Views

For `query(view)`: take every instance of the view's `source`, keep those its `filter` admits, and
project the view's `fields`. Compare rows **and their order** with the implementation's answer; a
view whose order the spec does not declare compares as a set, and the model says which.

**Unknown is an answer.** Where a view publishes a field no command ever set on that instance, the
model does not know its value. Report that as its own finding: whatever the implementation shows,
it rests on an undeclared default. That is how three invariants that held only by accident were
found (beyond10x/ess#112).

## What drives it

The model is the oracle; a runner drives it and the implementation side by side:

- a seeded generator picks actor, command and input (valid, at each guard boundary, and invalid) —
  the actor from those whose grants include the command, plus one that is not, to exercise refusal;
- after each step, compare the step results, then every view, then identity uniqueness;
- shrink a failing trace by dropping steps while it still fails;
- at the end, list every declared outcome never reached, and fail if the list is not empty.

## Proving the model is not the implementation's twin

A model written by reading the implementation agrees with it by construction. Write it from the IR
only, and before trusting it, plant one engine defect in the implementation (a view capped at 5
rows, a `wrong_state` that still writes, a reused identity, a state that cannot be re-entered). The
runner must catch it. A model that agrees with a planted defect is a copy, not a reference.
