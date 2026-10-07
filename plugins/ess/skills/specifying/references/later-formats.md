# ESS syntax beyond `ess/1`

What formats after `ess/1` add to the lending library of [syntax.md](syntax.md). Read it when a
rule needs one of the constructs below, or when `validate` refuses one as
`unsupported_format_version`.

A construct needs the `format:` the table names, in `system.yaml`; a higher format admits every
lower one, and under a lower header `validate` refuses the construct as `unsupported_format_version`
at its key. Raising the header past `ess/3` also brings one rule the `ess/1` library does not meet:
every field of an emitted event needs a `payload:` source, so `BranchOpened.branch_id` and
`CopyAdded.copy_id` get `{generated: true}` (the implementation mints the identity).

| to say | format | how |
|---|---|---|
| a text's characters, its length, the value synthesis starts from | `ess/11` | `alphabet:` on a `String` newtype, `.count` on text, `example:` on a scalar input |
| one external refusal for many commands | `ess/12` | top-level `outcome_groups:` selecting by `commands:`, `actor:` or `domain:` |
| an input a provider supplies (a deployed id) | `ess/13` | `fixture_inputs: {borrower_id: current-borrower}` on the command |
| a value read from the stored row, added to it, or minted | `ess/14` | `{subject: title}` (the row before this outcome, on `moves:`/`updates:`), `{increment: 1}` (`sets:` only), `{generated: true}` (now also in `sets:`), `{cleared: true}` (an `Optional` field holds nothing), `{input: shelf_mark, else: {generated: true}}` for an optional input. The text `subject.title` is refused as `misspelled_reference` |
| an unknown id answered apart from a wrong state | `ess/15` | `unknown_instance: true` with an `error:` (or `refuses: false`), below |
| a record removed | `ess/15` | `deletes: <Entity>` with `instance:`, no `sets:`, below |
| a record created past `initial` | `ess/15` | `into: OnLoan` beside `creates:` |
| success with no subject and no effect | `ess/15` | `accepts: nothing`, below; with a subject it is `preserves:` |
| a command every scenario runs inside | `ess/15` | `preconditions:` in `system.yaml`, below |
| a text prefix, an unstructured JSON value | `ess/15` | `prefix: "SM-"` on a `String` newtype; `of: Json` (a payload fills it from an input) |
| how an absent `Optional` travels | `ess/15` | `presence: omitted_when_absent` or `null_when_absent` on the field |
| an aggregate over an `Optional` field | `ess/15` | `aggregate: {sum: replacement_cents, skip_absent: true}`, result `Optional<Integer>`; an `Optional` group key makes absent one group |
| a field's own wire name, a leading underscore | any | `naming: {wire: copyId}` (same as flat `wire:`) on a field of an entity, struct, event or command input; a view field refuses it (`unknown field naming`). `_receipt` is a valid field name |
| a branch chosen by the subject's held state | `ess/3`; `ess/7` for the error default | `when_subject_state: <State>`, below |
| a command's typed answer, and an event field that repeats it | `ess/4`; `returns: true` from `ess/17` | `response: [{name, type}]` on the command, `returns: true` on the outcome, and `{response: code}` as an event payload source. `creates:` still takes its `instance:` from an emitted event field, so a created identity that is also the answer is minted once in the response and copied into the event. `sets:` refuses `{response: …}` |
| a description, and the records that explain a construct | any | `naming: {summary: …}` on an entity, type, actor or command; a flat `summary:` on a field, error or outcome; `refs: ["tracker:KEY-1"]` (`provider:key`) on a command or outcome. A flat `summary:` on an entity, actor or command is an unknown field |
| a value only its originating response may disclose | `ess/21` | `one_time_response: [token]` on the **outcome**, beside `returns: true`; on the command it is an unknown field. It names required `String` response fields, or transparent `String` newtypes; an `Optional` one is refused as `type_mismatch` |
| a rule on the current time | `ess/16`; `ess/22` | `now`, and offsets such as `now - 10m`, compared with a `Timestamp` in a command outcome's `when:` over its input; from `ess/22` also in `when_subject:` and `when_related:`. An invariant, a view filter, a selection or a set-effect filter refuses it: none of them is read while a request is handled |

```yaml
# system.yaml: every scenario, and every explorer sequence, first opens this branch
format: ess/15
system: library
version: v1

domains:
  - library.lending

preconditions:
  - command: library.lending.OpenBranch
    as: library.lending.Librarian
    input: {name: Central}
```

