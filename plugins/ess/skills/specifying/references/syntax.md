# ESS syntax by example

A small lending library in three files, with every section a specification usually needs. It
validates as written (`ess specify validate --path <directory>` → `library v1 — 3 file(s), valid`).
Copy the shape, not the domain: name your own entities, commands and events after your system.

## `system.yaml`

```yaml
format: ess/1
system: library
version: v1

domains:
  - library.lending
```

## `components.yaml`

Who runs the domain, which commands it accepts, which events it publishes, how it is reached.

```yaml
components:
  - component: lending-service
    summary: Holds every branch and the copies it lends.
    owns:
      domains:
        - library.lending
    accepts:
      commands:
        - library.lending.OpenBranch
        - library.lending.AddCopy
        - library.lending.LendCopy
        - library.lending.ReturnCopy
    publishes:
      events:
        - library.lending.BranchOpened
        - library.lending.CopyAdded
        - library.lending.CopyLent
        - library.lending.CopyReturned
    reached_by: network
```

## `domains/lending.yaml`

```yaml
domain: library.lending

summary: Branches that own copies of books and lend them out.

naming:
  wire: lending
  display: Lending

# Types: `newtype` over a primitive, `enum`, `struct` (with `fields`, optional `invariants`),
# `union` (tagged: `tag:` plus `variants:` name → type). Primitives include Uuid, String, Integer,
# Decimal, Boolean, Bytes, Timestamp, Duration; wrappers Optional<T>, List<T>, Map<K, V>.
types:
  - name: library.lending.BranchId
    kind: newtype
    of: Uuid

  - name: library.lending.CopyId
    kind: newtype
    of: Uuid

  - name: library.lending.Title
    kind: newtype
    of: String

  - name: library.lending.Format
    kind: enum
    variants: [Hardcover, Paperback, Audio]

# Entities: an identity, fields, relations to other entities, and a lifecycle whose transitions
# commands move. `owns` means the target cannot outlive this entity; `references` means it can.
entities:
  - name: library.lending.Branch
    identity:
      name: branch_id
      type: library.lending.BranchId
    fields:
      - name: name
        type: String
    relations:
      - name: copies
        kind: owns
        target: library.lending.Copy
        cardinality: many
        via: branch_id
    lifecycle:
      initial: Open
      states: [Open]
      terminal: [Open]

  # A lifecycle may have no final state: a copy goes out and comes back for as long as it exists,
  # so `terminal` is empty.
  - name: library.lending.Copy
    identity:
      name: copy_id
      type: library.lending.CopyId
    fields:
      - name: branch_id
        type: library.lending.BranchId
      - name: title
        type: library.lending.Title
      - name: format
        type: library.lending.Format
      - name: pages
        type: Integer
    invariants:
      - pages > 0
    lifecycle:
      initial: Available
      states: [Available, OnLoan]
      terminal: []
      transitions:
        - name: lend
          from: [Available]
          to: OnLoan
        - name: return
          from: [OnLoan]
          to: Available

# Actors: who may invoke which commands.
actors:
  - name: library.lending.Librarian
    may:
      - library.lending.OpenBranch
      - library.lending.AddCopy
      - library.lending.LendCopy
      - library.lending.ReturnCopy
    naming:
      display: Librarian

# Errors a command outcome can return.
errors:
  - name: library.lending.InvalidPageCount
    summary: The page count is not a positive number.
    fields:
      - name: submitted
        type: Integer

  - name: library.lending.CopyStateConflict
    summary: The copy is not in a state this command acts from, so nothing moved.
    fields:
      - name: state
        type: library.lending.Copy.State

# Commands: input, then outcomes. An outcome `creates` an entity or `moves` one through a
# transition, `emits` events with a `payload` built from `input.<field>`, or returns an `error`.
commands:
  - name: library.lending.OpenBranch
    naming:
      wire: open-branch
      display: Open a branch
    input:
      - name: name
        type: String
    outcomes:
      - name: opened
        creates: library.lending.Branch
        instance: branch_id
        emits:
          - library.lending.BranchOpened
        payload:
          library.lending.BranchOpened:
            name: input.name
        summary: The branch exists and holds no copies.

  # Two outcomes of one command: all but one need a `when` over the command's input, or the result
  # is not determined by the input and `validate` refuses it as `conflicting_declaration`. Error
  # outcomes count; `wrong_state: true` outcomes do not (LendCopy below has two without a `when`).
  - name: library.lending.AddCopy
    naming:
      wire: add-copy
      display: Add a copy
    input:
      - name: branch_id
        type: library.lending.BranchId
      - name: title
        type: library.lending.Title
      - name: format
        type: library.lending.Format
      - name: pages
        type: Integer
    outcomes:
      - name: added
        when: pages > 0
        creates: library.lending.Copy
        instance: copy_id
        emits:
          - library.lending.CopyAdded
        payload:
          library.lending.CopyAdded:
            branch_id: input.branch_id
            title: input.title
        summary: The copy is on the shelf and Available.

      - name: refused
        error: library.lending.InvalidPageCount
        summary: The page count was not positive, and nothing was added.

  # A command that moves an entity: `wrong_state: true` answers from every state the transition does
  # not start from, so the `from:` list lives in one place.
  - name: library.lending.LendCopy
    naming:
      wire: lend-copy
      display: Lend a copy
    input:
      - name: copy_id
        type: library.lending.CopyId
    outcomes:
      - name: lent
        moves: library.lending.Copy.lend
        instance: copy_id
        emits:
          - library.lending.CopyLent
        payload:
          library.lending.CopyLent:
            copy_id: input.copy_id
        summary: The copy is out on loan.

      - name: wrong-state
        wrong_state: true
        error: library.lending.CopyStateConflict
        summary: The copy is not Available, so nothing was lent.

  - name: library.lending.ReturnCopy
    naming:
      wire: return-copy
      display: Return a copy
    input:
      - name: copy_id
        type: library.lending.CopyId
    outcomes:
      - name: returned
        moves: library.lending.Copy.return
        instance: copy_id
        emits:
          - library.lending.CopyReturned
        payload:
          library.lending.CopyReturned:
            copy_id: input.copy_id
        summary: The copy is back on the shelf.

      - name: wrong-state
        wrong_state: true
        error: library.lending.CopyStateConflict
        summary: The copy is not on loan, so nothing was returned.

events:
  - name: library.lending.BranchOpened
    fields:
      - name: branch_id
        type: library.lending.BranchId
      - name: name
        type: String

  - name: library.lending.CopyAdded
    fields:
      - name: copy_id
        type: library.lending.CopyId
      - name: branch_id
        type: library.lending.BranchId
      - name: title
        type: library.lending.Title

  - name: library.lending.CopyLent
    fields:
      - name: copy_id
        type: library.lending.CopyId

  - name: library.lending.CopyReturned
    fields:
      - name: copy_id
        type: library.lending.CopyId

# Views: read models over an entity. `read_your_writes` or `eventual`; an optional `filter`.
views:
  - name: library.lending.AvailableCopies
    source: library.lending.Copy
    consistency: read_your_writes
    filter: state == Available
    fields:
      - name: copy_id
        type: library.lending.CopyId
      - name: title
        type: library.lending.Title
    naming:
      wire: available
      display: Available copies

  # An invariant is checked after every outcome, through a view that holds the entity in the
  # resulting state and publishes the fields the invariant reads. With no `filter`, this one
  # holds every state; without it, `ess verify conform synthesize` refuses the `pages > 0` checks.
  - name: library.lending.Copies
    source: library.lending.Copy
    consistency: read_your_writes
    fields:
      - name: copy_id
        type: library.lending.CopyId
      - name: pages
        type: Integer
    naming:
      wire: copies
      display: Copies
```

