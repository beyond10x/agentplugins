---
name: wave
description: Propose the next wave of the AEP wave plan and stop for approval; /aep:wave or when asked to propose or start the next wave. Hands off to aep:implementing in wave mode.
argument-hint: "[story-id...]"
---

# Start a wave

Load `aep:implementing` and run it in **wave** mode: read its
[references/wave.md](../implementing/references/wave.md) in full before acting.

- The wave: the story ids in `$ARGUMENTS`, which the operator gives as the wave; with none, the next
  undelivered wave of the repository's wave plan. With no wave plan, derive one from the accepted
  stories with `aep plan artifact waves --status <accepted>` and propose it with its first wave, as
  the reference says.
- Never a story from the backlog, and never accept one: a draft filed from a finding enters no wave
  until the operator accepts it and every planned wave is delivered.
- Do stage 1 only: check and scope the wave's stories, write the wave page, print the stage-1
  proposal with the skill version line from `aep:implementing`, and stop for the operator's approval.
- Stage 2 starts only on that approval, exactly as the reference describes.
- Move no artifact, and relay every refusal from `aep` or the gate unedited.
