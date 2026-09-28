# `recorded/` — diagnosing, red loop first

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 aep drive eval run \
    --case evals/diagnosing-red-loop-first \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a checkout with the slugify defect and story:slugify-unicode> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A small crate or package with a `slugify` function under `src/` that drops non-ASCII letters, a
test suite under `tests/` that passes because it has no case for them, and an AEP store holding
`story:slugify-unicode` in `active`. The defect must be real; a tree where the function is already
correct gives `the-fix-was-made` nothing to observe.
