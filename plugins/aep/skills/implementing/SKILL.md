---
name: implementing
description: Implement accepted AEP work, in one of two modes. A wave picks the stories that can be implemented at once, proposes the wave for approval, dispatches one implementor per story into its own worktree, sends each result to the adversary and merges what goes green. A drive hands one story to a governed `metaharness aep drive` run and reports the run id. Use when the operator asks to implement, build or deliver planned stories, to pick or start the next wave, to implement several stories in parallel or fan out across sub-agents, to drive a story or start a governed run, or asks why a wave's rules are instructions and a drive's are enforced. A wave proposes first and stops; a drive starts one run and reports; neither moves an artifact itself.
---

**Skill version 0.14.2** — the version in `.claude-plugin/plugin.json`; a wave's stage-1 proposal quotes it.

# Implementing accepted work

Planned work comes from the AEP store (`aep:planning`). This skill turns accepted stories into merged
code, in one of two modes:

| mode | use it for | who enforces the rules | read before acting |
|---|---|---|---|
| **wave** | several stories at once, in this session, with the operator approving each wave | you, the coordinating agent, by following them | [references/wave.md](references/wave.md) |
| **drive** | one story handed to the engine | the `metaharness` engine, which decides every transition | [references/drive.md](references/drive.md) |

Pick the mode from the request: "drive", "driven" or "governed run" means **drive**; implementing,
building, delivering, a wave or parallel work means **wave**. If the request fits neither, ask one
question that names both. Then read that mode's reference in full; this page only chooses.

## Rules for both modes

- Implement only stories the store shows as accepted; neither mode moves an artifact itself.
- Every unit works in its own managed worktree from the worktree plugin; never share a checkout.
- Report each suite's own output, never a summary of it.
- A refusal from `aep`, the gate or the driver is relayed unedited.

## Agents

- `story-scoper` — works out where one story lands and returns its Scope section; runs before a wave is proposed.
- `implementor` — implements one unit: the failing test first, then the smallest change.
- `adversary` — tries to break a unit that passes its own tests.
- `security-reviewer` — independently checks that the unit's safety and correctness invariants hold.
