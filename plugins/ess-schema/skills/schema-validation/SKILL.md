---
name: ess-schema
description: Validate Executable System Specifications and guide deterministic JSON Schema or OpenAPI projections through the `ess` command. Use when a repository contains an ESS `system.yaml`, generated schema or OpenAPI artifacts, an `ess-*` format, or the user asks about specification validation, compilation, schema generation, projection drift, adapter coverage, or unsupported semantics.
---

# ESS schema validation and projection

Treat the Rust-owned specification model and compiler as the authority. Do not infer a contract
from generated artifacts and do not repair a failed import by inventing lifecycle meaning.

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
