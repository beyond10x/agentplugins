---
name: implementing
description: Implement accepted AEP stories. Wave mode proposes the next wave of the `aep plan artifact waves` plan and stops for approval, then runs one implementor per story in its own worktree, adversary review, merges what is green, files pre-existing findings as backlog drafts. Drive mode hands one story to a governed `metaharness aep drive` run and reports its id. Use to implement or deliver planned stories, start the next wave, fan stories out to sub-agents, drive a story, or ask how a wave differs from a drive. Never moves an artifact.
---

**Skill version 0.26.0** — the version in `.claude-plugin/plugin.json`; a wave's stage-1 proposal quotes it.

# Implementing accepted work

Planned work comes from the AEP store (`aep:planning`). This skill turns accepted stories into merged
code, in one of two modes:

| mode | use it for | who enforces the rules | read before acting |
|---|---|---|---|
| **wave** | the next wave of a plan defined up front, several stories at once, in this session, with the operator approving each wave | you, the coordinating agent, by following them | [references/wave.md](references/wave.md) |
| **drive** | one story handed to the engine | the `metaharness` engine, which decides every transition | [references/drive.md](references/drive.md) |

Pick the mode from the request: "drive", "driven" or "governed run" means **drive**; implementing,
building, delivering, a wave or parallel work means **wave**. If the request fits neither, ask one
question that names both. Then read that mode's reference in full; this page only chooses.

The operator can also name the mode directly: `/aep:wave [story-id…]` (`aep:wave`) or
`/aep:drive <story-id>` (`aep:drive`), two commands that load this skill in that mode.

## The store's version, first

Before reading or writing the store, read `version` in `.engineering/project.yaml`. On anything but
`aep.project/5`, tell the user once, before any store write, which version it is and its upgrade;
do not wait for `aep` to print a notice:

| the store | upgrade |
|---|---|
| `aep.project/1`, or `.engineering/planning/journal.jsonl` with no `project.yaml` | `aep plan store migrate git --dry-run`, then `--verify`, on a clean `.engineering` (no `project.yaml`: add `--protocols` and `--profile`, `aep:planning` § 5), then commit. Every verb that reads or writes the plan exits 1 before any write, with `[unsupported_protocol_version]` (no `project.yaml`: an error naming `journal.jsonl` and the migration); only the migration opens the store: stop and report |
| `aep.project/2`, `/3` or `/4` | `cargo install --git https://github.com/beyond10x/aep --rev 9c0f1da44429ff935fa0b2d743457945d51e1c51 aep-cli`, then `aep plan store migrate git --verify` on a clean `.engineering`, commit, then install the current release (`aep:upgrade`). Every planning verb refuses the store: stop and report |

`aep:upgrade` carries the steps; the user decides when the migration runs. On `/1`, store work
waits until the user has approved the migration and reviewed what `--dry-run` would write,
`--verify` has passed, and the result is committed; do not carry on, open the store with an older
`aep`, or edit artifact files by hand.

## Rules for both modes

- Implement only stories the store shows as accepted; neither mode moves an artifact itself.
- Waves are derived up front, once, from the accepted stories (`aep plan artifact waves --status
  <accepted>`), by whoever plans the work; a coordinator runs the wave it is given, adds no story to
  it, and never accepts a story itself.
- A finding the unit introduced, or that its acceptance needs, goes back to the implementor. A
  pre-existing one becomes a draft in the backlog. Accepting it is the operator's act, and once
  accepted it waits until every planned wave is delivered: it was there before the wave, so it has no
  claim to go first.
- Every unit works in its own managed worktree from the worktree plugin; never share a checkout.
- Report each suite's own output, never a summary of it.
- A refusal from `aep`, the gate or the driver is relayed unedited.

## Agents

Each role's full procedure is `references/<role>.md` beside this skill; the agent file of the same
name is a thin Claude Code adapter over it. In a host without subagents, such as Codex, run the role
yourself from that file, in its own pass and within the tools it names.

- `story-scoper` — works out where one story lands and returns its Scope section; runs before a wave is proposed. ([procedure](references/story-scoper.md))
- `implementor` — implements one unit: the failing test first, then the smallest change. ([procedure](references/implementor.md))
- `adversary` — tries to break a unit that passes its own tests. ([procedure](references/adversary.md)) On request, several adversaries on different model families run as a panel. ([panel](references/adversary-panel.md))
- `security-reviewer` — independently checks that the unit's safety and correctness invariants hold. ([procedure](references/security-reviewer.md))
