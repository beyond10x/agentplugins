---
name: retrofitting
description: >-
  Derive an ESS specification for a system that already exists and has none — from its OpenAPI contract, its observed Kubernetes deployment, or its code — then validate it and hand it to conformance. Use when the user asks to retrofit, adopt, reverse-engineer or "get a spec out of" an existing service or repository, when a codebase has an OpenAPI document or a cluster but no `system.yaml` or `ess-inputs.yaml`, or when a specification must describe behaviour that is already shipped rather than behaviour still to be built. Not for a domain drafted from nothing, which is the `ess:specifying` skill; not for raising coverage of a suite that already exists, which is the `ess:testing-conformance` skill.
---

# Retrofitting a specification onto an existing system

A retrofit describes what the system **does**, not what it should do. Every declaration in the
draft is read from something in the repository — a contract, an observation, a line of code — and
cites it. What you cannot read is an `UNMAPPED:` marker, never a plausible value.

## 1. Inventory the sources, in order of authority

| Source | Command | What it gives you |
|---|---|---|
| an OpenAPI document | `aep plan reverse openapi --domain <domain> <openapi> --out <file>` | a draft `ess/1` domain: entities and fields from the schemas, with the decisions it could not take listed |
| the same OpenAPI document | `ess infra import openapi --path <openapi> --format yaml` | the operation and interface-type semantics ESS's adapter supports, with its coverage gaps |
| a running Kubernetes deployment | `ess infra import kubernetes …` (see `--help`) | a sanitized observation of what is deployed; the credential edge is explicit |
| code only | read it | handlers, persistence types, state enums, validation — each a citation |

Use every source that exists. Where two disagree, the draft records both and marks the field
`UNMAPPED:`; the disagreement is a finding, not something to resolve by choosing.

`ess infra import openapi` reads OpenAPI 3.0 and 3.1. A nullable field (`nullable: true`, or a 3.1
`type: [T, "null"]`) imports as a coverage gap at its pointer, because the interface has no null:
carry each gap into the draft as an `UNMAPPED:` marker, or, where the contract says the key is
always sent and may be `null`, declare it `Optional<T>` with `presence: null_when_absent`
(`ess/15`), which publishes exactly that. An object schema must be closed with
`additionalProperties: false`, or the import refuses it; that refusal is a finding about the
contract, so report it rather than editing the contract to pass.

`aep` is optional. Without it, write the domain by hand from the schemas, in the shape the `ess:specifying`
skill shows.

## 2. Draft, one domain at a time

Start from the smallest document that validates (the `ess:specifying` skill has it), then add one entity at
a time, running after each:

```console
ess specify validate --path <specification>
```

For each entity, cite where it came from beside it:

```yaml
  - name: billing.invoice.Invoice
    # read from: openapi.yaml components.schemas.Invoice; src/invoice/model.rs:14
```

Retrofit-specific rules:

- **A derived value needs its actual expression or an explicit gap.** `{generated: true}` can
  describe an implementation-produced value beyond an identity, but asserts no relationship to
  inputs or the clock. It does not specify `due_at = now + days × 24h`. If the current expression
  vocabulary cannot express that calculation, mark the calculation `UNMAPPED:` with its source
  line. A partial type/presence check must be reported as partial, even when all its scenarios
  pass. The presence of a generated timestamp does not verify the due date.
  Current `now` support is for command guards, including stored and related timestamp comparisons;
  it is not a general clock source for `sets:` or a time-relative view filter. Report those exact
  missing expressions, rather than claiming ESS has no clock-aware constructs.

- **Lifecycles are read, not designed.** Take states from an enum, a status column or the
  transitions the handlers perform. A state the code never enters is not declared. A transition
  whose trigger you cannot find is `UNMAPPED:`.
- **Commands are the operations that exist.** One command per mutating operation or handler. Do not
  add the command the system should have.
- **Relations need an owner decision.** `owns` against `references` is decided by what a delete
  does in the code today. No delete path found: `UNMAPPED:`.
- **Views follow reads.** One view per read operation clients use; its fields are the fields the
  response carries. An entity with `invariants` also needs a view holding every state, or the suite
  refuses the invariant checks (`ess:specifying`, conformance section). That view is structural,
  not a read the code has: say so in a comment. It is the one view a retrofit may add.
  A read by identity (`GET /tools/{id}`) is not a view of its own: `filter: id == param.id` leaves
  every outcome it observes unsynthesized (`ESS-SYNTH-005`). Declare the entity's view without that
  filter and say in a comment that the service also reads it by id.
