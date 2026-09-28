---
format: aep.planning-md/3
id: story:release-hold
kind: story
status: draft
title: A librarian releases a hold and the book goes back on the shelf
summary: 'Library release operation and the ReleaseHold command: OnHold goes to OnShelf with reserved_for_id cleared and HoldReleased emitted; every other state is refused.'
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
# A librarian releases a hold and the book goes back on the shelf

## Context

Epic decision 3b: a hold ends when a librarian releases it with `ReleaseHold`; there is no time-based
expiry (the specification has no clock, `spec/domains/lending.yaml:415-416`).

Specification it is held to:

- `spec/domains/lending.yaml:80-82` — transition `release_hold`, `OnHold -> OnShelf`.
- `spec/domains/lending.yaml:417-445` — `ReleaseHold`: `released` (clears `reserved_for_id`, emits
  `HoldReleased`), `wrong-state` (`BookStateConflict` from any state but `OnHold`), `no-such-book`.
- `spec/domains/lending.yaml:500-503` — event `HoldReleased { book_id }`.
- `spec/components.yaml:17`, `:28` — the component accepts `ReleaseHold` and publishes `HoldReleased`.

## Domain relations

- `Book → Member` via `reserved_for`: many-to-one, `references`; cleared by `release_hold`, leaving
  the book held for nobody — inferable from `spec/domains/lending.yaml`, entity
  `library.lending.Book`, relation `reserved_for` (lines 52-56).

## Scope

- `impl/library.go` — a release-hold method.
- `impl/conformance_test.go` — `ExecuteCommand` case for `library.lending.ReleaseHold`.
- `impl/essconform/` — regenerated locally if absent (git-ignored), not committed.

Shares both Go files with every other story decomposing `epic:book-reservations`, including its
sibling `story:collect-held-book` which depends on the same story; the two are not parallel-safe.

## Acceptance

With `impl/essconform` regenerated from `spec/`, `go test -v ./...` in `impl/` reports PASS (not SKIP) for the 7 scenarios `ReleaseHold/outcome/{released,no-such-book}`, `Book/transition/release_hold/by/library.lending.ReleaseHold/released` and `Book/state/{OnShelf,OnLoan,OnLoanReserved,Withdrawn}/refuses/ReleaseHold`, and fails no scenario.
