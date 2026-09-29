# Investigation techniques

The procedures behind the catalogue in [`SKILL.md`](../SKILL.md). Each technique names the failure
it was written from, what to run, and when it is done. The commands are examples for common
platforms (Linux, Kubernetes, Prometheus-style metrics, Git). Use the equivalent instrument where the
system differs, and name which one you used.

## 1. Capture before remediation

**Written from:** a process stopped serving calls while its health probe stayed green. The report
reached the operator and the pod was restarted about four minutes later. It was gone within a
minute, and with it every thread and lock state. The cause of the hang cannot now be determined.

**When:** the faulty process is still running, whether hung, leaking, spinning or misrouting.

Write everything to one capture file (`capture-<UTC timestamp>.txt` beside the incident record),
**in this order**, cheapest and most perishable first. Give each step a timeout, so one hung read
does not use up the window.

1. **Identity and placement:** process id, host or pod, node, address, image and digest, start
   time. (`kubectl get pod <pod> -o wide`, and `-o jsonpath` for
   `.status.containerStatuses[*].imageID`.)
2. **Per-thread state, without pausing the process:**

   ```console
   $ for t in /proc/<pid>/task/*; do echo "$(cat $t/comm) $(cat $t/wchan)"; done | sort | uniq -c | sort -rn
   $ cat /proc/<pid>/task/*/stack        # needs privilege; skip if refused, and say so
   $ grep -E 'State|Threads|VmRSS' /proc/<pid>/status
   ```

   Many threads parked on one `futex` or wait channel is the signature of a lock wait. That points
   toward a deadlock and does not prove one.
3. **The runtime's own dump**, when it has one: a thread dump (`jstack <pid>`, `kill -3`), a
   goroutine dump (`SIGQUIT`, or `/debug/pprof/goroutine?debug=2`), an application lock table
   (a PBX's `core show locks`, a database's lock view). Check first whether the signal kills the
   process. `SIGQUIT` ends a Go program.
4. **A full backtrace, only if the process is already lost for service:**
   `gdb -p <pid> -batch -ex 'thread apply all bt'`. It pauses the process while it runs. If the
   image carries no debugger, record that as a finding.
5. **Resource usage, against a healthy peer:** CPU, memory, open files, thread count
   (`kubectl top pod --containers`, `ls /proc/<pid>/fd | wc -l`).
6. **Logs:** the last few thousand lines with timestamps, from every container in the unit
   (`kubectl logs <pod> -c <container> --timestamps --since=<window>`), plus `--previous` when a
   container has restarted. Logs in a central store survive the pod, so read them there later. The
   ones only on the node do not.
7. **Orchestrator events and description:** `kubectl describe pod <pod>`,
   `kubectl get events --sort-by=.lastTimestamp`.

Then tell the operator the capture is done, with the file path, so remediation can go ahead. If
remediation happened first, write down what was not captured. That gap is part of the finding.

**Done when:** the capture file exists and says which of the seven steps ran, which were refused,
and why.

## 2. Sourced timeline

**Written from:** an alert burst was placed two hours after the deploy that caused it, because one
source was read in local time and another in UTC. The real outage was nearly written off as
coincidence.

One table, oldest first, **all times UTC and labelled so**:

| time (UTC) | event | source |
|---|---|---|
| 07:09:08 | last per-request log line from the suspect | `kubectl logs … --timestamps` |

- Convert every epoch explicitly (`date -u -d @<seconds>`). Chat and message-queue timestamps are
  usually epochs, and a screenshot's wall clock is in somebody's local zone. Never copy a time out
  of a person's or a bot's prose. Read it from the record.
- A row without a source is not a row. When a person reported something, the source is
  "<person>, in <where>", and the content is their claim, not a fact.
- Include what *did not* happen when it matters, with the query that showed it, such as "probe
  stayed Ready until 09:05".
- Give an announced time (a maintenance window, a scheduled deploy) its own row, labelled
  *announced*. The actual event gets the time the system recorded, such as a merge time or a
  process start time. These have differed by more than a day.

**Done when:** every row has a source, and every time is UTC.

## 3. Onset from state

