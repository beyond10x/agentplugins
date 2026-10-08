---
format: aep.planning-md/3
id: task:refresh-aep-0691
kind: task
status: implemented
title: Track AEP 0.69.1
summary: verified.json and the protocols source move to AEP 0.69.1
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T08:52:16Z", actor: "human:timo", revision: 4}
- {from: "proposed", to: "active", at: "2026-10-08T08:52:16Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-10-08T08:52:16Z", actor: "human:timo", revision: 6}
---
## Context

AEP 0.69.1 shipped on 2026-10-08 (verified.json pins 0.69.0). Its one changelog entry is a fix:
`aep plan artifact relate`, `new --relate` and `unrelate` write and take back an edge to another
declared workspace member's artifact (`<member>/<kind>:<name>`).

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.69.1, the planning store validates
under aep 0.69.1 with its `protocols` source at the 0.69.1 commit, and the pull request's `Gate`
and `Tools` checks pass.

## Scope

Cited: verified.json, .engineering/project.yaml, CHANGELOG.md, Cargo.toml, Cargo.lock, every
plugin manifest and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| cross-member `relate`/`new --relate`/`unrelate` fixed | none | no skill or page teaches cross-member edges or a workaround for them |
| eval AEP pin 0.68.0 | none | Metaharness 0.9.1, the newest, links AEP 0.68.0; the pair moves together |

## Verification

- aep 0.69.1 x86_64 Linux archive from the release, checksum OK; `aep --version` prints `aep 0.69.1`.
- `aep plan artifact validate` with 0.69.1: `valid`.
- `agentplugins-check tools`: exit 0; aep 0.69.1 218 spelled commands checked.
