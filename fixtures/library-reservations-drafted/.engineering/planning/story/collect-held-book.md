---
format: aep.planning-md/3
id: story:collect-held-book
kind: story
status: draft
title: The member a book is held for collects it
summary: 'Library collect operation and the CollectHold command: OnHold goes to OnLoan for the reserving member with BookBorrowed; anyone else gets BookHeldForAnotherMember.'
relations:
- decomposes: epic:book-reservations
- informed_by: executable-system-specification:library
- depends_on: story:return-reserved-book-to-hold
scope:
- confidence: cited
  path: impl/conformance_test.go
- confidence: cited
  path: impl/library.go
revision: 2
---
# The member a book is held for collects it

## Context

Epic decision 3a, as modelled: only the reserving member can take a held book, and anyone else is
refused with `BookHeldForAnotherMember`. The epic's "Where the model departs" section explains why
this is a separate command, `CollectHold`, rather than `BorrowBook` — `BorrowBook` is unchanged and
keeps answering `BookStateConflict` for a book on hold (`spec/domains/lending.yaml:197-203`).

Specification it is held to:

- `spec/domains/lending.yaml:77-79` — transition `collect`, `OnHold -> OnLoan`.
- `spec/domains/lending.yaml:374-413` — `CollectHold`: `held-for-another`
  (`BookHeldForAnotherMember` when `reserved_for_id != input.member_id`), `collected` (sets
  `borrower_id`, clears `reserved_for_id`, emits `BookBorrowed`), `wrong-state` (`BookStateConflict`
  from any state but `OnHold`), `no-such-book`.
- `spec/domains/lending.yaml:128-132` — error `BookHeldForAnotherMember`.
- `spec/domains/lending.yaml:464-469` — event `BookBorrowed { book_id, member_id }` (reused).
- `spec/components.yaml:16` — the component accepts `CollectHold`.

The synthesized scenarios answer `wrong-state`, not `held-for-another`, for a book on the shelf,
on loan, reserved or withdrawn, so the state check comes before the member guard; read the outcome
order at `spec/domains/lending.yaml:384-413` against those scenarios.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, `references`; compared with the collecting member,
  then cleared by `collect` — inferable from `spec/domains/lending.yaml`, entity
  `library.lending.Book`, relation `reserved_for` (lines 52-56).
- `Book → Member` via `borrower`: many-to-one, optional, `references`; set to the collecting member —
  inferable from `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `borrower` (lines 47-51).

## Scope

- `impl/library.go` — a collect method and an exported error for "held for another member".
- `impl/conformance_test.go` — `ExecuteCommand` case for `library.lending.CollectHold`; the new
  error maps to outcome `held-for-another`, error `library.lending.BookHeldForAnotherMember`.
- `impl/essconform/` — regenerated locally if absent (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`, including its
sibling `story:release-hold` which depends on the same story; the two are not parallel-safe.

## Acceptance

With `impl/essconform` regenerated from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for the 8 scenarios `CollectHold/outcome/{collected,held-for-another,no-such-book}`, `Book/transition/collect/by/library.lending.CollectHold/collected` and `Book/state/{OnShelf,OnLoan,OnLoanReserved,Withdrawn}/refuses/CollectHold`, and fails no scenario.
