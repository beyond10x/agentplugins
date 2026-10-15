# Current capabilities and runnable examples

Use this reference for the current ESS release's related records, set effects, event transports, protocol models
or compatibility gates. Paths below are relative to this reference directory; run with a scratch
output directory outside the specification inputs. Source examples are committed beside this file.

## Related records and selected effects

```console
ess specify validate --path examples/related-guard.yaml
ess verify conform synthesize --path examples/related-guard.yaml --out related-suite.json
ess specify validate --path examples/set-effects.yaml
ess verify conform synthesize --path examples/set-effects.yaml --out set-suite.json
ess specify validate --path examples/shelves.yaml
ess verify conform synthesize --path examples/shelves.yaml --out shelves-suite.json
ess specify validate --path examples/devices.yaml
ess verify conform synthesize --path examples/devices.yaml --out devices-suite.json
```

These examples synthesize 3, 14, 16 and 16 scenarios respectively, with zero refusals on the verified
release. `shelves.yaml` is the `ess/23` example: a renamed identity, per-element and bulk record
effects, enum attributes and the held state as a value ([later-formats.md](later-formats.md#ess23)).
`devices.yaml` is the `ess/25` example ([later-formats.md](later-formats.md#ess25)).
`CheckMember` uses `when_related: {via: input.member_id, exists: false}`; synthesis arranges the
present member and decoys, and separately asks with a missing identity. This is stronger than a
comment claiming registration is checked.

`Invite` updates its addressed session and uses `affects` to change and end other sessions of the
same team. `EndTeam` uses `instances` and reports `{count: changed}`. A selected move skips records
outside its transition's source states. These constructs do not declare transaction atomicity,
partial failure or effect ordering. A `where:` selector reads stored fields, not the selected
entity’s identity; from `ess/23`, `affects: each:` writes the record each element of an input list
names by its identity instead. Set-effect values cannot read `{increment: …}` (`ESS-COMMAND-009`).
The example selects by stored `team` and writes literal `on_hold`, not a per-holder counter.
Generated implementation targets and Entity Runtime lowering
still refuse set effects; a valid model and generated scenarios do not certify those targets.

The source language has related guards from `ess/18`, related lifecycle state from `/20`, and
row-set selectors, Optional references and several related rows from `/22`. Guard combinations
have narrower ordering rules in the current ESS release:

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

## HTTP routes (`ess-http/1`)

From 0.60.0 a binding beside the specification fixes the method, path, request encoding and
answers of the commands and views a `reached_by: network` component serves. It is pinned to the
specification as `ess-transport/1` is; a wrong digest is `ESS-HTTP-002`, which names the right one.
Every refusal carries an `ESS-HTTP-0NN` code and the key's path, and an unknown key is refused.

```yaml
type: ess-http/1
specification: {system: demo, version: v1, source_digest: "sha256:<hex>"}
refusals:
  malformed: {status: 400, body: {error: {literal: invalid_request}}}
routes:
  - command: demo.devices.RegisterDevice
    method: put
    path: /devices/{device_id}
    parameters: {device_id: input.device_id}
    outcomes:
      registered: {status: 201, body: none}
      reserved: {status: 422, body: {error: error}}
  - command: demo.devices.MoveTeam
    method: post
    path: /teams/move
    request: {encoding: form, unknown: ignore}
  - view: demo.devices.DeviceById
    method: get
    path: /devices/{device_id}
    parameters: {device_id: param.device_id}
    body: single
    empty: {status: 404}
```

```console
ess specify http validate --path <specification> --binding http.yaml
ess specify http compile --path <specification> --binding http.yaml --out http-ir.json
ess generate --path <specification> --kind openapi --http http.yaml --out <directory>
ess generate synthesize --path <specification> --target rust --http http.yaml --out <directory>
```

- **Request:** `request: {encoding: json | form | query, unknown: refuse | ignore}`. `query` is the
  only encoding of a command on `get` and carries every input the path does not map; `json` or
  `form` on `get`, `query` elsewhere, or `ignore` beside `json` is `ESS-HTTP-020`; a `form` or
  `query` field with no flat spelling (a struct, a union, a map, `Json`) is `ESS-HTTP-021`.
- **Answers:** `outcomes: {<outcome>: {status, body}}`, the body `envelope` (the default),
  `response` (bare, for a `returns: true` outcome without an error), `none` or `{<member>: error}`.
  Outcomes, headers and redirects are checked as `ESS-HTTP-022` to `-028`; an outcome not listed
  keeps the envelope and its derived status. A view route answers one row with `body: single` and
  `empty: {status: <4xx>}` (`ESS-HTTP-030`, `-031` when the view is not selected by identity).
  Top-level `refusals:` overrides `malformed`, `not_granted` and `unfinished` (`ESS-HTTP-029`).
- **Refinement:** a server binding that fills in a protocol binding's open paths names it with
  `refines: sha256:<hex>` and is checked with `--refines <file>` (`ESS-HTTP-003`, `-012`).

Without a path a route is open, and OpenAPI and the servers refuse it. `--http` serves on the
`rust` and `go` synthesis targets only; the Rust target still refuses set effects. The example
above validated, compiled and projected to OpenAPI against `examples/devices.yaml` laid out with a
`components.yaml`.

## Composition roles (`ess-composition/4`)

`roles:` says that the component one imported service selects plays the component another selects,
and which local command stands where in that contract:

```yaml
format: ess-composition/4
composition: demo-wire
services:
  - {key: demo, system: demo, version: v1, source_digest: <hex>, component: device-service}
  - {key: wire, system: wire, version: v1, source_digest: <hex>, component: wire-server}
roles:
  - service: demo
    plays: wire
    commands:
      - {local: demo.devices.RegisterDevice, stands: implements, contract: wire.devices.RegisterDevice}
      - {local: demo.devices.PingDevice, stands: gates, contract: wire.devices.TakeReading}
```

```console
ess specify compose --path composition.yaml --service demo=<spec> --service wire=<spec> --ownership-root <dir> --out <dir>/composition.json
```

`implements` compares inputs: every contract input is a local input of a conforming type, and a
local input the contract lacks is `Optional`; otherwise `role_input_drift`, naming every field.
`gates` is recorded, not executed. Also refused: `unknown_role_service` (a key not imported) and
`conflicting_role` (a service playing itself, a repeated role or command entry). `/1` to `/3`
refuse `roles`, even empty, as an unknown field. `--service` takes the specification source, and a
composition digest is bare hex, without the `sha256:` prefix the HTTP binding writes.

## Finite protocol checks

The terminal-response example is adapted from [the versioned upstream protocol examples](https://github.com/beyond10x/ess/tree/0.55.0/examples/protocols). It separates
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
