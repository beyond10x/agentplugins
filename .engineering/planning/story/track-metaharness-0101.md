---
format: aep.planning-md/3
id: story:track-metaharness-0101
kind: story
status: implemented
title: Track Metaharness 0.10.1 and the aep it links
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:58Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

Metaharness 0.10.1 (2026-10-20) is newer than `verified.json` (0.9.3). The eval workflow requires
`AEP_VERSION` to match the aep the metaharness release links (`.github/workflows/eval.yml:52-54`):
0.68.1 for metaharness 0.10.1.

## Acceptance

`verified.json` and `.github/workflows/eval.yml` name metaharness 0.10.1 (0.11.0 if tagged first) and
`AEP_VERSION` names the aep that release links; `CLAUDE_VERSION` stays as metaharness pins it;
skill text that quotes a changed metaharness command matches its `--help`; `agentplugins-check tools`
reports no metaharness problem.

## Scope

`verified.json`, `.github/workflows/eval.yml`, metaharness spellings in `plugins/**`, `CHANGELOG.md` (Unreleased).
