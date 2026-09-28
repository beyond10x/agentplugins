---
format: aep.planning-md/3
id: vision:lending-library
kind: vision
status: draft
title: A small lending library, specified in ESS and implemented in Go
summary: Books are added, members registered, and a member borrows a book and returns it.
revision: 2
---
# A small lending library, specified in ESS and implemented in Go

## Evidence

- spec/domains/lending.yaml:3 — "A small lending library. Books are added to the collection, members are registered, and a member borrows a book and later returns it."
- spec/components.yaml:3 — the single component `library-service` "Holds the collection and the members, and lends books to members."
- impl/library.go:1 — "Package library is a minimal in-memory lending library: books are added and withdrawn, members are registered, and a member borrows a book and returns it."
- spec/system.yaml:2 — `system: library`, version v1, one domain `library.lending`.

## Context

The repository has no README of its own (the only README the scan found, impl/essconform/README.md,
is generated and git-ignored), so the statement of purpose is taken from the specification's domain
summary and the Go package comment, which agree. The system is one domain, one component and one
actor (Librarian, spec/domains/lending.yaml:68): add books, register members, borrow, return and
withdraw books, and read three views (Catalogue, Members, BooksOnLoan). The ESS specification in
spec/ is the contract; impl/ is a Go implementation checked against it by a generated conformance
suite.

Nothing in the tree states goals beyond this: no roadmap, no stages, no non-goals.
