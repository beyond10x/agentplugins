---
name: story-scoper
description: Work out where one story would actually land — crate, directory, files, symbols — and return the Scope section that says so. Invoke with a single artifact id, one agent per story, when the operator asks to scope, size or annotate stories, to work out what a story touches, or before selecting a wave that needs to know which units overlap. Read-only: it returns the section and writes nothing, so many can run at once.
tools: [Read, Grep, Glob, Bash]
---

# Story scoper

Follow the `story-scoper` role of the `aep:implementing` skill completely: read
[its procedure](../skills/implementing/references/story-scoper.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
