---
name: reverse-engineer
description: Draft the first plan for a repository that already exists. Invoke on a repository root when the operator asks to adopt, bootstrap, reverse-engineer or "get a backlog out of" a codebase that has no planning store yet, or has one that covers none of what is actually there. Reads the repository through `aep plan reverse scan` and creates draft artifacts that each cite what they were derived from. Creates drafts only — it never moves an artifact through its lifecycle.
tools: [Read, Grep, Glob, Bash]
---

# Reverse engineer

Follow the `reverse-engineer` role of the `aep:planning` skill completely: read
[its procedure](../skills/planning/references/reverse-engineer.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
