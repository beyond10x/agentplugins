---
format: aep.planning-md/3
id: dependency-blocker:governed-dogfood-run
kind: dependency-blocker
status: open
title: aep story:governed-dogfood-run has not landed; the driven walk does not reach complete
relations:
- blocks: story:drive-entry-skill
revision: 1
---
# Blocker: the aep governed dogfood run

`story:drive-entry-skill` says in its Out of Scope that it is blocked until the `aep` repository's
`story:governed-dogfood-run` lands. That story is `draft` on `beyond10x/aep` `origin/main`
(b11db555f4, read 2026-09-28). Cleared when it reaches `implemented` there.
