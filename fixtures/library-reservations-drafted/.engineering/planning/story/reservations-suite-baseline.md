---
format: aep.planning-md/3
id: story:reservations-suite-baseline
kind: story
status: draft
title: Existing lending behaviour passes the regenerated reservations suite
summary: Regenerate impl/essconform from the reservations spec and extend the Book model and the three book views so the 17 pre-existing scenarios pass against it.
relations:
- decomposes: epic:book-reservations
- informed_by: executable-system-specification:library
scope:
- confidence: cited
  path: impl/conformance_test.go
- confidence: cited
  path: impl/library.go
revision: 2
---
# Existing lending behaviour passes the regenerated reservations suite

## Context

The specification now models reservations, and a suite synthesized from it (55 scenarios) no longer
matches the Go library: every Catalogue row it checks carries `reserved_for_id`, most scenarios also
query `BooksOnHold`, and `BooksOnLoan` is filtered on two states. Until the target answers those
views, even the 17 scenarios that exercise only the old commands cannot pass. This story is the
prefactoring every other reservations story builds on: it regenerates the suite, gives `Book` the
new states and the `reserved_for_id` field, and answers the three book views as specified. It adds
no new command; the four new commands keep answering `ErrUnsupported`, so their scenarios are
reported skipped, which `go test` does not fail.

Specification it is held to:

- `spec/domains/lending.yaml:25-56` — `Book` gains `reserved_for_id: Optional<MemberId>`.
- `spec/domains/lending.yaml:57-60` — Book states are `OnShelf, OnLoan, OnLoanReserved, OnHold, Withdrawn`.
- `spec/domains/lending.yaml:505-525` — `Catalogue` has a `reserved_for_id` field.
- `spec/domains/lending.yaml:539-553` — `BooksOnLoan` filters `state == OnLoan` or `state == OnLoanReserved`.
- `spec/domains/lending.yaml:555-569` — `BooksOnHold` (new): `book_id`, `title`, `reserved_for_id`, filtered `state == OnHold`.

The suite is regenerated with `ess verify conform synthesize --path spec --target go --out impl/essconform`
(run from the repository root; see `impl/essconform/README.md`), which must report
`55 scenario(s) (0 authored), 0 refusal(s)`. `impl/essconform/` is git-ignored: regenerating it is a
local precondition for this and every later reservations story, not a committed change.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, zero or one member per book, `references` (no
  ownership; a Member is never removed, `spec/domains/lending.yaml:87`) — inferable from
  `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `reserved_for` (lines 52-56).
  This story only stores and exposes the field; nothing sets it yet.
- `Book → Member` via `borrower`: many-to-one, optional, `references` — inferable from
  `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `borrower` (lines 47-51). Unchanged.

## Scope

- `impl/library.go` — add the `OnLoanReserved` and `OnHold` `BookState` constants and a
  `ReservedFor *string` (or equivalent) on `Book`. No behaviour of `Borrow`, `Return` or `Withdraw` changes.
- `impl/conformance_test.go` — `QueryView`: `Catalogue` rows add `reserved_for_id`; `BooksOnLoan`
  includes `OnLoanReserved` books; new `BooksOnHold` case.
- `impl/essconform/` — regenerated locally (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`; not parallel-safe with any of them.

## Acceptance

After regenerating `impl/essconform` from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for all 17 scenarios that use only `AddBook`, `RegisterMember`, `BorrowBook`, `ReturnBook` and `WithdrawBook` — `AddBook/outcome/added`, `RegisterMember/outcome/registered`, `BorrowBook/outcome/{borrowed,no-such-book}`, `ReturnBook/outcome/{returned,no-such-book}`, `WithdrawBook/outcome/{withdrawn,no-such-book}`, `Book/transition/{lend,return,withdraw}/…`, `Book/state/OnLoan/refuses/{BorrowBook,WithdrawBook}`, `Book/state/OnShelf/refuses/ReturnBook` and `Book/state/Withdrawn/refuses/{BorrowBook,ReturnBook,WithdrawBook}` — and fails none.
