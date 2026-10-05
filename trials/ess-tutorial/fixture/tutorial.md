---
title: Your first ESS specification
sidebar_label: Your first ESS specification
description: Specify a lending library and hold a Rust implementation to its generated conformance suite.
---

# Your first ESS specification

An Executable System Specification describes records, commands, refusals, events and readable views.
ESS validates that model and generates checks that an implementation must answer. This tutorial
uses **ESS 0.53.0 and Rust**, verified on 2026-10-05. The
[2026-09-28 Go recording](./first-ess-specification-2026-09-28.md) is retained as historical evidence.

You finish with a specification, generated documentation and OpenAPI, a small Rust library and
17 generated scenarios running against that library. The target adapter reports actual commands,
events and views; the ESS runner makes the assertions.

Install Rust and Cargo, and use Claude Code or Codex with the ESS plugin. The complete
[example directory](https://github.com/beyond10x/agentplugins/tree/main/website/docs/tutorials/first-ess-specification)
contains `spec/` and `impl/`. Copy those directories to an empty working directory to follow exactly.

## 1. Install ESS and its plugin

Ask your agent to follow the release's
[SETUP.md](https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md) and select ESS.
With `b10x` already installed:

```console
b10x init ess --host claude --out plan.json
b10x setup apply --plan plan.json --yes
```

Read the plan before applying it. Select `--host codex` for Codex and start a new session after
installation. The `ess:specifying` skill helps write and validate the model.

## 2. Describe the decisions

```text
Write a specification for a lending library in spec/. Each book is a physical copy with a title,
author and generated identity. Register members with a name and generated identity. A book starts
OnShelf, can be borrowed into OnLoan, returned to OnShelf and withdrawn into Withdrawn. Borrowing
and withdrawing refuse from other states. Store the borrower on the book; there is no loan record,
loan limit, due date or ISBN. A return only needs the book identity. Expose Catalogue, Members and
BooksOnLoan views with read-your-writes consistency. One network component owns the domain.
For this introductory model, borrower identity is recorded without checking registration.
```

This last choice bounds the tutorial's claim. ESS now has `when_related` guards for other records;
it is no longer true that all cross-record checks are unexpressible. Combining particular guards
can still be refused by validation. The separate validated related-guard and set-effects examples
in the [plugin resources](https://github.com/beyond10x/agentplugins/tree/main/plugins/ess/skills/specifying/references/examples)
show those capabilities without changing this tutorial's lifecycle checks.

## 3. Read the specification

`spec/system.yaml` selects the current source language:

```yaml
format: ess/22
system: library
version: v1
domains:
  - library.lending
```

`spec/ess-inputs.yaml` makes the source selection and toolchain explicit:

```yaml
format: ess-inputs/2
requires: ess 0.53.0
specification:
  - system.yaml
  - components.yaml
  - domains/lending.yaml
scenarios: []
```

In `domains/lending.yaml`, read `Book`'s lifecycle, then `BorrowBook`'s outcomes. `borrowed`
moves the book through `lend`, stores `input.member_id` and emits `BookBorrowed`. `wrong-state`
answers `BookStateConflict`; `no-such-book` answers `BookNotFound`. `ReturnBook` clears the
optional borrower field. The three views expose enough state for the suite to check these rules.
`components.yaml` declares the network component and its command and event surfaces.

## 4. Validate

```console
ess specify validate --path spec
```

```text
library v1 — 3 file(s), valid
```

Validation checks the declarations. It does not establish that an implementation obeys them.

## 5. Generate the public contract

```console
ess generate --kind docs --path spec --out out
ess generate --kind openapi --path spec --out out
ess specify graph --path spec --format mermaid
```

The output directories contain the domain documentation and the library service's OpenAPI contract.
Regenerate these when the specification changes.

## 6. Hold the Rust library to it

The Rust runner consumes canonical suite IR. The CLI's conformance package targets are `go` and
`typescript`; **there is no `--target rust` for conformance synthesis**. Generate the IR instead:

```console
ess verify conform synthesize --path spec --target ir --out impl/suite.json
cargo test --locked --manifest-path impl/Cargo.toml -- --nocapture
```

The synthesis summary is:

```text
17 scenario(s) (0 authored), 0 refusal(s), written to impl/suite.json
```

`impl/src/lib.rs` implements the library independently of the suite. `impl/tests/conformance.rs`
implements `ess_conformance::target::ConformanceTarget`, reads `suite.json`, admits it, and runs it
through `Runner`. `Cargo.toml` and `Cargo.lock` select ESS's exact 0.53.0 Rust crates.

Every scenario starts with an empty library. A mutation advances the store's revision; its answer
carries that revision as an opaque consistency token. A view request demanding `AtLeast(token)`
checks that revision before reading the same synchronous store. An invalid or future token is an
error, never permission to return a weaker read. Returning rows alone without a command token
caused 14 of the original tutorial's 17 scenarios to fail under current ESS.

The native runner's output is:

```text
running 2 tests
test read_refuses_invalid_or_future_consistency_tokens ... ok
conformance scenarios: {Passed: 17}
test conforms_to_generated_suite ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cargo counts two integration tests: one runs the 17 ESS scenarios, and one proves invalid or
future consistency tokens are refused. The ESS report counts the scenarios separately.
The test rejects an empty suite and every failed, unsupported or errored scenario.

## 7. Watch it catch a bug

In `impl/src/lib.rs`, change only `borrow`'s state guard:

```diff
-        if book.state != "OnShelf" {
+        if book.state == "Withdrawn" {
```

Run the same Cargo command. The scenario
`library.lending.Book/state/OnLoan/refuses/library.lending.BorrowBook` must fail: the library now
answers `borrowed` where the model requires `wrong-state`. Restore the guard and rerun; all
17 scenarios must pass again. A compile failure does not prove the suite catches the defect.

## 8. Keep it true

Make validation, regeneration and the native runner part of your build:

```console
ess specify validate --path spec
ess verify conform synthesize --path spec --target ir --out impl/suite.json
cargo test --locked --manifest-path impl/Cargo.toml -- --nocapture
```

Keep `suite.json` generated and commit the source model, implementation and lockfile. When ESS
releases, upgrade the CLI and both Rust dependencies together, regenerate the lockfile, and run
these checks before claiming verification against the new release.

## Beyond one record

Use `when_related` for a decision about another row, and `instances` or `affects` for selected
record effects. `affects` can also move selected records in `ess/22`. These declarations do not
by themselves promise atomic multi-record transactions. Validation, synthesis and code generation
have different supported subsets; keep each refusal visible and distinguish generated suite
coverage from implementation coverage. In ESS 0.53.0, an identity-addressed related guard beside `unknown_instance` is refused.
An existence-only input-related guard beside `wrong_state` is also refused. With `ess/22`,
`wrong_state` can coexist with related guards when at least one present-row predicate is
declared and every such predicate branch refuses. These are distinct ordering cases; validate
the exact declaration instead of treating all related lifecycle checks as unsupported.

Next, [plan a library feature](./first-governed-plan.md), use `ess:retrofitting` for an existing
service, or use `ess:hardening` to test what a green suite still misses.
