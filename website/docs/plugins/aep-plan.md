---
title: AEP Plan
---

# `aep-plan`

Use this plugin to work with AEP's governed planning substrate.

It provides:

- a planning skill that discovers the repository-local store and uses the canonical `aep` command;
- a story migration skill that adopts an existing backlog without rewriting or deleting its
  sources;
- a decomposer for turning a concrete outcome into related planning artifacts;
- a plan reviewer for checking readiness, evidence, and dependency shape;
- a reverse engineer for mapping an existing codebase into reviewable work;
- an acceptance critic for checking that each drafted item states an outcome somebody can observe;
- a design critic for checking the shape of a decomposition: coupling, cycles, split abstractions;
- a scope critic for checking that the set covers what it was drafted from, and nothing beyond it;
- a parallel-safety critic for naming the items that would land on the same file.

The four critics are a panel, not four separate reviews. After a decomposition is reported, the
planning skill dispatches them at once, records each verdict as an immutable review result related
to the artifacts it judged, revises the drafts on every verdict that asks for it, and stops after
two rounds with whatever is still open named in its report. None of them writes to the store.

Planning also refuses to decompose an epic or story that introduces an entity no ESS domain
declares. The domain is drafted and cited from the artifact first, and any relation that could not
be read from code, an OpenAPI document or an existing artifact is marked unmapped, never guessed.

The plugin respects store ownership: machine-owned artifact metadata is changed through AEP, not
by editing markdown frontmatter. A refusal from the lifecycle is a result to report, not a guard to
route around.

Install this plugin for planning. Add [`aep-drive`](./aep-drive.md) only when accepted work moves
into a development wave.
