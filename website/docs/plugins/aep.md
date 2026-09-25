---
title: AEP
---

# `aep`

Use this plugin to plan governed work in AEP's artifact store and to deliver accepted work through
the Agentic Development Protocol profile. Both halves drive the `aep` CLI.

## Planning

Use it to work with AEP's governed planning substrate.

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

## Delivery

It provides:

- the `implementing` skill, in wave mode: coordination guidance for several stories at once;
- a story scoper that turns an accepted story into bounded implementation units;
- an implementor role for an assigned unit;
- an adversary role that checks the result against scope, evidence, and repository invariants;
- the `implementing` skill, in drive mode: one governed `metaharness aep drive` run over a single story.

This plugin builds on AEP's planning substrate. It does not replace the repository gate, invent lifecycle
moves, or give implementors authority beyond their assigned unit.

### Two ways to deliver a story, and they enforce differently

The wave coordinates an interactive session: its rules are instructions the coordinating agent
follows. `drive` hands one story to the reference driver, where the step map's bounds are decided by
the engine rather than obeyed by an agent.

Driven runs are not finished work on the `aep` side. The walk has not yet reached `complete` —
`aep`'s `story:governed-dogfood-run` records two attempts that stopped before the review step — so
drive mode says so before it launches anything, prints the run id and how to follow it, and
moves no artifact itself.

Drive mode needs a Metaharness build that carries `metaharness aep drive`: AEP 0.55.0 refuses
a model-backed map itself and names that command. [Install](../install.md) names the build to use.

`b10x` treats both `metaharness` and `b10x-harness`, the Beyond10x agent loop that Metaharness's
`b10x` adapter runs, as optional CLIs of this plugin: it reports them, and `b10x install <cli>` adds
one. `b10x-harness` runs on Linux only.
