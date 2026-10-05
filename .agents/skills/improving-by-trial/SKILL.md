---
name: improving-by-trial
description: Improve this repository's plugins by trial — run fresh, isolated agents that get only a user's sentence and this repository's link, collect where they got stuck, fix what belongs here, file what belongs elsewhere, and repeat until the trial passes. Use when asked to trial, dogfood, test the onboarding or a plugin end to end, run a fresh agent against the plugins, or iterate on trial findings before a release.
---

# Improving the plugins by trial

A trial is a fresh agent with nothing but a user's sentence and a link. It shows what the skills
fail to say. The gate cannot show that: every check passes while an agent still guesses.

## 1. Build a sandbox from this checkout

```console
task trial:sandbox NAME=<name>
```

This creates `/var/tmp/b10x-trials-$USER/<name>/` (the Taskfile's `TRIALS`) with its own `home/`, a `b10x` built from this
checkout, and an `env` file. The `env` file sets `HOME` to the sandbox and `B10X_MARKETPLACE` to
this checkout, so `b10x init` installs the plugins as they are in the working tree, before any
release. It also unsets `ANTHROPIC_API_KEY`, `TMPDIR` and `TMPPREFIX`. A defined trial
(`task trial:run TRIAL=<name>`, § 2) builds its own sandbox and fixture; for an ad-hoc one, put the
fixture it needs (a small service, a `TODO.md`, an OpenAPI document) under `work/`, and commit it
there with `git` when the trial needs history or a remote.

Setup refuses an existing sandbox: it may hold leased trees or the only recovery copy of trial
work. Use a fresh `NAME` for a rerun, for example `task trial:run TRIAL=ess-tutorial NAME=ess-tutorial-rerun`,
or complete the cleanup procedure below before reusing its name.

To trial the released version instead, remove `home/.local/bin/b10x` and unset `B10X_MARKETPLACE`
in `env`; the agent then follows `SETUP.md` from the release.

## 2. Run it headless, never as a sub-agent

The round's trials are defined in `trials/`, one directory each:

| file | holds |
|---|---|
| `trials/<name>/trial.yaml` | `name`, `kind`, `prompt`, optional `dir` (the run's subdirectory of `work/`), `fixture` (a directory beside it), `remote` (a bare `origin` in the sandbox), `seeded`, `setup` (shell lines run from the sandbox root with its `env` before the run), `measures`, and `outputs` (name → path below the run's directory) |
| `trials/<name>/fixture/` | the small service or backlog the trial starts from |
| `trials/baseline.json` | the measures of the last accepted run of each trial |

`task check` validates every definition. Run one by name:

```console
task trial:run TRIAL=<name>
```

This builds a fresh sandbox (`trial:sandbox`), copies the fixture into `work/<dir>` and commits it
there (`agentplugins-check trial-prepare`), runs the definition's `setup` (installing the plugins
under test with `b10x init … --out plan.json` and `b10x setup apply --plan plan.json --yes`, or
seeding an older release), then runs the prompt. An ad-hoc trial still runs in an existing sandbox:

```console
task trial:run NAME=<name> PROMPT='<the user sentence>' [DIR=<subdirectory of work>] [SEEDED=true]
```

The task copies the operator's credentials in (mode 600), runs `claude -p` with the sandbox's
`env`, `--strict-mcp-config` (no MCP server, including the account's claude.ai connectors) and
stream-json output into `run.jsonl`, and deletes the credentials when it finishes. The sandbox
root is passed as `--add-dir`, so managed worktrees below its isolated home are accessible as
working directories without granting access to the operator's home.
The sandbox
`PATH` has `cargo` and `go`, and `go` is an allowed tool, so a trial can build and test an
implementation.

**Never run a trial as a sub-agent of the working session.** A sub-agent gets the session's own
agents and plugins whatever `HOME` its shell uses. In trial 3 the aep critics that ran were this
machine's, not the plugin's under test.

The prompt is what a user would type, plus the link when the trial is about finding the
repository. Ask the agent to end by listing the skills and agents it used, quoting any text that
confused it, and pasting the final command output verbatim. Give it no other hints.

## 3. Only an isolated run counts

`trial:run` checks isolation before it measures:

```console
agentplugins-check trial-isolation <sandbox>/run.jsonl --sandbox <sandbox> --version <version>
```

It fails when a plugin loaded from outside the sandbox home and the checkout, when a plugin is not
in the sandbox's own registry as it was before the run, when a plugin is not at the version under
test, or when the run used an agent or skill from a plugin the sandbox did not load. One that was
only offered (claude.ai account skills reach sub-agent sessions) is printed as a note. A failing run
is discarded, not interpreted.

**A sandbox under the home directory is not isolated.** Claude Code reads `CLAUDE.md`,
`.claude/CLAUDE.md` and `.claude/settings*.json` in every directory above its working directory. In
trial 3, sandboxes under `~/.cache` loaded the operator's `~/.claude/CLAUDE.md` (one agent followed
its commit rules) and a `.claude/settings.local.json` an earlier trial left in `~/.cache`, which
enabled `aep@b10x`. `trial:sandbox` refuses to build below any of those files; that is why `TRIALS`
is outside `$HOME`. Neither leak shows in the `init` event, so the placement is the only guard. An upgrade trial seeds older plugins on
purpose (`seeded: true` in its definition, or `SEEDED=true`), so plugins must match what the sandbox
was seeded with; a registry that did not exist before the run is read after it.

## 4. Read the run

`trial:run` ends with the numbers, one line each:

```console
agentplugins-check trial-report <sandbox>/run.jsonl --trial <name> [--baseline trials/baseline.json] [--write-baseline]
```

