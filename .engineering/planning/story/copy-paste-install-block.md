---
format: aep.planning-md/1
id: story:copy-paste-install-block
kind: story
status: active
title: One copy-paste install block on the install page and the README
relations:
- decomposes: epic:second-adopter-feedback
revision: 3
---
# Story: One copy-paste install block on the install page and the README

## Outcome

A reader of `website/docs/install.md` or `README.md` can install the marketplace and the three
plugins an AEP adopter needs by copying one block, and knows before starting that the `aep` binary
must be on `PATH`.

## Context

The second adopter installed through the 0.14.0 announcement's instruction, which names a repository
that has carried no marketplace file since 2026-09-01. `website/docs/install.md` describes the two
marketplace formats in prose and gives no command. The front-door plugin's own README does not give
one either. Derived from the epic this decomposes.

## Acceptance

- `website/docs/install.md` contains one fenced block with, in order: `/plugin marketplace add
  beyond10x/agentplugins`, `/plugin install aep-planning@beyond10x`, `/plugin install adp@beyond10x`,
  `/plugin install ess-schema@beyond10x`, and the Codex equivalent where one exists.
- The same page states that `aep-planning` and `adp` do nothing without the `aep` binary on `PATH`,
  names where it comes from (the protocol repository's releases), and shows `aep --version`.
- `README.md` carries the same block or links to the page by relative path.
- No page in this repository names the retired repository name as an install source
  (`rg "engineering-protocols"` over `README.md` and `website/docs` returns nothing).
- `task check` and `task site-build` pass.

## Out of Scope

Installing the binary itself; a preflight command (that is the protocol repository's story).

## Open Questions

None.
