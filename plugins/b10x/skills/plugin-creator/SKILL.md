---
name: plugin-creator
description: Create, update, review, or port installable plugins whose core commands, agent roles, skills, and bundled resources work in both Codex and Claude Code. Use when the user asks to scaffold a plugin, add plugin capabilities or marketplace metadata, make a Claude Code plugin work in Codex, make a Codex plugin work in Claude Code, or audit a plugin for cross-harness portability.
---

# Portable plugin creator

Build one portable capability layer and only the minimal host-specific metadata around it. Read
[references/compatibility.md](references/compatibility.md) before choosing a layout or claiming
that a component works in both hosts.

## 1. Establish scope

1. Read repository instructions and inspect existing plugin and marketplace conventions.
2. List the user outcomes the plugin must provide. Keep unrelated outcomes in separate plugins.
3. Record the target plugin name, version source, marketplace location, license, and whether any
   component needs network access, credentials, executable code, hooks, or an MCP server.
4. Preserve the repository's language and security rules. Do not introduce an executable helper
   merely because scaffolding would be shorter with one.

## 2. Make skills the portable source

Represent every user-visible workflow as `skills/<capability>/SKILL.md`. Both hosts load this
layout, its `references/`, `assets/`, and optional `scripts/` resources.

- For a requested command, create a skill with the command's behavior. Claude Code exposes plugin
  skills as slash shortcuts; Codex exposes them through skill selection and `$skill-name`.
- For a requested specialist agent, put the complete procedure and success criteria in a shared
  skill. Tell the skill to delegate when the host exposes subagents and to execute the same bounded
  procedure directly otherwise.
- Add `commands/<name>.md` or `agents/<name>.md` only as a thin Claude Code optimization. Never put
  behavior exclusively in those files or claim that Codex loads them as plugin components.
- Keep product names out of shared instructions unless a step genuinely differs by product. Put a
  genuine difference in an explicitly labelled adapter section.

Use concise skill descriptions that state both the capability and its triggering requests. Keep
large reference material beside the skill and tell the instructions exactly when to read it.

## 3. Create the package

Prefer this shared layout:

```text
<plugin>/
├── .codex-plugin/plugin.json
├── .claude-plugin/plugin.json
├── skills/
│   └── <capability>/
│       ├── SKILL.md
│       ├── agents/openai.yaml      # optional Codex presentation metadata
│       └── references/             # optional shared supporting material
├── agents/                         # optional Claude adapter only
├── commands/                       # optional Claude adapter only; prefer skills
├── hooks/                          # optional; only after host-by-host verification
├── .mcp.json                       # optional bundled MCP configuration
└── assets/                         # optional plugin presentation assets
```

Create both manifests with the same stable kebab-case name, version, description, author identity,
license, and repository. Point the Codex manifest's `skills` field at `./skills/`. Let Claude Code
discover the root `skills/` directory by convention unless its manifest needs an additional custom
path. Keep both marketplace entries aligned with the plugin directory.

Do not copy a skill to make it portable. Both manifests point to the same skill bytes.

## 4. Handle non-portable components honestly

- Convert a legacy flat command into a skill. Retain a command wrapper only when compatibility with
  an existing Claude invocation is required.
- Convert an agent's reusable role, decision points, and output contract into a skill. A Claude
  agent wrapper may select that skill; Codex uses the skill directly or delegates under its own
  supported orchestration.
- Treat hooks, local MCP servers, settings, output styles, workflows, monitors, themes, and LSP
  configuration as host-specific until both current host documents and an actual test establish a
  shared contract.
- Do not describe source compatibility as runtime compatibility. A component passes only after both
  hosts discover it and representative prompts produce the required result.

## 5. Validate before handoff

1. Parse both plugin manifests and every marketplace file.
2. Verify that plugin directory names, manifest names, versions, and marketplace entries agree.
3. Run the skill validator on every changed `SKILL.md`.
4. Run `claude plugin validate <plugin-directory>` when Claude Code is available.
5. Add the plugin to a local Codex marketplace, install it, start a new session, and confirm each
   shared skill is discoverable.
6. Test direct and indirect activation, incomplete input, a request that must not activate the
   skill, and at least one real outcome in both hosts.
7. Run the owning repository's full gate. Run its documentation build when public plugin behavior
   or installation guidance changed.

Report the shared capabilities, any host-specific wrappers, the exact validators and gates run,
and any component that remains host-specific. Never summarize a one-host test as dual-host proof.