| measure | read from |
|---|---|
| `tool_calls` | `tool_use` blocks in the run |
| `validate` | the last `ess specify validate` the run ran: `valid`, not valid, or not run |
| `synthesis` | `N scenario(s) … M refusal(s)` in the last `ess verify conform synthesize` output |
| `unmapped` | `UNMAPPED:` markers in the YAML files the run wrote, read from disk, not from its prose |
| `outputs` | which of the definition's `outputs` exist (a directory counts when it is not empty) |
| `go_test` | historical Go trials: passed, failed and skipped tests of the last `go test` (`-v` or `-json`); a package that does not build counts as a failure |
| `cargo_test` | current Rust trials: passed, failed and ignored tests from the last `cargo test`; the implementation also reports executed conformance scenarios |

A trial reports the measures its definition lists; without `--trial`, an ad-hoc run gets every
measure but `outputs`. With `--baseline` it exits 1 when a measure got worse than the trial's entry:
validate stops passing, refusals or `go test` failures go up, a synthesis or `go test` that ran no
longer runs, an output the baseline had is missing, or tool calls rise by more than 50%. Other
changes print and pass. `--write-baseline` records the run as the trial's entry; do that for an
isolated run the round accepts, and commit `trials/baseline.json` with the fixes it led to.

The numbers say what happened, not why. From `run.jsonl`, also collect:

| collect | how |
|---|---|
| stuck points | every refusal and error in `tool_result`s, verbatim |
| guesses | what the final report says it inferred or could not find |
| waste | calls repeated, files read twice, commands that failed and were retried unchanged |
| the outcome | the verbatim `validate` / `generate` / plan output it pasted |

A zero process exit or a `success` result can still say the agent is waiting for background work.
Confirm that the requested workflow actually finished. If necessary, resume the same isolated
session after its task notification, preserve both transcripts and repeat the isolation check;
do not accept a waiting message or permissive metric summary as completion.

Before writing a finding into a skill, reproduce each claimed behaviour with the released CLI. A
trial agent's explanation of a refusal is a hypothesis.

## 5. Triage every finding to one owner

| the cause | where the fix goes |
|---|---|
| a skill, agent, `SETUP.md`, the catalog or the `b10x` CLI | here: fix it on a branch, with a test for CLI behaviour |
| a product CLI or language (`ess`, `aep`, `worktree`) | an issue in that repository, created through the bot (`AGENTS.md`); the skill here documents the workaround until it is fixed |
| both | both: the workaround here, the issue there, each naming the other |

The issue body has the trial name, the exact command and output, the expected behaviour and the
workaround the skill now documents, and ends with the line *Found by an agentplugins trial*. The
bot creates it with the label `trial-finding` (`"labels": ["trial-finding"]` in the request). The
label is the ledger:

```console
gh search issues --owner beyond10x --label trial-finding --state open     # still broken
gh search issues --owner beyond10x --label trial-finding --state closed   # fixed: released yet?
```

Check both lists before each round. Delete a workaround from the skills once its issue is fixed
and the fix is in a release, not when the issue closes.

## 6. Fix, gate, re-run

1. Fix on a managed worktree branch.
2. Run `task check`, and `agentplugins-check tools` when a CLI command or the ESS example changed.
3. Rebuild the sandbox (`task trial:sandbox`) and re-run the trial that found the problem. It
   passes when the finding does not recur. New findings go back to step 5.
4. Release when every trial of the round passes.

Each round runs every trial in `trials/`: 4 ESS trials (`ess-new`, a new specification;
`ess-retrofit`, an existing service; `ess-pipeline`, generation plus a synthesized suite;
`ess-full-package`, every output plus a Rust implementation held to the suite), the current
`ess-tutorial`, and `aep-backlog`, `aep-tutorial`, `worktree-onboarding` and `upgrade-seeded`.
New executable fixtures use Rust. Change the domains and fixtures each
round so the agents cannot copy the previous answer from the skills; a changed trial starts a new
baseline entry.

### Every product release is re-verified

`verified.json` names, per tracked CLI, the release the skills were last
verified against. The daily `agentplugins-check tools` run fails with one line per CLI whose newest
release is newer. Then:

1. Run `agentplugins-check tools` and fix every command it reports.
2. Run the affected product's isolated trials against the new release. An ESS update includes
   `ess-full-package` and `ess-tutorial`; an AEP update includes `aep-backlog` and `aep-tutorial`.
   A complete resource refresh runs the whole round. Preserve historical baseline entries and
   record changed Rust trials under their actual measurement keys.
3. Set the CLI to the new release in `verified.json` in the same pull request.

## 7. Clean up

`trial:run` deletes the credentials it copied. When the round is released, check that no
credentials are left in any sandbox. Retain its run logs and meaningful generated work. Inspect
each sandbox's own Worktree registry and Git linked trees, with that sandbox's environment; never
substitute the operator's registry. Follow the Worktree skill to archive unpublished work, end
each owner's leases, finish trees and apply GC only to exact reviewed IDs. Adopt legacy linked
trees through the CLI before retiring them. Copy recovery archives outside the sandbox and verify
them before deleting their original container.

Only after every linked tree is retired and recovery is retained may the sandbox itself be
removed. AEP's read-only protocol snapshots may require making that exact sandbox writable first:

```console
ls /var/tmp/b10x-trials-$USER/*/home/.claude/.credentials.json
chmod -R u+w /var/tmp/b10x-trials-$USER/<name>
rm -rf /var/tmp/b10x-trials-$USER/<name>
```
