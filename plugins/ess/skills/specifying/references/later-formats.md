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
| an unknown id answered apart from a wrong state | `ess/15` | `unknown_instance: true` with an `error:` (or `refuses: false`), below. Since 0.60.0 a read-only command (only `preserves:` branches addressed by input, or `when_subject:` refusals) may declare it too; 0.59.0 refused that as `unreachable_branch` |
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
| a record or an answer that admits members it does not declare (a protocol's extension object) | `ess/24` | `undeclared_fields: ignored` on a `kind: struct` type, or on a command with a `response:`, where it governs the response only; [below](#ess24) |
| create the row, or update it by what the stored row holds | `ess/25` | an `unknown_instance: true` branch that `creates:`, beside `when_subject:` branches; [below](#ess25) |
| a text compared after the implementation trims it | `ess/25` | `{trimmed: {in_ignore_case: [none, unknown]}}` around a text operator, in `when:`, `when_subject:` and `when_related:` |
| one event per row a set effect changed | `ess/25` | `{event: <name>, each: changed, when_state: [<State>]}` in `emits:`, its payload read with `{changed: <field>}` |
| a refusal decided before the row is read (a credential, a token) | `ess/25` | `before_row: true` on an `external:` refusal |
| a read by identity that answers a default row for an unknown identity | `ess/25` | `absent: {row: {<field>: <literal>}}` on a view filtered `<identity> == param.<name>` |
| a value computed from the input, the stored row or a view's source row | `ess/27` | `{expr: "subject.hours + input.extra"}` in `sets:` and a `payload:`; `{name: elapsed, type: Duration, expr: "ended_at - started_at"}` on a view field; [below](#ess27-and-ess28) |
| a guard over a digest or encoding of the input (PKCE `S256`) | `ess/27` | `challenge != base64url(sha256(ascii(input.verifier)))` in `when:`, `when_subject:` or `when_related:` |
| a struct written as one of its fields | `ess/27` | `shorthand: <field>` on a `kind: struct` type whose other fields are all `Optional` |
| a step the system takes itself | `ess/27` | `system: true` on an actor; a command only such actors may issue is awaited, not sent, with an optional `await_within: PT<seconds>S` |
| the first row of an ordered selection | `ess/27` | `{related: {entity, where, order_by, first}}` |
| a candidate tested against its own related rows | `ess/27` | `exists: {in: {entity: <E>}, as: <b>, that: <predicate>}` in a selector's `where:`, tied by `<b>.<field> == <candidate field>` |
| each row an input list names moved or deleted by its own answer | `ess/27` | `moves:` or `deletes:` in an `affects:` entry with `each:`, which may carry a `where:` over the element |
| a Boolean event leaf in a binding condition | `ess/27` | `where: [event.bridged == false]` |
| unknown id, related guard and held state on one command | `ess/27` | one precedence rule orders every pair; [below](#ess27-and-ess28) |
| a view field read from the row a reference names | `ess/28` | `field: {related: {via: <reference>, field: <field \| state>}}`, typed `Optional<…>`; a filter may read `<reference>.<field>` |
| several related rows required at once | `ess/28` | `when_related: {all: [{via: input.<a>, predicate: …}, {via: input.<b>, predicate: …}]}` |
| the state of a related row as a value | `ess/28` | `{related: {via: input.<field>, field: state}}`, typed as that entity's `State` |
| a list of references naming a missing row | `ess/28` | `when_related: {via: input.<list>, exists: false}`; the payload reads the first such element with `{missing: input.<list>}` |
| a guard that says what an absent `Optional` compares as | `ess/29` | `coalesce(<fact>, <default>) <op> <literal>` in `when:`, `when_subject:` or `when_related:`, quoted inside a flow mapping; [below](#ess29) |
| an external cause that applies only to some stored rows | `ess/29` | `when_subject: {predicate: …}` beside `external:`; not with `before_row: true`; [below](#ess29) |
| a rule on the current time | `ess/16`; `ess/22` | `now`, and offsets such as `now - 10m`, compared with a `Timestamp` in a command outcome's `when:` over its input; from `ess/22` also in `when_subject:` and `when_related:`. An invariant, a view filter, a selection or a set-effect filter refuses it: none of them is read while a request is handled. For a view, take `from`/`to` view parameters the caller sends (`<field> >= param.from`, `<field> < param.to`), as the refusal's hint says |

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

Declare a branch the held state selects (`when_subject_state:`, `when_subject:`,
`when_state_changes:`) before an accepting `when:` branch or an `external:` branch that one request
could also satisfy. Declared the other way round, `validate` refuses it as `ESS-COMMAND-004`
(`conflicting_declaration`), naming both branches, with the reorder as the hint. An `external:`
branch with no input guard always overlaps. Reordering changes no answer: the held state selects
first in either order.

A specification lowered to Entity Runtime is refused there, not at `validate`, for value
expressions (`ValueExpressionUnsupported`), the `ess/15` outcome shapes (`OutcomeShapeUnsupported`)
and the case-insensitive operators (`CaseFoldUnsupported`); keep those out of a model that must lower.
A String type's `alphabet:` lowers to every field, argument and member it reaches (nested alphabets
as their intersection), and `<text>.count` lowers counted in Unicode scalar values. Still refused
there: an alphabet on a declared response field, a text literal outside its field's alphabet, or
nested alphabets that share no character (`AlphabetUnsupported`), and a text length read through a
quantifier element or a union payload (`TextLengthUnsupported`). The `ess` CLI does not run this
lowering, so these limits come from the ESS release notes; confirm them against Entity Runtime.

## From `ess/16` through `ess/29`

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
| `ess/24` | `undeclared_fields: ignored` on a struct type and on a command response: the record admits members it does not declare, while every declared field stays required and typed |
| `ess/25` | create-or-update selected by `when_subject:`; `trimmed:` text guards; per-row `emits:` with `each: changed`; `before_row: true` on an `external:` refusal; a view's `absent:` default row |
| `ess/26` | a named `external:` refusal of a command that addresses an existing row (`before_row: true` included) leaves the row unchanged: its scenario snapshots the row and requires it unchanged afterwards; a `compensates: true` refusal keeps its read-back |
| `ess/27` | typed expressions (`{expr: …}`) in values and view fields; digest and encoding functions in guards; values narrowed by their branch; `moves:`/`deletes:` in `affects: each:`; a struct `shorthand:`; Boolean leaves in binding conditions; `unknown_instance:` beside `when_related:`; one precedence rule for every guard pair; `{generated: true, present: true}` and `{payload: <Event>.<field>}` in `sets:`; system-issued commands; ordered heads; identity reads in set-effect filters; published values in selectors; `exists:` over a candidate's related rows |
| `ess/28` | view fields and filters read through a reference, one hop; `when_related: {all: […]}`; a related row's `state` as a value; `when_related:` over a list of references with `{missing: …}` |
| `ess/29` | `coalesce(<fact>, <default>)` on the left of a guard's comparison; `when_subject:` beside `external:`; a guarded branch declared below one a later precedence step answers is refused |

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

## `ess/24`

**Members a reader ignores.** A protocol whose readers must ignore members they do not recognise
(an extension object, a response a later revision may add to) says so where it applies, and
nowhere else. `undeclared_fields: ignored` on a `kind: struct` type admits members the struct does
not declare, wherever the struct is reached; on a command it governs the command's `response:`
object only, never its input. Every declared field stays required and typed. `refused` is the
default, and a specification that does not write the key keeps its bytes:

```yaml
types:
  - name: catalog.orders.Extension
    kind: struct
    undeclared_fields: ignored
    fields:
      - name: vendor
        type: String

commands:
  - name: catalog.orders.PlaceOrder
    undeclared_fields: ignored
    input:
      - name: item
        type: String
    response:
      - name: order_ref
        type: String
      - name: extension
        type: catalog.orders.Extension
    outcomes:
      - name: placed
        returns: true
```

`validate` refuses the key at its own line everywhere else: on a command with no `response:` as
`missing_declaration` (`ESS-COMMAND-005`), and on an event or an error as
`unsupported_construct` (`ESS-EVENT-009`, `ESS-ERROR-009`; the hint: declare it on the struct type
the fields belong to), as on every other declaration. Under a header below `ess/24` it is
`unsupported_format_version` naming `ess/24` (`ESS-COMMAND-009`, `ESS-TYPE-009`).

`ess generate --kind schema` and `--kind openapi` write `"additionalProperties": true` at the
opened response object and at the opened struct, wherever it is reached; the command input and
every closed object keep `false`. AsyncAPI follows, and the generated documentation says so on the
command and the struct. The suite carries the openness to the runners ([testing-conformance](../../testing-conformance/SKILL.md)),
and `ess verify diff` rates opening and closing ([spec-diff](../../hardening/references/spec-diff.md)).

## `ess/25`

[examples/devices.yaml](examples/devices.yaml) uses each construct below, validates, and
synthesizes 16 scenarios with 0 refusals on 0.60.0; the `interpreted` target passes all 16. The
excerpts are from it. Under a lower header `trimmed:`, a per-row `emits:` entry, `before_row:` and
`absent:` are `unsupported_format_version` naming `ess/25` (`ESS-COMMAND-009`, `ESS-VIEW-009`).

**Create or update, by what the stored row holds.** The creating branch answers an identity no row
carries, before any predicate reads a row; the `when_subject:` predicates must partition the stored
rows alone. Below `ess/25` the pairing is
`conflicting_declaration` with a hint naming `ess/25`:

```yaml
      - name: registered
        unknown_instance: true
        creates: demo.devices.Device
        instance: device_id
        sets: {label: input.label, team: input.team}
        emits: [demo.devices.DeviceRegistered]
        payload:
          demo.devices.DeviceRegistered: {device_id: input.device_id}
      - name: retired
        when_subject:
          predicate: state == Retired
        error: demo.devices.DeviceRetired
      - name: relabelled
        updates: demo.devices.Device
        instance: device_id
        sets: {label: input.label}
```

Guarding every update branch (`state == Active` on `relabelled`) validates alone, but beside the
example's `trimmed:` refusal it is `non_exhaustive_branches` ("subject-fact/input coverage is not
proved"): leave one update branch unguarded as the default. Entity Runtime refuses the command (`ExistenceSelectionUnsupported`).

**Text the implementation trims first.** `trimmed:` wraps `equals`, `in`, `equals_ignore_case`,
`in_ignore_case`, `starts_with`, `ends_with` or `contains`, so `" None "` is refused as `none` is:

```yaml
      - name: reserved
        when:
          label: {trimmed: {in_ignore_case: [none, unknown]}}
        error: demo.devices.ReservedLabel
```

Only ASCII whitespace (space, tab, LF, VT, FF, CR) is removed; a Unicode space compares as itself,
and synthesis sends one to catch a target that trims it. A literal that holds for no trimmed text
(`" unknown"`) is `empty_declaration`; outside `when:`, `when_subject:` and `when_related:` the key
is `unsupported_construct`. Entity Runtime refuses it (`TrimmedTextUnsupported`).

**One event per changed row.** On an outcome with `instances:`, an `emits:` entry
`{event, each: changed}` publishes once per changed row, in ascending identity order; `when_state:`
keeps only rows that held one of those states before the outcome, and `{changed: <field>}` or
`{changed: state}` reads the row as it was:

```yaml
      - name: moved
        updates: demo.devices.Device
        instances: {where: team == input.team}
        sets: {team: input.new_team}
        emits:
          - demo.devices.TeamMoved
          - {event: demo.devices.DeviceMoved, each: changed, when_state: [Active]}
        payload:
          demo.devices.TeamMoved: {team: input.team, moved: {count: changed}}
          demo.devices.DeviceMoved: {device_id: {changed: device_id}, from_team: {changed: team}}
```

On an outcome without `instances:` the entry is refused at its place in `emits:`. The Rust, Go,
Web and Clap targets and Entity Runtime still refuse set effects, and the generated explorers leave
such an outcome out.

**A refusal decided before the row is read.** `before_row: true` on an `external:` refusal answers
among the input-guarded refusals, before existence and the held state, so the held-state branches
may follow it:

```yaml
      - name: bad-token
        external: the device token is checked before the device is read
        before_row: true
        error: demo.devices.BadToken
      - name: taken
        when_subject_state: Active
        preserves: demo.devices.Device
        instance: device_id
```

It is `conflicting_declaration` on a branch that is not `external:`, without `error:`, beside
`compensates: true`, where the error payload reads a row, after a branch a later step answers, and
on a command with a `when_related:` branch. Entity Runtime refuses it (`OutcomeShapeUnsupported`).

**A default row for an unknown identity.** A view whose filter is exactly
`<identity> == param.<name>` declares `absent: {row: …}`: an unknown identity reads one row holding
it, each literal, and `null` in each `Optional` field `row:` leaves out. Synthesis reads the view for
an identity nothing stores and checks that row:

```yaml
  - name: demo.devices.DeviceById
    source: demo.devices.Device
    consistency: read_your_writes
    params: [{name: device_id, type: demo.devices.DeviceId}]
    filter: device_id == param.device_id
    absent: {row: {label: unregistered, team: none}}
```

`conflicting_declaration` on any other filter, an `Optional` identity parameter, beside an aggregate
or `paging:`, and for the identity field in `row:`; `missing_declaration` for a required projected
field without a value; `undeclared_reference` for a field the view does not project. Keep the
unfiltered view beside it: without `Devices`, the example keeps 7 of its 16 scenarios and refuses 9
(`ESS-SYNTH-001`, `-003`). Generated view queries do not serve it yet; `ess verify diff` reports a change as `view/<name>/absent-changed`
(`ess-diff/19`).

**Literals through a newtype** (every format, since 0.60.0). A literal in `sets:`, `payload:`, a
binding's `mapping:`, an enum variant's `attributes:` or a view's `absent:` row is checked against
its newtype's `alphabet:` and `invariants:`; one outside them is `type_mismatch`
(`ESS-COMMAND-002` for `sets:`: "`xyz` holds 'x', which is not in the alphabet of …"), where 0.59.0
checked only the representation. A specification that validated on 0.59.0 can be refused here.

An HTTP binding of commands and views (`ess-http/1`) and composition roles
(`ess-composition/4`) are documents beside the specification, not source formats:
[current-features.md](current-features.md#http-routes-ess-http1).

## `ess/27` and `ess/28`

A probe on 0.66.0 using the excerpts below validated, synthesized 11 scenarios with 0 refusals in
`ess-conformance/54`, and the `interpreted` target passed all 11. Under a lower header each is
`unsupported_format_version` naming the format it needs, or keeps the reading it had there.

```yaml
      - name: unavailable                     # ess/28: the related row's state as a value
        when_related: {via: input.device_id, predicate: state == Retired}
        error: demo.leases.DeviceUnavailable
        payload:
          demo.leases.DeviceUnavailable: {state: {related: {via: input.device_id, field: state}}}
      - name: extended                        # ess/27: a value computed from row and input
        updates: demo.leases.Lease
        instance: lease_id
        sets: {hours: {expr: "subject.hours + input.extra"}}
```

```yaml
      - name: device_label                    # ess/28: a view field through a reference
        type: Optional<String>
        field: {related: {via: device_id, field: label}}
```

`+ - * /` over `Integer` and `Decimal` are exact; `/` is an `Optional<Decimal>` (six places,
half-even, absent at a zero divisor), so a required field needs `coalesce(a, b)` or `round(…)`.
`Timestamp - Timestamp` is a `Duration`; `60s`, `5m`, `1h` are literals; `now` is not a value.
Synthesis checks the figure on inputs worth checking (a zero divisor, a rounding tie).

**One precedence rule** (`ess/27`): a command answers in the order of what each branch must read:
the request alone (input guards, `external:` with `before_row: true`), the addressed record's
existence, its held state, the rows around it (every `exists: false`, then every related row's
refusal), then the accepting and external branches; within a step, the first declared whose
condition holds. Pairs that were `conflicting_declaration` for want of an order (`when_related:`
beside `when_subject:`, `wrong_state:` or `unknown_instance:`) validate. Raising a header to
`ess/27` can therefore change which branch answers a request, and the suite.

Validating is not lowering. A command or view using an expression stays an obligation, named by
the expression, in the generated Rust and Go behaviours and queries and in Entity Runtime. A
command with a function guard stays an obligation in generated Rust and Go, and Entity Runtime
refuses the guard as `FunctionCallUnsupported`. A command copying a narrowed value stays an
obligation in generated Rust and Go. A command with an `all:` or a list guard stays an obligation
in generated Rust and Go, and Entity Runtime lowering refuses it, each by name. A view reading
through a reference (`ess/28`) keeps its query an obligation in generated Rust and Go. Synthesis of
a Rust or Go server or the web app refuses a `shorthand:` struct.

## `ess/29`

A probe on 0.67.0 using the excerpt below validated, synthesized 6 scenarios with 0 refusals in
`ess-conformance/34`, and the `interpreted` target passed all 6. Under `ess/28` each `coalesce`
guard is `unsupported_format_version` naming `ess/29`, and `external:` beside `when_subject:` is
`conflicting_declaration` naming `ess/29`.

```yaml
      - {name: no-job, unknown_instance: true, error: demo.jobs.NoSuchJob}
      - name: hashed-mismatch                 # asked only on a row whose method is hashed
        external: the outside service finds the hashed verifier differs
        when_subject: {predicate: method == hashed}
        error: demo.jobs.VerifierMismatch
      - name: exhausted                       # an absent attempts field counts as 0
        when_subject: {predicate: "coalesce(attempts, 0) >= 3"}
        error: demo.jobs.TooManyAttempts
```

`attempts == 3` over an absent `Optional` is unknown, so no record without the field is decided;
`coalesce(attempts, 0)` reads `0` there. The default has the fact's type and the call reads one
fact; anything else on the left is `type_mismatch`, and a call outside a command outcome's guards
is `unsupported_construct`. It compiles to the structured
`{compare: {left: {expr: "coalesce(attempts, 0)"}, op: …, right: …}}` that `ess/27` admits.
Generated Rust and Go decide it over a `Boolean`, `Integer`, `Decimal` or `String` fact; Entity
Runtime refuses it as `FunctionCallUnsupported`.

An `external:` branch with `when_subject:` is answered at the held-state step, in declaration
order among the branches the stored row selects, before `wrong_state:`. Generated Rust and Go ask
the provider only after the guard holds, and Entity Runtime lowers it to the guard and the host's
external evidence. `before_row: true` or the `{field, equals}` shape beside it is
`conflicting_declaration`.

**Declaration order must agree with the precedence rule** (`ess/29`). A guarded branch declared
below one a later step answers is refused wherever one request satisfies both. On 0.67.0 an input
guard declared below `unknown_instance:` validated under `ess/28` and was refused under `ess/29`:

```text
[conflicting_declaration] command.demo.jobs.RetryJob.outcomes.over-budget: `over-budget` answers at step 1 of the precedence order and `no-job`, declared above it, at step 2; where both hold, the declaration order and the precedence order disagree (hint: declare `over-budget` before `no-job`: step 1 answers first in either order)
```

Declare each refused branch above the one its refusal names. Input guards the finite prover shows
disjoint may come in either order, and a marker (`unknown_instance:`, `wrong_state:`,
`exists: false`) declared below a guarded branch is not refused. The probe of the section above,
raised to `ess/29`, validated on 0.67.0 and its 11 scenarios passed on the `interpreted` target.

## What validates and still gets no scenario

`ess verify conform synthesize` observes a record only through a view, and these limits come from
that. Each was reproduced on 0.55.0, the response row on 0.57.0, the opened-response rows on
0.58.0 and the stored-field row on 0.59.0; relay the refusal rather than reshaping the rule around
it.

| the model says | synthesis answers |
|---|---|
| a guard over a stored field (`when_subject: {predicate: holder != input.holder}`) or a `wrong_state` refusal, on an entity with no view | since 0.59.0, the success, refusal, transition and wrong-state scenarios are written: the row is arranged through the creating command and its `sets:`, and each scenario observes only what the declared views publish. Its outcome, error and events are the evidence for the rest, and a note per scenario names the fields no view publishes ("a refusal that changed it is not checked"). The absent-subject send asserts that no row appeared only through an immediate view. 0.58.0 and earlier refused each of these with `ESS-SYNTH-001` |
| `--synthesis-seed` for such a row | not applied when the row is held by an `owns` relation: `synthesis seeds: 1 selected, 0 applied`, naming the owner |
| a record invariant or a reading on the type of a typed `response:` field | the returning outcome has no scenario: `ESS-SYNTH-001` "record invariant on a response type … has no executable observer" |
| `invariants:` on a type (`value.count >= 1`) with no view publishing a field of it | `ESS-SYNTH-013` "type … has no scenario"; `alphabet:` alone is checked without one |
| `when_subject:` comparing `now` with a stored `Timestamp` the outcome set with `{generated: true}` | every scenario of the command is refused: no arranging branch sets the field from an input or a literal |
| `one_time_response:` on an outcome of a command whose response declares `undeclared_fields: ignored` | the outcome and every disclosure scenario: `ESS-SYNTH-001` "one-time response … declares `undeclared_fields: ignored`, which the one-time observer does not carry" |
| a `replays:` outcome (a retained result) over a response declared `undeclared_fields: ignored` | the replay has no scenario: `ESS-SYNTH-001` "retained result … declares `undeclared_fields: ignored`, which the exact retained-result observer does not carry" |
| a complete-subject snapshot over an opened response or struct | refused at synthesis by name, because that observer compares a closed record (from the release notes; not reproduced here) |

A `String` newtype with `alphabet:`, `prefix:` or value `invariants:` as a field of a typed
`response:` no longer costs the returning outcome its scenario: the suite (`ess-conformance/46`,
`/47` with coverage) carries the type's rules as `constraints` on the response step, and the Rust,
Go and TypeScript runners fail a returned value that breaks one (`ESS-CF-PAYLOAD`). The type's own
`ESS-SYNTH-013` row above still applies to a value invariant no view publishes.

A system whose source defines no read surface (a published protocol, a credential that is never
read back) declares no view, and keeps these refusals and notes visible in its report. Declaring a
view to silence them states a read surface the system does not have. A stored-field guard on such a
system now gets its scenarios, but they cannot check that a refusal left the guarded field alone:
the notes say which fields that covers, and an authored scenario against the real target is the
check that remains.
