---
name: init
description: Start with AEP in this project — make sure the `aep` CLI is available and take the first step. AEP is governed planning in a repository-local artifact store, and delivery of accepted work in reviewed waves. Use when the user wants to plan or deliver governed work, asks what AEP is or how to adopt it, asks to set up or install AEP, or when an AEP skill reports that `aep` is missing.
---

# Start with AEP

## 1. Have the CLI

Run `aep --version`. If it answers, go to step 2: `b10x:init` just installed it, or it was already there (`aep:upgrade` handles newer releases). If it is missing, install it with `b10x`:

```bash
b10x init aep --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: `cargo` when a Rust toolchain is on
`PATH`, otherwise a prebuilt, checksummed release archive. Ask the user which method they want if
both are possible, re-run with `--method cargo` or `--method prebuilt` to match, show the plan, and
after they confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## 2. First step here

See whether this repository already has a planning store:

```bash
aep plan artifact list
```

A store answers with its artifacts. No store: `aep:planning` § 5 (*Starting from a repository that
has no store*) says how a first one is created; an existing backlog in markdown moves in with
`aep:migrating`.

## 3. Pick the work

| the task | skill |
|---|---|
| plan, decompose, review or reverse-engineer work | `aep:planning` |
| move an existing backlog into the store | `aep:migrating` |
| implement accepted stories (wave, or one governed drive run) | `aep:implementing` |

A skill that is not loaded yet in this session prints with `b10x skill aep:<skill>`.

## Next

- Plan: `aep:planning`. Deliver: `aep:implementing`.
- Drive mode also needs `metaharness`; `b10x init aep` offers it.
- Later, `aep:upgrade` checks for a newer plugin and CLI.
