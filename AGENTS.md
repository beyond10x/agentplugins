# AGENTS.md — agentplugins

The `b10x` marketplace: every Beyond10x plugin, and the `b10x` CLI that installs them with their
CLIs. Serves O2 (decisions as data) and O3 (any harness).

## Map

| path | what |
|---|---|
| `plugins/<name>/` | every plugin: `b10x`, `aep`, `ess`, `worktree`, `connectors` ([structure](website/docs/structure.md)) |
| `.claude-plugin/marketplace.json`, `.agents/plugins/marketplace.json` | the two marketplace files |
| `catalog.json` | products, plugins, binaries, retired names — no versions |
| `crates/b10x/` | the setup CLI; `plan.rs` is pure and fixture-tested |
| `crates/agentplugins-check/` | the gate: marketplace, catalog, concept, retired names, CLI spellings, evals; `tools` checks skills against the newest CLI releases |
| `evals/` | eval corpus ([`evals/README.md`](evals/README.md)) |
| `website/` | public docs; must pass `task site-build` |
| `SETUP.md` | agent bootstrap, published as a release asset |
| `.agents/skills/improving-by-trial/` | how plugins are improved: isolated headless trials (`task trial:sandbox`, `task trial:run`), triage, fix, re-run |

## Rules

- Follow [`website/docs/structure.md`](website/docs/structure.md): rules R1–R8 decide plugin, skill,
  agent and doc placement and names. `task check` enforces them (`crates/agentplugins-check/src/concept.rs`).
- `b10x` (`init`, `upgrade`, `setup`) is the only supported way to change a user's installed plugins; it snapshots first.
- Instructions spell the grouped CLI verbs (`aep plan …`, `ess specify …`) and `aep`, never
  `protocol`. The gate refuses flat spellings outside `CHANGELOG.md`, `changes/`, `.engineering/`
  and `.github/workflows/`.
- Retired names appear only where the gate allows them (`CHANGELOG.md`, `changes/`,
  `.engineering/`, `catalog.json`, `crates/b10x/`, the checker's own table).
- Anything executable is Rust.
- Trial findings for another repository become an issue there, labelled `trial-finding` by the
  bot; the skill here documents the workaround until the fix is released ([`improving-by-trial`](.agents/skills/improving-by-trial/SKILL.md)).

## Gate

```console
task check          # fmt, clippy, tests, agentplugins-check
task site-build     # when website/ changes
cargo run --locked --bin agentplugins-check -- tools    # network: every spelled command exists in the newest aep, ess, worktree
```

## Planning

Non-trivial work gets an artifact in the AEP store first (`aep plan artifact list`, then create or
select one) and keeps it current through `aep plan artifact`. Never edit `.engineering/planning/` by
hand. `aep:planning` is the instruction surface for this.

## Publishing

Commit and push through `b10x-gates bot --repo . -- <git-command>` as `b10x-bot[bot]`; keep hooks.
No credential or token machinery lives in this repository. A release is a bare annotated tag on
`main` after `CHANGELOG.md`, the workspace version and every carried plugin manifest agree; the
release workflow reruns the gate and publishes the `b10x` archives, `SHA256SUMS` and `SETUP.md`,
with the version's `CHANGELOG.md` section as the release notes.
Source publication needs no Atlas checkout.

<!-- b10x-docs-operations:start -->
## Public documentation operations

This repository owns the public source and presentation allowlist in `b10x.docs.yaml`. The generated credential-free `.github/workflows/b10x-docs-bundle.yml` passively packages only those declared files for the exact successful `main` commit; it must never run repository code. The generated `.github/workflows/b10x-docs-check.yml` runs the publisher's per-source checks on every pull request and main push, with read-only contents and no credentials; it is deliberately separate from the shared gate, which runs on `pull_request_target` with a secret and never reads candidate source. Atlas selects the latest successful bundle with every other catalog source, and Website plus Docs System own rendering, shared components, search, and feeds. Do not add a standalone docs deployer or put App credentials in this public repository. If Atlas catalogs a former Pages workflow, that file remains repository-owned validation: preserve its bespoke checks while keeping exact read-only permissions, an unconditional pull-request trigger, and no deployment primitives. Project Pages at `/agentplugins/` is only the generated stable redirect façade in `.github/workflows/b10x-docs-pages.yml`; content-only publication never rebuilds it.

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

<!-- b10x-release-operations:start -->
## Release completion

An ordinary release completes after this repository's exact tag, required source checks,
published release and required artifacts are verified. A pushed tag with unfinished checks or
uploads is queued; report it as released only after those requirements succeed.

Atlas reconciliation and public documentation publication run asynchronously. Do not wait for
Atlas or Website, update Website source locks or bootstrap snapshots, promote consumer pins,
release shared docs tooling, or redeploy documentation façades as part of an ordinary source
release. Report documentation as pending unless its publication was actually verified. A background
documentation failure does not invalidate a successful source release.

Keep this repository's provenance, correctness, security, compatibility and artifact verification
requirements. Shared rendering, routing or delivery-control changes still require their relevant
integration gates. A release request does not authorize deployment or downstream releases.
Repositories without a release unit retain their existing publication policy. This completion
boundary supersedes older instructions that attach synchronous documentation ceremony to each
source release.
<!-- b10x-release-operations:end -->
