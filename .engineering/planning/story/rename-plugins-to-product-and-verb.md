---
format: aep.planning-md/1
id: story:rename-plugins-to-product-and-verb
kind: story
status: implemented
title: aep-planning, adp and ess-schema become aep-plan, aep-drive and ess-specify
summary: Rename the three product plugins, their skill and agent ids, both marketplace manifests, the check crate's PLUGINS table, eval cases and expectations, and the website; recorded transcripts are not rewritten.
relations:
- decomposes: epic:plugins-named-by-product-and-verb
scope:
- confidence: cited
  path: .agents/plugins/marketplace.json
- confidence: cited
  path: .claude-plugin/marketplace.json
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: README.md
- confidence: cited
  path: crates/agentplugins-check
- confidence: cited
  path: evals
- confidence: cited
  path: plugins
- confidence: cited
  path: website
revision: 6
---
# Story: aep-planning, adp and ess-schema become aep-plan, aep-drive and ess-specify

## Context

| old | new |
|---|---|
| plugin `aep-planning` | `aep-plan` |
| plugin `adp` | `aep-drive` |
| plugin `ess-schema`, skill `ess-schema` | `ess-specify`, skill `specify` |
| skill ids | `aep-plan:planning`, `aep-plan:story-migration`, `aep-drive:drive`, `aep-drive:wave`, `ess-specify:specify` |
| agent ids | `aep-plan:<agent>`, `aep-drive:<agent>` |

Skill directory names other than `ess-schema` stay (`planning`, `story-migration`, `drive`,
`wave`). Plugin versions stay at the workspace version; `task check` fails on a disagreement.

Reference sites (counts from `rg` on 2026-09-03, `!target !node_modules !CHANGELOG.md
!evals/*/recorded !plugins`): `crates/agentplugins-check/src/evals.rs` 25,
`website/docs/install.md` 13, `crates/agentplugins-check/src/main.rs` 11 (the `PLUGINS` table and
the tests at lines 285-316), `README.md` 10, `evals/README.md` 9, eight `evals/*/case.yaml` and
`evals/*/expectations.trace.yaml` files, `website/src/pages/index.tsx` 3,
`website/docusaurus.config.ts` 3, `website/docs/intro.md` 3, `website/docs/golden-path.md` 2,
`website/docs/plugins/{aep-planning,adp,ess-schema}.md`, both `marketplace.json`, every
`plugins/*/.claude-plugin/plugin.json` and `.codex-plugin/plugin.json`, and the skill and agent
bodies under `plugins/` that name each other (the `beyond10x` routing skill, the `planning` skill's
guardrail 7 naming the `ess-schema` skill, the `wave` skill naming `adp:` agents).

`evals/*/recorded/` transcripts are evidence of runs that happened under the old names and are
not rewritten. `agentplugins-check` replays them against `expectations.trace.yaml`; if a renamed
expectation no longer matches a recorded transcript, that is a finding for the operator (re-record
under the new names, a paid run), not something to patch in the recording.

## Acceptance

`task check` exits 0 with the `PLUGINS` table naming `aep-plan`, `aep-drive` and `ess-specify`, and
the `retired_names` sweep in `agentplugins-check` finds no retired plugin id in any authored file,
with exactly these exemptions, each visible in a diff: the AEP wire ids `adp/1` and `adp/default`;
expectation rows in `evals/*/expectations.trace.yaml` marked `# recorded-under-this-name` because
the recording predates the rename; the sweep's own `RETIRED` table in
`crates/agentplugins-check/src/main.rs`; `CHANGELOG.md`, `changes/`, `.engineering/` and
`evals/*/recorded/` as records. The sweep also refuses a path segment equal to a retired name under
`plugins/` and `website/docs/plugins/`. The replay gate refuses any `trace-report/1` whose
`verdict` is not `ok` and any report claiming another format. The install block in `README.md`
and `website/docs/install.md` installs the three new names.

## Notes

`changes/*.yaml` are dated change records and stay as written. `CHANGELOG.md` gains an Unreleased
entry that names the old and new plugin names and the skill and agent id map.

Cross-repository hazard (adversary round 1, finding 2): `aep`'s live preflight `EVAL-RUN-018` is
keyed on the `ess-schema:` skill prefix, so a case naming `ess-specify:*` spawns a paid run without
refusing on a runner that has no `ess`. Until `aep` ships `story:plugin-names-follow-agentplugins`,
no `run-eval` label may be applied to this change; `.github/workflows/eval.yml` installs no `ess`.
