---
format: aep.planning-md/3
id: story:track-releases-1026
kind: story
status: active
title: Track AEP 0.78.0, ESS 0.69.2, Metaharness 0.19.0 and Connectors v0.59.0
summary: Move verified.json, the eval pins and the protocols pin to the newest releases and re-verify the skills
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-26T19:14:43Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-26T19:14:44Z", actor: "human:timo", revision: 4}
---
## Context

`agentplugins-check upstream` and `agentplugins-check tools` on 2026-10-26 report four releases newer than
`verified.json`: AEP 0.78.0 (0.76.0), ESS 0.69.2 (0.68.0), Metaharness 0.19.0 (0.14.0) and Connectors
v0.59.0 (v0.56.0). `tools` on unchanged `main` found every spelled command present in all four releases;
only the four version problems failed. AEP 0.78.0 makes `aep plan artifact evidence --from` admit the
suite majors ESS 0.69.2 writes. Metaharness 0.19.0 links AEP 0.77.0 (its `Cargo.lock`), so the eval AEP
pin moves to 0.77.0 with it. The planning protocols pin moves to the 0.78.0 tag commit.

## Acceptance

The AEP, ESS, Metaharness and Connectors skills describe each consumer-visible change of the releases
or the change record says why none applies; `verified.json` names aep 0.78.0, ess 0.69.2, metaharness
0.19.0 and connectors v0.59.0; `.github/workflows/eval.yml` names metaharness 0.19.0 with eval AEP 0.77.0,
ess 0.69.2 and connectors v0.59.0; `.engineering/project.yaml` names protocols at aep 0.78.0; the ESS
tutorial set moves to 0.69.2 together; `agentplugins-check tools` exits 0; the `ess-tutorial`,
`ess-full-package`, `aep-backlog` and `aep-tutorial` trials measure no worse than
`trials/baseline.json`; `task check` and `task site-build` exit 0.

## Scope

`plugins/{aep,ess,connectors}/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`.engineering/project.yaml`, `website/docs/plugins/*.md`, `website/docs/tutorials/first-ess-specification*`,
`trials/{ess,aep}-tutorial/fixture/**`, `fixtures/library-reservations-drafted/**`, `CHANGELOG.md`.

## Verification

On the candidate tree, 2026-10-26: `task check` exit 0; `agentplugins-check tools` exit 0 (aep 0.78.0 230,
ess 0.69.2 105, worktree 0.18.2 52, metaharness 0.19.0 15 and connectors v0.59.0 32 spelled commands;
tutorial 17/17 scenarios passed); `task site-build` exit 0. Before the checker change, `tools` refused
`connectors server` as absent from the v0.59.0 `cli.yaml`, although the release binary's help lists it
as a native integration command routed by `apps/connectors/src/mcp/entry.rs`.

Isolated trials at plugin version 0.24.2, each isolation check passed:

| trial | tool calls (baseline) | result |
|---|---|---|
| `ess-tutorial` | 65 (67) | valid, 17 scenarios 0 refusals, 5/5 outputs, cargo test 2/0; no measure worse |
| `ess-full-package` | 99 (67) | valid, 27 scenarios 0 refusals, 0 unmapped, 9/9 outputs, cargo test 5/0; no measure worse. A first run ended on an expired sign-in after 74 turns and was discarded |
| `aep-backlog` | 245 (268) | store valid; no measure worse |
| `aep-tutorial` | 568 (453) | timed out at 2400 s inside the wave's second adversary pass; the same isolated session was resumed once and finished (both transcripts kept, isolation passed on each). 50 scenarios 0 refusals, 4/4 outputs; the final `cargo test` was redirected to `.engineering/runs/2026-10-26-wave-1-merge-6fb3884.txt` in the trial repository, which holds 8 passed 0 failed (baseline 7/0), so `trial-report` reads it as not run |

Baselines are unchanged. Trial wording findings, all in text this change did not alter, are left for the
next tutorial wording round: the governed-plan page does not say a Worktree profile must be activated
(`worktree doctor --check` answered `no active profile`), it still cites ESS 0.56 and `ess/22` where its
fixture is `ess/23` and the reservation model needs `ess/27`, and its `ReserveBook.member_id` example
note did not reproduce; the ESS tutorial's `wrong-state` answer for an unknown book in `ReturnBook` and
`WithdrawBook` confused the agent.
