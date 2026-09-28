---
format: aep.planning-md/3
id: story:cancel-reservation
kind: story
status: draft
title: A librarian cancels a reservation while the book is still on loan
summary: 'Library cancel operation and the CancelReservation command: OnLoanReserved goes back to OnLoan with reserved_for_id cleared; every other state is refused.'
relations:
- decomposes: epic:book-reservations
- informed_by: executable-system-specification:library
- depends_on: story:reserve-book-on-loan
- depends_on: story:return-reserved-book-to-hold
scope:
- confidence: cited
  path: impl/conformance_test.go
- confidence: cited
  path: impl/library.go
revision: 2
---
# A librarian cancels a reservation while the book is still on loan

## Context

Epic decision 4: a reservation can be cancelled while the book is on loan. Once the book is on hold
the reservation is ended by `CollectHold` or `ReleaseHold`, not by `CancelReservation`, which refuses
there.

Specification it is held to:

- `spec/domains/lending.yaml:71-73` — transition `cancel_reservation`, `OnLoanReserved -> OnLoan`.
- `spec/domains/lending.yaml:344-372` — `CancelReservation`: `cancelled` (clears `reserved_for_id`,
  emits `ReservationCancelled`), `wrong-state` (`BookStateConflict` from any state but
  `OnLoanReserved`), `no-such-book`.
- `spec/domains/lending.yaml:488-491` — event `ReservationCancelled { book_id }`.
- `spec/components.yaml:15`, `:26` — the component accepts `CancelReservation` and publishes `ReservationCancelled`.

It depends on `story:reserve-book-on-loan` because `cancelled` needs a reserved book, and on
`story:return-reserved-book-to-hold` because one of its refusals (`OnHold`) needs a held book.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, `references`; cleared by `cancel_reservation`,
  the borrower keeping the book — inferable from `spec/domains/lending.yaml`, entity
  `library.lending.Book`, relation `reserved_for` (lines 52-56), with the lifecycle at lines 57-85.

## Scope

- `impl/library.go` — a cancel-reservation method.
- `impl/conformance_test.go` — `ExecuteCommand` case for `library.lending.CancelReservation`.
- `impl/essconform/` — regenerated locally if absent (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`; not parallel-safe with any of them.

## Acceptance

With `impl/essconform` regenerated from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for the 7 scenarios `CancelReservation/outcome/{cancelled,no-such-book}`, `Book/transition/cancel_reservation/by/library.lending.CancelReservation/cancelled` and `Book/state/{OnShelf,OnLoan,OnHold,Withdrawn}/refuses/CancelReservation`, and fails no scenario.
