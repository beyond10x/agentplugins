---
format: aep.planning-md/3
id: task:ess-tutorial-wording-round-0211
kind: task
status: draft
title: 'ESS tutorial: five passages the 2026-10-07 trial agent could not read unaided'
summary: Clarify step 3, step 6, Node::Null for an absent Optional, and which status an invalid token is
relations:
- informed_by: task:refresh-ess-055-worktree-010-connectors-031
revision: 2
---
## Context

The isolated `ess-tutorial` trial of 2026-10-07 (ESS 0.55.0, plugins 0.21.0) passed: `valid`,
17 scenarios, 0 refusals, 17/17 conformance scenarios, 2 tests. Its agent quoted five passages of
`website/docs/tutorials/first-ess-specification.md` as unclear:

1. Step 6, "it maps only the five commands and three views": the page never names the five commands.
2. Step 3, "`wrong-state` answers `BookStateConflict`; `no-such-book` answers `BookNotFound`": said
   only for `BorrowBook`; the 17 scenarios need it for return and withdraw too.
3. Step 3 is titled "Read the specification", but the page never shows `domains/lending.yaml`.
4. "Use `Node::Text` for string values": silent that an absent `Optional` is `Node::Null`; the suite
   expects `"borrower_id": null`.
5. "An invalid or future token is an error": unclear whether that is the runner's `error`
   (`Unavailable`) or `Unsupported`.

The `worktree-onboarding` run of the same day read `worktree:init`'s headless workspace root as
covering every repository in the parent directory; it resolved it correctly, and no change is
proposed for it.

## Acceptance

Each of the five passages names what the agent had to infer, the tutorial's trial fixture copy
(`trials/ess-tutorial/fixture/tutorial.md`) matches the page, and a fresh isolated `ess-tutorial`
run quotes none of the five.

## Scope

Cited: website/docs/tutorials/first-ess-specification.md, trials/ess-tutorial/fixture/tutorial.md.

## Trial of 2026-10-08

The isolated `ess-tutorial` trial on ESS 0.56.0 and plugins 0.22.0 passed (valid, 17 scenarios, 0
refusals, 5/5 outputs, cargo test 2/0, 67 tool calls). Its agent again quoted "Use `Node::Text` for
string values" (an absent `borrower_id` must be `Node::Null`; one scenario failed until it was),
"maps only the five commands and three views" (the five are never named), and the
`book.state != "OnShelf"` guard (it assumes a string state).
