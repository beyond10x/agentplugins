# Writing what an agent reads

A skill, an agent procedure and an `AGENTS.md` are read by a model mid-task, cold. The goal is that
the agent takes the same path every run. Five rules do most of that. Each example below is a line
from a skill in this repository.

## 1. A description names each trigger once

The `description:` is read on every turn to decide whether to load the skill. Every word costs
context on every turn, so it states what the skill does and then each **distinct** case that should
load it — once. Synonyms for one case are one trigger written twice.

- Before: "Use when the user asks to scaffold a plugin, create a plugin, make a new plugin, or start
  a plugin." (four phrasings of one case)
- After: "Use when the user asks to scaffold a plugin, add plugin capabilities or marketplace
  metadata, make a Claude Code plugin work in Codex, make a Codex plugin work in Claude Code, or
  audit a plugin for cross-harness portability." (`authoring-plugins`: five distinct cases, one
  phrasing each.)

## 2. Every step ends on a checkable completion criterion

An agent that cannot tell done from not-done stops early, pulled by the steps it can see ahead.
State the condition that ends the step, as something the agent can check, and make it demanding
where thoroughness matters.

- Before: "Build a feedback loop for the bug."
- After: "This step is done when you can name one command, and you have already run it and shown its
  output, that is red-capable, deterministic, fast and unattended." (`aep:diagnosing` § 1.)

## 3. Reference that only some branches need goes behind a pointer

Put in the file what every run needs. Move what only one branch needs into `references/` and point
at it with a line that says **when** to read it. The pointer's wording decides whether the agent
reaches it, so name the condition, not just the file.

- Before (what it would otherwise be): the full syntax listing inline in `ess:specifying`.
- After: "[references/syntax.md](references/syntax.md) shows every one of those sections in a small
  specification that validates; read it before writing the first command." (`ess:specifying`.)

## 4. State the target behaviour, not only the prohibition

A prohibition names the thing it forbids, and the name makes it more available to the model, not
less. Say what to do. Keep a prohibition only as a hard guardrail you cannot phrase positively, and
then put the positive target beside it.

- Before: "Do not infer that development work is ready merely because a planning request exists."
  (`b10x:routing`.)
- After: "Treat development work as ready only when its story is `active` in the store; a planning
  request routes to `aep:planning`."

## 5. Cut the sentence the model already obeys

A line that does not change behaviour against the model's default is load with no effect. Test each
sentence by asking what the agent would do without it. A weak word is the same failure: "be
careful" and "be thorough" are no-ops, and the fix is a checkable instruction.

- Before: "do not collide".
- After: "The brief declares the split by symbol and line range — `fn routes_of`,
  `bundle.rs:381-560`". A range is checkable; an instruction to be careful is not
  (`aep:implementing`, `references/branch-and-merge.md`).

## Before you hand off

Read the changed file top to bottom as the agent will: once, in order, mid-task. For each step,
point at its completion criterion. For each `Do not` and `Never`, point at the positive target
beside it or delete it. For each pointer, say which branch reaches it.

---

These rules are adapted from `skills/productivity/writing-for-agents` in
`github.com/mattpocock/skills` (MIT).
