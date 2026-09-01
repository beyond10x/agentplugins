# AGENTS.md — agentplugins

## Serves

- **O2 — decisions as data, with evidence.** Publishes the curated AEP, ADP and ESS instruction
  surfaces used to plan, develop and validate governed work.
- **O3 — any harness, observed and compared.** Keeps those instruction surfaces portable across
  supported agent harnesses.

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

Commit and push through the organization bot tooling owned by private Atlas. This repository never
carries credential, token-minting or bot-authenticated git wrappers.
