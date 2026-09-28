---
format: aep.planning-md/3
id: story:borrow-member-check-tested
kind: story
status: draft
title: The registered-member check on Borrow is exercised by a test
summary: The only caller of Library.Borrow passes requireMember=false, so ErrUnknownMember is reached by no test.
relations:
- serves: vision:lending-library
- informed_by: executable-system-specification:library
revision: 3
---
# The registered-member check on Borrow is exercised by a test

## Evidence

- spec/domains/lending.yaml:197-199 — "The borrowing member must be a registered member. ... Decided: the implementation checks it; this specification does not, and no scenario covers it." (Was :148-150 before the reservation model was added; the text is unchanged.)
- impl/library.go:38-39 — `ErrUnknownMember` "is returned when a borrow names a member nobody registered."
- impl/library.go:82-85 — `Borrow(bookID, memberID string, requireMember bool)`; "the conformance target turns it off."
- impl/library.go:93-97 — the check itself, taken only when `requireMember` is true.
- impl/conformance_test.go:88-90 — the conformance target calls `t.lib.Borrow(bookID, memberID, false)`: "the suite borrows for members it never registered, so it is off here."

## Context

The specification hands the registered-member rule to the implementation and says no scenario
covers it, so the conformance suite cannot be the thing that checks it — and the target turns it
off. impl/conformance_test.go is the only test file in the repository and the only caller of
`Borrow` in the tree, so the branch at impl/library.go:93-97 and `ErrUnknownMember` are executed by
nothing. A regression there (the check removed, inverted, or run after the state change) would pass
every test in the repository. Recorded as of the single commit 30e5ba2 (2026-09-28); there is no
older history to date it by.

## Acceptance

A Go test in impl/ borrows a shelved book for an unregistered member with `requireMember` true,
receives `ErrUnknownMember`, and observes the book still `OnShelf` with no borrower — and `go test
./...` runs it.
