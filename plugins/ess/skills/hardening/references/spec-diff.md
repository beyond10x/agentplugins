# Spec diff in the gate: classifying changes

Technique 7. The gate compares the specification with the one at the last release tag, and fails
on a breaking change nobody acknowledged.

## The diff

`ess verify diff` compares two specification directories:

```console
ess verify diff --from <release-spec-dir> --to <spec-dir> --format json
```

`--from` and `--to` are paths, so materialise the tagged revision first, for example
`git archive <tag> <spec-dir> | tar -x -C <scratch>`. The `ess-diff` document lists `changes`, each
with a stable `id` (`type/<name>/variant-removed/<variant>`), a `relation` (`expanded`, `narrowed`
or `changed`) and the change itself.

`ess` names each change and its relation; **it does not decide whether a change is breaking.** Until
it does, classify with the rule below.

## The rule

**Additive** (not breaking):

- an added item — a type, entity, field, command, outcome, event, view, actor
- an added enum variant
- an added lifecycle transition
- an added grant (an actor `may` one more command)
- an added `accepts` or `publishes` entry
- a wording change — `summary`, `naming.display`, descriptions

**Breaking:** everything else, and **anything whose `relation` is `narrowed`**, whatever its kind. A
removal, a rename, a changed guard, a changed invariant, a changed type, a changed `wire` name — all
breaking. When a change's kind is not on the additive list, it is breaking; the list grows by
decision, not by argument.

## Acknowledgement

A breaking change passes the gate only when acknowledged, and an acknowledgement is a **committed
file keyed to the release tag** — for example `spec-acknowledgements/<tag>.yaml` — listing each
acknowledged change by its `id`, with one line saying why it is acceptable.

The gate:

1. materialises the spec at the last release tag;
2. runs the diff;
3. classifies each change by the rule;
4. fails on any breaking change whose `id` is not in the acknowledgement file for that tag, naming
   the `id`;
5. fails on any acknowledged `id` that no longer appears in the diff — a stale acknowledgement
   hides the next change with the same name.

A new release tag starts an empty acknowledgement file; the previous tag's never carries over.

**Plant a defect before trusting it:** on a branch, remove one enum variant. The gate must fail and
name the `variant-removed` id; add that id to the acknowledgement file and it must pass.
