---
format: aep.planning-md/3
id: task:refresh-aep-0713-worktree-0182-connectors-050
kind: task
status: active
title: Track AEP 0.71.3, Worktree 0.18.2 and Connectors v0.50.0
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-14T11:03:25Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-14T11:03:25Z", actor: "human:timo", revision: 3}
---
## Context

AEP 0.71.3, Worktree 0.18.2 and Connectors v0.49.0 and v0.50.0 were published after release
0.22.10, and the scheduled Tools check on `main` reports them as newer than `verified.json`. ESS
0.59.0 and Metaharness 0.9.3 are still the newest.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.71.3, worktree 0.18.2 and
connectors v0.50.0. Each consumer-visible change in their release notes is reflected in the owning
skill or recorded under Classification. The isolated `worktree-onboarding` trial measures no worse
than `trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, CHANGELOG.md, Cargo.toml, Cargo.lock, every
plugin manifest and `**Skill version**` line, plugins/aep/skills/planning/SKILL.md,
plugins/connectors/skills/{init,integrating,upgrade}, website/docs/plugins/connectors.md.

## Classification

| Change | Resource | Decision |
|---|---|---|
| aep 0.71.3: evidence records name report and suite by relative path and digest; prints `counts` or `does not count:` | aep:planning | one paragraph added |
| worktree 0.18.2: `archive` works on Git before 2.42 | none | `worktree skill` and every `--help` equal 0.18.1's |
| connectors v0.49.0: CLI builds and runs on Windows from source, Credential Manager custody, no Windows binary | connectors init, integrating, setup reference, plugin page | Windows stated as a source build |
| connectors v0.50.0: store writer marker, `metadata_schema_newer` | connectors:upgrade | refusal, recovery and state-directory sharing rule added |
| connectors v0.50.0: OAuth loopback answers 400 after the flow decided | none | internal fix |
