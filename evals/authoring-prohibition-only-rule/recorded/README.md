# `recorded/` — a wording review of a prohibition-only rule

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/authoring-prohibition-only-rule \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/b10x \
    --cwd <a checkout holding the seeded plugins/demo/skills/tidying/SKILL.md> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A directory `plugins/demo/skills/tidying/` whose `SKILL.md` has a valid frontmatter, one step
without a completion criterion ("Tidy the module."), and one rule that is only a prohibition ("Do
not edit generated files."). A file with no defect gives `the-prohibition-was-reported` nothing to
find.
