---
format: aep.planning-md/3
id: story:track-releases-1123
kind: story
status: draft
title: Track AEP 0.80.2, ESS 0.98.0, Worktree 0.21.0 and Connectors v0.76.0
summary: Move verified.json and the eval pins to the newest releases and re-verify the skills
revision: 4
---
## Context

The check "Skills match the newest CLI releases" (`.github/workflows/tools.yml`) failed on main
(`ccdb2bcf`) with four releases newer than `verified.json`: AEP 0.80.2 (0.79.0), ESS 0.98.0
(0.90.0), Worktree 0.21.0 (0.19.4) and Connectors v0.76.0 (v0.69.0). Metaharness 0.30.0 is
unchanged and still links AEP 0.79.0, so the eval AEP stays 0.79.0.

AEP 0.80.0 adds packed stores (`aep plan store pack`/`unpack`, `aep.project/6`, `closed:` lifecycle
states; 0.79.x refuses both); 0.79.1 compares wave scope by path component; 0.80.1 and 0.80.2 are
fixes (`--via` takes no value, a version pin on an edge to the store's own artifact is refused).
ESS 0.91.1 to 0.98.0 add `ess/36` (expression measures, ordered enums) and `ess/37` (empty), suite
pair `/64`/`/65`, `ess-scenario/7` `covers:`, `ESS-SYNTH-025`, `--target process` with
`ess-conformance-target/1`, and break validation of enum ordering comparisons and of a late
`exists: false` branch; `--component` runs report one component only. Worktree 0.20.0 prints gc
progress and 0.21.0 adds `create --seed-target` and the Cargo 1.100 profile layout. Connectors
v0.70.0 to v0.76.0 add `invoke --deadline-seconds`, `--credential-env`, read-only Google
profiles, `metadata_missing`, `authentication_changed`, token recipients, and v0.76.0 changes the
metadata store one way.

## Acceptance

The AEP, ESS, Worktree and Connectors skills describe each consumer-visible change of the
releases, or this record says why none applies; `verified.json` names aep 0.80.2, ess 0.98.0,
worktree 0.21.0 and connectors v0.76.0; `.github/workflows/eval.yml` names ess 0.98.0, connectors
v0.76.0 and worktree 0.21.0 with eval AEP 0.79.0 beside Metaharness 0.30.0; the ESS tutorial set
moves to 0.98.0 and `ess/37`, the governed-plan tutorial to AEP 0.80.2; the planning protocols pin
moves to AEP 0.80.2; `agentplugins-check tools` exits 0; the ESS round, the AEP trials and
`worktree-onboarding` are run and their standings recorded.

## Scope

`plugins/{aep,ess,worktree,connectors}/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`.engineering/project.yaml`, `CHANGELOG.md`, `fixtures/library-reservations-drafted`,
`trials/{ess-tutorial,aep-tutorial}`, `trials/baseline.json`, `website/docs/{tutorials,plugins}`.

## No instruction change

- AEP 0.80.1 fixes (mermaid node ids, board counts and providers, seeding of ids differing by a
  separator, `serve` review outcomes, `findings` ordering): no skill taught a workaround.
- AEP 0.80.2 `--via` help: no skill spells `--via`.
- Connectors new catalog reads (Grafana, HubSpot owners, Drive `files.getMedia`): discovered
  through `operations list`, which the skill already teaches.
- Cited closed issues aep#60 and ess#585, #587, #626, #629, #662, #682: the citing text already
  describes the released fix.

## Trial standings

Isolated runs of 2026-11-23 on the plugins of this branch, with AEP 0.80.2, ESS 0.98.0 and Worktree 0.21.0, measured by `agentplugins-check trial-report` against `trials/baseline.json`. Wall times include the wait for a build slot.

| Trial | Result | Measures | Wall |
|---|---|---|---|
| `aep-backlog` | passed | 53 tool calls (baseline 268) | 1078 s |
| `worktree-onboarding` | passed | 14 tool calls | 2027 s |
| `ess-pipeline` | passed | valid, 14 scenarios, 0 refusals, outputs 4 of 4 | 2011 s |
| `ess-new` | passed | valid, 0 `UNMAPPED:` markers | 2082 s |
| `ess-tutorial` | passed | valid, 17 scenarios, 0 refusals, outputs 5 of 5, cargo 2 passed | 2634 s |
| `ess-full-package` | passed | valid, 17 scenarios, 0 refusals, outputs 9 of 9, cargo 4 passed | 2893 s |
| `upgrade-seeded` | passed | 8 tool calls; installed `aep` 0.80.2 and `connectors` v0.76.0 from 0.12.0 plugins | 1515 s |
| `ess-retrofit` | worse | valid, 22 scenarios, 3 refusals (baseline 0) | 2138 s |
| `ess-retrofit` (rerun after the text fix) | worse | valid, 22 scenarios, 3 refusals | 291 s |
| `aep-tutorial` | did not finish | 47 scenarios, 0 refusals, outputs 4 of 4; cargo test not measured | 2909 s |
| `aep-tutorial` (rerun) | worse by the measure | 402 tool calls, 47 scenarios, 0 refusals, outputs 4 of 4; wave 1 merged; cargo test counted as not run | 2880 s |

- `ess-retrofit`: all 3 refusals are `ESS-SYNTH-005` from one view filter that orders a generated `due_at` against `now`. ESS 0.90.0 refuses the same specification the same way, so this is not a regression of 0.98.0. The skill now says to keep such a filter and report it; the agent did. Filed as beyond10x/ess#819.
- `aep-tutorial`: the headless session ended after the implementor's turn, while its background `cargo test` for the red run still built, so the wave was never measured. The agent's own baseline run before the wave passed 17 of 17 scenarios and 2 cargo tests.
- `aep-tutorial` rerun: the wave finished. Critics approved in round 2, the implementor and the adversary ran, and wave 1 merged with ESS `{Passed: 26, Failed: 12, Unsupported: 9}`: the 26 required scenarios pass, and the 21 deferred ones belong to wave 2. Cargo on `main` gave 2 + 8 passed. `trial-report` counts cargo test as not run because the last `cargo test` sent its output to a file, and the measure fails closed on missing output. The run hit `workspace-not-activated` at `worktree create`; the tutorial now says `/worktree:init` activates a profile. Left as notes: the tutorial's "In ESS 0.56" guard caveat predates 0.98.0, and the agent's first `ReturnBook` shape hit `ESS-SYNTH-008`.
- No trial covers Connectors beyond `upgrade-seeded`, which installs `connectors` v0.76.0.
- Baselines are not moved by this story.
