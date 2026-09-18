---
name: specify
description: >-
  Validate Executable System Specifications and guide deterministic JSON Schema or OpenAPI projections through the `ess` command. Use when a repository contains an ESS `system.yaml` or `ess-inputs.yaml`, generated schema or OpenAPI artifacts, an `ess-*` format, when a story or epic introduces an entity — a noun the plan needs a typed home for, whether or not a specification exists yet, including a repository with no `system.yaml` anywhere, where the domain is drafted from nothing — or when the user asks about specification validation, compilation, schema generation, projection drift, adapter coverage, or unsupported semantics. An entity introduction names the noun and something typed about it: an identifier, a field, or a relation to another noun. A story that mentions a noun in passing, an acceptance line, a status change or a plan's prose is not one.
---

# ESS schema validation and projection

Treat the Rust-owned specification model and compiler as the authority. Do not infer a contract
from generated artifacts and do not repair a failed import by inventing lifecycle meaning.

## Ask the binary what it speaks

Do this before writing a `format:` line, because everything below depends on the answer and no
example here can carry it. The authored format has **four majors, and they are not interchangeable**:

| | what it adds |
|---|---|
| `ess/1` | the original |
| `ess/2` | `Binary64` — a new numeric domain, not a spelling |
| `ess/3` | outcome guards over the subject's *held* state, bounded event accessors, ordered list selection |
| `ess/4` | declared error wire names, typed command responses, and **complete emitted payload ownership** |

Declare the highest your `ess` binary accepts. A build older than a construct refuses it **by name**,
so the failure is loud and immediate rather than silent — which is why adding a key needs no new
major, and why copying a `format:` out of a document written a year ago is the expensive mistake.

Declaring an older major than you need is the quiet one. The specification still validates, so
nothing tells you that the branch you wanted was available two majors ago, that a rule you recorded
as unstatable has a spelling, or that a payload field whose source nothing checks would have been
refused. Establish the answer first; re-declaring later means re-reading everything you concluded
under the old one.

## Starting a domain from nothing

A story or epic can introduce a noun before any specification exists. That is still this skill's
job: the domain is drafted first, so the noun has a typed home before stories are written around it.

Where an OpenAPI document already describes it, start from the contract rather than a blank file —
and know what it gives you:

```console
aep plan reverse openapi --domain <domain> <openapi-document> --out <file>
```

**It drafts types, not entities.** An OpenAPI document is a set of payload shapes; an entity is a
thing with an identity and a life, and neither is in there. On a real contract of fifty-odd
operations the draft came back with nearly two hundred types, **zero entities**, and an `UNMAPPED:`
marker for every decision it could not take — its own footer says so: which of these types is an
entity, what its identity field is, which states it holds, and who may issue each command are "the
first question this draft cannot answer".

So take the type vocabulary from it and get identity and lifecycle from the implementation. The
state machines are usually written down already — an enum with a name like `SourceState` or
`RunState` — and that enum is worth more than the whole contract, because it is what the code
actually branches on.

Otherwise write the smallest document that validates — two files, and nothing that is not required:

```yaml
# system.yaml
# The highest major this binary accepts — establish it, do not copy it.
format: ess/4
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
    relations:
      - name: lines
        kind: owns
        target: warehouse.shipment.ShipmentLine
        cardinality: many
        via: shipment_id
    lifecycle:
      initial: Draft
      states: [Draft]
      terminal: [Draft]

  - name: warehouse.shipment.ShipmentLine
    identity:
      name: line_id
      type: Uuid
    fields:
      - name: shipment_id
        type: Uuid
      - name: sku
        type: String
    lifecycle:
      initial: Draft
      states: [Draft]
      terminal: [Draft]
```

Then check it before anything is written around it:

```console
$ ess specify validate --path <specification>
warehouse v1 — 2 file(s), valid
```

That is the output, exit status 0, over both files as printed — including the `relations:` block and
its second entity. It is a validated starting point, not an illustrative shape. The line names the
system and the file count; it carries no release number, and neither should anything you copy from
here. Change the names and semantics to match the repository you actually read, then validate the
changed specification again.

