# Store conventions

This reference describes the Git backend’s file layout. SQLite and PostgreSQL remain distinct
selected backends; use the CLI for their records and preserve their database identity.

What the on-disk store looks like, and which CLI operation changes each part. This file carries only
what the CLI cannot answer at runtime — the vocabulary questions (kinds, statuses, legal moves,
relations) all have commands, and those commands are the authority. See `SKILL.md` §2.

## Layout

```
.engineering/
├── project.yaml                  version: aep.project/5, store: {git: {}}
├── planning/
│   ├── epic/
│   │   └── passkey-login.md
│   ├── story/
│   │   ├── credential-store.md
│   │   └── registration-ceremony.md
│   └── task/
│       └── ceremony-fixtures.md
└── evidence/
    └── story/
        └── credential-store/
            └── 20260830T140200Z-8477cd9d6ad8.json
```

The artifact files are the store's authority: there is no journal, state directory or event log
beside them, and no file is a projection of another. Each evidence record is one JSON file written
once by `aep plan artifact evidence` and never edited. Status history lives in each artifact's
`transitions` list, and every other version of a file in Git history.

One directory per kind, one file per artifact, no nesting below the kind directory. The store root
defaults to `.engineering/planning/` and moves with `--store <dir>` — a repository may keep more than
one, and nothing in a file records which store it belongs to.

A typed blocker is an ordinary artifact under a directory of its own —
`credential-blocker/api-token-scope.md` — because the type of a blocker is its **kind**. Every
`<type>-blocker` reaches the one `blocker` ladder by its last hyphen segment, so a store gets a new
type by using the name; `artifacts/kinds/blocker.yaml` writes down the ones a tree has agreed on and
checks nothing against them. `aep plan artifact kinds` lists none of them, so the command that
answers whether a store can hold one is `aep plan artifact lifecycle <type>-blocker` — see
`SKILL.md` guardrail 6.

## Names and ids

An artifact's id is `<kind>:<slug>`, and the path is `<kind>/<slug>.md`. The two are the same fact
written twice, so they cannot be allowed to disagree:

| Rule | Why |
|---|---|
| `id` in frontmatter equals `<kind>:<slug>` from the path | the id is how relations point at this file; a mismatch makes an edge resolve to nothing |
| slug is lowercase, hyphen-separated, no dots | it is a filename and an id segment at once |
| the file lives under the directory named by its `kind` | `aep plan artifact list --kind` reads the directory, and a misfiled artifact is invisible to it |

Renaming therefore is not a `mv`. Moving a file by hand breaks every relation that names its old id
and leaves `validate` to find the wreckage. Create the new artifact, re-point the relations — `relate`
on the new id and `unrelate` on the old one — and archive the old one through its lifecycle.

## Mutation ownership

No planning-store file is edited directly. One command surface owns every change:

| Change | Command |
|---|---|
| create an artifact, with its body when you have it | `aep plan artifact new … [--from <path|->]` |
| replace its complete markdown body | `aep plan artifact body <id> --from <path|->` |
| add a relation | `aep plan artifact relate` |
| take one back, when it was wrong or pointed the wrong way | `aep plan artifact unrelate` |
| move lifecycle status, including lifting a blocker | `aep plan artifact move` |
| change a title, summary, owner, tag, reference or model digest | `aep plan artifact set` |
| record which surfaces a story lands on, read (`cited`) or worked out (`--inferred`) | `aep plan artifact scope <id> --add <path>` |
| record an observation a later move rests on, by kind or from a conformance report | `aep plan artifact evidence` |

This makes the store a single-writer system: a write takes the store's writer lock, validates, and
changes exactly one artifact file (`evidence` adds one evidence file), bumping its `revision` once.
Two branches that write the same artifact therefore conflict on its `revision:` line in Git; resolve
by keeping one side and re-applying the other side's change through the CLI, then run `validate`,
which refuses a merge whose `transitions` do not end in its `status`.

## Frontmatter

Everything above the `---` is structured. Ownership is what decides whether you may edit it:

| Field | Owner | Notes |
|---|---|---|
| `id` | machine | set at creation, never edited; equals `<kind>:<slug>` |
| `kind` | machine | fixed at creation; changing a kind means a new artifact |
| `status` | machine | **only** `aep plan artifact move` writes this — see guardrail 1; `validate` refuses one that disagrees with the last transition |
| `revision` | machine | bumped by the CLI once per write; a review is bound to the revision it saw |
| `transitions` | machine | append-only; `move` adds one line `{from, to, at, actor, revision, decided_on?}`, and a move carried over by a migration is marked `imported: true` |
| `relations` | machine | written by `aep plan artifact new --relate` and `aep plan artifact relate` |
| `title` | descriptive | set at creation by `aep plan artifact new --title`; no verb changes one afterwards |
| `summary` | descriptive | one or two sentences; optional |
| `format` | machine | `aep.planning-md/3` in an `aep.project/5` store, written by the CLI |
| `withholds` | machine | optional; set by `aep plan artifact new --withholds <evidence-kind>`. The evidence kind this artifact is stopping anybody from producing, and only meaningful beside a `blocks:` relation — `validate` reports it otherwise |
| `scope` | machine | story only; written by `aep plan artifact scope`, each entry a `path` and a `confidence` of `cited` or `inferred`. `aep plan artifact waves` reads nothing else |
| `model_digest` | machine | `executable-system-specification` only; written by `aep plan artifact set --model-digest`, refused by name on any other kind |

"Machine" means the CLI validates it against a document you do not control from the file. A
hand-written `status` is not a faster move — it is an unvalidated one, and it looks identical to a
legal one afterwards, which is what makes it expensive.

## Body

Everything below the closing `---` is authored by you and the operator, but written through
`aep plan artifact body`. There is no required section list at this layer; a kind may declare
expected sections in `artifacts/kinds/<kind>.yaml`, and `aep plan artifact kinds` reports what a
kind is for. Write plainly: what this is, why now, what counts as done.

One convention worth keeping: **a story or task carries a single acceptance statement**, one
sentence, in the form of an observable outcome. It is what a reviewer checks against and what the
`plan-reviewer` agent looks for.

## A complete file

`.engineering/planning/story/credential-store.md`:

```markdown
---
format: aep.planning-md/3
id: story:credential-store
kind: story
status: proposed
title: Store and retrieve passkey credentials
summary: Persist WebAuthn credential ids and public keys, and look them up at assertion time.
relations:
- decomposes: epic:passkey-login
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-08-30T13:40:02Z", actor: "human:operator", revision: 2}
---

## Context

Sign-in is being moved to passkeys. The assertion ceremony needs a credential record it can look up
by user handle, and registration needs somewhere to put one. Nothing else in the epic can be
demonstrated until this exists.

## Acceptance

A credential registered through the ceremony is returned by lookup on the next sign-in attempt, and
survives a process restart.

## Notes

Storage backend is settled (the existing Postgres schema); the open question is whether the public
key is stored COSE-encoded or normalised on write. Raised in `epic:passkey-login`.
```

Two frontmatter lines are descriptive; the rest are the CLI's. Everything that took thought is
below the fence, which is the intended shape: the CLI keeps the graph honest, and the file stays a
document a person can read.
