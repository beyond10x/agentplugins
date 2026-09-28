# `recorded/` — the adversary and a test that cannot fail

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 aep drive eval run \
    --case evals/adversary-tautological-test \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <the implementor's worktree, suite green, with the seeded test> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A Rust crate with `total(&[Line]) -> Result<Money, Error>` under `src/` that sums wrongly (for
example, drops the last line), a unit brief naming a scratch directory, and `tests/total.rs` whose
only assertion is `assert!(total(&lines).is_ok())`. The suite is green because of that test.
