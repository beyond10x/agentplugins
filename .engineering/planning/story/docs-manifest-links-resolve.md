---
format: aep.planning-md/3
id: story:docs-manifest-links-resolve
kind: story
status: active
title: agentplugins-check refuses a b10x.docs.yaml URL that names no page
summary: Every https://beyond10x.github.io/docs/agentplugins/<path>/ in b10x.docs.yaml maps to website/docs/<path>.md.
scope:
- confidence: cited
  path: crates/agentplugins-check/src/main.rs
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-09-28T13:12:54Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-09-28T13:12:54Z", actor: "human:timo", revision: 3}
---
# Story: the docs manifest links resolve

## Outcome
A `b10x.docs.yaml` section URL under this repository's route base that names no page fails
`task check`, before it reaches the public website build.

## Context
From 2026-09-24 to 2026-09-28 every Atlas "Publish unified documentation" run failed: the only
broken link was `/ecosystem/agentplugins/` → `/docs/agentplugins/plugins/beyond10x/`, the reference
URL `b10x.docs.yaml` kept after 0.14.0 renamed that page (atlas run 36390633830). The public site
stayed on the 2026-09-24 08:20 publication for four days. 0.17.0 fixed the URL.

## Acceptance
`agentplugins-check` maps every `url` in `b10x.docs.yaml` that starts with the surface's
`canonicalUrl` to `website/docs/<path>.md` (or `<path>/index.md`, or the intro at the root) and
refuses one with no page, naming the line; a unit test shows the old `plugins/beyond10x/` URL
refused.
