# Current capabilities and runnable examples

Use this reference for ESS 0.53.0's related records, set effects, event transports, protocol models
or compatibility gates. Paths below are relative to this reference directory; run with a scratch
output directory outside the specification inputs. Source examples are committed beside this file.

## Related records and selected effects

```console
ess specify validate --path examples/related-guard.yaml
ess verify conform synthesize --path examples/related-guard.yaml --out related-suite.json
ess specify validate --path examples/set-effects.yaml
ess verify conform synthesize --path examples/set-effects.yaml --out set-suite.json
```

These examples synthesize 3 and 14 scenarios respectively, with zero refusals on 0.53.0.
`CheckMember` uses `when_related: {via: input.member_id, exists: false}`; synthesis arranges the
present member and decoys, and separately asks with a missing identity. This is stronger than a
comment claiming registration is checked.

`Invite` updates its addressed session and uses `affects` to change and end other sessions of the
same team. `EndTeam` uses `instances` and reports `{count: changed}`. A selected move skips records
outside its transition's source states. These constructs do not declare transaction atomicity,
partial failure or effect ordering. Generated implementation targets and Entity Runtime lowering
still refuse set effects; a valid model and generated scenarios do not certify those targets.

The source language has related guards from `ess/18`, related lifecycle state from `/20`, and
row-set selectors, Optional references and several related rows from `/22`. Guard combinations
have narrower ordering rules in ESS 0.53.0:

- An identity-addressed `when_related` beside `unknown_instance` is refused.
- For `via: input.member_id`, an existence-only `exists: false` guard beside `wrong_state` is
  refused. This is the combination tried against the introductory library tutorial.
- In `ess/22`, `wrong_state` is admitted when at least one present-row predicate is declared and
  every present-row predicate branch refuses. For example, a refusal guarded by
  `when_related: {via: input.member_id, predicate: name == "blocked"}` beside the missing-member
  refusal validates after removing the separate `unknown_instance` branch. The addressed
  subject’s existence and held state answer before the present-row refusals.

This is a documented distinction, not a blanket ban on related guards with lifecycle checks.
Use only predicates the domain actually requires, then inspect synthesis independently of
validation. Adding an invented refusal just to admit a combination changes the contract.

## Event transport and a Rust publisher

`transport.yaml` names the exact source digest of `related-guard.yaml`. Refresh it only after
recompiling that model and inspecting its current provenance; a stale digest must refuse.

```console
ess specify transport validate --path examples/transport.yaml --spec examples/related-guard.yaml
ess generate client --path examples/related-guard.yaml --component library-service --transport examples/transport.yaml --target rust --package library-events --out client
```

The example generates one publish operation and a `client-report.json` listing application
obligations. The publisher's batching and flush/close operations do not prove remote consumption.
`ess-transport/2` also supports parameterized NATS subjects from required String payload fields;
this example uses a literal subject. Generated clients refuse at-least-once delivery: it is not a
promise an application can infer from JetStream alone.

## Finite protocol checks

The terminal-response example is adapted from ESS 0.53.0's `examples/protocols`. It separates
queueing the response, observing transport flush, closing and receiving the response.

```console
ess specify protocol validate --path examples/terminal-response.yaml
ess verify protocol run --path examples/terminal-response.yaml --actions examples/terminal-response.actions.json --out terminal.trace.json
ess verify protocol replay --path examples/terminal-response.yaml --trace terminal.trace.json
ess verify protocol explore --path examples/terminal-response.yaml
```

Use a fresh trace path: output is create-new. This model run is five steps and replays as passed;
bounded exploration visits eight states and nine transitions. It is **model evidence**. To check a
real transport implement `ess_conformance::protocol::ProtocolTarget` and record actual ordered
observations with `capture_target` or `run_target`. Missing observations, exhausted bounds and
unsettled obligations are inconclusive, not passing. Exit 0 means success, 1 refusal or
contradiction, and 2 inconclusive. Fixed-offset calendar windows do not follow daylight saving;
protocol logical time is not a universal distributed clock.

## Compatibility of a revision

```console
ess verify diff --from examples/related-guard.yaml --to examples/related-guard.yaml --compatibility --fail-on breaking-or-unknown --format json
```

This unchanged-model control exits 0. For a red control, remove a command grant or an event field in
a copy and compare that copy as `--to`; inspect the dimensions and require exit 4 for an
unacknowledged breaking or unknown result. `--dimension callers`, `readers` and `history` select
the affected consumers. An `ess-diff-acknowledgements/1` file is bound to both endpoint digests;
a stale file is a refusal, not a waiver. See the hardening skill's spec-diff reference for the gate.
