---
format: aep.planning-md/1
id: epic:plugins-named-by-product-and-verb
kind: epic
status: implemented
title: Plugins named by product and verb
summary: Rename aep-planning to aep-plan, adp to aep-drive and ess-schema to ess-specify (skill specify), so every product plugin carries its product and its verb.
revision: 4
---
# Epic: Plugins named by product and verb

## Outcome

The marketplace `beyond10x` offers `aep-plan`, `aep-drive`, `ess-specify`, `beyond10x` and
`workspace-hygiene`. Each product plugin carries its product's name plus the verb it performs, so
nobody reads `aep` as "the planning part" or `adp` as "the workflows" again.

## Why now

The operator did exactly that on 2026-09-03: the only AEP-branded plugin was `aep-planning`, and
the execution plugin (skills `drive` and `wave`, agents implementor, adversary, story-scoper) was
branded `adp`, an acronym for a 2,037-line vocabulary crate. Sibling epics: `aep`
`epic:area-layout`, `ess` `epic:area-layout`. Analysis:
`~/.cache/beyond10x-notes/2026-09-03-aep-ess-structure.md`.

## Scope

Plugin directories, both marketplace manifests, both per-plugin manifests, skill and agent ids,
`agentplugins-check`, eval cases and expectations, the website pages, README and install page.

## Out of scope

Recorded eval transcripts under `evals/*/recorded/` (evidence of past runs; never rewritten).
References held by other repositories: `aep`'s eval runner hardcodes the `ess-schema:` prefix
(`aep` `story:plugin-names-follow-agentplugins`); `metaharness/evals/aep/README.md`. The Atlas
catalog components `agentplugins/{aep-planning,adp,ess-schema}` and the ADR (coordinator).
