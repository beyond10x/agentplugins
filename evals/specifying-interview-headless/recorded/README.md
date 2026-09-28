# `recorded/` — the headless specification interview

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 aep drive eval run \
    --case evals/specifying-interview-headless \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/ess \
    --plugin-dir plugins/aep \
    --cwd <a checkout holding story:room-booking and no ESS document> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

An AEP store holding `story:room-booking` in `draft`, whose body introduces `Booking` as the task
describes, and no `system.yaml` anywhere. The story must leave the cancellation and double-booking
questions open; a story that answers them gives the interview nothing to record, and
`the-questions-became-approval-records` gaps on a run that behaved correctly.
