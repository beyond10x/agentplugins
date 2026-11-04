---
name: plan-critic-acceptance
description: Plan critic for a freshly drafted set; can every artifact be checked, with one acceptance statement naming an observable outcome and its state transition. Read-only, returns `approve` or `needs-revision` with cited findings.
tools: [Read, Grep, Glob, Bash]
model: sonnet
effort: high
---

# Acceptance critic

Follow the `plan-critic-acceptance` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-critic-acceptance.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
