# `recorded/` — the decomposer's expand, migrate, contract

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 aep drive eval run \
    --case evals/decomposer-expand-migrate-contract \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a workspace of four crates using Tenant, with the epic in its store> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A Cargo workspace of at least four crates that each use a `Tenant` type declared in one of them, and
an AEP store holding `epic:rename-tenant-to-account` in `draft` with no story decomposed from it.
The epic introduces no new noun, so guardrail 7 does not stop the decomposition first.
