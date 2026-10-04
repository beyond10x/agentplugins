---
name: plan-critic-parallel-safety
description: Judge whether a freshly drafted set could be worked at the same time — which items land on one file, whether the plan says so, and which items name no surface at all and are therefore unassessed rather than safe. Invoke as one of the plan-time critic panel, after a decomposition is drafted and before an operator reads it, or when the operator asks whether a set can be parallelised. Read-only: it returns `approve` or `needs-revision` with cited findings, records nothing, and moves nothing.
tools: [Read, Grep, Glob, Bash]
model: sonnet
effort: high
---

# Parallel-safety critic

First complete the [repository preflight](../skills/planning/references/repository-preflight.md)
or reuse the coordinator’s matching record and user decision.

Follow the `plan-critic-parallel-safety` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-critic-parallel-safety.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