Every line above is load-bearing, and the refusals say why. An entity without `identity`, `fields`
or `lifecycle` is refused as a missing field. A state with no outgoing transition must be listed
under `terminal:`, or the compiler refuses it as `dead_end_state`. A transition no command outcome
takes is refused as `missing_causation` — so a first domain has one state and no transitions, and
gains a second state only together with the outcome that moves it. The header's `domains:` list and
the declaring source must agree in both directions; either half alone is a refusal.

**A relation is an entry, not a convention.** Two entities linked by a field that happens to be
named after the other one are not related as far as anything can check; the `relations:` entry is
what makes the link a fact a program reads. Three more refusals come with it, and they are the
reason it is worth writing:

| Refused | What it means |
|---|---|
| an unknown `target` | the far entity is not declared anywhere in the specification. A relation to a noun nobody typed is the failure the guardrail exists to catch, and it now fails at `validate` rather than at the first schema projection |
| a missing or mistyped `via` | the linking field is not there, or it is not the type of the identity it is supposed to carry. `via` lives on whichever side holds the field: on the **target** for `owns` — the child's field typed by the owner's identity — and on the **source** for `references` |
| a second owner | two entities both claiming to `own` the same one. Ownership decides what a delete does, and two answers to that is not a richer model, it is an undecided question written down twice |

`owns` and `references` are not stylistic. `owns` says the far side does not stand on its own — it
has no meaning without its owner, so a delete of the owner cannot leave it behind; `references` says
it does stand on its own and outlives the link. What the delete *does* — refuse while children
remain, or take them with it — is a command outcome and not a field of the relation, so `owns`
narrows that question without answering it. Where you cannot say which kind it is, that is the
`UNMAPPED:` case below and not a coin toss.

Grow it from there — types, commands, events, views, and a component that owns the domain — running
`ess specify validate` after each addition rather than at the end.

`--path` takes one ESS file or a directory. A directory is read through its `ess-inputs.yaml` when
one exists — an exact file list, which is what lets authored inputs sit in nested directories beside
generated files — and otherwise through the `system.yaml` layout above, which the CLI
calls legacy and still accepts.

**A draft is a proposal, never a silent completion.** Every relation you could not read from code,
an OpenAPI document or an existing artifact is written with an `UNMAPPED:` marker beside the place
it would go, and named again in the report:

```yaml
      # UNMAPPED: the epic says a shipment has a carrier; no code, contract or
      # artifact here says what a carrier is. Ask before typing it.
```

**A relation whose cardinality or ownership you cannot read is `UNMAPPED:`, not an entry with a
plausible value.** `cardinality: many` and `cardinality: one` project different schemas and imply
different stories, and `owns` against `references` decides whether the far side can stand on its own
at all — so a guess there is not a smaller guess than inventing the relation. Write the marker
where the entry would go:

```yaml
      # UNMAPPED: a shipment has lines, and nothing here says whether deleting the
      # shipment deletes them (owns) or orphans them (references). Ask.
```

Imports never guess, and a domain an agent drafts is an import. Never invent a field type, a
lifecycle state or an edge to make a document validate — leave the marker, and name what would
settle it.

## A refusal is real when a handler performs it

A contract documents intent. A handler is what happens. Where the two differ the handler wins, and
they differ more often than a published document suggests — so check a refusal **by reading the code
that would raise it**, not by reading the response it is documented to return.

What this catches, from one real specification audited against its own service:

- **A refusal nothing performs.** The contract listed a conflict for revoking a credential the agent
  did not have. The handler clears the field and answers success, with no state check at all. The
  branch was fiction, and a conformance suite would have failed a correct implementation for it.
- **An idempotent operation modelled as a conflict.** Two deletes answered success whether or not
  the row was there. That is not a refusal; it is an accepted call that moves nothing, and the model
  has a way to say exactly that.
- **An entity that is not a row.** A type named for the tenant turned out to be a request extractor
  over the auth context — nothing stores it, nothing has its fields. It had been given an identity
  and a lifecycle on the strength of its name.

Prefer a deletion to an addition when auditing. A declared refusal nobody executes reads as a rule
and is none; keep every refusal you *do* find visible, and name the first concrete type or semantic
rule that would close it.

## Read before changing

From the specification root, establish the current answer:

```console
ess specify validate --path <specification>
ess specify compile --path <specification> --format json
```

