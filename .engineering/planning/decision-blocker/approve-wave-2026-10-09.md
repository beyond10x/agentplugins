---
format: aep.planning-md/3
id: decision-blocker:approve-wave-2026-10-09
kind: decision-blocker
status: cleared
title: 'Approve the wave of 2026-10-09: hardening findings block and upstream refresh'
relations:
- blocks: story:hardening-findings-block
- blocks: task:follow-releases-2026-10-08
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-08T22:38:10Z", actor: "human:timo", revision: 3}
---
# Decision blocker: approve the wave of 2026-10-09

## Question

Approve one wave of two units on integration branch `wave/findings-block`, one pull request, then
one patch release?

## Units

| unit | files | risk |
|---|---|---|
| story:hardening-findings-block | `plugins/ess/skills/hardening/SKILL.md` | low, skill text |
| task:follow-releases-2026-10-08 | `verified.json`, `.github/workflows/eval.yml`, `.github/workflows/shared-gates.yml`, `.engineering/project.yaml`, `website/package.json` and lockfile, `plugins/worktree/skills/managing-worktrees/`, `plugins/connectors/skills/integrating/SKILL.md` | medium, tool pins |

The units share no file. `story:coordinator-returns-blockless-report` stays draft until the aep
release that makes the findings block mandatory.

## Options

- A: the wave as listed; one PR; release 0.22.3 after green CI. Cost: one CI full gate, builds
  under 2G.
- B: only the upstream task now; the hardening text with the coordinator change later. Cost: a
  second release.

Recommendation: A.

## Decision

Decided 2026-10-09: option A. Both units in one wave on `wave/findings-block`, one pull request, patch release 0.22.3 after green CI.
