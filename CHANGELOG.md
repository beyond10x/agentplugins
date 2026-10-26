# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.24.2] - 2026-10-26

### Changed

- Track AEP 0.78.0: `aep:planning` names the suite majors `aep plan artifact evidence --suite` records and the `UnsupportedVocabulary` refusal of a catalog `--suite-input` lineage; the planning protocols pin moves to 0.78.0
- Track Metaharness 0.19.0; the eval workflow runs it with AEP 0.77.0, the revision it links; the drive reference names `metaharness doctor`, a step map's `commands:` and `aep drive resume --evidence`
- Track ESS 0.69.2: the ESS skills describe `ess/31`, the new `ess-http/1` forms and `ess generate types` roots and Rust options; the tutorial set moves with it and its specification to `ess/31`
- Track Connectors v0.59.0; `connectors:integrating` describes `connectors list`, the local stdio MCP server, `oauth2_grant` sign-in and `connections token`
- `agentplugins-check tools` admits a Connectors native integration command (`server`) once the release binary's help verifies it; such commands are routed outside `cli.yaml`
- `agentplugins-check tools` admits a Connectors native integration command (`server`) that the release binary's help verifies; it is routed outside `cli.yaml`

## [0.24.1] - 2026-10-23

### Changed

- Track ESS 0.68.0: the ESS skills describe `ess/30`; the tutorial set moves with it and its specification to `ess/30`
- Track Connectors v0.56.0; `connectors:integrating` describes catalog `bindings` and `operations list|describe --connection`
- Track Metaharness 0.14.0; the eval workflow runs it with AEP 0.68.1, the revision it links; `aep:implementing` states the `resume --budget-usd` floor
- `b10x` pages name the local pin, the `connectors` CLI and every CLI `tools` checks; the documentation link points at the site root

## [0.24.0] - 2026-10-21

### Changed

- **Breaking:** Connectors skills spell v0.55.0's `instances`, `--connector`/`--instance` and `connections reconnect`
- **Breaking:** `CHANGELOG.md` follows Keep a Changelog 1.1.0; `release verify` needs `## [x.y.z] - YYYY-MM-DD`
- Track AEP 0.76.0; `aep:planning` names `aep plan artifact redact` as the only way to change an immutable record
- Track ESS 0.67.0: the ESS skills describe `ess/27` to `ess/29` and suites to `/57`; the tutorial set moves with it
- Track Metaharness 0.12.0; the eval workflow runs it with AEP 0.68.1 and Claude Code 2.1.305
- `agentplugins-check tools` refuses an eval `CLAUDE_VERSION` the pinned Metaharness adapter does not pin
- `agentplugins-check tools` refuses deprecated Connectors spellings
- The shared gates workflow runs gates 0.1.22
- Sample tracker keys and project paths in skills, a `b10x` test and the golden-path transcript are neutral

## [0.23.2] - 2026-10-17

### Changed

- Track ESS 0.62.0 and Connectors v0.53.1; `b10x upgrade` now offers them
- `ess:specifying`: `ess generate types --rust-raw-json`, `ess/26`, and reads by identity a scenario cannot decide
- `ess:hardening`: `ess-history/3`, `Timestamp` inputs and `deletes:` in the explorers, `unwitnessed` mutants
- `ess:testing-conformance`: `hold_now` scenarios on `now` boundaries (`ess-conformance/50`, `/51`)
- Connectors skills: custody of write-created credentials, new OpenRouter, Runpod and GitLab reads
- The eval workflow runs Connectors v0.53.1

## [0.23.1] - 2026-10-16

### Added

- `aep:planning`: `aep plan artifact spend` reads `task_spend` records against an optional baseline and target

### Changed

- Track AEP 0.74.0 and Connectors v0.52.0; `b10x upgrade` now offers them
- Connectors skills: the approval clock is a Roughtime quorum; a Slack `"ok": false` read is a named refusal
- The eval workflow runs Connectors v0.52.0

## [0.23.0] - 2026-10-15

### Added

- `b10x` reads a local pin file, `~/.config/b10x/pins.toml`, which wins over a repository pin

### Changed

- `b10x check` reports a CLI held back at its local pin, with the reason, instead of inviting an upgrade
- `b10x upgrade` skips a locally pinned CLI and its finding names the pin and its reason
- The `b10x:upgrade` and `b10x:init` skills, `SETUP.md` and the install page document the local pin
- Track AEP 0.73.0, ESS 0.60.0 and Connectors v0.51.0; `b10x upgrade` now offers them
- `aep:planning`: `evidence --kind task_spend --from <file>` records what a task spent (AEP 0.73.0)
- ESS skills follow 0.60.0 (`ess/25`, `ess-http/1`, `ess-composition/4`) with a runnable `devices.yaml` example
- The tutorials, the trial fixtures and the eval workflow run ESS 0.60.0
- Connectors skill: Slack `chat.update` and `chat.delete`, `clock_refusal`, the catalog provider binary
- The eval workflow runs Connectors v0.51.0

## [0.22.11] - 2026-10-14

### Changed

- Track AEP 0.71.3, Worktree 0.18.2 and Connectors v0.50.0; `b10x upgrade` now offers them
- `aep:planning`: a conformance record names its report and suite by path and digest
- Connectors skills and page: the CLI builds and runs on Windows from source since v0.49.0
- Worktree 0.18.2 changes no skill text, so `worktree:managing-worktrees` is unchanged
- The eval workflow runs Worktree 0.18.2 and Connectors v0.50.0

## [0.22.10] - 2026-10-13

### Changed

