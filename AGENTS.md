# AGENTS.md — agentplugins

This repository serves Atlas O2 and O3 by publishing the curated instruction surface used to plan,
develop, and validate work across harnesses.

## Invariants

- Marketplace identity is `beyond10x` in every marketplace format.
- Keep exactly the focused plugin boundaries described in `README.md`; do not recreate a mixed
  catch-all plugin.
- The AEP canonical command in instructions is `aep`. `protocol` is compatibility only and must not
  become the authored spelling again.
- Do not mention or depend on retired plugin references, former marketplace identities, or the
  historical source-repository name.
- Plugin folder names and manifest names are identical.
- Changes to a `SKILL.md` must pass the skill validator; plugin changes must pass the plugin
  validator and `task check`.
- Anything executable in this repository is Rust.

## Gate

```console
task check
```

Commit and push through the organization bot tooling from a sibling repository until this
repository has its own generated bot wrappers.
