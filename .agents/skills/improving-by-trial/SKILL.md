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
release. It also unsets `ANTHROPIC_API_KEY`, `TMPDIR` and `TMPPREFIX`. Put the fixture the trial
needs (a small service, a `TODO.md`, an OpenAPI document) under `work/`, and commit it there with
`git` when the trial needs history or a remote.

To trial the released version instead, remove `home/.local/bin/b10x` and unset `B10X_MARKETPLACE`
in `env`; the agent then follows `SETUP.md` from the release.

## 2. Run it headless, never as a sub-agent

```console
task trial:run NAME=<name> PROMPT='<the user sentence>' [DIR=<subdirectory of work>]
```

The task copies the operator's credentials in (mode 600), runs `claude -p` with the sandbox's
`env`, `--strict-mcp-config` (no MCP server, including the account's claude.ai connectors) and
stream-json output into `run.jsonl`, and deletes the credentials when it finishes.

**Never run a trial as a sub-agent of the working session.** A sub-agent gets the session's own
agents and plugins whatever `HOME` its shell uses. In trial 3 the aep critics that ran were this
machine's, not the plugin's under test.

The prompt is what a user would type, plus the link when the trial is about finding the
repository. Ask the agent to end by listing the skills and agents it used, quoting any text that
confused it, and pasting the final command output verbatim. Give it no other hints.

## 3. Only an isolated run counts

`trial:run` ends with:

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
purpose: run it with `SEEDED=true`, so plugins must match what the sandbox was seeded with.

## 4. Read the run

From `run.jsonl`:

| collect | how |
|---|---|
| tool calls | count of `tool_use` blocks |
| stuck points | every refusal and error in `tool_result`s, verbatim |
| guesses | what the final report says it inferred or could not find |
| waste | calls repeated, files read twice, commands that failed and were retried unchanged |
| the outcome | the verbatim `validate` / `generate` / plan output it pasted |

Before writing a finding into a skill, reproduce each claimed behaviour with the released CLI. A
trial agent's explanation of a refusal is a hypothesis.

## 5. Triage every finding to one owner

| the cause | where the fix goes |
|---|---|
| a skill, agent, `SETUP.md`, the catalog or the `b10x` CLI | here: fix it on a branch, with a test for CLI behaviour |
| a product CLI or language (`ess`, `aep`, `worktree`) | an issue in that repository, created through the bot (`AGENTS.md`); the skill here documents the workaround until it is fixed |
| both | both: the workaround here, the issue there, each naming the other |

The issue body has the trial name, the exact command and output, the expected behaviour, the
workaround the skill now documents, and ends with the line *Found by an agentplugins trial*. That
line is the ledger: the bot may not label issues (403), so the list is a body search.

```console
gh search issues --owner beyond10x --state open --match body "Found by an agentplugins trial"
```

Check that list before each round, and delete a workaround from the skills once its issue is
fixed and released.

## 6. Fix, gate, re-run

1. Fix on a managed worktree branch.
2. Run `task check`, and `agentplugins-check tools` when a CLI command or the ESS example changed.
3. Rebuild the sandbox (`task trial:sandbox`) and re-run the trial that found the problem. It
   passes when the finding does not recur. New findings go back to step 5.
4. Release when every trial of the round passes.

Each round has 3 ESS trials (a new specification; a retrofit of an existing service; generate plus
a synthesized suite), and 1 trial each for aep planning, worktree, and onboarding or upgrade.
Change the domain each round so the agents cannot copy the previous answer from the skills.

## 7. Clean up

`trial:run` deletes the credentials it copied. When the round is released, check that no
credentials are left in any sandbox and remove the sandboxes:

```console
ls /var/tmp/b10x-trials-$USER/*/home/.claude/.credentials.json
rm -rf /var/tmp/b10x-trials-$USER/<name>
```
