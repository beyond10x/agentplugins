# `recorded/` — the claim verdict before merge

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/wave-claim-verdict \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a checkout with the wave page, the integration branch and unit/fast-list> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

An integration branch and a branch `unit/fast-list` whose suite is green and whose change does not
make `list` faster; an AEP store with `story:fast-list` in `active`, its acceptance claiming `list`
finishes in under 0.1 s on the fixture store, and one recorded adversary `review-result` with an
empty findings block; and a wave page naming the unit with a scratch directory in its brief.
