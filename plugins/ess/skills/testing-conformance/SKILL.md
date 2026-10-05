---
name: testing-conformance
description: >-
  Raise and audit the coverage of an ESS conformance suite against a real implementation, and judge whether a passing suite is testing anything. Use when a repository holds a synthesized conformance suite — an `ess-conformance/*` document, a generated conformance runner, a target that answers `ExecuteCommand`/`QueryView`, or a CI job that runs one — and the task is to raise the number of scenarios that execute, work out why scenarios are skipped, decide whether a green suite would catch a regression, build or fix a test double for an upstream plane, or set the gate that holds a suite's counts. Also use when a conformance job fails in CI and not locally. Not for authoring or validating the specification itself, which is the `ess:specifying` skill; not for a unit or integration suite that no ESS document obliges.
---

# ESS conformance coverage

A conformance suite is a measurement instrument. This skill is about whether the instrument reads
anything, and about raising what it reads without lying about it.

Three numbers describe a suite, and only one of them is coverage:

| number | what it means |
|---|---|
| `passed` | the implementation was asked and answered correctly |
| `skipped` | the target could not ask — **no information about the implementation** |
| `failed` | the implementation was asked and answered wrongly |

`skipped` is the number to attack, and `passed` is the only number that may be claimed as progress.

## Before anything: does a green run mean anything?

**Break the behaviour and watch a named scenario go red.** If none does, the suite is nominal there
and raising its count makes it more nominal.

This is not a formality. On a suite with 156 passing scenarios, changing the implementation so an
upstream state mapped onto the wrong internal state **moved no verdict at all**. The mapping was the
thing those scenarios existed to check.

The mechanism generalises, so learn to spot it by reading rather than by mutating:

- An outcome guard (`when:` over an input field) is resolved by the target from the **input**. The
  target picks the outcome whose guard the input matches; it never reads the resulting state back.
- So a specification that folds several upstream values onto one internal value is **unfalsifiable
  unless a view exposes the field the fold produced**.
- The fix is a view field, not a better target. Add the folded field to the view, then re-run the
  mutation and confirm it now fails a scenario by name.

Do the mutation test once per mapping you rely on, and record in the commit that you did it and what
failed. A scenario nobody has ever seen fail is a scenario that has never been tested.

**A green suite does not prove the implementation's model.** It proves the fields its expectations
read. If the implementation's entities, events or views were written by hand rather than generated
(`ess:specifying`, "Implementation code comes from the specification"), a field no expectation
names can have the wrong type, or be missing, and every scenario stays green. The same holds for
guards: a synthesized scenario witnesses one value per `any_of`, so a guard over `[A, B]` is
exercised with `A` and never with `B`. Cover the other values with the implementation's own tests,
and prove each one can fail by breaking it once.

**A cheaper check comes first: would the scenario pass against a target that does nothing?** Point
the suite at a target whose every `ExecuteCommand` returns an accepted outcome with no state change
and whose every `QueryView` returns an empty row. A scenario that passes there asserts only that a
command was accepted, or only an absence, and cannot fail for a wrong mapping. For example, a
`cancel_booking` scenario that checks the command's outcome is `accepted` passes against that
target; the same scenario fails there once it also asserts that the `bookings` view shows
`state: Cancelled` for the booking it cancelled. Add the observed-state assertion before running any
mutation, because a mutation can only turn a scenario red when the scenario reads what changed.

Breaking the implementation by hand is the check here. `ess verify conform mutate` breaks the
specification instead and scores every mutant's suite against your implementation through
`--emit`/`--collect`; run it once the suite is green (`ess:hardening`, technique 1). Mutation is one
of eight hardening techniques. Once the suite is green, `ess:hardening` carries the
rest — random command sequences against a reference model, caller replay, determinism, metamorphic
relations, guard analysis, a spec diff in the gate and a design review — and the order to run them in.

## Ranking the skips

**Never rank skips by construct.** Counting which ESS constructs appear in skipped scenarios and
inferring cause from the counts is guessing. Done once on a real suite it was **wrong about every
cause**: it reported that the largest block was blocked by a hand-written enum table, and measurement
showed **zero** skips were an enum while 60% were one unrelated cause.