- Track Worktree 0.18.1; its skill text and help are unchanged, so no skill text changes
- The eval workflow runs Worktree 0.18.1

## [0.22.9] - 2026-10-13

### Changed

- Track Worktree 0.18.0 and Connectors v0.48.0
- `b10x` installs Connectors from its checksummed release archive from v0.48.0; `tools` checks its binary's help
- **Breaking:** `worktree:managing-worktrees` 0.18.0: cleanup deletes ignored entries `.worktreekeep` does not keep
- Connectors skills and pages name v0.48.0: prebuilt archives, macOS notes, Slack writes under approval
- The eval workflow runs Worktree 0.18.0 and Connectors v0.48.0, the latter from its archive

## [0.22.8] - 2026-10-11

### Changed

- Track ESS 0.59.0
- ESS skills follow 0.59.0: authored acts state their caller under `caller:` (`ess-scenario/5`)
- The eval workflow runs ESS 0.59.0

### Fixed

- The ESS tutorial says every command answer carries the revision and an unset optional field is `Node::Null`

## [0.22.7] - 2026-10-10

### Changed

- Track ESS 0.58.0 and Connectors v0.45.0
- ESS skills follow 0.58.0: `ess/24` and `undeclared_fields: ignored`; tutorials and fixtures require it
- Connectors skills and page name v0.45.0 and its grown GitLab, Slack, Jira and Kubernetes selections
- The eval workflow runs ESS 0.58.0 and Connectors v0.45.0 and keeps AEP 0.68.0

## [0.22.6] - 2026-10-10

### Changed

- Track AEP 0.71.2, ESS 0.57.0, Connectors v0.43.0 and Worktree 0.15.0
- `worktree:managing-worktrees`: archives leave build layout out of `dirty.patch`; `prune-archives` shrinks old ones
- `aep:planning`: report/2 beside an ordinary suite; `aep:upgrade` repairs a store holding a repeated transition
- ESS skills follow 0.57.0 (`ess-diff/15`–`/17`, `ess-mutation-report/5`); tutorials and fixtures require it
- Connectors skills and page name v0.43.0 and the GitLab, Jira and SQL operations an adapter must permit
- The eval workflow runs ESS 0.57.0, Connectors v0.43.0 and Worktree 0.15.0; the store's protocols move to AEP 0.71.2

## [0.22.5] - 2026-10-09

### Changed

- Track Connectors v0.39.0; catalog selections naming `credential` parameters need it

## [0.22.4] - 2026-10-09

### Changed

- Waves and critic rounds return a review without a ` ```findings ` block to its author when the store requires one
- Track AEP 0.71.0 and Connectors v0.38.0

## [0.22.3] - 2026-10-08

### Added

- `ess:hardening` ends its report with a ` ```findings ` block that `aep plan artifact findings` compares

### Changed

- Track AEP 0.70.0, Worktree 0.14.0, Connectors v0.37.0 and Metaharness 0.9.3
- The eval workflow runs Metaharness 0.9.3 and keeps AEP 0.68.0; protocols, gates and Docs System pins move

## [0.22.2] - 2026-10-08

### Changed

- Track AEP 0.69.1
- Track Worktree 0.13.0: `gc` without `--id` assesses only the current repository; commands accept a tree's id or path

## [0.22.1] - 2026-10-08

### Changed

- Track Connectors v0.33.0: upgrade every binary that opens a store before `setup init` creates one

### Fixed

- `connectors:integrating` spells `operations list --adapter <alias> --family <id>`; the old command exited 2

## [0.22.0] - 2026-10-08

### Changed

- Track AEP 0.69.0, ESS 0.56.0, Worktree 0.12.1 and Connectors v0.32.0
- `ess:specifying` documents the ESS 0.55.0 constructs authors reached for and could not find
- ESS 0.56.0: declare a held-state branch first (`ESS-COMMAND-004`); tutorials and fixtures require 0.56.0
- `worktree:managing-worktrees` follows 0.12.1: `target/` is cache, nested repositories are archived as images
- `aep:upgrade` says what `aep plan store migrate git --verify` compares since AEP 0.69.0
- Trials on 0.22.0 (`ess-tutorial`, `ess-full-package`, `worktree-onboarding`) are the new baselines

### Fixed

- Connectors v0.32.0: JSON answers carry schemas as JSON values; the skills no longer decode a schema string
- `ess:testing-conformance`: the `skipped` count for unsupported setup is the Go runner's; Rust fails on it

## [0.21.2] - 2026-10-07

### Fixed

- **Breaking:** The `aep` skills say the newest `aep` refuses an `aep.project/1` store until it is migrated
- An agent can start every command skill; only a skill with both flags and an `Operator-only:` sentence cannot

## [0.21.1] - 2026-10-07

### Changed

- Track Worktree 0.11.0; the `worktree-onboarding` baseline is the run of 2026-10-07

### Fixed

- `worktree:managing-worktrees` step 1 no longer reads as an instruction to publish every commit

## [0.21.0] - 2026-10-07

### Changed

- Track ESS 0.55.0: new specifications start on `ess/23`; `ess:specifying` reads the newest format from the CLI
- `verified.json` tracks ESS 0.55.0, Worktree 0.10.0 and Connectors v0.31.0 after `tools` and three trials
- Track Connectors v0.31.0: `connectors:integrating` follows a refusal's `next_action`, incl. `revalidate_connection`
- The design and parallel-safety critics flag a reader not ordered after its producer by `depends_on`
- The live eval installs ESS 0.55.0, Worktree 0.10.0 and Connectors v0.31.0

