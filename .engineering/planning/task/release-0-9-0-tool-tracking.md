---
format: aep.planning-md/1
id: task:release-0-9-0-tool-tracking
kind: task
status: implemented
title: Release 0.9.0 tracking AEP 0.55.0, ESS 0.20.0 and Metaharness 0.7.0
owner: claude-release-0-9-0
relations:
- informed_by: task:hygiene-connectors-release-0-8-1
revision: 9
---
## Intent

Release Agentplugins 0.9.0 with current AEP 0.55.0 guidance, the existing archive-backed ESS 0.20.0 install pin and Metaharness 0.7.0 for governed model execution. Retain the prepared plugin changes and the independent source-publication guidance already on main.

## Acceptance

`task check` exits 0 on a tree where every plugin manifest, the workspace version, `CHANGELOG.md`
and every install pin agree on 0.9.0; the install page and README pin AEP 0.55.0 and ESS 0.22.0 and
quote their `--version` lines; the `drive` skill's launch block carries `METAHARNESS_LIVE=1` and
names the Metaharness build that carries `metaharness aep drive`; the planning skill's verb table
matches `aep plan artifact --help` at 0.55.0; the ESS skill's example is re-validated on ESS 0.22.0
and teaches `ess-inputs.yaml` and the conformance-report evidence path; no plugin cites a
pre-0.51.0 crate path.

## Findings this tracks (review of 2026-09-10 against `afb852e`)

- `metaharness aep drive` exists on Metaharness `main` from commit `1fbbd30` (2026-09-09) and in no
  tagged release; 0.6.5 (2026-09-04) predates it, and the installed 0.6.4 answers
  `unrecognized subcommand 'aep'`. The drive skill omitted `METAHARNESS_LIVE=1`, which the host
  requires for any map with an `llm` step.
- `plugins/aep-plan/skills/planning/SKILL.md` § 2 claimed a complete verb table and lacked `set`,
  `scope`, `waves`, `findings`, `review-value`; no plugin taught `evidence --from` (AEP 0.43.0),
  `--suite` for report/2 (0.55.0), `set --model-digest` or the `executable-system-specification`
  lifecycle (0.50.0).
- `plugins/ess-specify/skills/specify/SKILL.md` attributed its validate output to ESS 0.5.1 and
  named neither `ess-inputs.yaml` (ESS 0.21.0) nor `ess verify conform`.
- 9 lines in 6 plugin files cited `crates/aep-cli/…` and `crates/aep-domain/…`, moved under
  `crates/edge/` and `crates/govern/` in AEP 0.51.0.
- `README.md` pinned AEP 0.51.0 / ESS 0.11.1 while `website/docs/install.md` pinned 0.52.0 / 0.13.1.

## Out of scope

`website/docs/golden-path.md` stays a recording at AEP 0.44.0 / ESS 0.5.1; `.github/workflows/eval.yml`
keeps its 0.44.0 / Metaharness 0.4.2 pins by its own comment; the connectors skill stays at 0.7.1
although v0.7.2 is tagged.

## Authorization

The operator explicitly requested clean committed checkouts, new dated changelogs, annotated tags and GitHub Releases with notes for Harness, Metaharness, Agentplugins, MCP and Atlas. This authorizes completing the prepared Agentplugins 0.9.0 release through the bot. The standing instruction DO NOT RUN ANY GATES applies: no new gates, tests, validators, live runs or binary packaging are performed in this release cut. Earlier preparation evidence remains historical and is not asserted for the final release commit.

## Pin change, 2026-09-10

ESS is pinned at 0.20.0, not 0.22.0. The `Release` workflow dispatched for ESS 0.22.0 (run
34483491460) failed its gate: 11 `ess-xtask` tests, all with `stale or unreviewed schema metadata
row wire:RawSpecFile#/definitions cli-binding-resolution`, reproduced locally at the tag. The tag
therefore has no archives and the install page cannot download it. ESS 0.20.0 is the newest release
with a green `Release` run and archives; the skill example validates on it byte-identically. AEP
0.55.0's archives were backfilled by dispatch (run 34483487110) and the install block was run
against them. Re-pinning to a fixed ESS release is a later task.

## Release verification

Reviewed the merged source changes and dated CHANGELOG entry, matched the workspace and plugin-manifest versions at 0.9.0, and confirmed current GitHub assets for the existing AEP 0.55.0 and ESS 0.20.0 pins. The drive installation now uses the published Metaharness 0.7.0 tag. Release publication is verified by exact main/tag object ids, bot identity, GitHub Release metadata and release-note readback; the protected release evidence is retained outside source. This is a source-only release with no new correctness claim.
