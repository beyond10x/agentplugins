# The cross-family adversary panel

Before acting, complete the [repository preflight](../../planning/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

An optional form of the wave's adversary step, run **only when the operator asks for it** — "attack
it with a panel", "a second model family", "cross-model review". The default is one adversary, as
[wave.md](wave.md) describes. A panel costs one full attack per reviewer.

Models from different families miss different defects. A finding two families raise independently
is the strongest signal a review produces; a finding only one raises still deserves reading, and a
point where they contradict each other is worth the operator's attention.

## Who sits on it

One reviewer per model family available to you, each running the [adversary](adversary.md)
procedure unchanged, against the same worktree, with **the same brief file** and no knowledge of the
others' findings. The families and the model within each come from the operator or from the pins
the agents declare — never choose them yourself.

- In Claude Code, the `aep:adversary` agent is one seat.
- A seat on another family runs in a host that serves that family, for example Codex for a GPT
  model, reading `references/adversary.md` and the brief file directly.
- **With one family available, the panel is one reviewer.** Say so in the report — a panel of one is
  the ordinary adversary step, not a cross-family review.

Each seat writes tests only, as the adversary procedure says. Seats attacking one worktree at once
must be told distinct test-file names in their briefs, so two seats never edit one file.

## Record each seat

Each seat's report is its own `review-result`, created with `new --from` from the report as it
returned, fenced findings block included, and related to the story with `reviews:`. One record per
seat, written one at a time: the store takes one writer. Record every finding's outcome with
`--kind review_outcome` as the routing table in [wave.md](wave.md) says.

## Synthesise, then route

After every seat has returned, sort each finding into one of four buckets, and mark each with the
families that raised it:

| Bucket | Put a finding here when |
|---|---|
| **act on** | it is a real defect in correctness, security or maintainability for what the story asked, backed by a failing case |
| **consider** | the point holds, and whether it is worth fixing now is a trade-off the operator should see |
| **noted** | it is true and not actionable for this unit: context-dependent, low impact, or premature |
| **dismissed** | it is wrong or missing context; one line says why |

Two findings worded differently at one `file:line` with one verdict are one finding raised by two
families: merge them and list both. Then route the unit on the **act on** bucket exactly as a single
adversary's findings are routed in [wave.md](wave.md) § *Route the result*.

The report adds one table, the agreement map: per finding, which families raised it, and every
place one family said the opposite of another.

---

The panel, the consensus weighting and the four buckets are adapted from
`pstack/skills/interrogate` in `github.com/cursor/plugins` (MIT).
