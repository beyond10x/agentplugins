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
- **A command the code accepts and ignores in every state it does not act from is `wrong_state:
  true` with `refuses: false`** and no error: the scenario then requires success and no change,
  which is the code's behaviour. Add no error the code never raises. A command has at most one
  `wrong_state:` outcome, so when it ignores in one state and refuses in another, write no
  `wrong_state:` at all (`format: ess/7` or later): guard each branch by the held state, and let
  one effect-free error answer the rest.

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
- **An entity the code gives no status still needs a lifecycle.** The language requires one, so
  declare a single state that is both `initial` and `terminal` (`Stocked`), and say in a comment
  that it is structural, not read from the code. That is the one state a retrofit may name.
- **Wire values keep their spelling in a comment.** A stored status `in_transit` becomes the state
  `InTransit`; state names must start upper-case.
- **A rule the language cannot hold stays in the code, and is named.** A limit read from the
  addressed record's stored fields is `when_subject: {predicate: …}`, compared with the request as
  `input.<field>` from `ess/15`; a constraint across records, or on another entity, is `UNMAPPED:`
  with its source line ([syntax reference](../specifying/references/syntax.md), "What `when` can
  and cannot say").

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
