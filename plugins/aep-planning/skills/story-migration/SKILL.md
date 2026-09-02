---
name: story-migration
description: Migrate a repository's legacy work tracking — story trees, TODO.md, plan and issue documents — into the governed AEP planning store, without deleting or rewriting the sources. Use when the user asks to migrate, import, port or convert an existing backlog into AEP, when a repository is adopting AEP and already has work written down somewhere, or when a store has been adopted beside a legacy backlog nobody retired. Read it before creating the first artifact in a repository that already tracks work in markdown.
---

**Skill version 0.3.6** — the version in `.claude-plugin/plugin.json`.

# Migrating legacy tracking into the store

## 1. The failure this exists to prevent

Adoption is easy and migration is not, so the common outcome is a repository with two backlogs and
no record that one replaced the other.

It has already happened once. `beyond10x/connectors` adopted AEP on 2026-09-01 and created six
artifacts while leaving seventy-three `docs/stories/S-*.md` in place. Neither side names the other.
The new `epic:remote-secrets` covers the ground `S-034` claims, and `S-034` was never annotated,
moved or archived. `CHANGELOG.md` cited `S-NNN` ids until the day of adoption and silently stopped.
Nothing anywhere says the old backlog is superseded, so both look current and the older one looks
merely neglected.

A migration that leaves the sources unreferenced has not migrated anything. It has forked the plan.

**So the deliverable is not the artifacts. It is the artifacts plus the backlinks.** § 3 step 5 is
the step this skill exists for; the rest is how to earn the right to write it.

## 2. Ask, do not assume

Two vocabularies matter and neither belongs in this file.

**The store's**, which the CLI answers: `aep artifact kinds`, `aep artifact lifecycle <kind>`,
`aep artifact relations`. A kind list written down here goes stale the first time a tree adds one,
and an agent reciting `draft → proposed → active` from memory will propose an illegal move in a
store that renamed a rung. See the `planning` skill § 2 — it is the authority on this and is not
restated here.

**The repository's**, which only the files answer. There is no legacy format. Observed shapes so
far: numbered story trees with YAML frontmatter (`S-001-…md`, `id`/`status`/`epic`/`areas`),
prefix-less story files with a bold prose `**Status:**` line and no frontmatter at all, `Plan:
<title> (DEV-nnn)` documents whose status is a sentence in a `Status` section, flat `TODO.md`
bullets with no identity of any kind, and incident write-ups that are evidence rather than work.
Do not pattern-match onto the first one you recognise. § 3 step 1 exists to find out.

## 3. The procedure

Six steps. Steps 1 and 2 end by stopping and showing the operator what you found; nothing is written
before both are accepted.

### Step 1 — inventory

Find what the repository already tracks, without assuming a layout. Look for `docs/stories/`,
`docs/issues/`, `docs/epics/`, `.agents/plans/`, `.agents/bugs/`, `TODO.md`, `ROADMAP.md`, and
`*/docs/PLAN.md`. Read each candidate's first twenty lines rather than trusting its directory name.

Report one row per location: path, file count, line count, the convention you detected, and what
told you. Then stop.

An empty finding is a real finding. A repository with nothing to migrate should adopt through
`aep reverse init` and the `planning` skill § 5, not through this one.

### Step 2 — classify

One row per source document, and a decision you can defend for each:

| column | rule |
|---|---|
| source | the path |
| kind | from `aep artifact kinds`; a plan is not automatically a `story` |
| slug | derived from the title, see § 4 |
| status | the rung its own text evidences, and the quote that evidences it |
| relations | edges to other sources, from `depends_on`, prose references, or a shared epic |
| confidence | **read** with a `path:line`, or **inferred** |

Mark every inferred cell. A table that mixes what was read with what was guessed is worse than no
table, because it gets trusted exactly where it is weakest.

Then stop, and show it.

### Step 3 — backfill

Most fields do not exist in the source. That is the work, and it is where a migration invents
things if it is not held to a rule.

- **Title** — the H1. If there is none, the filename, said so.
- **Slug** — from the title, lowercased, hyphenated, checked against the id charset in § 4. Legacy
  ids (`S-001`, `DEV-630`) do **not** become slugs; they go in the body, because the store's id is
  `<kind>:<slug>` and a number carries no meaning outside its old tree.
- **Status** — read from the document's own words and **quote those words in the body**. Prose is
  the evidence for the rung, so it travels with it.
