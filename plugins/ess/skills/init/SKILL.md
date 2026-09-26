---
name: init
description: Start with ESS (Executable System Specifications) in this project — make sure the `ess` CLI is installed, explain what ESS does, and take the first step of a specification. Use when the user wants to start writing specs, asks what ESS is or how to adopt it, asks to set up or install ESS, points at the ESS repository, or when another ESS skill reports that `ess` is missing. Installs CLIs only after the user confirms the plan.
---

# Start with ESS

ESS is a typed model of a system — entities, commands, events, views and the component that runs
them — that validates, compiles, projects JSON Schema and OpenAPI, and synthesises conformance
suites. The `ess` CLI is the authority; these skills tell you how to drive it.

## 1. Have the CLI

Run `ess --version`. If it answers, go to step 2: `b10x:init` just installed it, or it was already there (`ess:upgrade` handles newer releases). If it is missing, install it with `b10x`.
Plan for the host you run in (`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init ess --host claude --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: the prebuilt, checksummed release archive
by default, or `cargo` (a source build) when the user asks for it and a Rust toolchain is on `PATH`.
When both are possible, say which is planned and offer the other; re-run with `--method cargo` if
they choose it, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first; its step 1 asks the user before it installs `b10x`.

## 2. Read what is already here

Look for `system.yaml` or `ess-inputs.yaml` in the repository. When neither exists, go to step 3.

When one does, establish the repository's standing before offering any work, with two commands:

```bash
ess specify validate --path <directory holding it>
```

then the repository's own conformance command, read from its `AGENTS.md` or its task runner
(`Taskfile.yml`, `Makefile`, `package.json` scripts, the CI job that runs the suite). Run that
command as the repository spells it; do not assemble a runner invocation of your own.

Report both results as the tools printed them: the validation line (`<system> v<n> — <n> file(s),
valid`, or every refusal verbatim) and the suite's `passed`, `skipped` and `failed` counts with the
command and its exit status. No conformance command found: say so, and name where you looked.

When the specification is valid and the suite is green, recommend `ess:testing-conformance`: a green
suite nobody has broken is the next question, and that skill's first section is how to ask it. When
validation refuses or a scenario fails, that refusal or failure is the work; route it through step 3.

## 3. Pick the work

| the task | skill | agent |
|---|---|---|
| write a new specification, add an entity, validate, compile or project one | `ess:specifying` | `ess:author` |
| give an existing codebase a specification it never had | `ess:retrofitting` | `ess:retrofitter` |
| raise or audit what a conformance suite actually tests | `ess:testing-conformance` | `ess:conformance` |

A skill that is not loaded yet in this session prints with `b10x skill ess:<skill>`.

## 4. Rules that hold in every ESS skill

- The compiler's output is the answer. Relay every refusal verbatim; never edit generated output
  around one.
- A draft an agent writes is an import: mark what you could not read from a source with
  `UNMAPPED:` and name it in the report.
- Spell a command by its area: `ess specify|generate|verify|infra <verb>`.
- Finish with the repository's own gate and report the exact command and exit status.

## Next

- New specification: `ess:specifying` — its `references/syntax.md` shows every section in a spec
  that validates.
- Existing system: `ess:retrofitting`.
- Later, `ess:upgrade` checks for a newer `ess` and plugin.
