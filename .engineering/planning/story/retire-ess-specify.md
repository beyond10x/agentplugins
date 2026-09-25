---
format: aep.planning-md/1
id: story:retire-ess-specify
kind: story
status: implemented
title: Retire ess-specify; spec-driven work routes to the ESS repository's own plugin
revision: 5
---
# Retire `ess-specify`; spec-driven work routes to the ESS repository's own plugin

## Why

ESS now ships its agent plugin from `beyond10x/ess` (`plugins/ess/`, marketplace `ess`), at the
same version as the `ess` binary, and the binary prints the same skills through `ess skill`. The
copy here released on its own cadence and drifted: `0.9.2` told adopters to install ESS `0.22.1`
while ESS was at `0.29.0`.

## Acceptance

`plugins/ess-specify` and its marketplace entries are gone; `task check` prints
`valid: marketplace beyond10x, 5 focused plugin(s)`; `ess-specify` is a retired name the gate
refuses in authored files; the `beyond10x` front door and the install guide send spec-driven work to
the ESS marketplace and `ess skill`.

## Scope

- `plugins/ess-specify/**` (deleted), `.claude-plugin/marketplace.json`, `.agents/plugins/marketplace.json`
- `crates/agentplugins-check/src/{main.rs,evals.rs}`
- `evals/ess-specify-new-entity/**` (deleted), `evals/golden-path-end-to-end/**`, `evals/README.md`
- `plugins/beyond10x/skills/beyond10x/**`, `plugins/aep-plan/skills/planning/SKILL.md`
- `README.md`, `website/**`, `CHANGELOG.md`

## Not in scope

- An eval case for the ESS plugin. ESS runs no eval corpus; the dropped case is not re-homed.


## Outcome (2026-09-25)

`ess-specify` was retired in agentplugins 0.10.0 (PR #11, `514ef9e0`, release run 35920939084). The routing target then moved: ess#68 removed the ESS repository plugin, and the ESS plugin ships from this repository as `plugins/ess` (`ess@b10x`, #17 `f03fa470`, #18 `7574cb86`, release 0.14.1).