Rank them from the target's own refusal reasons. A target returns a sentinel for "I cannot expose
this", and the reason it wraps around that sentinel is the only authority on why a scenario skipped.

If the runner in use reports the sentinel's text rather than the wrapped reason, **capture the reason
at the source rather than waiting for a runner release**: add an environment-gated recorder at the
target's entry points — the command execution, the view query and the entity setup — that appends the
subject and the reason when the returned error wraps the sentinel.

```go
func recordRefusal(subject string, err error) {
	if err == nil || !errors.Is(err, essconform.ErrUnsupported) {
		return
	}
	path := os.Getenv("CONFORMANCE_REFUSALS")
	if path == "" {
		return
	}
	// append "subject\treason"
}
```

Wire it with a named return and a `defer` so no call site changes.

A `--target typescript` suite reads a refusal from the `cause` chain: the generated package's
`README.md` says a target throws `ErrUnsupported`, or `new Error("...", { cause: ErrUnsupported })`
so the reason can carry which command it was. The recorder is the same shape, wrapped around
`executeCommand`, `queryView` and `establishEntity` (the entity setup, on a target that implements
`EntitySetupTarget`):

```ts
import { appendFileSync } from "node:fs";
import { ErrUnsupported } from "./index.js";

function recordRefusal(subject: string, err: unknown): void {
  if (!(err instanceof Error) || err.cause !== ErrUnsupported) {
    return;
  }
  const path = process.env.CONFORMANCE_REFUSALS;
  if (!path) {
    return;
  }
  appendFileSync(path, `${subject}\t${err.message}\n`);
}

async function recorded<T>(subject: string, call: () => T | Promise<T>): Promise<T> {
  try {
    return await call();
  } catch (err) {
    recordRefusal(subject, err);
    throw err;
  }
}

// inside the target:
//   executeCommand(request) { return recorded(request.command, () => this.execute(request)); }
//   queryView(request)      { return recorded(request.view, () => this.query(request)); }
//   establishEntity(request) { return recorded(request.entity, () => this.establish(request)); }
```

Rethrow unchanged, so the runner still reads the sentinel and the verdict does not move. A bare
`throw ErrUnsupported` carries no reason and is not recorded; that is the generic refusal described
below.

Either way, verify the counts are identical with the recorder on and off — an instrument that
perturbs what it measures is not one. Then group the reasons and attack the largest block whose
cause is not a missing ESS construct.

Refusal reasons are worth writing well for this reason alone: a target that refuses with a generic
message destroys its own diagnosis. Every refusal should name what it could not spell.

## Building a double for an upstream plane

Most skips on a real suite are not missing constructs — they are planes the target does not drive. A
double for one is where coverage is cheapest and where a fake proves the least if built carelessly.

**Ground the payload in what the producer emits, never in what your parser accepts.** A parser
ignores unknown keys, so a two-key fake passes and proves nothing about the payload the real system
publishes. Read the producer's own specification and the code that marshals it. Where the producer is
a different repository from the consumer, the producer's specification is the source and the
consumer's struct is not admissible evidence about the wire.

**Carry the keys the consumer ignores.** A fake that emits only what the consumer reads cannot tell a
consumer that ignores a field from a producer that never sent one. Those are opposite findings and
one of them is a defect.

**Read the event's name from the model.** Where the consumer routes on a channel or topic and ignores
the event name, a wrong name passes silently — one shipped for a day this way. The name a message
travels under is declared once, in the model; read it there, not from the double.

**Delivered is not received.** Handing a message to a transport is not the same as the consumer being
able to route it. Waiting for the handoff and calling it delivery leaves ordering to chance: on a
fast machine the subscribe wins, slowed down it loses, and the only symptom is a scenario burning its
whole barrier. Wait for the subscription, not the handoff.

**Address the subject the way the implementation will match it.** A double that publishes to a
channel keyed by a raw identifier while the implementation subscribes with a normalised one produces
a timeout per scenario and no other signal. One suite spent ten minutes of wall clock on exactly
this, which is also why parallelising it would have been the wrong repair: it would have hidden them.

