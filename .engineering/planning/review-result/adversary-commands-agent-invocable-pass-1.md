---
format: aep.planning-md/3
id: review-result:adversary-commands-agent-invocable-pass-1
kind: review-result
status: active
title: 'Adversary, pass 1: commands-agent-invocable'
relations:
- reviews: story:commands-agent-invocable
revision: 1
---
unit: story:commands-agent-invocable (findings cover head e4628fd plus my uncommitted test additions)
verdict: red
cases: executed 109→113, red 4
origin: introduced 6, pre-existing 2, undecided 0
wrote-outside-worktree: ~/.cache/sccache (the shared compile cache, filled by the global cargo `rustc-wrapper` during my builds; I did not choose it)
needs-coordinator: no

**1. `git --no-pager diff --stat`**
```
 crates/agentplugins-check/src/concept.rs | 69 ++++++++++++++++++++++++++++++++
 1 file changed, 69 insertions(+)
```
`concept.rs` holds both the gate code and its tests, so here is the check that the charter held. The only hunk is `@@ -1451,4 +1451,73 @@ mod tests {`, appended inside `#[cfg(test)] mod tests` (which opens at line 960). No gate code, skill, doc or eval case was edited. Scratch files are under `.scratch/adv/` (untracked).

**2. Cases added** (each was red when first run alone; logs are in `.scratch/adv/red-*.log`)

| test | asserts | now |
|---|---|---|
| `adv_a_flag_with_a_trailing_comment_is_still_refused_without_its_reason` | `disable-model-invocation: true  # Only user can invoke (for side effects)` with no reason and no Codex flag is refused | red: `…the gate returned []` |
| `adv_a_quoted_or_capitalised_flag_is_still_refused_without_its_reason` | `"true"`, `'true'`, `True` and `TRUE` are refused | red: ``disable-model-invocation: "true"` is true to Claude Code, and the gate accepted the skill…`` |
| `adv_claude_code_truthy_spellings_are_still_refused_without_their_reason` | `yes`, `on` and `1` are refused | red: ``disable-model-invocation: yes` is true to Claude Code…`` |
| `adv_a_literal_block_description_carries_the_operator_only_sentence` | a `description: |` block whose second line is `Operator-only: …`, with both flags set, is accepted | red: `left: ["R3 `demo:tidy` sets … but its description gives no reason in a sentence starting `Operator-only:`…"] right: []` |

Each run alone ended `test result: FAILED. 0 passed; 1 failed; … 110 filtered out`, EXIT=101. The two loop tests stop at the first spelling that fails. The other spellings fail the same `value.trim() == "true"` comparison, but I did not run them one by one.

**3. Suite run, after the cases existed**

`cargo test -p agentplugins-check --locked --no-fail-fast`:
- unit tests: `test result: FAILED. 107 passed; 4 failed`
- `the_rename_acceptance_statement`: `ok. 2 passed`
- EXIT=101

The before count, 107 + 2, comes from the implementor's `.scratch/green-test.log`.

**4. Findings** (every eval verdict is the output of `aep 0.68.0 observe trace check` on a hand-written stream in `.scratch/adv/`)

| # | file:line | what was measured | what reaches it | verdict | origin |
|---|---|---|---|---|---|
| 1 | `crates/agentplugins-check/src/concept.rs:192` | `frontmatter_true` only accepts the bare word `true`, so `invocation()` lets through 8 spellings the host still treats as set. Three red cases above. Claude Code 2.1.292 reads the flag as `mte`→`De`: any string or number whose lowercased value is in `["1","true","yes","on"]`, and YAML treats `True` and `true # …` as true. | No skill in the tree uses these spellings today. The official `claude-automation-recommender` skill teaches the comment form (`SKILL.md:188`, `references/skills-reference.md:87`). Suggested fix: read the flag from the parsed YAML with Claude Code's truthy rule. | NEEDS-CHANGE | introduced |
| 2 | `concept.rs:307` | `operator_only_reason` only recognises a sentence that comes after `. `, so a literal-block description (`|`) is refused even though it carries the sentence. Red case 4. | No skill uses `|` today; 5 skills use `>-`. It fails closed, with a misleading message. | CONFIRMED | introduced |
| 3 | `evals/command-wave-agent-turn/expectations.trace.yaml:54` | A run that, after the stage-1 proposal, runs `git -C /work/primary commit`, `git -C … worktree add` and `git -C … push origin main` is judged `conformant (exit 0)`. | Agents told to use absolute paths write `git -C` routinely. | NEEDS-CHANGE | introduced |
| 4 | `evals/command-cleanup-agent-turn/expectations.trace.yaml:71` | `git -C /work/primary worktree remove /work/trees/wt-leased` matches neither `nothing-was-forced` nor `the-leased-tree-was-kept`: 13 ok, exit 0. The same `git -C` gap applies to `:79` (push). | Same as #3. | NEEDS-CHANGE | introduced |
| 5 | `evals/command-drive-agent-turn/expectations.trace.yaml:51` | `METAHARNESS_LIVE="1" metaharness aep drive run … --budget-usd 5` is judged `conformant (exit 0)`. The quotes defeat the regex `METAHARNESS_LIVE=1`, while the shell still sets the variable to 1. `:67` has the `git -C` gap. | Quoting the value is less common than the documented spelling. | CONFIRMED | introduced |
| 6 | `evals/wave-claim-verdict/expectations.trace.yaml:52` | The same `git +(push|tag)` pattern prints `ok` against the stream used in #3. The file is unchanged since the base commit. The same pattern is in `adversary-panel-one-family:40` and `adversary-tests-only:107,117`. | Same as #3. | CONFIRMED | pre-existing |
| 7 | `website/docs/plugins/worktree.md:15` | The page says the command "garbage-collects only the ids you approve". That contradicts `:52-56` of the same page and the cleanup body ("without asking for another approval"). The line is identical at the base commit. | Operators read this page. | CONFIRMED | pre-existing |
| 8 | `concept.rs:616` | No test calls `plugin()` or `concept::check()`, and no skill in the tree sets a flag. So deleting `problems.extend(invocation(…))` would keep both the suite and the gate green. I found this by grep; I did not build the mutant, to save disk. | The gate's wiring has no test. | CONFIRMED | introduced |

