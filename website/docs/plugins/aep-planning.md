---
title: AEP Planning
---

# `aep-planning`

Use this plugin to work with AEP's governed planning substrate.

It provides:

- a planning skill that discovers the repository-local store and uses the canonical `aep` command;
- a decomposer for turning a concrete outcome into related planning artifacts;
- a plan reviewer for checking readiness, evidence, and dependency shape;
- a reverse engineer for mapping an existing codebase into reviewable work.

The plugin respects store ownership: machine-owned artifact metadata is changed through AEP, not
by editing markdown frontmatter. A refusal from the lifecycle is a result to report, not a guard to
route around.

Install this plugin for planning. Add [`adp`](./adp.md) only when accepted work moves into a
development wave.
