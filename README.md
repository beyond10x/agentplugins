# Beyond10x Agent Plugins

Curated marketplace identity: `beyond10x`.

The repository deliberately contains five focused plugins:

- `beyond10x`: marketplace navigation, public resource discovery, and portable plugin creation.
- `aep-plan`: governed planning, decomposition, plan review, and reverse engineering.
- `aep-drive`: wave coordination, story scoping, implementation, and adversarial review.
- `ess-specify`: ESS specification, validation and deterministic schema/OpenAPI projection guidance.
- `workspace-hygiene`: safe creation, leases, publication checks, and cleanup for Git worktrees.

`beyond10x` is the front door, not a catch-all. It routes a task to the smallest specialist and
keeps plugin-creation workflows portable by making shared skills the canonical implementation for
Codex and Claude Code. It does not copy or replace the specialists' instructions.

## Install

`aep-plan` and `aep-drive` drive the `aep` CLI; `ess-specify` drives the `ess` CLI. Install the
verified Linux or macOS archives for [AEP `0.51.0`](https://github.com/beyond10x/aep/releases/tag/0.51.0)
and [ESS `0.11.1`](https://github.com/beyond10x/ess/releases/tag/0.11.1) first, then check them with
`aep --version` and `ess --version`. The complete download and checksum commands are in
[`website/docs/install.md`](website/docs/install.md).

Paste this pinned block into a Claude Code session:

```text
/plugin marketplace add https://github.com/beyond10x/agentplugins.git#0.7.0
/plugin install aep-plan@beyond10x
/plugin install aep-drive@beyond10x
/plugin install ess-specify@beyond10x
/reload-plugins
```

Add `/plugin install beyond10x@beyond10x` for the front door and
`/plugin install workspace-hygiene@beyond10x` for managed worktrees. Codex offers the same plugins
from the same repository, following `.agents/plugins/marketplace.json`; its exact non-interactive
CLI bootstrap and upgrade commands are in [`website/docs/install.md`](website/docs/install.md).

Codex marketplace metadata lives at `.agents/plugins/marketplace.json`; Claude plugin marketplace
metadata lives at `.claude-plugin/marketplace.json`. Each plugin owns its own manifest and only the
skills or agents in its stated scope.

Run `task check` before publishing. The gate fails on missing focused content, mismatched plugin
names, a marketplace identity other than `beyond10x`, or plugin versions that disagree with the
workspace release. Run `task site-build` for the public documentation under `website/`.

The adopter guide is published at <https://beyond10x.github.io/agentplugins/>. This repository
contains no credential or bot-token delivery machinery; release mutations are performed through
the private organization tooling outside this tree.

## Evals

Every agent and user-facing skill has an eval case beside it under [`evals/`](evals/): one
`eval-case/1` per subject, judged by a `trace-spec/1` document, run by the `aep` CLI. A change that
breaks a charter turns a row red instead of being noticed by a reader.

Free, offline, and part of `task check`:

```console
$ task evals
valid: 8 eval case(s), 1 recorded transcript(s) replayed
```

It validates every case, resolves every `subject:` to an agent or a skill that exists here, and
replays whatever transcripts are recorded with `aep drive eval run --stream`, which spends nothing. An
empty `recorded/` and a machine with no `aep` on `PATH` are both printed notices, never a red gate.

Live, which costs money:

```console
$ METAHARNESS_LIVE=1 aep drive eval run --corpus evals --workflow adp/default \
    --arm plugin --harness claude --plugin-dir plugins/aep-plan \
    --cwd <a working tree> --budget-usd 20 --assume-usd-per-run 5 \
    --observed-at <date> --redact --out <a directory outside this repository>
```

Without `METAHARNESS_LIVE=1` the runner accepts the corpus and refuses to spawn, by name:

```console
$ aep drive eval run --corpus evals --workflow adp/default --arm plugin --harness claude \
    --out eval-out --observed-at 2026-09-03
error: eval-out — 1 refusal(s):
  EVAL-RUN-002 a spawn costs money and `METAHARNESS_LIVE=1` is not in this environment. Set it
  deliberately, or pass `--stream FILE` to ingest a run that already happened, which spends nothing
```

### What a full live run costs

| | |
|---|---|
| cases in the corpus | **8** |
| per-case cap | **$5** — `story:plugin-eval-cases`, the operator's default |
| one full run, one arm, one harness | **$40** |
| `EVAL_BUDGET_USD` default | **$20** — `story:eval-ci-gates`, the operator's default |

**So a full sweep does not fit its own default budget, and that is the intended behaviour rather
than an oversight.** `.github/workflows/eval.yml` computes `cases × $5` before it installs a tool,
and refuses with those four numbers in the check summary when the product exceeds
`EVAL_BUDGET_USD`. What fits inside $20 is a diff-scoped run of up to four cases, which is what a
pull request touching one agent or one skill actually selects. Running the whole corpus is a
deliberate act: raise the repository variable, or dispatch one case at a time.

The cap is a **cap, not an estimate** — no recorded run has priced this corpus yet, so nothing here
claims a full sweep will cost $40 rather than refusing above it. `--assume-usd-per-run` is what the
runner charges a run whose stream states no cost, and it is set to the per-case cap so the runner's
own pre-spawn check is made against the budgeted number and not against its optimistic default.

### CI

`ci.yml` runs the free half on every pull request and it is what blocks a merge. `eval.yml` runs the
live arm only with the `run-eval` label or a manual dispatch, only for the cases whose subject the
diff touched, under the budget above, with the organization bot's credential and never a personal
key. It informs; it does not gate.

<!-- b10x-docs:start -->
## Documentation

[Agent Plugins documentation](https://beyond10x.github.io/docs/agentplugins/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