### Fixed

- Track Worktree 0.10.0: `worktree sweep --all-profiles`; `worktree:init` fixes three confusing sentences

## [0.20.1] - 2026-10-06

### Changed

- Track Worktree 0.9.0: work ends with `worktree finish --discard-cache --archive <tree>`
- A sub-agent that creates a tree finishes it or returns its id and path for the parent to finish
- AEP wave teardown archives unpublished wave commits locally instead of keeping the tree
- The governed-plan tutorial and the Worktree plugin page describe the new finish

## [0.20.0] - 2026-10-05

### Changed

- Refresh ESS resources for the current source language and bundle executable examples
- The lending walkthrough is Rust with canonical suite IR and the native ESS runner; 17 scenarios pass
- **Breaking:** Connectors guidance moves to v0.28.0; 0.7.x configuration and credentials need a migration
- Match eval AEP 0.68.0 to Metaharness 0.9.1's embedded source and install the eval prerequisites
- Current-tool and upstream checks cover eval, protocol and package pins and executable ESS examples
- Refresh setup, routing, delivery and maintenance instructions; documentation publication is asynchronous
- Governed-plan trials install Worktree; the tutorial explains partial-wave obligations and archive recovery

### Fixed

- Repair upgrades from a frozen local marketplace by planning the source change before plugin upgrades
- Measure Rust trials with Cargo summaries and close two false-green cases

## [0.19.2] - 2026-10-03

### Changed

- An explicit Worktree cleanup request authorizes exact eligible removals without a second approval prompt
- Verify the skills against AEP 0.68.0, ESS 0.52.0 and Worktree 0.8.2
- Clarify Rust and Go implementation synthesis and Go conformance package placement

### Removed

- Restore the Rust 1.99 gate without weakening test assertions; remove obsolete AEP command inventory entries

### Security

- The release workflow has read-only permissions; publication goes through the organization bot

## [0.19.1] - 2026-09-29

### Added

- `ess:specifying`: implementation code comes from the specification through `synthesize`, never by hand

### Changed

- `ess:testing-conformance`: a green suite proves only the fields its expectations read
- `verified.json` does not move: aep 0.65.0 and ess 0.43.0 are not yet re-verified

## [0.19.0] - 2026-09-29

### Added

- `aep:investigating` skill: ten techniques for incidents that cannot be re-run, recorded as an `incident-report`

### Changed

- `aep:diagnosing` and `b10x:routing` send an incident that cannot be re-run to `aep:investigating`
- No eval case covers the new skill yet; `evals/README.md` lists it with the uncovered ones
- `verified.json` still pins aep 0.64.0 and ess 0.39.0; aep 0.65.0 and ess 0.42.0 are not yet re-verified

## [0.18.0] - 2026-09-28

### Added

- Tutorial: Your first governed plan, an AEP plan, four critics and a reviewed wave for the ESS tutorial's library
- `agentplugins-check upstream` reports what moved upstream; `.agents/skills/following-upstream` acts on it

### Changed

- `agentplugins-check` refuses a `b10x.docs.yaml` link that names no page
- `verified.json` pins aep 0.64.0 and ess 0.39.0; `ess:hardening` covers the concurrent explorer
- `aep:planning` checks each critic's verdict line; the decomposer relates each story to the epic's objective
- Eval recording moves to `metaharness aep drive eval run` with aep 0.64.0

### Fixed

- `task trial:sandbox` can recreate a sandbox that ran `aep`

## [0.17.0] - 2026-09-28

### Added

- Tutorial: Your first ESS specification, from an empty directory to a Go implementation passing 17 scenarios

### Changed

- `install.md` installs every CLI through `b10x`; the docs name the five shipped plugins
- `agentplugins-check` refuses the retired display names "AEP Plan", "AEP Drive" and "ESS Specify"
- `verified.json` pins ess 0.38.0; trial-round fixes to `ess:specifying` and `ess:testing-conformance`
- This repository's planning store is `aep.project/5`

## [0.16.0] - 2026-09-28

### Added

- `ess:specifying` gains `references/later-formats.md`, each example validated under ess 0.37.0

### Changed

- **Breaking:** `aep` describes the Git-native store `aep.project/5`; skills name the upgrade when a store is older
- `ess:retrofitting`, `ess:init`, `ess:upgrade` and `ess:testing-conformance` follow ess 0.37.0
- The specification interview skips its questions when the request asks for a finished specification
- `worktree:managing-worktrees` covers `archive` contents and interrupted removals; `worktree:init` agent guidance

### Fixed

- `ess:hardening` uses `ess verify conform mutate`; it no longer says the audit is unshipped
- The `ess-pipeline` trial fixture sets the fields its invariant reads; four ESS trial baselines are re-recorded

## [0.15.0] - 2026-09-28

### Added

- `aep:diagnosing` skill: a command that goes red on the symptom before any hypothesis
- `ess:specifying` interviews before drafting a domain nobody has described (`references/interview.md`)
- On request, the adversary step runs as a cross-family panel (`references/adversary-panel.md`)
- `b10x:authoring-plugins` carries five rules for writing what an agent reads (`references/writing.md`)
- These methods are adapted from `mattpocock/skills` and `cursor/plugins`, both MIT, in this repository's words

### Changed

- The adversary and `ess:testing-conformance` reject a test that passes when every call returns a default
- A wave verifies a green unit's claim against the base before merging; only `VERIFIED` merges
- The decomposer cuts vertical slices and drafts mechanical changes as expand, migrate and contract
- `evals/` holds 17 cases; its README recounts coverage from the `subject:` fields

