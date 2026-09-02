---
format: aep.planning-md/1
id: story:golden-path-crud-feature
kind: story
status: active
title: A golden-path page from feature idea to critiqued plan, with one worked CRUD feature
relations:
- decomposes: epic:second-adopter-feedback
revision: 3
---
# Story: A golden-path page from feature idea to critiqued plan, with one worked CRUD feature

## Outcome

An engineer who has installed the plugins can open one page, type the prompts it shows, and arrive
at an epic, its stories, a scope per story and a recorded critique, on a repository that already
exists — and can compare what they got with the example the page shows.

## Context

The second adopter's words were "some straight to the point instructions / golden path examples
could help too as I am still unsure if I even used them properly". The site has an intro, an install
page, a chooser and a trust page; the planning skill's worked example is three CLI commands; nothing
shows a Claude Code or Codex session. Derived from the epic this decomposes.

## Acceptance

- A new page `website/docs/golden-path.md`, in the sidebar after the install page, with these
  sections in order: prerequisites (one line, linking the install page); adopt an existing repository
  (`aep reverse init`, `aep reverse scan` — the literal prompt to type, and what the agent should
  answer); file the feature as an epic (literal prompt); decompose it (literal prompt that dispatches
  the decomposer, and what its report's fourth section means); scope the stories; run the critic
  panel; implement one story through the `adp` wave.
- The worked example is a CRUD feature that introduces a new entity with a many-to-one relation to an
  existing entity (for example a "commercial client" that belongs to exactly one "account"), and the
  page shows the decomposer naming that relation as `requires-stakeholder-input` and filing a
  `decision-blocker` for it, rather than drafting stories around it.
- Every prompt on the page is one the reader can paste verbatim; every command output shown was
  produced by running it, not written by hand, and the page says which version of `aep` produced it.
- The front-door plugin's resource list names the page.
- `task check` and `task site-build` pass.

## Out of Scope

A video; a page per harness (the prompts are the same; where Codex differs, one callout says how).

## Open Questions

None.
