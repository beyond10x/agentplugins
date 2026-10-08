---
name: investigating
description: >-
  Investigate a production incident, an outage or a question about a running system from evidence that can be cited — capture state before anyone remediates, build a sourced UTC timeline, date an onset from an instrument that can see a negative, compare against a healthy peer, and label every claim verified or inferred, with a catalogue of ten techniques. Use when the user says investigate, "what happened", "when did this start", "is it still happening", "has this shipped", "is this deployed", reports an alert, an outage, a hung or crashing process or a customer-visible failure, or asks for an incident report or a postmortem. Not for a defect that can be reproduced on demand, which is `aep:diagnosing`; not for checking a change before it merges, which is `aep:implementing`.
---

**Skill version 0.22.2** — the version in `.claude-plugin/plugin.json`.

# Investigating a live system

`aep:diagnosing` starts by building a command that goes red on the bug. A live incident has no such
command: it happened once, on a system that keeps moving, and the evidence is destroyed by the
restart that ends it. Here the work is **keeping the evidence and saying where each fact came
from**, because nothing can be re-run later to check it.

Redact every secret and every personal identifier (a phone number, an email address, an account
holder's name) in what you show and what you write: `<REDACTED>` in its place.

## The rule that makes any of this count

**Every specific is a claim, and every claim carries its source.** A time, a count, a version, a
duration, a name and a "nothing happened" are each claims. Next to each one, write the command whose
output it came from, or the file and line. A claim you did not read from a command or a file is
labelled **inferred**, or it is not written.

Two consequences:

- **An absence is a claim like any other.** "No alert fired", "no tag exists" and "nobody asked"
  need the query that returned nothing, run against the system that owns the answer — technique 5.
- **An unverified detail that makes the finding look bigger is the one to distrust first.** That is
  the direction plausible-but-wrong details run in.

## The catalogue

| # | technique | the question | what it prevents |
|---|---|---|---|
| 1 | capture before remediation | what state exists now that a restart will destroy? | a root cause that becomes unknowable because the hung process was restarted before anyone looked |
| 2 | sourced timeline | what happened, in which order, according to what? | an effect placed before its cause by a time-zone slip; a timeline nobody can re-check |
| 3 | onset from state | when did this really start? | an onset dated from the first delivered alert, weeks after the condition began |
| 4 | peer differential | does this signal also appear on something healthy? | a routine warning mistaken for the cause because it was the loudest line in the suspect's log |
| 5 | negative claims | does this really not exist / not happen? | "no fix exists" while the fix was merged; a delta scan read as an inventory |
| 6 | ship state | is the change actually running there? | a ticket status, a version range or an image tag name read as deployment |
| 7 | hypothesis ledger | which claims are observed and which are read off code? | a cause, a frequency or a severity asserted from reading a code path |
| 8 | check the check | what does this green probe, gate or scanner actually cover? | "Ready" or "exit 0" cited as evidence about something the check never looked at |
| 9 | complete reads | did the read return everything? | a truncated read treated as the whole file, and written back |
| 10 | post-incident | what was the impact, why was it not detected, and what was lost? | follow-ups that live only in a chat thread |

Procedures, with the commands and the failure each one was written from:
[references/techniques.md](references/techniques.md).

## The order

1. **Is it still happening?** If yes, run **technique 1 first**, before any other reading and before
   anyone restarts, redeploys, drains or kills anything. Remediation is the operator's decision;
   capture takes seconds and is yours. If someone has already remediated, say so, and record what
   was lost.
2. **Open the record.** In a repository with an AEP store:

   ```console
   $ aep plan artifact new incident-report <slug> --title "<the symptom, in the reporter's words>"
   ```

   Outside one, a markdown file in the project's incident location. Either way the capture from step
   1 goes beside it.
3. **Timeline** (2), then **onset** (3). The timeline is the spine every later step writes onto.
4. **Peer differential** (4) on every anomaly the timeline turns up, before it becomes a hypothesis.
5. **Hypothesis ledger** (7): 3–5 ranked candidate causes, each with the observation that would
   refute it. Test with read-only probes. When a hypothesis leads to code that can be exercised,
   hand that part to `aep:diagnosing`.
6. **Techniques 5, 6, 8 and 9** whenever a claim of their kind is about to be written: an absence, a
   ship state, a green check, or a read that feeds a decision.
7. **Post-incident** (10) once the system is stable.

Record each load-bearing observation against the incident as it is made:

```console
$ aep plan artifact evidence incident-report:<slug> --kind health_observation \
    --source "<the command>" --ref "<the capture file or a URL>" --at <instant, UTC>
```

`metric_observation` for a metric series, `deployment_result` for what a deployment actually ran.

## Boundaries

- **Read-only by default.** A read of a live system (logs, metrics, process state, an API `GET`)
  is yours to run. A restart, a rollback, a scale change, a configuration write, a post to a shared
  channel or a ticket write is the operator's: propose it, with the evidence, and wait.
- A debugger attached to a live process pauses it. Say so, and prefer a non-pausing read
  (`/proc/<pid>/task/*/stack`, a runtime's own dump signal) when the process is still serving.
- Never put a secret or a personal identifier into a query that is logged, a file that is committed
  or a message that is posted.

## Reporting

Verdict first: **resolved / ongoing / unknown**, and **root cause known / hypothesis / unknown**.
Then the timeline table, each row with its source. Then the ledger: verified claims, inferred claims,
refuted hypotheses. Then what evidence was lost, and the detection gap. Then the follow-ups, each one
an artifact:

```console
$ aep plan artifact new story <slug> --title "<the gap>" --relate informed_by:incident-report:<slug>
```

## Next

- A hypothesis points at code that can be exercised: `aep:diagnosing`.
- A follow-up story is ready to build: `aep:implementing`.
- The incident needs a written postmortem: technique 10, then
  `aep plan artifact new postmortem <slug> --title "<title>" --relate derived_from:incident-report:<slug>`.
