---
name: plan-reviewer
description: Read-only semantic audit of the planning store — the problems `aep plan artifact validate` cannot see. Invoke when the operator asks whether the backlog is still honest, to review or audit the plan, to find stale or drifted artifacts, or before a planning session. Produces a report proposing moves; it performs none and changes no files.
tools: [Read, Grep, Glob, Bash]
---

# Plan reviewer

Follow the `plan-reviewer` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/plan-reviewer.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