State names start with an upper-case letter (`OnLoan`); `validate` refuses `on_loan`. Enum
variants may be written as the source spells them.

## What `when` can and cannot say

A predicate (`when`, `invariants`, a view's `filter`) is one comparison (`pages > 0`,
`format == Hardcover`) or a bare fact path. `&&`, `||` and `in [...]` are refused, and a list's
length cannot be tested. A view that should hold "state A or B" is two views, one per state.

A `when` reads only the command's own input. A condition on another entity — "the branch must be
open", "the customer is active" — is not an input guard. Express it as a transition of that entity (a
command that `moves` it, answered by `wrong_state` from states it does not start from), or leave an
`UNMAPPED:` marker naming the rule and report it; never invent an outcome the compiler cannot decide.

Two cases trials hit:

| rule | how to write it |
|---|---|
| a value stored on the entity decides the outcome ("express parcels over 20 kg are refused at dispatch", with the weight given at create) | not expressible: a `when` sees only the dispatch input. Mark it `UNMAPPED:` at the outcome, citing the source line. A guard over stored fields is proposed in ESS's [design note](https://github.com/beyond10x/ess/blob/main/docs/design/cross-record-and-stored-field-guards.md) |
| two records must not overlap ("a room cannot be booked twice for one hour") | make the contested unit an entity with its own lifecycle (a `Slot` that is `Free` or `Booked`); a second booking is then `wrong_state` on that slot. Overlap between arbitrary time ranges is not expressible; mark it `UNMAPPED:` |

**Compare two fields through one struct.** A right-hand side without a dot is a literal, so
`ends_at > starts_at` compares `ends_at` with the text `"starts_at"` and `validate` refuses it.
Declare the two fields in one struct (`Window` with `starts_at` and `ends_at`, both `Timestamp`),
take it as input, and guard on `window.ends_at > window.starts_at`; `synthesize` orders the two
instants. `Duration` does not compare with a number (it is text), so a length is an `Integer` in a
named unit (`duration_minutes > 0`).
