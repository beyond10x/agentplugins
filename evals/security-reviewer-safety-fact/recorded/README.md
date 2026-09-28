# `recorded/` — the security reviewer's safety fact

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/security-reviewer-safety-fact \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <the implementor's worktree, suite green> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A Rust crate whose unit removes a guard from a cache-eviction path under `src/`, a green suite under
`tests/`, an AEP store holding `story:cache-evict-fast-path` whose `## Scope` section carries a
`Safety fact` line at step 2, and a unit brief naming a scratch directory.
