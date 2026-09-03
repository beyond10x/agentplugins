---
title: AEP Drive
---

# `aep-drive`

Use this plugin to deliver accepted development work through the Agentic Development Protocol
profile.

It provides:

- wave coordination guidance;
- a story scoper that turns an accepted story into bounded implementation units;
- an implementor role for an assigned unit;
- an adversary role that checks the result against scope, evidence, and repository invariants;
- a `drive` entry that starts one governed `aep drive` run over a single story.

This plugin builds on AEP's planning substrate. It does not replace the repository gate, invent lifecycle
moves, or give implementors authority beyond their assigned unit.

## Two ways to deliver a story, and they enforce differently

The wave coordinates an interactive session: its rules are instructions the coordinating agent
follows. `drive` hands one story to the reference driver, where the step map's bounds are decided by
the engine rather than obeyed by an agent.

Driven runs are not finished work on the `aep` side. The walk has not yet reached `complete` —
`aep`'s `story:governed-dogfood-run` records two attempts that stopped before the review step — so
the `drive` skill says so before it launches anything, prints the run id and how to follow it, and
moves no artifact itself.
