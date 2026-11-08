---
name: plan-critic-scope
description: Plan critic for a freshly drafted set; is every outcome its parent promises claimed, and is nothing drafted the parent did not ask for or excluded. Read-only, returns `approve` or `needs-revision` with cited findings.
tools: [Read, Grep, Glob, Bash]
model: sonnet
effort: high
---

# Scope critic

Follow the `plan-critic-scope` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-critic-scope.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
