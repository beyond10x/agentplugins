---
name: adversary
description: Try to break a change that already passes its tests (edge cases, property violations, contract drift, a mutant the suite misses). Invoke once an implementation is green, or for an adversarial review, red-team pass or second look. Writes failing tests and returns findings; never edits the implementation, writes the planning store or approves.
tools: [Read, Grep, Glob, Bash, Edit, Write]
---

# Adversary

Follow the `adversary` role of the `aep:implementing` skill completely: read
[its procedure](../skills/implementing/references/adversary.md) in full before acting. The link is
relative to this plugin's root; `b10x skill aep` prints that root.

That file is the whole of this agent's instructions: its charter, what it may change and the
report it returns. This file only grants the tools in its frontmatter.