**Written from:** a condition was dated to the first day an alert about it was delivered. The
underlying counter showed it had been happening every 48 hours for more than three weeks before
that. Roughly nineteen occurrences had produced no alert.

- **An alert, a notification or a digest cannot give you an onset.** It shows events that were
  produced *and* delivered. Its silence says nothing about the condition. A rule may not have
  existed yet, may not have been routed, or may have been rate-limited.
- Find the instrument that can see a negative. A counter (`…_restarts_total`), a state
  (`…_last_terminated_reason`), a metric series, the repository for a merge, the cluster for a
  deploy.
- **Query back past the apparent onset**, not up to it. If the condition is already present at the
  start of your window, the onset is "before <window start>". Say that, and do not report the
  window start as the onset.
- **Name the datasource before you accept a retention limit.** "Logs only go back 30 days" belongs
  to one store. Another one in the same system may hold 49 days.
- If the stream and the state disagree, the difference is a finding in its own right: other alerts
  of that class may be missing too.

**Done when:** the onset is stated as "at or before <time>", with the instrument and the query window
named.

## 4. Peer differential

**Written from:** a hung node's log was full of one repeated warning, and it read as the cause. A
healthy peer in the same deployment logged 115 of the same warning in the same four hours.

For every anomaly on the suspect, run the same read on at least one healthy peer (same version,
same role, same window) before it enters the ledger:

| signal | suspect | healthy peer | counts as evidence? |
|---|---|---|---|
| warning X per 4 h | 120 | 115 | no |
| memory | 940 MiB | 176 MiB | yes |

A signal that the peers share is background. If there is no healthy peer (a single instance, or all
instances affected), say so. The differential is then against the same instance's own earlier
window.

**Done when:** every anomaly in the ledger has its peer reading beside it.

## 5. Negative claims

**Written from:** three separate failures. First, "no release names this fix" was written while two
tags carrying it had existed for seventeen minutes. The scanner that was read only reports what
moved since its last run. Second, "nobody has asked for this" was written about a request made
twice in the same chat thread, because only one reply of the thread was in the reader's slice.
Third, "no content" in two thousand captured messages turned out to be a capture that never read
the field the content lived in.

Before writing that something does not exist, did not happen or was never asked:

1. **Query the system that owns the answer, directly**, and say that you did: the tag list, the
   merge request, the release page, the full thread.
2. **A delta is not an inventory.** Anything incremental (a scan past a watermark, a digest, a
   "since last run" slice) can only say "not in this window".
3. **A reply needs its thread.** Read the whole thread, not the one message and its root.
4. **If a whole source looks empty or uniform, suspect the capture first.** Uniform emptiness from a
   busy source usually means the reader is broken, not the source.
5. **A bot's or a person's "none exist" is a claim to check.** It is not a source.

**Done when:** each absence in the report names the direct query that returned nothing.

## 6. Ship state

**Written from:** three failures. First, a declared version range was read as current behaviour
("a downgrade is queued for four environments"). When measured, the four ran four different
versions, and the new one had reached none of them, because the range resolves only when the
environment next syncs. Second, a ticket in "Backlog" was read as "no code exists" while three
merges and four tags carried the fix. Third, "deployed nowhere" was concluded by comparing tag
*names* while the fix was running.

Four states, four systems. Never infer one from another:

| state | system of record | the read |
|---|---|---|
| intended | the tracker | the ticket status. It records intent only |
| implemented | every repository that could carry it | merged changes that name it, in *all* candidate repositories |
| releasable | tags or releases | the tags that contain the merge (`git tag --contains <sha>`) |
| deployed | the running system | the digest the process actually runs, from the pod status and not the spec's tag |

For "deployed", follow the provenance chain: running digest → the registry's tags on that digest →
keep only an immutable reference (a version tag or a commit SHA, never a moving name like
`latest`) → ancestry check against the fix commit (`git merge-base --is-ancestor <fix> <ref>`).
Use the registry's push time as a bound. An image pushed before the fix commit cannot contain it.

Distinguish **deployed nowhere** (provenance resolved everywhere and absent) from **unknown**
(provenance did not resolve somewhere). Name which environment did not resolve.

**Done when:** each ship claim names its state and the read that established it.

## 7. Hypothesis ledger