**A target that decides an outcome from its own bookkeeping has not witnessed it.** Where two
outcomes are observably identical through the interface, report unsupported. Consulting a flag the
target set itself reports a pass nobody earned.

## Auditing the specification a suite enforces

A suite enforces the model, so a wrong model makes a correct implementation fail. When auditing,
prefer a deletion to an addition.

**A declared refusal nobody executes is a fiction waiting to be found.** Check every branch that
claims the implementation rejects a command in some state, and check it **by running it, not by
reading it**. On one model, seven such branches were audited in two passes: five were fictional, and
the two the first pass missed were the two whose scenarios were `skipped` — never once executed,
which is exactly why reading had not caught them.

**"Declared for the ladder" is a confession.** A branch that exists to satisfy a structural need
rather than because the implementation does it will be wrong, and its own summary often admits it
while still misdescribing the behaviour.

**A restatement of another owner's vocabulary drifts, and no check sees it unless it is in the
checked set.** Look for a local type that re-declares values another specification owns. One
restated a consumer's internal fold under a name that said wire — including a value the producer does
not declare, and omitting five it sends — and the existing cross-repository check never looked at it
because it was not a payload type. The correction is a deletion: point the field at the owner.

**Widening a guard changes which branch a re-report takes.** Widening transitions to run from more
states can make a message that merely re-reports its own state match its own guard where it
previously fell through to a default branch. The prose on that default branch must follow, and a
purpose-built check is what catches the stale version.

## The gate

Hold the counts in a committed baseline and let CI compare against it.

- **Floor `answered` (`passed + failed`), not `passed`.** Where one scenario flaps between pass and
  fail, a `passed` floor fails the gate while nothing regressed. A pass that becomes a *failure* is
  caught separately, by name, against a quarantine list; the `answered` floor catches what the name
  check cannot see — a pass that becomes a *skip*.
- **Ceiling `skipped`.** A change may lower it and never raise it.
- **Floor `total`, and allow it to fall only when `answered` is flat.** Deleting a fiction lowers
  `total` and must not raise `passed`. `total` falling with `answered` flat is a fiction leaving;
  `total` falling with `answered` falling is coverage leaving. Record which one happened, in the
  baseline, in the same change.
- **Quarantine by name, never by count.** A known failure is a named scenario, so a *different*
  failure cannot hide behind it.
- **The job's verdict is the report document, not the runner's exit code.** A suite that skipped
  everything also exits zero, which is the whole problem. Check that the suite ran, that something
  was asserted, that every failure is a known one, and that the counts hold — as four separate
  checks.
- **A suite with no declared coverage claim can never report `passed`** as its own status, however
  green the run. If that status is meant to mean something, declare the claim.

**Never make a red gate green by widening a timeout or extending the quarantine list.** Both produce
a green job with the behaviour never exercised, which is the failure the ratchet exists to catch.

## When the job fails in CI and not locally

- **Run the job's own script, in the job's own image**, read out of the CI definition rather than
  reconstructed. Check what the job template contributes before concluding the reproduction is
  faithful.
- **`-race` is the reproducer for a timing failure; a CPU limit is not.** The race detector slows
  every operation uniformly, where a container CPU limit still allows a full-speed burst inside each
  scheduling period. On one three-pipeline CI failure, half a CPU reproduced nothing and `-race`
  reproduced it on the first try — reporting **zero data races**, which was itself the finding: the
  bug was ordering, not concurrency.
- **A scenario that fails at exactly the barrier duration is an ordering bug, not a slow one.** The
  event never arrived; look for something dropped, not something late.
- **A local gate stricter than CI hides the steps after it.** Where a task runner stops at the first
  failure, a step that blocks locally and only reports in CI means the local gate never reaches what
  follows it. Keep the two in agreement, or put the stricter step last.
- **A job absent from a failed-job list may be skipped rather than passing** — an earlier stage
  failing skips the rest. Read the status; do not infer it from absence.
