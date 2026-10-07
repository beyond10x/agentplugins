---
name: wave
description: Start an AEP implementation wave, as /aep:wave or when the operator asks an agent to propose or start the next wave. Hands off to aep:implementing in wave mode, which proposes the wave and stops for approval.
argument-hint: "[story-id...]"
---

# Start a wave

Load `aep:implementing` and run it in **wave** mode: read its
[references/wave.md](../implementing/references/wave.md) in full before acting.

- Candidates: the story ids in `$ARGUMENTS`; with none, the stories the store shows as accepted.
- Do stage 1 only: scope the candidates, write the wave page, print the stage-1 proposal with the
  skill version line from `aep:implementing`, and stop for the operator's approval.
- Stage 2 starts only on that approval, exactly as the reference describes.
- Move no artifact, and relay every refusal from `aep` or the gate unedited.
