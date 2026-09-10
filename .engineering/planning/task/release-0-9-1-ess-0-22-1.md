---
format: aep.planning-md/1
id: task:release-0-9-1-ess-0-22-1
kind: task
status: implemented
title: Release 0.9.1 pinning ESS 0.22.1
owner: claude-release-0-9-1
relations:
- informed_by: task:release-0-9-0-tool-tracking
revision: 4
---
## Intent

Move the ESS pin from 0.20.0 to 0.22.1 once ESS 0.22.1 exists with archives, and release
Agentplugins 0.9.1 with the gates run on the tagged tree.

## Acceptance

`task check` and `task site-build` exit 0 on the tree tag 0.9.1 points at; the install page's ESS
block downloads and verifies `ess-0.22.1-<target>.tar.gz` and prints `ess 0.22.1`.

## Notes

ESS 0.22.1 is delivered through `beyond10x/ess` PR #26 (story:release-gate-green-at-the-tag
there): re-reviewed schema metadata rows, re-rendered support block, refreshed fuzz lock. The
0.9.0 release note records that 0.9.0 was cut without running gates.
