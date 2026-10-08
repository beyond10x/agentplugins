---
format: aep.planning-md/3
id: story:hardening-findings-block
kind: story
status: active
title: ess:hardening closes its report with a findings block
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T22:38:10Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-08T22:38:10Z", actor: "human:timo", revision: 3}
---
# Story: ess:hardening closes its report with a findings block

## Outcome

A hardening report ends with the same fenced `findings` block the AEP adversary and security
reviewer already close with, so a coordinator records it verbatim and `aep plan artifact findings`
compares two hardening passes by signature instead of by re-reading prose.

## Context

`plugins/ess/skills/hardening/SKILL.md` § Reporting (lines 96-101 on main at 856cb96) asks for prose
only: per technique, the planted defect, the green result, the command and the findings. The
adversary (`plugins/aep/skills/implementing/references/adversary.md:221`) and the security reviewer
(`references/security-reviewer.md:209`) already end with a YAML list in a ` ```findings ` fence
carrying `file`, `line`, `category`, `severity`, `verdict`, `origin`, `message`, and `[]` when
nothing was found. `aep plan artifact findings` matches findings by file, category and message
with the line within three; the category is free text.

Emitting the block is valid under every released `aep`; no version gate is needed.

## Acceptance

- § Reporting of `plugins/ess/skills/hardening/SKILL.md` requires a closing ` ```findings ` block,
  with one example, a field table and the rule that an empty pass is `[]`.
- Its `category` values are `spec-gap`, `missing-scenario`, `ess-limit`, `equivalent` and
  `false-positive`, each defined in one line.
- The block's fields and the `[]` rule match `adversary.md` § "The same findings, once more, in a
  fenced block" word for word where they overlap.
- An example block from the skill, recorded with `aep plan artifact new review-result … --from`,
  is parsed by `aep plan artifact findings` of the newest `aep` without a refusal.
- `task check` passes.

## Out of Scope

The coordinator returning a report that has no block to its author: that is
`story:coordinator-returns-blockless-report`, which waits for the `aep` release that makes the
block mandatory.