- **Choose the no-op shape from the actual state behavior.** If every state outside the command’s
  transition is accepted without change, use one `wrong_state: true` outcome with `refuses: false`
  and no error. If some states are ignored and others refused, use held-state branches instead
  (`ess/7` or later): `preserves:` for a successful no-op and an effect-free error for the rest.
  The following example is this mixed case and therefore declares no `wrong_state:` outcome.

  ```yaml
  - name: retired
    when_subject_state: Active
    moves: shop.tools.Tool.retire
    instance: id
    emits: [shop.tools.ToolRetired]
    payload:
      shop.tools.ToolRetired: {tool_id: input.id}
  - name: already-retired         # ignored: success, nothing changes
    when_subject_state: Retired
    preserves: shop.tools.Tool
    instance: id
  - name: in-maintenance          # refused in every other state
    error: shop.tools.InMaintenance
  ```

  `when_subject_state:` goes only on a `moves:`, `updates:` or `preserves:` branch with `instance:`,
  never beside `wrong_state:` in the same command (`conflicting_declaration`). A `preserves:`
  branch carries no error, event or `sets:` (`refusal_mutated_state`); a refusal names no subject.
- **A service that publishes no events still needs one per success outcome that changes a
  record.** `validate` refuses an outcome that neither emits nor names an error (`empty_change`); a
  view does not count. Declare an event for the fact the outcome produces (`OrderPaid`), leave it
  out of the component's `publishes:` list because the service does not publish it, and say so in a
  comment and the report. A success that changes nothing is not this case: `preserves:` with a
  subject, `accepts: nothing` without one (`ess/15`).
- **The code's own shapes have constructs; use them rather than `UNMAPPED:`**
  ([later formats](../specifying/references/later-formats.md); raise `format:` to the one named):

  | the code | declare |
  |---|---|
  | deletes the row at the end of its life | `deletes:` (`ess/15`), not a terminal state no row holds |
  | inserts a row already past the first status | `into: <State>` beside `creates:` (`ess/15`) |
  | answers an unknown id differently from a wrong state (a `404`) | `unknown_instance: true` with its error (`ess/15`) |
  | runs every request inside a session or tenant that must exist first | system `preconditions:` (`ess/15`) |
  | needs a deployed id a generator cannot invent | `fixture_inputs:` on the command (`ess/13`) |
  | writes a stored value plus one, or echoes the stored value | `{increment: 1}`, `{subject: <field>}` (`ess/14`) |
  | sends a JSON key under another spelling, a key starting `_`, an opaque JSON body | `naming: {wire: <key>}` on the field, the name `_key` as is, `Json` (`ess/15`) |
  | compares text ignoring case | `equals_ignore_case` / `in_ignore_case` (`ess/15`, ASCII only; Unicode folding in the code is a disagreement to report) |
  | renames a record's key in place (`UPDATE … SET name = $new`) | `updates:` whose `sets:` writes the identity, with a `when_related:` refusal for a taken key (`ess/23`) |
  | deletes every row a query matches | `deletes:` with `instances: {where: …}` and `{count: changed}` (`ess/23`) |
  | upserts one row per element of a request list | `affects:` with `each:` and `instance:` over a `distinct:` member (`ess/23`) |
  | keeps a fixed table of facts per enum value | enum `attributes:`, read in guards as `<field>.<attribute>` (`ess/23`) |
  | reports the current status in an error body | `{subject: state}` in the error payload (`ess/23`) |
  | returns or reads an object whose readers must ignore members they do not know (an extension object, `#[serde(flatten)]` into a map, `additionalProperties: true`) | `undeclared_fields: ignored` on the struct type, or on the command for its response (`ess/24`); events and errors stay closed |
- **An entity the code gives no status still needs a lifecycle.** The language requires one, so
  declare a single state that is both `initial` and `terminal` (`Stocked`), and say in a comment
  that it is structural, not read from the code. That is the one state a retrofit may name.
- **Wire values keep their spelling in a comment.** A stored status `in_transit` becomes the state
  `InTransit`; state names must start upper-case.
- **A rule the language cannot hold stays in the code, and is named.** A limit read from the
  addressed record's stored fields is `when_subject: {predicate: …}`, compared with the request as
  `input.<field>` from `ess/15`; another entity can be read with `when_related:` (`ess/18`), including selected row sets in
  `ess/22`. `instances:` and `affects:` express selected record effects, and `affects: each:`
  per-element ones from `ess/23`. Validate the exact
  combination and record `UNMAPPED:` with its source line only for a rule or target the current
  release actually refuses; [current examples](../specifying/references/current-features.md)
  distinguish declaration, synthesis and implementation support.

## 3. Prove the draft describes the system

A validated draft only says the specification is coherent. It says nothing about the system until a
suite runs against it:

```console
ess verify conform synthesize --path <specification> --out <suite.json>
```

Then follow the `ess:testing-conformance` skill: build or pick a target that drives the real implementation,
run the suite, and break one behaviour to prove a named scenario goes red. A retrofit is done when
the suite runs against the real system, not when `validate` exits 0.

## 4. Report

- the specification path and `ess specify validate` output, verbatim
- every `UNMAPPED:` marker, one line each, with what would settle it
- every place two sources disagreed
- the suite counts (`passed`/`skipped`/`failed`) if a run happened, or that none did yet

## Agents

- `retrofitter` — derives a specification for an existing system under this skill.

## Next

- The draft validates: extend it with `ess:specifying`; hold the implementation to it with `ess:testing-conformance`.