### Security

- The story scoper returns a `Safety fact`; the security reviewer proves it with a test before calling a change safe

## [0.14.17] - 2026-09-27

### Added

- `aep:review-plan` and `aep:decompose` commands hand off to `aep:planning`

### Changed

- **Breaking:** The eleven `aep` agents are thin adapters; each role's procedure moved to its skill's `references/`
- `b10x:authoring-plugins` describes the command kind and thin agents over skill-owned roles

### Fixed

- `b10x:routing` sends upgrade requests to the upgrade skills and names every skill; the gate refuses a gap

## [0.14.16] - 2026-09-26

### Added

- R3 gains a third kind of skill, the command, which `agentplugins-check` enforces
- `worktree:cleanup` (`/worktree:cleanup`) reviews managed trees and removes only approved ids
- `aep:wave` and `aep:drive` commands start `aep:implementing`; they are no longer retired names

### Changed

- `verified.json` moves `worktree` to 0.8.1
- `b10x:routing` lists the three commands

### Fixed

- The `**Skill version**` line reads the plugin version, and the gate refuses one that differs

## [0.14.15] - 2026-09-26

### Changed

- `managing-worktrees` is regenerated from `worktree skill` 0.8.0, keeping its name and three local steps
- `verified.json` moves `aep` to 0.60.0, `ess` to 0.35.0 and `worktree` to 0.8.0

## [0.14.14] - 2026-09-26

### Changed

- `verified.json` moves `ess` to 0.34.0

### Fixed

- The ESS syntax reference's lending example sets `pages` on `AddCopy`, which `ess` 0.34.0 requires

## [0.14.13] - 2026-09-26

### Added

- `ess:hardening`: eight specification-hardening techniques for after a conformance suite is green

### Changed

- `ess:conformance` offers the catalogue when a coverage task ends on a green suite

## [0.14.12] - 2026-09-26

### Added

- `ess:testing-conformance` shows a TypeScript refusal recorder beside the Go one

### Changed

- `ess:init` checks an existing specification first and reports its validation and conformance counts
- Every `b10x init` or `upgrade` command a plugin prints names `--host`; `agentplugins-check` refuses one that does not
- `b10x init`/`upgrade --host <one>` says what a plan for the other host would do

### Fixed

- The ESS syntax reference lists the structured predicate forms and no longer says to split "A or B" views

## [0.14.11] - 2026-09-26

### Added

- `verified.json` records the releases the skills were verified against; `tools` fails when a newer one exists
- `b10x pin <cli> <version>` and `b10x unpin <cli>` keep a committed `b10x.toml`

### Fixed

- `b10x check` refreshes the newest releases at most once a day and names `/b10x:upgrade` when anything is older
- `b10x upgrade` and `b10x setup plan` read plugin versions from the newest release, not a stale local copy

## [0.14.10] - 2026-09-25

### Changed

- `ess:specifying`: every `--kind` of `ess generate`, one `--out` per kind, and what OpenAPI and AsyncAPI need
- `ess:testing-conformance`: generate the Go suite inside the module; return payload numbers as `float64`
- Trials allow `cargo`; the aep, worktree and upgrade prompts say nobody will answer questions

### Fixed

- `agentplugins-check trial-report` recognises every spelling of `go test`; it had reported a run as "not run"
- `b10x setup plan` with no selection keeps every installed product; it proposed uninstalling `connectors`

## [0.14.9] - 2026-09-25

### Added

- `trials/<name>/trial.yaml` defines seven trials; `task trial:run TRIAL=<name>` runs one
- Trial `ess-full-package`: every ESS output, then a Go implementation run against the generated suite
- `agentplugins-check trial-report` measures a run and exits 1 when a measure is worse than the baseline

## [0.14.8] - 2026-09-25

### Added

- `catalog.json`: `b10x-harness`, an optional Linux-only binary of `aep`; catalog binaries gain `platforms`

### Changed

- Docs name `b10x-harness` beside `metaharness`

### Fixed

- `b10x` chooses prebuilt only when the release's `SHA256SUMS` lists this machine's target
- `b10x install <cli>` follows the same default; since 0.14.7 it chose `cargo` whenever `cargo` was on `PATH`
- `aep:init`: `metaharness` is optional and installed with `b10x install metaharness`

## [0.14.7] - 2026-09-25

### Changed

- **Breaking:** `b10x` installs CLIs from the checksummed prebuilt archive by default, not with `cargo`
- `b10x:init` offers prebuilt first; the product `init` skills, `SETUP.md` and the docs say the same

## [0.14.6] - 2026-09-25

### Changed

- `ess:retrofitting`: `ess infra import openapi` reads OpenAPI 3.0 as well as 3.1
- Syntax reference: compare two fields through one struct; the flat form is refused
- `aep:planning` points at the store's refusal of an empty findings fence instead of quoting the old message

### Removed

- `ess:specifying`: the note that the CLI help calls the `system.yaml` layout "legacy" is gone

## [0.14.5] - 2026-09-25

### Changed

- `ess:retrofitting`: a command the code silently ignores in some state gets no `wrong_state:` outcome
- `worktree:init`: `curl --create-dirs` for the profile; any local file works; headless workspace root
- `worktree:managing-worktrees`: always pass `--id` to `gc`

## [0.14.4] - 2026-09-25

### Changed

