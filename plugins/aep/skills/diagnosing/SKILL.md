---
name: diagnosing
description: Diagnose a hard bug or a performance regression by building a red-capable feedback loop before any hypothesis, then ranked falsifiable hypotheses, one-variable probes, a regression test at the right seam, and evidence recorded in the AEP store. Use when the user says diagnose, debug, "why is this failing", "this is slow", or reports something broken, throwing, flaky or slower than before. Not for a production incident that cannot be re-run, which is `aep:investigating`; not for a failing CI job whose cause is already named in its log; and not for raising conformance coverage, which is `ess:testing-conformance`.
---

**Skill version 0.20.0** — the version in `.claude-plugin/plugin.json`.

# Diagnosing a failure

The work here is building the loop. With a command that goes red on this bug and green without it,
bisection, hypotheses and instrumentation have something to run against. Without one, reading code
produces theories nobody can check.

Redact every secret in what you show: write `<REDACTED>` in its place, and build loops that read
credentials from the environment rather than from arguments.

## 1. Build a red-capable loop

Find one command that exercises the reported symptom. Try, roughly in this order:

1. a failing test at whichever seam reaches the bug;
2. an HTTP request against a running local instance;
3. a CLI invocation on a fixture input, diffed against a known-good output;
4. a headless browser script that asserts on the DOM, the console or the network;
5. a captured request, payload or event log replayed through the code path in isolation;
6. a minimal harness that boots only the part of the system the bug needs;
7. a property or fuzz loop over many generated inputs, for "sometimes wrong";
8. `git bisect run` over a boot-and-check command, when the bug appeared between two known states;
9. the same input through the old and the new version, outputs diffed.

When a person must act (click, approve, plug something in), write the steps as a numbered list the
person follows and pastes output back from, so the loop keeps its shape.

Then tighten it. Make it faster by skipping unrelated setup. Make the assertion name the user's
exact symptom rather than "did not crash". Make it deterministic: pin the clock, seed randomness,
isolate the filesystem, stub the network. For a flaky bug, raise the reproduction rate — repeat
the trigger, add load, narrow the timing window — until it fails often enough to debug.

**This step is done when you can name one command, and you have already run it and shown its
output, that is:**

- **red-capable** — it drives the real code path and asserts the reported symptom;
- **deterministic** — the same verdict every run, or a pinned high rate for a flaky bug;
- **fast** — seconds, not minutes;
- **unattended** — it runs without a person, or through the numbered-steps form above.

Record the red run against the story that owns the fix:

```console
$ aep plan artifact evidence <story-id> --kind test_result --source "<the command>" --at <instant>
```

Until that command exists, do not form a hypothesis. If you cannot build one, stop and say what you
tried, then ask for access to an environment that reproduces it, a redacted captured artifact (a HAR
file, a log, a core dump), or permission to add temporary instrumentation.

## 2. Reproduce and minimise

Run the loop and confirm it fails with the symptom the user reported, not a different failure
nearby. Then cut inputs, callers, configuration and steps one at a time, re-running after each cut,
until removing any remaining element makes the loop go green. The smaller the repro, the fewer
places the cause can be, and the minimal repro becomes the regression test.

## 3. Hypothesise

Write 3–5 hypotheses, ranked, before testing any. Each states its prediction: "if X is the cause,
changing Y makes the bug disappear, and changing Z makes it worse". A hypothesis with no prediction
is discarded or sharpened. Show the ranked list to the user if one is present — they often know
which one was ruled out last week — and continue with your ranking if nobody answers.

## 4. Probe one variable at a time

Each probe tests one prediction. Prefer a debugger or REPL breakpoint; otherwise add targeted logs at
the boundaries that separate the hypotheses. Prefix every debug line with one tag, such as
`[DEBUG-a4f2]`, so removing them is a single search. For a performance regression, measure a baseline
first and bisect against it; logs mislead there.

## 5. Fix with a regression test at the right seam

Write the regression test before the fix, at a seam where the test reproduces the bug the way it
happens at the real call site. A test at a shallower seam — one caller where the bug needs two, a
unit that cannot build the chain that triggered it — passes for the wrong reason.

**A missing seam is itself the finding.** File it as a draft story named after the seam that is
missing, so the architecture that stopped the bug being locked down is on the plan:

```console
$ aep plan artifact new story <seam-slug> --title "<the seam that is missing>" \
    --relate informed_by:<the story being debugged>
```

With a seam: turn the minimal repro into the failing test, watch it fail, fix, watch it pass, then
re-run the step 1 loop against the original, un-minimised scenario. Record the green run the same
way as the red one.

## 6. Clean up

Before reporting done:

- the step 1 loop no longer reproduces;
- the regression test passes, or the missing seam is filed;
- a search for the debug tag finds nothing;
- throwaway harnesses are deleted or moved to a directory named as debug scaffolding;
- the hypothesis that turned out right is stated in the commit message.

---

The loop-first order and the phase structure are adapted from `skills/engineering/diagnosing-bugs`
in `github.com/mattpocock/skills` (MIT).
