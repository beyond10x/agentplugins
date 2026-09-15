---
format: aep.planning-md/1
id: task:release-0-9-2-connectors-skill-v1-line
kind: task
status: implemented
title: Release 0.9.2 shipping the connectors-skill v1-line correction
owner: claude-release-0-9-2
relations:
- informed_by: task:release-0-9-1-ess-0-22-1
revision: 4
---
## Intent

Ship the connectors-skill correction recorded under Unreleased as a release, so the pin people
install stops sending them at the wrong CLI line.

## Acceptance

`task check` and `task site-build` exit 0 on the tree tag 0.9.2 points at; the `connectors` skill
and `website/docs/plugins/connectors.md` in the released tree link the v1 line's `v0.7.2` release
rather than `beyond10x/connectors`' generic releases page, whose *Latest* is the connectors_v2 CLI
that skill does not drive.

## Notes

The caveat paragraph in `README.md` said the correction "ships in the next tag". This is that tag,
so the paragraph is removed rather than bumped. `CHANGELOG.md`'s Unreleased section becomes
`## [0.9.2] — 2026-09-15` with its content unchanged: its four bullets are what this release
contains. Three references stay at 0.9.1 on purpose — the `## [0.9.1]` changelog section, the
sentence in the 0.9.2 bullets naming what the released 0.9.1 skill did, and
`task:release-0-9-1-ess-0-22-1`.

Closes ORG-0081 of the org-state review of 2026-09-15.
