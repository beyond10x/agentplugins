# AGENTS.md — agentplugins

## Serves

- **O2 — decisions as data, with evidence.** Publishes the curated AEP, ADP and ESS instruction
  surfaces used to plan, develop and validate governed work.
- **O3 — any harness, observed and compared.** Keeps those instruction surfaces portable across
  supported agent harnesses.

## Invariants

- Marketplace identity is `beyond10x` in every marketplace format.
- Keep exactly the focused plugin boundaries described in `README.md`. The `beyond10x` front door
  may route to specialists and teach portable plugin authoring, but it must not absorb their
  workflows or become a mixed catch-all.
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

The adopter-facing Docusaurus site lives under `website/` and is published at
<https://beyond10x.github.io/agentplugins/>. A website change must also pass `task site-build`.
The networked site build stays outside the offline Rust gate.

Commit and push through the organization bot tooling owned by private Atlas. This repository never
carries credential, token-minting or bot-authenticated git wrappers.

## Releases

Bare annotated tags are releases. Before tagging, `CHANGELOG.md`, the workspace version, and every
plugin manifest version must agree. The release workflow reruns `task check`, builds the public
site, verifies that agreement, and only then publishes the GitHub release.

<!-- b10x-docs-operations:start -->
## Public documentation operations

This repository owns the public source and presentation allowlist in `b10x.docs.yaml`; the unified [beyond10x Website](https://beyond10x.github.io/docs/agentplugins/) passively collects those declared files from the exact commit in `website/sources.lock.json`. Atlas owns discovery grouping/order; Website and Docs System own rendering, shared components, search, and feeds. Do not add a standalone docs deployer or put App credentials in this public repository. If Atlas catalogs a former Pages workflow, that file remains repository-owned validation: preserve its bespoke checks while keeping exact read-only permissions, an unconditional pull-request trigger, and no deployment primitives. Project Pages at `/agentplugins/` is only the generated redirect façade in `.github/workflows/b10x-docs-pages.yml`.

From the complete organization workspace, verify the contract with a clean Atlas checkout at the current remote `main`. Set `B10X_ATLAS_CHECKOUT` to a managed Atlas worktree when the primary checkout is dirty or stale; never infer command availability from the primary alone.

```bash
atlas_checkout="${B10X_ATLAS_CHECKOUT:-atlas}"
atlas_head="$(git -C "$atlas_checkout" rev-parse HEAD)"
atlas_main="$(git -C "$atlas_checkout" ls-remote origin refs/heads/main | awk '{print $1}')"
test -z "$(git -C "$atlas_checkout" status --porcelain)"
test "$atlas_head" = "$atlas_main"
cargo run --manifest-path "$atlas_checkout/Cargo.toml" --locked -q -- \
  --store "$atlas_checkout/catalog/store" docs reconcile --workspace . --check
```

Keep internal plans, stories, ADRs, decisions, worklogs, security material, and research out of the public allowlist unless a repository authority explicitly declares them public.
<!-- b10x-docs-operations:end -->