- `ess:specifying`: a second source document goes into the same system, a new system or an external boundary
- `ess:specifying`: `## Next` starts with the owner reviewing every marker and contradiction, then planning
- `ess:specifying`: a headless run needs `--allowedTools "Bash(ess:*)"`
- Syntax reference: a predicate is one comparison or a bare fact path; "state A or B" is two views
- `ess:retrofitting`: a service that publishes no events declares one per success outcome
- `worktree:init`: `worktree doctor --check` fails with `no active profile` since worktree 0.7.1
- `agentplugins-check trial-isolation` fails a run on an outside skill only when the run used it

### Fixed

- `ess:specifying`: never invent an entity so that a command type-checks; markers become questions
- The docs intro and `b10x skill --help` name the current skills (`init`, `specifying`, …)

### Security

- `SETUP.md` asks the user before it installs `b10x`; product `init` skills install CLIs only after confirmation

## [0.14.3] - 2026-09-25

### Added

- `agentplugins-check trial-isolation` checks a trial loaded only its sandbox's plugins; sandboxes leave `$HOME`

### Changed

- `ess:specifying`: upper-case state names, `ess-inputs.yaml`, `generate --out`, views for entity invariants
- The syntax reference shows a limit read from stored state, no double booking and ordered time ranges
- `ess:testing-conformance` explains the built-in targets and `synthesize --target go|typescript`
- `aep:migrating`: `reverse init` creates the store; a new noun gets its ESS domain before its stories
- `b10x:init` and `aep:init` offer `ess` with `aep`
- `worktree:init`: where a profile comes from and goes; `worktree:managing-worktrees`: branch before committing
- `agentplugins-check tools` also synthesizes and generates the ESS syntax example
- The README lists every plugin's skills and agents, linked to their files; the gate keeps it exact

### Fixed

- `ess:retrofitting`: `ess infra import openapi` reads OpenAPI 3.1 only; its description named retired skills
- `aep:planning`: `reverse init` values, body files in the repository, `[]` for empty findings; no `ess skill`
- The design and parallel-safety critics name the same two remedies for a shared file
- `b10x:routing` routes ESS and worktree work to their skills instead of to `ess skill`
- `b10x upgrade` installs the replacement of a legacy plugin it removes

## [0.14.2] - 2026-09-24

### Changed

- A product's `init` skips the install step when its CLI already answers
- `ess:specifying`'s syntax example is a lending library, not a todo service
- **Breaking:** `b10x upgrade` updates only what is installed and no longer proposes new plugins
- Plan output lists the marketplace refresh that `apply` runs, under "Also runs"

### Fixed

- `SETUP.md` creates `~/.cache` before `mktemp` in it; a fresh home has none
- `b10x:init`'s plan example uses placeholders instead of values an agent can paste unchanged

## [0.14.1] - 2026-09-24

### Fixed

- Publish what 0.14.0 could not: its release smoke test looked for a retired skill name

## [0.14.0] - 2026-09-24

### Added

- `init` and `upgrade` skills on every plugin; `/b10x:init` is guided onboarding
- `b10x init <products>` and `b10x upgrade [<products>]` plan only the named or installed products
- `ess:specifying` gains `references/syntax.md`, a validated todo service in three files
- **Breaking:** `agentplugins-check tools` replaces `remote`; checks spelled commands against the newest releases

### Changed

- Every plugin lives here: `ess` and `worktree` join `plugins/` at this repository's version
- **Breaking:** Skills renamed under R3: `b10x:init`, `ess:init`, `ess:specifying`, `ess:retrofitting` and others
- `b10x check` says when a CLI is older than the newest release and when no product is set up

## [0.13.1] - 2026-09-24

### Added

- `b10x skill <plugin>` lists an installed plugin's skills and agents; `b10x skill <plugin>:<skill>` prints one

### Fixed

- `b10x:installing` plans for the host it runs in and writes the plan with `--out` alone

## [0.13.0] - 2026-09-24

### Added

- The plugin concept, enforced: `website/docs/structure.md` states eight rules the `concept` check refuses

### Changed

- **Breaking:** `aep-plan` and `aep-drive` are one plugin, `aep`, with three skills and eleven agents
- **Breaking:** Skills renamed to activities, e.g. `b10x:routing` (was `guide`); old ids are retired names
- README is one paragraph and a table of plugins; the reference moved to the plugin pages

### Fixed

- `b10x setup undo` refreshes both hosts' marketplaces after restoring settings

## [0.12.0] - 2026-09-24

### Added

- `b10x setup plan`, `apply` and `undo`: one-sentence onboarding through `SETUP.md`
- Setup migrates earlier installs, retired plugin ids and marketplaces on both hosts and every scope
- Release archives for four targets, `SHA256SUMS` and `SETUP.md` are published with each release

### Changed

- **Breaking:** The front-door plugin `beyond10x` is now `b10x`, with a SessionStart hook that runs `b10x check`
- This repository names no version of what it points at; `agentplugins-check` refuses a pinned remote entry
- `ess@b10x` is back in the catalog, pointing at the ESS repository; `ess@ess` installs are migrated

### Fixed

- Correction: Codex accepts `git-subdir` entries, so both hosts install `worktree@b10x` and `ess@b10x`

## [0.11.0] - 2026-09-24

### Added

- `agentplugins-check remote` checks the remote entry's tag and commit on every pull request and daily

### Changed

- **Breaking:** The marketplace identity is `b10x` in both formats; plugins install as `<plugin>@b10x`
- The Claude Code marketplace lists `worktree` as a `git-subdir` entry pinned to Worktree 0.6.0
- The routing skill, the wave skill, README, install guide and website point at `worktree`

### Removed

- **Breaking:** `workspace-hygiene` is retired; Worktree ships its own plugin, `worktree`

