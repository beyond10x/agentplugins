# Beyond10x Agent Plugins

Curated marketplace identity: `b10x`.

## Point your agent here

Tell Claude Code or Codex:

> Set up Beyond10x: follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md

The agent installs the `b10x` binary, shows what is installed now — plugins on both hosts, the
binaries they drive, earlier installs under retired names — asks which products you want, lists
every change, and applies it only after you confirm. Run it again at any time to upgrade; the `b10x`
plugin's session-start check says when a plugin and its binary have drifted apart.

| product | plugins | binary |
|---|---|---|
| `aep` | `aep-plan` (planning, decomposition, plan review, reverse engineering), `aep-drive` (wave coordination, story scoping, implementation, adversarial review) | `aep`; `metaharness` for `aep-drive:drive` |
| `ess` | `ess` (specify, retrofit, validate, project, conformance), from [`beyond10x/ess`](https://github.com/beyond10x/ess) | `ess`, at the plugin's version |
| `worktree` | `worktree` (managed Git worktrees, leases, recovery proof, cleanup), from [`beyond10x/worktree`](https://github.com/beyond10x/worktree) | `worktree`, at the plugin's version |
| `connectors` (optional) | `connectors` (provider setup, diagnostics, governed invocation) | not managed |

Every install also gets `b10x`: the `setup` skill that drives the above, the `guide` router and the
portable `plugin-creator`.

## How it stays current

- **This repository names no version of anything it points at.** `ess` and `worktree` are
  `git-subdir` entries with a repository and a path only; the host installs whatever that product's
  default branch serves, which is the product's own release. `catalog.json` lists products, plugins,
  binaries and retired names, and no versions.
- **Versions are resolved when setup runs:** a plugin's version from the marketplace, a binary bound
  to a plugin at that plugin's version, a binary bound to `latest` at its newest release.
- **`agentplugins-check`** refuses a pinned remote entry and a catalog that disagrees with either
  marketplace file; **`agentplugins-check remote`** (every pull request, `main` push and daily)
  refuses a product whose default branch serves a plugin version it never released.
- **On the machine**, `b10x check` runs at session start and prints one line per drift.

## The `b10x` binary

| command | does |
|---|---|
| `b10x setup plan [--products aep,ess,…] [--host claude\|codex\|all] [--json] [--out <file>]` | read both hosts, `PATH` and project settings; print findings and exact actions; write nothing |
| `b10x setup apply --plan <file> --yes` | refuse a stale plan, snapshot every file it changes, run the actions, check that the result converged |
| `b10x setup undo [<snapshot>]` | restore the files the newest (or named) snapshot holds |
| `b10x setup guide` | print the setup instructions an agent follows |
| `b10x check` | the session-start drift check; offline, prints only problems |
| `b10x install <binary> [--tag <tag>]` | install one catalog binary from its checksummed release archive, or with `cargo install --git --tag` |

`B10X_MARKETPLACE=<local checkout>` makes setup register and read that checkout instead of this
repository, for testing an unpublished marketplace.

## Manual install

Claude Code: `/plugin marketplace add beyond10x/agentplugins`, then `/plugin install <plugin>@b10x`.
Codex: `codex plugin marketplace add beyond10x/agentplugins`, then `codex plugin add <plugin>@b10x`.
The binaries are yours to match; [`website/docs/install.md`](website/docs/install.md) has the
commands. Setup does all of this and checks it.

## Repository

Codex marketplace metadata lives at `.agents/plugins/marketplace.json`; Claude plugin marketplace
metadata lives at `.claude-plugin/marketplace.json`. Each plugin this repository carries owns its
manifest and only the skills or agents in its stated scope. `b10x` is the front door, not a
catch-all: it routes a task to the smallest specialist and does not copy the specialists'
instructions.

Run `task check` before publishing. The gate fails on missing focused content, mismatched plugin
names, a marketplace identity other than `b10x`, a pinned remote entry, a catalog that disagrees
with the marketplaces, or plugin versions that disagree with the workspace release. Run
`task site-build` for the public documentation under `website/`.

The adopter guide is published at <https://beyond10x.github.io/agentplugins/>. This repository
contains no credential or bot-token delivery machinery; release mutations are performed through
the private organization tooling outside this tree.

## Evals

Eight cases under [`evals/`](evals/) — one `eval-case/1` each, judged by a `trace-spec/1` document,
run by the `aep` CLI — name 7 of this repository's 10 agents and 4 of its 8 skills in their
`subject:` fields. Those `subject:` fields are the source of truth for eval coverage: every
coverage number here and in [`evals/README.md`](evals/README.md) is counted from
`evals/*/case.yaml`, never from a prose table. Covered: the agents `aep-drive:adversary`, `aep-drive:story-scoper`,
`aep-plan:decomposer`, `aep-plan:plan-critic-acceptance`, `aep-plan:plan-critic-design`,
`aep-plan:plan-critic-parallel-safety` and `aep-plan:plan-critic-scope`, and the skills
`aep-drive:drive`, `aep-drive:wave`, `aep-plan:planning` and `connectors:connectors`. Not
covered: the agents `aep-drive:implementor`, `aep-plan:plan-reviewer` and
`aep-plan:reverse-engineer`, and the skills `aep-plan:story-migration`, `b10x:setup`, `b10x:guide` and
`b10x:plugin-creator`. A change that breaks a covered charter
turns a row red instead of being noticed by a reader; a change to an uncovered one does not.

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
$ METAHARNESS_LIVE=1 metaharness aep drive eval run --corpus evals --workflow adp/default \
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
