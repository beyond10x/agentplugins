# The reference model: a pattern

Techniques 2, 3 and 5 compare an implementation with what the spec says happens. That needs a
**reference model**: a small interpreter over the compiled IR that, given a state and a command,
returns the outcome the spec declares. On one adopter it was about 150 lines of TypeScript.

**Status.** `ess` does not ship this. [beyond10x/ess#114](https://github.com/beyond10x/ess/issues/114)
proposes shipping the mutation audit and a seeded sequence runner built on this model as `ess`
features, emitted beside the suite for `--target typescript` and `--target go` and driven through the
same `Target` interface; the issue carries a draft (`explore.ts`) for the TypeScript target. Until it
ships, write the model yourself from the pattern below, and check that issue before starting — once
it ships, use what it emits instead.

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
   `input_field` and `literal` values, combined with and/or/not. Exactly one non-`wrong_state`
   outcome must match; zero or two is a spec defect, report it rather than choosing.
2. **Check the lifecycle.** For an outcome that `moves` an instance through a transition, look the
   instance up by the outcome's `instance` field. Missing instance, or current state not in the
   transition's `from`: answer with the command's `wrong_state` outcome instead, and **change
   nothing** — no `sets`, no events. (That a `wrong_state` answer still applied `sets` was one of the
   defects this found.)
3. **Apply.**
   - `creates`: a new identity (refuse to reuse one in `issued`), the lifecycle's `initial` state, the
     fields the outcome sets.
   - `moves`: set the state to the transition's `to`.
   - `updates` / `sets`: write each field from its value expression (`input.<field>` or a literal).
   - `error`: change nothing.
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
