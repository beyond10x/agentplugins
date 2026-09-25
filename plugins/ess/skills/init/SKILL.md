---
name: init
description: Start with ESS (Executable System Specifications) in this project — make sure the `ess` CLI is installed, explain what ESS does, and take the first step of a specification. Use when the user wants to start writing specs, asks what ESS is or how to adopt it, asks to set up or install ESS, points at the ESS repository, or when another ESS skill reports that `ess` is missing. Installs CLIs only after the user confirms the plan.
---

# Start with ESS

ESS is a typed model of a system — entities, commands, events, views and the component that runs
them — that validates, compiles, projects JSON Schema and OpenAPI, and synthesises conformance
suites. The `ess` CLI is the authority; these skills tell you how to drive it.

## 1. Have the CLI

Run `ess --version`. If it answers, go to step 2: `b10x:init` just installed it, or it was already there (`ess:upgrade` handles newer releases). If it is missing, install it with `b10x`:

```bash
b10x init ess --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: `cargo` when a Rust toolchain is on
`PATH`, otherwise a prebuilt, checksummed release archive. Ask the user which method they want if
both are possible, re-run with `--method cargo` or `--method prebuilt` to match, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first; its step 1 asks the user before it installs `b10x`.

## 2. Pick the work

| the task | skill | agent |
|---|---|---|
| write a new specification, add an entity, validate, compile or project one | `ess:specifying` | `ess:author` |
| give an existing codebase a specification it never had | `ess:retrofitting` | `ess:retrofitter` |
| raise or audit what a conformance suite actually tests | `ess:testing-conformance` | `ess:conformance` |

A skill that is not loaded yet in this session prints with `b10x skill ess:<skill>`.

## 3. Rules that hold in every ESS skill

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
