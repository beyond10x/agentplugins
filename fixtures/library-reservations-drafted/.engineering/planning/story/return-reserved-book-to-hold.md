---
format: aep.planning-md/3
id: story:return-reserved-book-to-hold
kind: story
status: draft
title: A returned reserved book is held for the member who reserved it
summary: 'Library.Return branches on state: OnLoan goes to OnShelf as before, OnLoanReserved goes to OnHold and the ReturnBook command answers held with BookHeld naming the reserving member.'
relations:
- decomposes: epic:book-reservations
- informed_by: executable-system-specification:library
- depends_on: story:reserve-book-on-loan
scope:
- confidence: cited
  path: impl/conformance_test.go
- confidence: cited
  path: impl/library.go
revision: 2
---
# A returned reserved book is held for the member who reserved it

## Context

Epic decisions 2 and 9: a returned reserved book does not go back on the shelf; it enters `OnHold`,
and the return has its own `held` outcome emitting `BookHeld`. Decision 5: a held book cannot be
withdrawn (`WithdrawBook` still acts only from `OnShelf`).

Specification it is held to:

- `spec/domains/lending.yaml:74-76` — transition `return_to_hold`, `OnLoanReserved -> OnHold`.
- `spec/domains/lending.yaml:228-272` — `ReturnBook`: `returned` when `OnLoan` (unchanged), `held`
  when `OnLoanReserved` (clears `borrower_id`, emits `BookHeld` with `member_id` taken from the
  book's `reserved_for_id`), `wrong-state` otherwise, `no-such-book`.
- `spec/domains/lending.yaml:19-23` — the `Optional<MemberId> -> MemberId` conversion that makes
  `BookHeld.member_id` always present.
- `spec/domains/lending.yaml:493-498` — event `BookHeld { book_id, member_id }`.
- `spec/domains/lending.yaml:555-569` — `BooksOnHold` now has rows to show.
- `spec/components.yaml:27` — the component publishes `BookHeld`.

`reserved_for_id` stays set in `OnHold`; only `borrower_id` is cleared.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, `references`; survives the return into `OnHold`
  and names the member in `BookHeld` — inferable from `spec/domains/lending.yaml`, entity
  `library.lending.Book`, relation `reserved_for` (lines 52-56), with the conversion at lines 19-23.
- `Book → Member` via `borrower`: many-to-one, optional, `references`; cleared on return — inferable
  from `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `borrower` (lines 47-51).

## Scope

- `impl/library.go` — `Return` accepts `OnLoanReserved` as well as `OnLoan` and reports which way
  the book went (and, for a hold, for whom), without changing the `OnLoan -> OnShelf` path.
- `impl/conformance_test.go` — the `library.lending.ReturnBook` case answers `held` with a
  `library.lending.BookHeld` event when the book went on hold.
- `impl/essconform/` — regenerated locally if absent (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`; not parallel-safe with any of them.

## Acceptance

With `impl/essconform` regenerated from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for the 7 scenarios `ReturnBook/outcome/held`, `ReturnBook/outcome/wrong-state`, `Book/transition/return_to_hold/by/library.lending.ReturnBook/held` and `Book/state/OnHold/refuses/{BorrowBook,ReserveBook,ReturnBook,WithdrawBook}`, and fails no scenario.
