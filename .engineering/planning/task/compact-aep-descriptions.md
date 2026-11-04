---
format: aep.planning-md/3
id: task:compact-aep-descriptions
kind: task
status: draft
title: Compact the aep skill and agent descriptions
summary: The 22 aep skill and agent descriptions carry the same triggers in at least 40% fewer characters
revision: 1
---
## Context

Every session with the `aep` plugin enabled carries its 11 skill descriptions and 11 agent
descriptions in context on every turn, whether or not the session plans or delivers anything. The
descriptions had grown to 10,369 characters (about 2,590 tokens at characters / 4), much of it
restating procedure the skill body already holds.

## Acceptance

Every `aep` skill and agent `description` is shorter, together at least 40% fewer characters, and
each still names the same triggers, the same exclusions (`aep:diagnosing` against
`aep:investigating`, conformance coverage to `ess:testing-conformance`) and, for a command, the
activity it hands off to. Each frontmatter block still parses as YAML. `CHANGELOG.md` records the
change under Unreleased. `task check` passes and the pull request's checks are green.

## Scope

plugins/aep/skills/*/SKILL.md (frontmatter `description` only), plugins/aep/agents/*.md
(frontmatter `description` only), CHANGELOG.md.
