---
format: aep.planning-md/1
id: story:drive-entry-skill
kind: story
status: draft
title: /drive runs one story under aep drive from a Claude Code session
summary: A skill that runs aep doctor then launches aep drive run for one story in a hermetic scratch home; blocked on the aep walk reaching complete.
owner: plugins
tags:
- drive
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 1
---
# Story: `/drive` runs one story under `aep drive` from a Claude Code session

## Outcome

An operator in an ordinary Claude Code session says *drive story X* and a governed run starts in which every tool call is decided at the metaharness seam — the protocol-level loop that today is in force only when somebody runs `aep drive` from a terminal.

## Context

The `adp/default` workflow's back-edges and its independent-verifier requirement are engine-decided under `aep drive` and not in force in an interactive session (`aep/workflows/development/default.yaml:34-113`; review of 2026-09-02 §6a). No session path invokes `drive`. On the `aep` side the walk has never reached `complete` (`story:governed-dogfood-run`: two attempts, stopped in `establish_verifiers` and `adversarial_verify`), and the native arm cannot write (`story:confined-driven-workspace`). This skill is the entry; those two stories are what it waits on.

## Acceptance

- `/drive <story-id>` checks `aep doctor`, then launches `aep drive run` for the story with the project's step map, in a hermetic scratch home so the nested harness does not inherit the session's identity or tools, and prints the run id and the `aep drive watch` line.
- The skill never performs a store move itself; the run's moves are the driver's.
- On a refusal (lock held, evidence missing) the skill relays the driver's reason verbatim and stops.
- The golden-path page gains a final step: *drive the first story*.

## Out of Scope

Making the walk reach `complete`. That is `aep` `story:governed-dogfood-run`; this story is blocked until it lands and says so.

## Ambiguities

- `requires-stakeholder-input` — whether a nested `metaharness run claude` from inside Claude Code is supported or the skill must tell the operator to run it from a terminal. Decides: metaharness owner. Default: try the nested hermetic launch; fall back to printing the terminal command.
- `inferable` — `aep drive watch` is a proposed verb (`aep` `story:drive-watch-is-a-verb`, draft); until then the skill prints the script path.

## Open Questions

None.
