# `recorded/` — the adversary panel with one family

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/adversary-panel-one-family \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a checkout with the wave page, the integration branch and unit/invoice-total> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

The `adversary-tautological-test` tree on a branch `unit/invoice-total`, an integration branch, an
AEP store holding its story in `active`, and a wave page naming the unit. The harness must serve one
model family only; a run under a harness with two would make `the-report-says-one-family` wrong for
a reason that is not about the plugin.