## [0.10.0] - 2026-09-23

### Changed

- The front door and the planning skill route spec-driven work to `ess:specify`
- Workspace, every plugin manifest, the three version-stamped skills and every install pin move to 0.10.0

### Removed

- **Breaking:** `ess-specify` is retired from this marketplace; ESS ships its own plugin, `ess`
- The eval case `ess-specify-new-entity` is removed; the corpus has eight cases

## [0.9.3] - 2026-09-18

### Added

- Skill `ess-specify:coverage`: raise and audit a conformance suite's coverage against a real implementation
- It ranks skips by refusal reason, grounds doubles in the producer, floors `answered` and names `-race`
- Agent `aep-drive:security-reviewer`: the adversary's verification, framed as defensive verification

### Changed

- `ess-specify`'s manifests, the README and the eval-coverage counts follow: 10 skills rather than 9

## [0.9.2] - 2026-09-15

### Changed

- The `connectors` skill and page link the last v1 release, `v0.7.2`, and the 0.7.x surface they drive
- Eval coverage is counted from the `subject:` fields: 7 of 10 agents and 5 of 9 skills
- README and the install page read the Metaharness to AEP pin from `crates/metaharness-aep/Cargo.toml`

### Fixed

- The `inspect` upgrade check arrived at Connectors `v0.7.2`; the earlier "0.7.1 has none" claim is corrected

## [0.9.1] - 2026-09-10

### Changed

- Pin ESS `0.22.1` instead of `0.20.0`; ESS `0.22.0` had no archives
- The ESS skill's example is re-validated on ESS `0.22.1`
- Align the golden path's model-driving command with Metaharness
- Unlike `0.9.0`, this release ran `task check` and `task site-build` on the tagged tree

## [0.9.0] - 2026-09-10

### Added

- Document ESS conformance synthesis, execution and evidence ingestion

### Changed

- Align installation guidance on AEP `0.55.0` and ESS `0.20.0`, both with published native archives
- Use Metaharness `0.7.0` for governed model runs through `metaharness aep drive`
- The planning skill's command inventory follows AEP `0.55.0`
- Update example crate paths for AEP's area-based layout; manifests and install pins move to `0.9.0`
- Publish source through standalone bot tooling without an Atlas checkout

## [0.8.1] - 2026-09-07

### Changed

- Regenerate Workspace Hygiene from Worktree 0.4.0: storage inspection, lease maintenance, verified cleanup
- Connectors guidance follows the released 0.7.1 surface
- Refresh current marketplace installation pins to 0.8.1 and Worktree installation to 0.4.0
- Waves create and retire trees through the Worktree skill and CLI, with explicit lease duties

## [0.8.0] - 2026-09-05

### Added

- Publish the connectors plugin in the release-pinned installation guides for both hosts
- Add the `connectors` plugin to both marketplaces, with one skill for the Connectors 0.6.0 CLI
- `agentplugins-check` refuses a flat CLI spelling in any `.md`, `.yaml` or `.yml` this repository authors

### Changed

- The planning skill names `aep plan artifact unrelate` (AEP 0.53.0) for taking an edge back
- Every authored document teaches the grouped CLI spelling (`aep plan …`, `ess specify …`)
- The eval corpus's command matchers accept either spelling
- The install page pins AEP 0.52.0 and ESS 0.13.1, the first releases with the grouped spelling

### Fixed

- Fix Connector readiness evaluation for Claude and Codex command traces
- Repair YAML quoting in the ESS skill description so every released plugin validates

## [0.7.0] - 2026-09-04

### Added

- `agentplugins-check` refuses a retired plugin name in any authored file's text or path

### Changed

- **Breaking:** Plugins renamed: `aep-planning` is `aep-plan`, `adp` is `aep-drive`, `ess-schema` is `ess-specify`
- **Breaking:** The ESS skill `ess-schema:ess-schema` is `ess-specify:specify`; reinstall under the new names
- Five `golden-path-end-to-end` rows keep the plugin names of their 2026-09-03 recording
- The eval case `ess-schema-new-entity` is now `ess-specify-new-entity`; plugin pages follow the rename

### Fixed

- A replay is judged by its report's `verdict`, not by its exit status, which was 0 on contradicted runs

## [0.6.2] - 2026-09-03

### Added

- Publish Codex CLI commands for a release-pinned install and for replacing a stale marketplace pin

### Changed

- Non-trivial cross-repository and release work enters the owning AEP store before implementation
- Refresh adopter guidance to ESS 0.9.2 and align every plugin manifest and skill version with this release

## [0.6.1] - 2026-09-03

### Changed

- The golden-path cost row and the documented invocation say `15`, not `5`
- Planning guardrail 2: never create an artifact to see its template; body files go under `$TMPDIR`

### Fixed

- The golden-path case's step 1 names the protocol source the page names
- Two golden-path rows stop contradicting a correct run
- `.github/workflows/eval.yml` installs the pinned `aep` into `$HOME/.local/bin`, where runs find it

## [0.6.0] - 2026-09-03

### Added

- The planning and wave skills record an `approval-record` at each stop when no operator is present
- The golden path gains § Running it without an operator

### Changed

- **Breaking:** Rename the ESS skill directory to `skills/ess-schema`; the invocation is now `/ess-schema:ess-schema`
- The `golden-path-end-to-end` case uses that instruction and checks the records it produces
- The decomposer returns its unanswerable questions instead of holding a turn open for them
- The adversary writes its failing case and captures red before it runs the suite
- Sharpen the `ess-schema` trigger to entity introductions that name something typed about the noun
- Record that `the-skill-was-offered` gapped on the skill's spelling, not on its trigger

