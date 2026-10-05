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

**A limit per holder** ("a member can have five packets out at once"). Keep the count on the entity

the command addresses, and guard on it:

```yaml
entities:
  - name: seeds.lending.Member
    identity: {name: member_id, type: Uuid}
    fields:
      - {name: name, type: String}
      - {name: packets_out, type: Integer}
    invariants:
      - packets_out >= 0
      - packets_out <= 5
    lifecycle:
      initial: Active
      states: [Active]
      terminal: [Active]
```

```yaml
  - name: seeds.lending.BorrowPacket
    input:
      - {name: member_id, type: Uuid}
    outcomes:
      - name: at-limit
        when_subject:
          predicate: packets_out >= 5
        error: seeds.lending.LimitReached
      - name: borrowed
        updates: seeds.lending.Member
        instance: member_id
        sets: {packets_out: {increment: 1}}
        emits: [seeds.lending.PacketBorrowed]
        payload:
          seeds.lending.PacketBorrowed: {member_id: input.member_id}
  - name: seeds.lending.ReturnPacket
    input:
      - {name: member_id, type: Uuid}
    outcomes:
      - name: nothing-out
        when_subject:
          predicate: packets_out <= 0
        error: seeds.lending.NothingOut
      - name: returned
        updates: seeds.lending.Member
        instance: member_id
        sets: {packets_out: {increment: -1}}
        emits: [seeds.lending.PacketReturned]
        payload:
          seeds.lending.PacketReturned: {member_id: input.member_id}
```

The creating command (`Join`) sets `packets_out: 0`. Synthesis arranges a member only through that
`sets:` and does not repeat `BorrowPacket`, so the `at-limit` scenario is refused (`ESS-SYNTH-003`).
That refusal is the expected result: name it in your report. Do not give `Join` a starting count
only so synthesis can reach the limit; no member joins holding packets.
From `ess/16`, `affects:` changes selected records beside the addressed member; `ess/22` also
allows their lifecycle moves. This expresses multi-record effects, but not transaction atomicity.
See [current-features.md](current-features.md) for validated set-effect and related-guard examples.

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

A specification lowered to Entity Runtime is refused there, not at `validate`, for value
expressions (`ValueExpressionUnsupported`), the `ess/15` outcome shapes (`OutcomeShapeUnsupported`)
and the case-insensitive operators (`CaseFoldUnsupported`); keep those out of a model that must lower.

## From `ess/16` through `ess/22`

The source language and conformance-suite version are different contracts. Choose the source
format for the construct; let synthesis select its required suite format. With ESS 0.53.0:

| format | additions |
|---|---|
| `ess/16` | related values; literal fallbacks; optional aggregate presence; `input_absent`; `existing_instance`; caller attributes; view paging; bounded retries; `instances` and `affects` |
| `ess/17` | typed direct responses with `returns: true` |
| `ess/18` | `when_related`; stored `state` in subject predicates; lists of subject states; binding delivery context |
| `ess/19` | `payload` sources for declared error fields |
| `ess/20` | related row lifecycle `state` in predicates |
| `ess/21` | `one_time_response` non-disclosure contracts for required String response fields |
| `ess/22` | explicit fact operands and constant offsets; UTF-8 byte lengths; instant comparison; `distinct` list keys; selected row guards/reads; Optional and two-hop related reads; several related rows; calendar windows; compensating external refusals; conditional aggregates and binding payload guards; per-outcome failure policy; lifecycle moves in `affects`; view grants; unit union variants; dotted input values |

For related guards, selected effects, event transports, client generation, finite protocol models
and compatibility gates, read and run [current-features.md](current-features.md). Check the actual
selected target: a source construct validating does not mean code generation or Entity Runtime
can lower it. The current lowering report lists each unsupported construct by name.

For a valid stored state bounded arrangement cannot reach, ESS 0.53.0 admits an explicit
`--synthesis-seed <authored-file> <instance>` containing a typed setup row. The suite records seed
provenance and selects `/42` or `/43`; the target must establish and validate that real row.
A seed does not execute the authored document's timeline or replace a command's assertions. For a
missing input candidate, first supply a truthful `example:`: the reservation fixture's distinct
member identity restores the wrong-state scenario without changing the rule or injecting state.