```yaml
  - name: library.lending.ReturnCopy
    naming:
      wire: return-copy
      display: Return a copy
    input:
      - name: copy_id
        type: library.lending.CopyId
      - name: title
        type: library.lending.Title
      - name: note
        type: Optional<String>
    outcomes:
      - name: wrong-title
        when_subject:
          predicate: title != input.title
        error: library.lending.TitleMismatch

      - name: returned
        moves: library.lending.Copy.return
        instance: copy_id
        sets:
          loans: {increment: 1}
        emits:
          - library.lending.CopyReturned
        payload:
          library.lending.CopyReturned:
            copy_id: input.copy_id
            title: {subject: title}
            note: input.note
            _receipt: {generated: true}
        summary: The copy is back on the shelf.

      - name: wrong-state
        wrong_state: true
        error: library.lending.CopyStateConflict
        summary: The copy is not on loan, so nothing was returned.

  - name: library.lending.WithdrawCopy
    input:
      - name: copy_id
        type: library.lending.CopyId
    outcomes:
      - name: withdrawn
        deletes: library.lending.Copy
        instance: copy_id
        emits: [library.lending.CopyWithdrawn]
        payload:
          library.lending.CopyWithdrawn:
            copy_id: input.copy_id

      - name: no-such-copy
        unknown_instance: true
        error: library.lending.CopyNotFound

  - name: library.lending.SuggestTitle
    input:
      - name: title
        type: library.lending.Title
    outcomes:
      - name: placeholder
        when:
          title: {in_ignore_case: ["untitled", "tbd"]}
        error: library.lending.ReservedTitle

      - name: noted
        accepts: nothing
```

```yaml
  - name: library.lending.CopyReturned
    fields:
      - name: copy_id
        type: library.lending.CopyId
        naming: {wire: copyId}
      - name: title
        type: library.lending.Title
      - name: note
        type: Optional<String>
        presence: omitted_when_absent
      - name: _receipt
        type: Uuid
```

These blocks, and every row of the table, are excerpts of the library raised to `ess/15` with the
fields, errors, events and views they name added (`loans`, `CopyNotFound`, `CopyWithdrawn`, a
`Copies` view publishing `state`, `title` and `loans`); that whole validates and synthesizes 23
scenarios with 0 refusals.

What synthesis does with them: an `unknown_instance:` scenario sends an id nothing holds; after a
`deletes:` it requires no `read_your_writes` view to hold the row, then re-sends for the unknown-
instance answer; `accepts: nothing` requires every parameterless immediate view unchanged; a
`when_subject` with `input.` gets one scenario with the input equal to the stored value and one
different. Two limits observed on this library: comparing or grouping by
`branch_id`, the `via:` of `Branch owns Copy`, left those scenarios unbuilt (`ESS-SYNTH-003`,
`ESS-SYNTH-017`), so the examples use `title`.

**A limit per holder** ("a member can have five packets out at once"). When each packet records
its borrower and lifecycle state, address that packet and count the rows already lent to the
member (`ess/22`):

```yaml
      - name: at-limit
        when_related:
          entity: seeds.lending.Packet
          where:
            all:
              - borrower_id == input.member_id
              - state == Lent
          count: {gte: 5}
        error: seeds.lending.LimitReached
      - name: borrowed
        moves: seeds.lending.Packet.lend
        instance: packet_id
        sets: {borrower_id: input.member_id}
        emits: [seeds.lending.PacketBorrowed]
        payload:
          seeds.lending.PacketBorrowed:
            packet_id: input.packet_id
            member_id: input.member_id
```

This models the limit without a separate counter. Declare the Packet fields, lifecycle, typed
inputs, event, error and observable views around this excerpt. The seed-library trial validates
this shape but synthesis refuses the at-limit arrangement with `ESS-SYNTH-001`; validation does
not make the boundary executable. Retain that refusal and cover the intended boundary with an
authored scenario against the real target. Do not promise a fixed refusal code for every model.

A separate Member counter guarded by `when_subject` and changed by `{increment: 1}` is valid
for a counter-only model, but does not also move a particular Packet. The current `affects`
selector rejects selecting by the selected entity’s identity, and set effects reject
`{increment: …}`. The validated set-effects example instead selects by stored `team` and writes
literal fields; it is not a recipe for an atomic packet-plus-member-counter transaction.

Current related-row support also distinguishes identity-addressed guards from row-set guards:
several identity-addressed rows may be read, but mixing an identity-addressed member-existence
guard with the packet row-set limit is refused. Name that missing registration check explicitly
if the real domain requires it; do not claim that the limit verifies membership.

**A branch chosen by the held state.** When one command succeeds from one state, does nothing in a
second and refuses in the rest, guard each branch with `when_subject_state:` and let one
effect-free error answer every other state (`format: ess/7` or later):