### Fixed

- Fix `the-scope-was-actually-tested`, which gapped in all eight eval cases on their first live recording

## [0.5.1] - 2026-09-03

### Changed

- Re-record the eight-step golden path against AEP 0.44.0 and ESS 0.5.1

### Fixed

- Correct the adopter path to install and verify the current AEP and ESS release binaries

## [0.5.0] - 2026-09-03

### Added

- Add `evals/`: one `eval-case/1` per agent and user-facing skill, run by `aep eval run --corpus evals`
- `agentplugins-check` validates the eval corpus; `task evals` runs that half alone
- Replay recorded transcripts with `aep eval run --stream`; a stream without its run manifest is refused
- Add `.github/workflows/eval.yml`: a label-gated, diff-scoped live eval under `EVAL_BUDGET_USD`
- Add `README.md` § Evals with the cost arithmetic
- Golden path step 3 models the new noun before decomposing it
- Add a `drive` skill to `adp`: `/drive <story-id>` launches `aep drive run` with a stated budget

### Changed

- Record no transcript and synthesize none; each case's `recorded/` names the live command that fills it
- Pin the four plan critics to `model: sonnet` and `effort: high`; `agentplugins-check` refuses a missing pin
- The wave selects on `aep artifact waves`; when it and the prose reading disagree, the verb wins
- Write a story's scope twice: the `## Scope` section and `aep artifact scope --add` entries
- Record a `review_outcome` for every review finding
- The critic rubric and the adversary end their reports with a ` ```findings ` block
- A domain relation is a `relations:` entry in an `ess/1` document
- The decomposer's `inferable` citation for a relation points at an `ess/1` document

## [0.4.0] - 2026-09-02

### Added

- Add a golden-path page: a recorded run from a feature idea to a critiqued plan
- Add a plan-time critic panel of four read-only critics to `aep-planning`
- Add a domain-first guardrail to the planning skill
- Adopt a planning store of this repository's own under `.engineering/`

### Changed

- `website/docs/install.md` and `README.md` have one copy-paste install block
- The decomposer enumerates the epic's domain relations before drafting any story
- Widen `ess-schema` to fire when a story or epic introduces an entity
- The planning skill's first-store instruction is `aep artifact list`

## [0.3.7] - 2026-09-02

### Changed

- Teach both skills AEP's `refs:` field (AEP 0.41.0)

### Fixed

- Correct `story-migration`: a re-run is refused, not replayed
- Correct the version line in `planning` and `adp/wave`

## [0.3.6] - 2026-09-02

### Added

- Add the `story-migration` skill to `aep-planning`

### Changed

- Migrated artifacts keep each source's dates, read from git
- Point `planning` § 5 at it

## [0.3.5] - 2026-09-02

### Changed

- Refresh the generated `worktree` skill: `gc --repo` selects the workspace profile; `status` takes no filter
- Pin the public installation guidance to the matching Worktree 0.3.2 release

## [0.3.4] - 2026-09-02

### Changed

- Pin the documentation redirect façade to the 21-source Website verifier runtime

## [0.3.3] - 2026-09-02

### Changed

- Refresh the generated `worktree` skill for guarded recovery of stranded pre-0.3 legacy trees
- Pin the public installation guidance to the matching Worktree 0.3.1 safety release

## [0.3.2] - 2026-09-02

### Changed

- Refresh the generated `worktree` skill for review-bound cleanup and removal recovery
- Pin the public installation guidance to the matching Worktree 0.3.0 safety release

## [0.3.1] - 2026-09-01

### Fixed

- GitHub release publication is idempotent, so rerunning a successful tag publication keeps the release

## [0.3.0] - 2026-09-01

### Added

- Add the `workspace-hygiene` plugin with the `worktree` CLI-generated skill
- Document installation and the separation between lifecycle guidance and the Rust toolchain

## [0.2.0] - 2026-09-01

### Added

- Add the `beyond10x` front-door plugin for routing, resource discovery and plugin creation

## [0.1.1] - 2026-09-01

### Fixed

- Correct the marketplace documentation relationship to the public AEP Service documentation surface

## [0.1.0] - 2026-09-01

### Added

- Publish the curated `beyond10x` marketplace with focused AEP planning, ADP, and ESS schema plugins
- Publish adopter documentation for discovering, selecting, and installing each plugin

[Unreleased]: https://github.com/beyond10x/agentplugins/compare/0.24.2...HEAD
[0.24.2]: https://github.com/beyond10x/agentplugins/compare/0.24.1...0.24.2
[0.24.1]: https://github.com/beyond10x/agentplugins/compare/0.24.0...0.24.1
[0.24.0]: https://github.com/beyond10x/agentplugins/compare/0.23.2...0.24.0
[0.23.2]: https://github.com/beyond10x/agentplugins/compare/0.23.1...0.23.2
[0.23.1]: https://github.com/beyond10x/agentplugins/compare/0.23.0...0.23.1
[0.23.0]: https://github.com/beyond10x/agentplugins/compare/0.22.11...0.23.0
[0.22.11]: https://github.com/beyond10x/agentplugins/compare/0.22.10...0.22.11
[0.22.10]: https://github.com/beyond10x/agentplugins/compare/0.22.9...0.22.10
[0.22.9]: https://github.com/beyond10x/agentplugins/compare/0.22.8...0.22.9
[0.22.8]: https://github.com/beyond10x/agentplugins/compare/0.22.7...0.22.8
[0.22.7]: https://github.com/beyond10x/agentplugins/compare/0.22.6...0.22.7
[0.22.6]: https://github.com/beyond10x/agentplugins/compare/0.22.5...0.22.6
[0.22.5]: https://github.com/beyond10x/agentplugins/compare/0.22.4...0.22.5
[0.22.4]: https://github.com/beyond10x/agentplugins/compare/0.22.3...0.22.4
[0.22.3]: https://github.com/beyond10x/agentplugins/compare/0.22.2...0.22.3
[0.22.2]: https://github.com/beyond10x/agentplugins/compare/0.22.1...0.22.2
[0.22.1]: https://github.com/beyond10x/agentplugins/compare/0.22.0...0.22.1
[0.22.0]: https://github.com/beyond10x/agentplugins/compare/0.21.2...0.22.0
[0.21.2]: https://github.com/beyond10x/agentplugins/compare/0.21.1...0.21.2
[0.21.1]: https://github.com/beyond10x/agentplugins/compare/0.21.0...0.21.1
[0.21.0]: https://github.com/beyond10x/agentplugins/compare/0.20.1...0.21.0
[0.20.1]: https://github.com/beyond10x/agentplugins/compare/0.20.0...0.20.1
[0.20.0]: https://github.com/beyond10x/agentplugins/compare/0.19.2...0.20.0
[0.19.2]: https://github.com/beyond10x/agentplugins/compare/0.19.1...0.19.2
[0.19.1]: https://github.com/beyond10x/agentplugins/compare/0.19.0...0.19.1
[0.19.0]: https://github.com/beyond10x/agentplugins/compare/0.18.0...0.19.0
[0.18.0]: https://github.com/beyond10x/agentplugins/compare/0.17.0...0.18.0
[0.17.0]: https://github.com/beyond10x/agentplugins/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/beyond10x/agentplugins/compare/0.15.0...0.16.0
[0.15.0]: https://github.com/beyond10x/agentplugins/compare/0.14.17...0.15.0
[0.14.17]: https://github.com/beyond10x/agentplugins/compare/0.14.16...0.14.17
[0.14.16]: https://github.com/beyond10x/agentplugins/compare/0.14.15...0.14.16
[0.14.15]: https://github.com/beyond10x/agentplugins/compare/0.14.14...0.14.15
[0.14.14]: https://github.com/beyond10x/agentplugins/compare/0.14.13...0.14.14
[0.14.13]: https://github.com/beyond10x/agentplugins/compare/0.14.12...0.14.13
[0.14.12]: https://github.com/beyond10x/agentplugins/compare/0.14.11...0.14.12
[0.14.11]: https://github.com/beyond10x/agentplugins/compare/0.14.10...0.14.11
[0.14.10]: https://github.com/beyond10x/agentplugins/compare/0.14.9...0.14.10
[0.14.9]: https://github.com/beyond10x/agentplugins/compare/0.14.8...0.14.9
[0.14.8]: https://github.com/beyond10x/agentplugins/compare/0.14.7...0.14.8
[0.14.7]: https://github.com/beyond10x/agentplugins/compare/0.14.6...0.14.7
[0.14.6]: https://github.com/beyond10x/agentplugins/compare/0.14.5...0.14.6
[0.14.5]: https://github.com/beyond10x/agentplugins/compare/0.14.4...0.14.5
[0.14.4]: https://github.com/beyond10x/agentplugins/compare/0.14.3...0.14.4
[0.14.3]: https://github.com/beyond10x/agentplugins/compare/0.14.2...0.14.3
[0.14.2]: https://github.com/beyond10x/agentplugins/compare/0.14.1...0.14.2
[0.14.1]: https://github.com/beyond10x/agentplugins/compare/0.14.0...0.14.1
[0.14.0]: https://github.com/beyond10x/agentplugins/compare/0.13.1...0.14.0
[0.13.1]: https://github.com/beyond10x/agentplugins/compare/0.13.0...0.13.1
[0.13.0]: https://github.com/beyond10x/agentplugins/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/beyond10x/agentplugins/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/beyond10x/agentplugins/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/beyond10x/agentplugins/compare/0.9.3...0.10.0
[0.9.3]: https://github.com/beyond10x/agentplugins/compare/0.9.2...0.9.3
[0.9.2]: https://github.com/beyond10x/agentplugins/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/beyond10x/agentplugins/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/beyond10x/agentplugins/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/beyond10x/agentplugins/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/beyond10x/agentplugins/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/beyond10x/agentplugins/compare/0.6.2...0.7.0
[0.6.2]: https://github.com/beyond10x/agentplugins/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/beyond10x/agentplugins/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/beyond10x/agentplugins/compare/0.5.1...0.6.0
[0.5.1]: https://github.com/beyond10x/agentplugins/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/beyond10x/agentplugins/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/beyond10x/agentplugins/compare/0.3.7...0.4.0
[0.3.7]: https://github.com/beyond10x/agentplugins/compare/0.3.6...0.3.7
[0.3.6]: https://github.com/beyond10x/agentplugins/compare/0.3.5...0.3.6
[0.3.5]: https://github.com/beyond10x/agentplugins/compare/0.3.4...0.3.5
[0.3.4]: https://github.com/beyond10x/agentplugins/compare/0.3.3...0.3.4
[0.3.3]: https://github.com/beyond10x/agentplugins/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/beyond10x/agentplugins/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/beyond10x/agentplugins/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/beyond10x/agentplugins/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/beyond10x/agentplugins/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/beyond10x/agentplugins/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/beyond10x/agentplugins/releases/tag/0.1.0
