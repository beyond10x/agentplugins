---
format: aep.planning-md/1
id: story:finding-signature-ledger
kind: story
status: draft
title: The second adversary attack knows what the first found
relations:
- decomposes: epic:second-adopter-feedback
revision: 1
---
# Story: The second adversary attack knows what the first found

## Outcome

The `adp` wave can tell whether a second adversarial pass found residue of the first pass's findings
or new ground, by comparing finding signatures (`file:line` + verdict + origin) rather than prose.

## Context

Filed from the 2026-09-02 review of a third-party plugin, which keeps a code-computed finding ledger
across review iterations. Not scheduled; recorded so the idea has an owner and a place.

## Acceptance

- The wave skill's adversary route compares the two finding tables by signature and reports
  `residue`, `new` and `cleared` counts.

## Out of Scope

A CLI verb for the ledger (protocol repository).

## Open Questions

Whether the ledger should live in `review-result` bodies or as evidence records — operator decides.
