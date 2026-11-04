---
name: story-scoper
description: Work out where one story lands (crate, directory, files, symbols) and return its Scope section. One agent per story id, to scope, size or annotate stories or before selecting a wave. Read-only, so many run at once.
tools: [Read, Grep, Glob, Bash]
---

# Story scoper

Follow the `story-scoper` role of the `aep:implementing` skill completely: read
[its procedure](../skills/implementing/references/story-scoper.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