`validate` and `compile` accumulate diagnostics. Relay every refusal; do not stop at the first or
edit generated output around it.

## Writing a rule the grammar can hold

An invariant, a `when:` guard and a view `filter:` are all one predicate language, and it has two
forms. The string form is the one everybody writes:

```yaml
- reminder_count >= 0
- total.amount >= 0
- defined(last_indexed_at)
```

**A compound is not infix.** `kind == WebCrawl or kind == ZendeskHc` does not parse as a
disjunction; the whole string is read as one fact path, and the refusal you get says *undeclared
reference* — which reads as "this language has no `or`" and is worth knowing it is not. Compounds
are structured:

```yaml
- any:
    - not: defined(refresh_interval)
    - refresh_interval == Off
    - kind: {any_of: [WebCrawl, ZendeskHc]}
```

That one is the shape most real field rules take: an **implication**, written as the disjunction it
is equal to. "A schedule may only be set on a kind that can be fetched" becomes "either there is no
schedule, or it is off, or the kind is one of these". `all`, `any` and `not` nest; `any_of` is
membership; `defined(path)` asks whether an `Optional` is there.

One catch worth planning around: an invariant is only checked where a view publishes the fields it
reads. A rule over two fields needs both of them projected somewhere, or synthesis refuses to assert
it rather than emitting a claim that cannot fail.

## Deterministic projections

Use the projection command, never a handwritten parallel generator:

```console
ess generate --path <specification> --kind schema --out <directory>
ess generate project openapi --path <specification> --out <directory>
```

The same typed IR must produce the same ordered files and bytes. Compare a regenerated temporary
tree with the committed tree before replacing anything. A stale committed file is drift; a file no
projection owns is not authority.

## Conformance is a record, not a claim

In the planning store an `executable-system-specification` is `conforming` because a suite ran and
its report says so, never because somebody moved it there. The report is what crosses from ESS to
AEP:

```console
ess verify conform synthesize --path <specification> --scenarios <scenario-directory> --out <suite.json>
ess verify conform run --path <specification> --target <reference> --report-out <report.json>
aep plan artifact evidence <specification-artifact> --from <report.json>
```

`synthesize` writes the suite the specification obliges — nothing for a domain with no commands or
outcomes, so the two-entity draft above yields `0 scenario(s)` and no report worth recording.
`evidence --from` reads the kind, the source and the instant out of the report and refuses a report
of no scenarios or with no `spec_digest`. Then `aep plan artifact set <artifact> --model-digest
<hex>` ties the record to the model the suite ran against, and
`aep plan artifact move <artifact> --to conforming` is decided by the store.

**A built-in `--target` tells you nothing about your implementation.** `--target` lists what this
binary carries, and those are reference and interpreted targets for the model's own examples — one
of them says in its own source that it "derives no behaviour yet: it executes no part of the model".
Pointed at your specification it returns every scenario *unsupported*, which prints like a result
and is the absence of one. A real target is a crate you write against the conformance target trait,
linking the implementation under test; until one exists, the suite is a proposal about your system
and not a statement about it.

**Synthesis arranges a subject by replaying commands, and reads no entity relations.** So a command
that creates a child gets a fabricated owner identity and no owner arranged — and if the
implementation requires the owner to exist, every such scenario reports unsupported. That is not a
defect in your model; it is a precondition the model has no way to state.

Authored scenarios are the answer to what synthesis cannot derive. They live in their own directory,
are selected with `--scenarios`, and say what a person knows and the generator cannot: which row a
declared order puts first, or a starting state no command reaches. A scenario may establish an
entity row directly rather than inventing a command to create it — that is a test-adapter
capability, and it asserts no command, event or history the model does not have.

Two traps, both cheap to hit:

- A suite carrying the newer vocabularies is refused by the ordinary runner entry point **by name**:
  those require admitting the suite first so its exact bytes can be paired with an explicit report.
  The message says so; it is not a broken suite.
- A green count means nothing until you have seen it go red. Break one behaviour deliberately and
  confirm a *named* scenario fails. Then hand off to `ess-specify:coverage`, which is about whether
  the instrument reads anything at all — `passed` is the only number that may be claimed as
  progress, and `skipped` is no information rather than a small failure.

Finish by running the repository's full gate and report the exact command and exit status.