- **Ticket ids** — `DEV-\d+`, `S-\d+` and similar become body references and, where the tracker has
  a URL, a link. Never an artifact id.
- **Dates** — see § 4. They are retained, in the body, from git.
- **Anything with no source** — left unset. A migration does not get to decide a priority nobody
  wrote down.

### Step 4 — write

One artifact at a time, body supplied at creation:

```console
$ aep artifact new story dispatch-retry-backoff \
    --title "Back off ACD v3 dispatch retries" --from body.md
created story:dispatch-retry-backoff (draft) at .engineering/planning/story/dispatch-retry-backoff.md
```

Then `aep artifact relate` for each edge, then `aep artifact move` for each artifact whose
evidenced rung is above the initial one — **one move per rung, never a jump**, and each move's
output relayed.

Where a rung needs evidence the store does not hold, **stop at the last legal rung and relay the
refusal**. Do not assert evidence to get past it. A story that is `active` with a note saying what
would make it `implemented` is honest; one that is `implemented` on a migration's say-so is a
false record that looks identical to a true one.

### Step 5 — backlink, both directions

Neither direction is optional, and this is the step § 1 is about.

- **Source → artifact.** Every migrated file gains a line naming the artifact that now carries it.
  At the top for a short file, under a `## Superseded by` heading for a long one. The file is not
  deleted, not moved, not emptied.
- **Artifact → source.** Every body ends with the `## Provenance` block from § 4.

Where a source file is an index that lists the others — a `docs/stories/README.md`, a `TODO.md` —
annotate the index once and say it is an index, rather than editing every bullet.

### Step 6 — validate and report

```console
$ aep artifact validate
```

Relay its output verbatim; it names each artifact and each defect, and that is the part anybody can
act on. Then report three lists, not one: what was migrated, what was **skipped and why**, and what
**could not reach its evidenced rung**, with the refusal quoted.

## 4. Rules that hold whatever the legacy shape is

**There is no `--dry-run`.** No verb in the `aep` CLI has one. Steps 1 and 2 are the dry run: they
produce the plan as a document and stop. Do not reach for a flag that does not exist, and do not
start writing to find out whether the classification was right.

**Creation is refused, not overwritten.** `aep artifact new` refuses an id that already has a
document, at two layers, and the refusal names the path. Treat it as the answer: either the
artifact is already migrated, or the slug collides and needs a different one.

**A re-run is safe.** The create command's idempotency key is derived from the artifact id, so
re-running an identical migration replays rather than duplicating. This is the property that makes
the whole procedure repeatable, and it is worth testing: run it twice, and the second run must
leave the tree unchanged.

**Status is never written directly.** `aep artifact new` has no `--status` flag; every artifact is
created at its kind's initial rung and walks up through `aep artifact move`. Do not hand-edit
`status:` in frontmatter — an unvalidated status is indistinguishable in the file from a legal one,
which is what makes it expensive.

**Slugs must match the id charset.** An artifact name is `[A-Za-z0-9][A-Za-z0-9._/-]*`. There is no
slugify helper in the CLI, so derive it and check it before calling `new`; a title with a colon, an
ampersand or a leading digit-free symbol will be refused.

**Dates come from git, never from the filesystem.** A fresh checkout resets mtime, and a migration
run against one would date every artifact to the day it ran. AEP frontmatter carries no timestamp
field on purpose, and `aep artifact set` writes only `--title`, `--summary`, `--owner` and tags —
there is no door for an arbitrary key. So dates live in the body:

```markdown
## Provenance

Migrated from `.agents/plans/DEV-630_dispatch-retry-backoff.md`.

- First written 2026-06-16 · last touched 2026-06-16 · 1 revision
- Status quoted from that file, line 17: **PLANNED — not yet implemented**
- Ticket [DEV-630](https://babelforce.atlassian.net/browse/DEV-630)
```

The three git facts, in order:

```console
$ git log --follow --diff-filter=A --format=%aI -- <path> | tail -1   # first written
$ git log -1 --format=%cI -- <path>                                   # last touched
$ git log --oneline --follow -- <path> | wc -l                        # revisions
```

Quote dates the document states about itself too — `**Option A … LANDED** (2026-04-28)`,
`## Verification (2026-08-10)` — because those are claims somebody made, and git only knows when
the file changed.

**Nothing is deleted.** Not a source file, not a line in one, not a legacy index. A backlog that
turns out to have been retired years ago is retired by saying so in it, and by moving its artifact
to `archived` through the lifecycle.