**Written from:** a review stated that a defect fired "on every run" by reading the branch that
reaches it. Runtime data showed at most once a day. The severity had been set from the code-read
number.

Keep three lists, and put every claim in the report into exactly one of them:

- **Verified:** observed, with the source (technique 2's rule).
- **Inferred:** follows from verified facts or from reading code, and has not been observed. It is
  labelled *inferred* wherever it appears.
- **Refuted:** a hypothesis, the observation that refuted it, and its source.

Rules:

- **Code-path inference is a hypothesis.** It becomes a finding only with a runtime observation:
  a log line, a counter or a trace of one instance end to end.
- **Before blaming a code path, find its callers.** "Who calls this, and does that caller run
  here?" rules out many theories quickly.
- **Frequency and severity come from runtime data only.**
- **"This cannot happen" needs every form enumerated.** Loops, retries, watchers, timers and
  recursion, each with its exit condition read. One keyword search does not show that something
  cannot happen.
- **Quote a vague statement verbatim, and mark it "unclear".** Do not invent a mechanism that
  makes it make sense.
- Write 3–5 ranked hypotheses, each with the observation that would refute it, before probing any.

**Done when:** no claim in the report sits outside the three lists.

## 8. Check the check

**Written from:** two failures. First, a health probe reported Ready for two hours while the
process served no requests, so the platform never replaced it. Second, a drift gate exited 0 and
was cited as confirming a sentence it had never extracted, because it resolved references in
qualified form only and the sentence used the short form.

Before citing a probe, a gate, a scanner or a monitor as evidence:

1. **What does it read?** A readiness endpoint that answers from a sidecar says nothing about the
   main process. A gate that iterates the entries it finds says nothing about the entries it did
   not find.
2. **State its denominator.** "36 of 36 clean" when 56 are declared is a check over a subset.
3. **Is your claim in its scan set?** Look for your file, line or entity in the check's own
   coverage output. Finding the same identifier somewhere else does not count.
4. **Has it ever gone red on this case?** Run it against a case whose answer you already know.
   A check that has never failed on a known-bad case has measured nothing.
5. **Empty output is not "all clear"** until you know the check could not have crashed, been rate
   limited or timed out silently.

A probe that stayed green through the incident is a **detection gap**, and it goes into technique 10.

**Done when:** every check cited as evidence has its scope stated beside it.

## 9. Complete reads

**Written from:** a file API returned the first 64 KiB of a 134 KB file with a `truncated` flag
nobody read. The content was edited and written back, deleting 1,793 lines, and reached a merge
request before a person caught it.

A partial read that reports success looks exactly like a complete one, unless you check its length
or its hash:

- Check the truncation or `has_more` flag on every response.
- Compare the byte count returned with the size the API reports.
- Before editing through an API, compare the hash of what you hold with the hash the API reports
  (`git hash-object <file>` against the blob id). After writing, read back and compare again.
- For a paged API, know which end the first page comes from. With some APIs it depends on which
  bound was sent. Page until the envelope says there is no more.
- A query capped at N rows (an SQL console, a log search) that returns exactly N is truncated until
  shown otherwise.

**Done when:** every read that fed a decision or a write has its completeness check recorded.

## 10. Post-incident

Once the system is stable, write the report from techniques 2 and 7. Do not write it from memory.
In an AEP store:

```console
$ aep plan artifact new postmortem <slug> --title "<title>" --relate derived_from:incident-report:<slug>
```

Sections, each with sources:

- **Impact:** who was affected, how many, for how long, and whose count it is (a reporter's
  estimate is labelled as one).
- **Timeline:** the table from technique 2.
- **Cause:** from the verified list, or "unknown" plus the leading hypothesis marked *inferred*.
- **Detection gap:** why monitoring did not see it, or saw it late (technique 8), and the time from
  onset to the first human report.
- **Evidence lost:** what technique 1 could not capture and why.
- **Follow-ups:** each one an artifact
  (`aep plan artifact new story <slug> --title "<the gap>" --relate informed_by:incident-report:<slug>`),
  not a sentence. The standard follow-ups are the detection gap, the capture gap (tooling missing
  from the image, no dump signal) and the cause, if known.

**Done when:** each follow-up exists as an artifact, and the report carries no unsourced specific.