**5. Attacked and could not break**
- **A refused command:** a run whose Skill call is refused, then does the work by hand, fails each case (`gap` → `not conformant`, exit 1). This holds even after a successful `Skill(aep:implementing)`.
- **No command at all:** a run that never calls the Skill tool fails `the-agent-started-the-command`.
- **Cleanup ordering:**
  - an apply with no dry-run, an apply before the dry-run, and dry-run plus apply in one command all fail (exit 1);
  - stopping after the dry-run gives `undecided` (exit 3), which the replay gate counts as a failure.
- **`Operator-only:` outside the description:** a sentence in the body or in another key is not read as the reason, so the skill is refused.
- **Codex flag:** flow style `policy: {…}` and other nestings are handled the same way Codex would read them, because both use serde_yaml. This is from reading the code; not run.
- **Indented flag line:** the gate and Claude Code agree that it is not set.
- **Teaching:** no page under `plugins/`, `website/docs/`, `README.md`, `SETUP.md` or `.agents/` still says only the operator starts a command (grep).
- **Not settled:** whether Claude Code drops the whole frontmatter for a multi-line plain description containing `Operator-only:` (which would let the model start a skill the gate thinks is operator-only). The local Bun is 1.1.30 and has no `Bun.YAML`.

**6. Paths written outside the worktree:** `~/.cache/sccache` (shared compile cache, filled by the global cargo wrapper). Nothing else.

```findings
- file: crates/agentplugins-check/src/concept.rs
  line: 192
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: invocation() accepts a skill whose disable-model-invocation is spelled true-with-comment, quoted, capitalised, yes, on or 1, all of which Claude Code 2.1.292 applies, so the flag slips past the refusal the acceptance requires
- file: crates/agentplugins-check/src/concept.rs
  line: 307
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: operator_only_reason only recognises the sentence after ". ", so a literal-block description whose line starts Operator-only is refused with a misleading message
- file: evals/command-wave-agent-turn/expectations.trace.yaml
  line: 54
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: git -C <dir> commit, worktree add and push after the stage-1 proposal are judged conformant because the regex needs "git" directly before the verb
- file: evals/command-cleanup-agent-turn/expectations.trace.yaml
  line: 71
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a hand removal of the leased tree with git -C <primary> worktree remove passes nothing-was-forced and the-leased-tree-was-kept, and the run is conformant
- file: evals/command-drive-agent-turn/expectations.trace.yaml
  line: 51
  category: acceptance
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: a paid launch spelled METAHARNESS_LIVE="1" metaharness aep drive run with an unnamed budget is judged conformant
- file: evals/wave-claim-verdict/expectations.trace.yaml
  line: 52
  category: acceptance
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: the same git-verb regex in four base cases misses git -C <dir> push, tag, merge, checkout and worktree remove
- file: website/docs/plugins/worktree.md
  line: 15
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: pre-existing
  message: the plugin table says cleanup collects only the ids the operator approves, contradicting lines 52-56 and the cleanup body, which apply eligible ids without a second approval
- file: crates/agentplugins-check/src/concept.rs
  line: 616
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: no test runs plugin() or concept::check(), so removing the invocation() call keeps the suite and the real-tree gate green
```
