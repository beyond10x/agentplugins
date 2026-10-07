# ESS syntax by example

A small lending library in three files, with every section a specification usually needs. It
validates as written (`ess specify validate --path <directory>` → `library v1 — 3 file(s), valid`).
Copy the shape, not the domain: name your own entities, commands and events after your system.
Keep each domain’s first segment equal to `system` and update every qualified reference together.
It is written in `format: ess/1`, the lowest header these constructs need; [later-formats.md](later-formats.md) adds what formats through `ess/23` say. A new document still starts on the newest format the installed `ess` implements ([SKILL.md](../SKILL.md#starting-a-domain-from-nothing)).

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
# Decimal, Boolean, Bytes, Timestamp, Duration, Binary64 (ess/2) and Json (ess/15); wrappers
# Optional<T>, List<T>, Map<K, V>.
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

# Commands: input, then outcomes. An outcome `creates` an entity, `moves` one through a
# transition or `updates` its fields, `emits` events with a `payload` built from `input.<field>`,
# or returns an `error`. `instance:` on `creates:` names a field of an emitted event that carries
# the new identity (`branch_id` of BranchOpened); on `moves:`/`updates:` it names the input field
# holding the identity (`copy_id` of LendCopy). Either other way is `undeclared_reference`.
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

  # Two outcomes of one command: exactly one has no `when` (the default) and every other needs a `when`
  # over the input. Two without a `when` is `conflicting_declaration`; every branch with a `when` is `non_exhaustive_branches`. Error
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
        # An invariant reads `pages`, so the creating outcome must set it (ESS-COMMAND-018).
        sets:
          pages: input.pages
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
  # not start from, so the `from:` list lives in one place. A `copy_id` no record carries gets, in
  # order: the command's `unknown_instance:` outcome (ess/15, later-formats.md); else its not-found
  # outcome, an `external: <reason text>` refusal whose `error:` carries a field typed `CopyId`; else this
  # `wrong_state` outcome, which is what LendCopy answers here.
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

# Views: read models over an entity. `read_your_writes` or `eventual`; an optional `filter`. A view
# the caller narrows takes `params: [{name: title, type: library.lending.Title}]` with
# `filter: title == param.title` (the key is `params:`); the creating outcome must `sets:` the field.
# Never narrow on the identity (`filter: id == param.id`): synthesis refuses every outcome that view
# observes (ESS-SYNTH-005, "bound by nothing a scenario knows"). A read by id is the unfiltered view.
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

A predicate (`when`, `invariants`, a view's `filter`) has a compact form and a structured form, and
`validate` accepts both in all three places. The full grammar is ESS's
[predicate reference](https://beyond10x.github.io/docs/ess/reference/predicates).

The compact form is one string: a comparison (`pages > 0`, `format == Hardcover`), a bare fact path
(present and truthy), `defined(note)`, or `not` before any of those. `&&`, `||` and `in [...]`
inside that string are refused; write a conjunction, a disjunction or a set in the structured form.
A string that does not parse is refused at its line as `unparsable_predicate` (`ESS-SPEC-012`), and
an unquoted `null` as `null_comparison` (`ESS-SPEC-017`): write `defined(note)` or
`not defined(note)`, or quote `"null"` to compare with that text.

| to say | structured form |
|---|---|
| A or B ("state A or B" is one view, not two) | `filter: {any: [state == Available, state == OnLoan]}` |
| A and B | `when: {all: [pages > 0, format == Hardcover]}`; a bare list is an implicit `all` |
| not A, none of A and B | `when: {not: pages <= 0}`, `when: {none: [format == Audio, pages > 900]}` |
| a range, by operator | `when: {pages: {gte: 1, lte: 5000}}` — `eq`, `ne`, `lt`, `lte`, `gt`, `gte` |
| one of a set | `when: {format: [Hardcover, Paperback]}`, or `{in: […]}`, `{any_of: […]}`, `{one_of: […]}`. The set holds literal values: an operand naming a parameter or an input (`{in: param.titles}`) is refused as `type_mismatch`; write `exists: {in: param.titles, as: t, that: title == t}` for a list, `title == param.title` for one value |
| none of a set | `when: {format: {none_of: [Audio]}}`, or `{not_in: […]}` |
| present or absent | `when: {note: {exists: false}}` (= `not defined(note)`), `{defined: true}`, `{truthy: true}` |
| every or some element of a list | `forall: {in: tags, as: t, that: t != ""}`, `exists: {in: tags, as: t, that: …}` — no compact form |
| a list's length | `tags.count >= 0` |
| text starts with, ends with, contains (`ess/8`, case-sensitive) | `when: {title: {starts_with: "Draft: "}}`; `ends_with`, `contains` alike |
| text equal to one of a set, ignoring ASCII case (`ess/15`) | `when: {title: {in_ignore_case: ["untitled", "tbd"]}}`; one literal is `equals_ignore_case` |

`validate` accepting a form does not mean `ess verify conform synthesize` can witness it. Two
refusals to expect: an invariant over a list field (`tags.count`, a `forall` over `tags`) validates
and is then refused as reading what no view publishes, even with the field in a view; and a presence
guard over a required input (`{defined: true}`, `{exists: false}`) is refused because no candidate
input leaves the field out. Relay such a refusal verbatim rather than reshaping the rule around it.

A `when` reads the command input; `when_subject:` reads the addressed row and, from `ess/18`,
its stored lifecycle `state`. `when_related:` reads another entity, by typed identity from
`ess/18` or by a row selector from `ess/22`. The current
[related-record examples](current-features.md) validate and synthesize these forms. Validate each
combination before declaring it supported: guard ordering and target lowering still have limits.

Four cases trials hit:

| rule | how to write it |
|---|---|
| a value stored on the addressed entity decides the outcome ("express parcels over 20 kg are refused at dispatch", with the weight given at create) | `when_subject: {predicate: {all: [service == Express, weight_kg > 20]}}` on the refusing outcome (`ess/9`). It reads the entity’s stored fields and, from `ess/18`, `state`; a branch selected solely by held state can use `when_subject_state:` ([later-formats.md](later-formats.md) shows it and its two limits); an open comparison needs a default branch, and a view must publish every guarded field |
| a stored value compared with the request ("a return scanned with another title is refused") | `when_subject: {predicate: title != input.title}` (`ess/15`); the input side is always `input.<field>` on the right. ReturnCopy in [later-formats.md](later-formats.md) |
| two records must not overlap ("a room cannot be booked twice for one hour") | make the contested unit an entity with its own lifecycle (a `Slot` that is `Free` or `Booked`); a second booking is then `wrong_state` on that slot. For arbitrary ranges, an `ess/22` related-row selector can express overlap using `starts_at < input.ends_at` and `ends_at > input.starts_at`; validate and inspect synthesis refusals for the exact predicates and arrangement |
| a holder may hold at most N ("a member can have five packets out at once") | Address the packet and guard its borrow with `when_related: {entity: seeds.lending.Packet, where: {all: [borrower_id == input.member_id, state == Lent]}, count: {gte: 5}}` (`ess/22`). This counts actual loans and changes only the addressed packet. Validation can accept the rule while synthesis refuses its boundary arrangement; report the exact refusal. See [later-formats.md](later-formats.md) for the limits. |

**Compare two typed facts.** From `ess/22`, a bare word on the right that names a field is
a fact reference; `ends_at > starts_at` compares the two `Timestamp` facts as instants. An explicit
`{fact: starts_at}` operand removes ambiguity. Earlier formats need a dotted typed path such as
`window.ends_at > window.starts_at`. Constant offsets (`upper <= lower + 5`), UTF-8 byte counts
(`label.utf8_bytes`) and `distinct` list keys also require `ess/22`. A `Duration` is not an
Integer; name the unit for a numeric length (`duration_minutes > 0`).