```yaml
  - name: shop.tools.RetireTool
    input: [{name: id, type: Uuid}]
    outcomes:
      - name: retired
        when_subject_state: Active
        moves: shop.tools.Tool.retire
        instance: id
        emits: [shop.tools.ToolRetired]
        payload:
          shop.tools.ToolRetired: {tool_id: input.id}
      - name: already-retired
        when_subject_state: Retired
        preserves: shop.tools.Tool
        instance: id
      - name: in-maintenance
        error: shop.tools.InMaintenance
```

Two limits, both `conflicting_declaration`: `when_subject_state:` goes only on a `moves:`,
`updates:` or `preserves:` branch with `instance:` (not on an `error:`, `external:` or `creates:`
branch), and a command that uses it declares no `wrong_state:` outcome. A `preserves:` branch
carries no error, event or `sets:` (`refusal_mutated_state`).

A held-state branch with no input guard claims every request in its states, an `external:` one
included. Where it claims an external branch in every state that branch is taken from, synthesis
writes no scenario for the external branch (`ESS-SYNTH-003`) or for the states reached only
through it (`ESS-SYNTH-004`), and names the guards. To keep the scenario, give the held-state
branch an input guard the external case does not meet.

A specification lowered to Entity Runtime is refused there, not at `validate`, for value
expressions (`ValueExpressionUnsupported`), the `ess/15` outcome shapes (`OutcomeShapeUnsupported`)
and the case-insensitive operators (`CaseFoldUnsupported`); keep those out of a model that must lower.

## From `ess/16` through `ess/23`

The source language and conformance-suite version are different contracts. Choose the source
format for the construct; let synthesis select its required suite format.
`ess specify formats --since ess/15` prints what the installed build says each format added.
With current ESS:

| format | additions |
|---|---|
| `ess/16` | related values; literal fallbacks; optional aggregate presence; `input_absent`; `existing_instance`; caller attributes; view paging; bounded retries; `instances` and `affects` |
| `ess/17` | typed direct responses with `returns: true` |
| `ess/18` | `when_related`; stored `state` in subject predicates; lists of subject states; binding delivery context |
| `ess/19` | `payload` sources for declared error fields |
| `ess/20` | related row lifecycle `state` in predicates |
| `ess/21` | `one_time_response` non-disclosure contracts for required String response fields |
| `ess/22` | explicit fact operands and constant offsets; UTF-8 byte lengths; instant comparison; `distinct` list keys; selected row guards/reads; Optional and two-hop related reads; several related rows; calendar windows; compensating external refusals; conditional aggregates and binding payload guards; per-outcome failure policy; lifecycle moves in `affects`; view grants; unit union variants; dotted input values |
| `ess/23` | identity re-key by `updates:`; the held state as a value, `{subject: state}`; bulk `deletes:` with `instances:` and `deletes:` in `affects`; typed enum variant attributes; one record per input-list element with `affects: each:`; a `when_subject` predicate refusal asserts the whole record unchanged; row-set selectors on updates, deletes and upserts, scoped by a member of a struct identity |

For related guards, selected effects, event transports, client generation, finite protocol models
and compatibility gates, read and run [current-features.md](current-features.md). Check the actual
selected target: a source construct validating does not mean code generation or Entity Runtime
can lower it. The current lowering report lists each unsupported construct by name.

In every format an `affects:` entry selects its rows with `where:` or, from `ess/23`, writes them
with `each:`; an entry with neither is refused as `missing_declaration`.

## `ess/23`

[examples/shelves.yaml](examples/shelves.yaml) uses each construct below, validates, and
synthesizes 16 scenarios with 0 refusals; the `interpreted` target passes all 16. The excerpts are
from it.

**A renamed identity.** An `updates:` whose `sets:` writes the entity's identity moves the record:
it comes to rest under the identity written, every other field is carried over, and the old
identity names nothing. The command must declare its collision answer as a `when_related:`
refusal on the written input, or `validate` refuses the outcome as `missing_declaration`:

```yaml
      - name: taken
        when_related:
          entity: demo.shelves.Shelf
          where: code == input.new_code
          exists: true
        error: demo.shelves.CodeTaken
      - name: recoded
        updates: demo.shelves.Shelf
        instance: code
        sets: {code: input.new_code}
        emits: [demo.shelves.ShelfRecoded]
        payload:
          demo.shelves.ShelfRecoded: {code: input.code, new_code: input.new_code}
```

The identity write is refused beside `compensates:`, in a create-or-update pair, on an entity a
declared relation carries, and on a struct identity. Generated Rust renames the row; generated Go
refuses the write (`this target cannot move a record to another identity`), and so do the Web and
Clap targets (`MissingRepresentation`) and Entity Runtime (`IdentityChangeUnsupported`).

**One record per input element.** `each:` names an input list and `instance:` the member that
addresses a record; a held record is updated and a missing one created in the lifecycle's
`initial` state. A declared `distinct:` must keep that member distinct:

```yaml
      - name: duplicated
        when: {not: {distinct: {in: arrivals, as: a, by: a.copy_id}}}
        error: demo.shelves.DuplicateArrival
      - name: shelved
        updates: demo.shelves.Shelf
        instance: code
        sets: {label: input.label}
        emits: [demo.shelves.CopiesShelved]
        payload:
          demo.shelves.CopiesShelved: {code: input.code}
        affects:
          - entity: demo.shelves.Copy
            each: {in: input.arrivals, as: arrival}
            instance: arrival.copy_id
            sets: {shelf: input.label, format: arrival.format}
```

`each:` needs a subject and is refused beside `where:`, `moves:` or `deletes:`.

**Every row a filter selects, removed.** `deletes:` takes `instances:` and counts the rows with
`{count: changed}`; an `affects:` entry may also declare `deletes: <Entity>` over an entity of the
outcome's own domain, also beside a `deletes:` subject:

```yaml
      - name: cleared
        deletes: demo.shelves.Copy
        instances: {where: shelf == input.shelf}
        emits: [demo.shelves.ShelfCleared]
        payload:
          demo.shelves.ShelfCleared: {shelf: input.shelf, removed: {count: changed}}
```

**Facts a closed set carries.** An enum declares `attributes:` in the `{name, type}` shape of
`fields:`, and each variant fills them with typed literals. A guard, invariant or view filter reads
`<fact>.<attribute>`, lowered to membership over the variants that satisfy it:

```yaml
  - name: demo.shelves.Format
    kind: enum
    attributes:
      - {name: loanable, type: Boolean}
    variants:
      - {name: Hardcover, attributes: {loanable: true}}
      - {name: Paperback, attributes: {loanable: true}}
      - {name: Reference, attributes: {loanable: false}}
```

```yaml
      - name: not-loanable
        when_subject:
          predicate: format.loanable == false
        error: demo.shelves.NotLoanable
```

Projections carry `x-ess-attributes`, and the types-only Rust, Go and TypeScript outputs an
accessor per attribute. Reading an attribute as a value in `sets:` is refused, and
`ess verify diff` reports an attribute change as unclassified.

**The state the record held.** `{subject: state}` reads the lifecycle state before the outcome,
in an error payload, an event payload or `sets:`:

```yaml
      - name: wrong-state
        wrong_state: true
        error: demo.shelves.CopyStateConflict
        payload:
          demo.shelves.CopyStateConflict: {current: {subject: state}}
```

Generated Rust and Go implementations, and Entity Runtime, refuse `instances:` and `affects:`
(`this target cannot change the rows a filter selects`); `each:` is refused by every code target
and Entity Runtime. Validate and synthesize against the specification, then check the target you
actually ship with its own generation command.

For a valid stored state bounded arrangement cannot reach, current ESS admits an explicit
`--synthesis-seed <authored-file> <instance>` containing a typed setup row. The suite records seed
provenance and selects `/42` or `/43`; the target must establish and validate that real row.
A seed does not execute the authored document's timeline or replace a command's assertions. For a
missing input candidate, first supply a truthful `example:`: the reservation fixture's distinct
member identity restores the wrong-state scenario without changing the rule or injecting state.

## What validates and still gets no scenario

`ess verify conform synthesize` observes a record only through a view, and these limits come from
that. Each was reproduced on 0.55.0; relay the refusal rather than reshaping the rule around it.

| the model says | synthesis answers |
|---|---|
| a guard over a stored field (`when_subject: {predicate: holder != input.holder}`) or a `wrong_state` refusal, on an entity with no view | `ESS-SYNTH-001` "no witness": choosing an input that matches the stored value needs a view with no filter that projects the identity, `state` and the guarded fields, `read_your_writes` or an `eventual` one it waits for. The facts are observed, not assumed: the creating outcome's `sets:` and a seed do not stand in for the view |
| `--synthesis-seed` for such a row | not applied when the row is held by an `owns` relation: `synthesis seeds: 1 selected, 0 applied`, naming the owner |
| a `String` newtype with `alphabet:` or `invariants:` as a field of a typed `response:` | the returning outcome has no scenario: `response constrained type needs an executable invariant/reading observer` |
| `invariants:` on a type (`value.count >= 1`) with no view publishing a field of it | `ESS-SYNTH-013` "type … has no scenario"; `alphabet:` alone is checked without one |
| `when_subject:` comparing `now` with a stored `Timestamp` the outcome set with `{generated: true}` | every scenario of the command is refused: no arranging branch sets the field from an input or a literal |

A system whose source defines no read surface (a published protocol, a credential that is never
read back) declares no view, and keeps these refusals visible in its report. Declaring a view to
silence them states a read surface the system does not have.
