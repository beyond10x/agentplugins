---
format: aep.planning-md/3
id: story:reserve-book-on-loan
kind: story
status: draft
title: A librarian reserves a book on loan for a member
summary: 'Library.Reserve and the ReserveBook command: a book OnLoan moves to OnLoanReserved with reserved_for_id set; the borrower and every other state are refused.'
relations:
- decomposes: epic:book-reservations
- informed_by: executable-system-specification:library
- depends_on: story:reservations-suite-baseline
scope:
- confidence: cited
  path: impl/conformance_test.go
- confidence: cited
  path: impl/library.go
revision: 2
---
# A librarian reserves a book on loan for a member

## Context

Epic decisions 1, 6 and 7: one reservation per book, made by the Librarian on a member's behalf,
and only for a book that is on loan to somebody else. The library gains a reserve operation and the
conformance target a `library.lending.ReserveBook` case.

Specification it is held to:

- `spec/domains/lending.yaml:57-73` — transition `reserve`, `OnLoan -> OnLoanReserved`.
- `spec/domains/lending.yaml:302-342` — `ReserveBook`: `already-borrower` (`MemberAlreadyHasBook`
  when `borrower_id == input.member_id`), `reserved` (sets `reserved_for_id`, emits `BookReserved`),
  `wrong-state` (`BookStateConflict` from any state but `OnLoan`), `no-such-book` (`BookNotFound`).
- `spec/domains/lending.yaml:134-138` — error `MemberAlreadyHasBook`.
- `spec/domains/lending.yaml:481-486` — event `BookReserved { book_id, member_id }`.
- `spec/components.yaml:14`, `:25` — the component accepts `ReserveBook` and publishes `BookReserved`.

The synthesized scenarios show a state conflict answered before the borrower guard (a book on the
shelf or already reserved answers `wrong-state`), so the implementor should read the outcome order
at `spec/domains/lending.yaml:314-342` against those scenarios rather than assume it.

`Borrow` and `Withdraw` already refuse any state but `OnShelf`, so the `OnLoanReserved` refusals of
`BorrowBook` and `WithdrawBook` should need no change beyond being reachable now.

Whether the reserving member is a registered member is not checked here: the specification leaves it
to the implementation (`spec/domains/lending.yaml:302-304`) and the epic puts the registered-member
check out of scope. This story adds no such check.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, zero or one member per book, `references`, set by
  `reserve` and held until the reservation is cancelled, collected or released — inferable from
  `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `reserved_for` (lines 52-56),
  with the lifecycle at lines 26-32 and 57-85.
- `Book → Member` via `borrower`: many-to-one, optional, `references` — inferable from
  `spec/domains/lending.yaml`, entity `library.lending.Book`, relation `borrower` (lines 47-51);
  read here to refuse the current borrower.

## Scope

- `impl/library.go` — a reserve method and an exported error for "member already has this book".
- `impl/conformance_test.go` — `ExecuteCommand` case for `library.lending.ReserveBook`; `refusal`
  (or its replacement) maps the new error to outcome `already-borrower`, error
  `library.lending.MemberAlreadyHasBook`.
- `impl/essconform/` — regenerated locally if absent (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`; not parallel-safe with any of them.

## Acceptance

With `impl/essconform` regenerated from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for the 9 scenarios `ReserveBook/outcome/{reserved,already-borrower,no-such-book}`, `Book/transition/reserve/by/library.lending.ReserveBook/reserved`, `Book/state/OnShelf/refuses/ReserveBook`, `Book/state/Withdrawn/refuses/ReserveBook` and `Book/state/OnLoanReserved/refuses/{ReserveBook,BorrowBook,WithdrawBook}`, and fails no scenario.
