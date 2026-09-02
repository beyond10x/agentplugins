---
name: ess-schema
description: Validate Executable System Specifications and guide deterministic JSON Schema or OpenAPI projections through the `ess` command. Use when a repository contains an ESS `system.yaml`, generated schema or OpenAPI artifacts, an `ess-*` format, when a story or epic introduces an entity — a noun the plan needs a typed home for, whether or not a specification exists yet — or when the user asks about specification validation, compilation, schema generation, projection drift, adapter coverage, or unsupported semantics.
---

# ESS schema validation and projection

Treat the Rust-owned specification model and compiler as the authority. Do not infer a contract
from generated artifacts and do not repair a failed import by inventing lifecycle meaning.

## Starting a domain from nothing

A story or epic can introduce a noun before any specification exists. That is still this skill's
job: the domain is drafted first, so the noun has a typed home before stories are written around it.

Where an OpenAPI document already describes it, do not hand-write the domain. Draft it from the
contract, and read the decisions the draft says it could not take:

```console
aep reverse openapi --domain <domain> <openapi-document> --out <file>
```

Otherwise write the smallest document that validates — two files, and nothing that is not required:

```yaml
# system.yaml
format: ess/1
system: warehouse
version: v1

domains:
  - warehouse.shipment
```

```yaml
# domains/shipment.yaml
domain: warehouse.shipment

entities:
  - name: warehouse.shipment.Shipment
    identity:
      name: shipment_id
      type: Uuid
    fields:
      - name: destination
        type: String
    lifecycle:
      initial: Draft
      states: [Draft]
      terminal: [Draft]
```

Then check it before anything is written around it:

```console
$ ess validate --path <specification>
warehouse v1 — 2 file(s), valid
```

That is the verbatim output of `ess` 0.3.0 over exactly those two files, exit status 0.

Every line above is load-bearing, and the refusals say why. An entity without `identity`, `fields`
or `lifecycle` is refused as a missing field. A state with no outgoing transition must be listed
under `terminal:`, or the compiler refuses it as `dead_end_state`. A transition no command outcome
takes is refused as `missing_causation` — so a first domain has one state and no transitions, and
gains a second state only together with the outcome that moves it. The header's `domains:` list and
the declaring source must agree in both directions; either half alone is a refusal.

Grow it from there — types, commands, events, views, and a component that owns the domain — running
`ess validate` after each addition rather than at the end.

**A draft is a proposal, never a silent completion.** Every relation you could not read from code,
an OpenAPI document or an existing artifact is written with an `UNMAPPED:` marker beside the place
it would go, and named again in the report:

```yaml
      # UNMAPPED: the epic says a shipment has a carrier; no code, contract or
      # artifact here says what a carrier is. Ask before typing it.
```

Imports never guess, and a domain an agent drafts is an import. Never invent a field type, a
lifecycle state or an edge to make a document validate — leave the marker, and name what would
settle it.

## Read before changing

From the specification root, establish the current answer:

```console
ess validate --path <specification>
ess compile --path <specification> --format json
```

`validate` and `compile` accumulate diagnostics. Relay every refusal; do not stop at the first or
edit generated output around it.

## Deterministic projections

Use the projection command, never a handwritten parallel generator:

```console
ess generate --path <specification> --kind schema --out <directory>
ess project openapi --path <specification> --out <directory>
```

The same typed IR must produce the same ordered files and bytes. Compare a regenerated temporary
tree with the committed tree before replacing anything. A stale committed file is drift; a file no
projection owns is not authority.

## Adapter contract

- Importers produce typed IR plus coverage, diagnostics, and unresolved references.
- Projectors produce artifacts plus obligations and refusals; they never apply infrastructure.
- An importer never guesses missing semantics.
- Round trips are claimed only for an adapter's declared supported subset.
- Source-specific detail belongs in explicit typed structures, not an arbitrary JSON property bag.

When an adapter refuses a construct, keep the refusal visible and name the first concrete type or
semantic rule that would close it. Do not propose a generic facet registry or a new persisted format
merely because one source carries more fields.

## Format changes

A new format version is needed when meaning, identity, reference rules, canonicalization, names, or
the persisted envelope changes. Adding internal Rust capabilities is not enough. Before extending a
strict v1 reader such as `infra-ir/1`, add an old-reader compatibility test; unknown fields are
currently refused.

Finish by running the repository's full gate and report the exact command and exit status.
