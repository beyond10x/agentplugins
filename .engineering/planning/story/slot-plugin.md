---
format: aep.planning-md/3
id: story:slot-plugin
kind: story
status: draft
title: A slot plugin lets Claude Code and Codex sessions build through the shared slot pool
summary: Add the slot plugin, its catalog entry, checker rosters and docs; tools passes once slot 0.1.0 is released
revision: 1
---
# Story: A slot plugin lets Claude Code and Codex sessions build through the shared slot pool

## Outcome

An agent in Claude Code or Codex can install the `slot` plugin and CLI through `b10x`, start the
slot daemon once, and run every build through the shared pool with the `slot` prefix, following
one activity skill.

## Context

slot (beyond10x/slot) is one Rust binary: `slot serve` owns a GNU-make jobserver pool shared by
every build on the machine and serves a live view on 127.0.0.1:7411; `slot <command…>` runs a
build through it with the same output and exit code; `--local` bypasses it; `SLOT_HOST` moves
builds to a build server. Its first release (0.1.0, Linux x86-64 and ARM64 archives plus
`SHA256SUMS`, Cargo package `b10x-slot-cli`) is not published yet.

## Acceptance

- `plugins/slot/` carries both manifests at the repository version and the skills `init`,
  `upgrade` and `building`; both marketplaces, `catalog.json` (optional, Linux-only product,
  `archive` and `cargo` install), `verified.json` and the checker's rosters list it.
- The `b10x:routing` table reaches every slot skill; `b10x:init` offers the product.
- `website/docs/plugins/slot.md`, the sidebar, the structure table and the README row and tree
  describe it.
- `agentplugins-check tools` checks only slot's own subcommands and options and never runs a
  wrapped build command; it passes against the published 0.1.0 release.
- `task check` and `task site-build` pass.

## Out of Scope

Changes to slot itself; a trial round, which follows the first release.

## Open Questions

- `tools` and `upstream` cannot pass until beyond10x/slot exists and has released 0.1.0.
