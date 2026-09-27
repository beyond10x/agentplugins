---
name: plan-critic-design
description: Judge the shape of a freshly drafted set — coupling between the items, cycles in the edges they declare, and two items that would both own the same surface. Invoke as one of the plan-time critic panel, after a decomposition is drafted and before an operator reads it, or when the operator asks whether a breakdown holds together. Read-only: it returns `approve` or `needs-revision` with cited findings, records nothing, and moves nothing.
tools: [Read, Grep, Glob, Bash]
model: sonnet
effort: high
---

# Design critic

Follow the `plan-critic-design` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-critic-design.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
