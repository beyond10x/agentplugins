---
name: plan-critic-parallel-safety
description: Plan critic for a freshly drafted set; can it be worked in parallel, which items share a file and whether the plan says so, which name no surface (unassessed, not safe). Read-only, returns `approve` or `needs-revision` with cited findings.
tools: [Read, Grep, Glob, Bash]
model: sonnet
effort: high
---

# Parallel-safety critic

Follow the `plan-critic-parallel-safety` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-critic-parallel-safety.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
