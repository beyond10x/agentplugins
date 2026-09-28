---
format: aep.planning-md/3
id: epic:book-reservations
kind: epic
status: draft
title: Members can reserve a book that is on loan
summary: A book on loan can be reserved for one member; when it comes back it is held for them instead of going on the shelf.
relations:
- serves: vision:lending-library
- informed_by: executable-system-specification:library
revision: 1
---
# Members can reserve a book that is on loan

## Outcome

A librarian can reserve a book that is on loan for a member. When that book is returned it does
not go back on the shelf: it is held for the member who reserved it until they collect it, or until
a librarian releases the hold. The Go library in `impl/` does this, and the conformance suite
synthesized from the specification passes against it.

## Model

The behaviour is specified in ESS before any code is written around it, and the specification is
the contract every story below is held to:

- `spec/domains/lending.yaml:19` — the `Optional<MemberId> -> MemberId` conversion `BookHeld` needs.
- `spec/domains/lending.yaml:44` — `Book.reserved_for_id`, with a `reserved_for` relation to `Member`
  (`references`, cardinality `one`).
- `spec/domains/lending.yaml:59` — Book states become `OnShelf, OnLoan, OnLoanReserved, OnHold,
  Withdrawn`, with transitions `reserve`, `cancel_reservation`, `return_to_hold`, `collect`,
  `release_hold` beside the existing `lend`, `return`, `withdraw`.
- `spec/domains/lending.yaml:229` — `ReturnBook` branches on state: `returned` (OnLoan -> OnShelf,
  `BookReturned`) or `held` (OnLoanReserved -> OnHold, `BookHeld` with the reserving member).
- `spec/domains/lending.yaml:305` — `ReserveBook` (new), refused with `MemberAlreadyHasBook` for the
  current borrower and with `BookStateConflict` from any state but `OnLoan`.
- `spec/domains/lending.yaml:344` — `CancelReservation` (new).
- `spec/domains/lending.yaml:375` — `CollectHold` (new), refused with `BookHeldForAnotherMember` for
  anyone but the reserving member.
- `spec/domains/lending.yaml:417` — `ReleaseHold` (new).
- `spec/domains/lending.yaml:507`, `:540`, `:556` — `Catalogue` gains `reserved_for_id`,
  `BooksOnLoan` includes reserved loans, `BooksOnHold` (new) lists held books.
- `spec/components.yaml` — the component accepts the four new commands and publishes the four new
  events.

`ess specify validate --path spec` → `library v1 — 3 file(s), valid`.
`ess verify conform synthesize --path spec` → `55 scenario(s) (0 authored), 0 refusal(s)` (was 17).

## Decisions (operator, 2026-09-28)

1. One reservation per book at a time; a second is refused (`BookStateConflict` from `OnLoanReserved`).
2. A returned reserved book enters a new state, `OnHold`.
3. a. Only the reserving member can take a held book; anyone else gets `BookHeldForAnotherMember`.
   b. A librarian releases a hold with `ReleaseHold`; there is no time-based expiry (the spec has no clock).
4. A reservation can be cancelled while the book is on loan (`CancelReservation`).
5. A held book cannot be withdrawn; `WithdrawBook` still acts only from `OnShelf`.
6. The Librarian actor reserves on a member's behalf; no Member actor is added.
7. Reserving a book on the shelf, or one the member already has on loan, is refused.
8. Whether the reserving member is registered is left to the implementation, as for `BorrowBook`.
9. `Catalogue` shows `reserved_for_id`; `BooksOnHold` is added; a reserved return has its own `held`
   outcome emitting `BookHeld`.

### Where the model departs from what was agreed

Decision 3a said the reserving member *borrows* the held book. ESS refuses one command that both
branches on the book's state and guards on a stored field (`ESS-COMMAND-004`,
`spec/domains/lending.yaml:203`), so a held book is taken with a separate command, `CollectHold`.
`BorrowBook` is unchanged and answers `BookStateConflict` for a book `OnHold`, whoever asks.

## Out of scope

- Queues of reservations (decision 1).
- Hold expiry by time (decision 3b).
- A Member actor or member self-service (decision 6).
- A network-facing service; `impl/` stays an in-memory package.
- The registered-member check itself — `story:borrow-member-check-tested` owns that for `Borrow`.

## Acceptance

`go test ./...` in `impl/`, run against an `impl/essconform` regenerated from the specification
above (`ess verify conform synthesize --target go`), passes all 55 synthesized scenarios with none
skipped.