- **When a log is unreachable, look for the structured surface first.** A check-run annotations API
  names the failing task in plain JSON where a log endpoint offers only a redirect to an archive.
  Reproducing blind before looking costs hours and can surface a failure the CI environment does not
  even have.

## Reporting a coverage change

State the counts before and after, from the report document rather than the log, and say which
scenarios moved and in which direction. Two claims need naming explicitly because they are the ones
most often assumed:

- **A skip that became a failure is not progress.** Report it as a failure with its reason, and say
  whether it is the target's fault or the implementation's.
- **A fiction removed is not coverage added.** If `passed` did not move, say so.

Three consecutive runs with identical counts, and one run of the job's own script in the job's own
image, before claiming a number.

## Which target runs your suite

`ess verify conform run --target` names a target built into the `ess` binary. None of them runs
your implementation:

| `--target` | what it is | on your own domain |
|---|---|---|
| `interpreted` | executes the selected specification through ESS’s reference model; requires `--path` matching the suite | checks model consistency; unsupported constructs remain explicit |
| `oracle-fixture` | a hand-written implementation of ESS's own `examples/oracle-fixture` | every scenario `error` |
| `billing` | a hand-written implementation of ESS's own `examples/billing` | every scenario `error` |

A green interpreted run is evidence about the model and runner, not about your implementation.
Current ESS runs the committed related-guard example as 3 passed, 0 failed, 0 error and 0 unsupported.
Use an adapter over your actual implementation for product conformance.

To hold your implementation to the suite, generate it as a test package in the implementation's
language and implement the package's `Target` interface over your service:

```console
ess verify conform synthesize --path <specification> --target go --out <dir>
ess verify conform synthesize --path <specification> --target typescript --out <dir>
```

Generate the Go package **inside the implementation's own module**: `--out <module root>` writes
`<module root>/essconform`, the import path its `README.md` shows (`<module>/essconform`). A copy
outside the module has no `go.mod` to import from. The package's
`README.md` lists the methods and the wiring test. A method you cannot answer returns
`ErrUnsupported` (a skip), never a made-up result. A refused command still sets `Outcome` to the
refusing outcome’s name beside `Error`: the runner compares both. Successful mutations return a
backend-issued consistency token; a view honors `AtLeast(token)` before returning rows. An adapter
that cannot meet freshness returns a failure, never a successful weaker read.
Keep a suite for a domain nobody has implemented; run it once something answers it.

The current runner contract:

- Suites from `ess-conformance/8` on run only with `ESS_REPORT_FORMAT=2` set: without it `go test`
  stops before the first scenario with a report-format refusal. Set it in the command, `ESS_REPORT_FORMAT=2 go test ./...`, or in the CI job.

- A command with `fixture_inputs:` needs the target to supply the values: implement
  `FixtureValues` (Go) or `fixtureValues` (TypeScript). Without it every scenario using a fixture is
  an explicit skip, so a fixture provider is the first thing to check when skips cluster there.
- Go and TypeScript have admitted `deletes:`, `accepts: nothing` and `presence:` suites
  (`ess-conformance/22`–`/25`). Current ESS also includes shared runtime coverage for
  the newer expression, binding, aggregation and seed formats through `/43`. Generate with the
  same release as the runner and inspect any target-specific refusal instead of carrying an old
  suite-version ceiling.
- Both packages also carry the random-sequence explorer (`explore`/`Explore`); `ess:hardening`
  technique 2 says how to wire it.

## Agents

- `conformance` — raises or audits a suite's coverage under this skill.

## Next

- A scenario reveals a gap in the specification: `ess:specifying`.
- The suite is green and the question is what it still misses: `ess:hardening`.

For a Rust implementation, synthesize `--target ir --out <suite.json>` and use the native
`ess_conformance::target::ConformanceTarget` with `AdmittedSuite` and `Runner`. Conformance
synthesis has no `--target rust`; that target belongs to implementation generation. The current
[Rust tutorial](https://github.com/beyond10x/agentplugins/tree/main/website/docs/tutorials/first-ess-specification)
pins the runner crates and rejects empty, failed, unsupported and errored runs.
