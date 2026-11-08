# Governed writes and explicit service endpoints

## Local writes

An operation must be admitted by the instance and covered by the user's existing authorization.
Inspect `connectors approvals --help` and the selected operation's current write requirements.
Use `approvals prepare` with the exact `--connector`, `--instance`, `--connection`, `--operation`,
`--schema`, `--revision` and input document intended for invocation. This produces the
reconstructed subject for review. Missing policy, signing custody or clock prerequisites need
their documented setup; a refused invocation does not authorize changing them. Since `v0.55.0` an
`invalid_input` that `approvals policy-set`, `prepare` or `issue` decides names the refused
argument in `Failure.field` (such as `proof_output`, `approve_subject` or `revision`), never its
value: correct that argument, not another. `issue` refuses a `--proof-output` that is relative,
in a directory other users can read, an existing file or behind a symbolic link; use a new
private absolute path.

When the exact effect and target are authorized, `approvals issue` takes the same request,
`--approve-subject` from preparation and a new `--proof-output` protected path. Invoke with that
same request, `--approval-file` pointing to the proof and the required `--idempotency-key`
(since `v0.60.0` through `invoke`; `approvals` still selects with `--connector` and `--instance`):

```bash
connectors --output json invoke '<connector>/<instance>' '<operation>' --connection '<id>' \
  --schema '<schema>' --revision '<revision>' --input-file '<file>' \
  --approval-file '<proof>' --idempotency-key '<key>'
```

A write without `--approval-file` is refused `approval_required` before any
provider call, and its `approval_steps` names the commands that issue the proof: `approvals
prepare`, `approvals clock-check`, `approvals issue`. `--each` and `--walk` are refused on a write;
`--fields` is admitted and never hides `request_id`, `mutation` or `source_audit`.
Keep proof bytes out of model context. Never invent approval evidence, signing keys, a subject or
a revision. The current CLI rejects approval files and idempotency keys on read operations.

An approval is bound to exact request bytes and current authority. A changed input, description
or connection must be reassessed and prepared again. After an uncertain write, preserve its
idempotency key and follow the operation's documented observation/recovery contract; a new key
would describe another attempt. Report ambiguity rather than automatically resending.

Since `v0.47.0` Slack's bot selection carries two such writes, `conversations.open` and
`chat.postMessage`; a direct message is two invocations, open then post, each with its own
preparation, approval and idempotency key. A Slack answer whose `ok` is not the JSON boolean
`true` is `outcome_unknown`, not a refusal, and Slack's `error` string is not returned: check the
target conversation before any new attempt.

Since `v0.51.0` the same selection carries `chat.update` (edit) and `chat.delete` (delete), each
for a message `channel` and `ts` name and each only for the bot's own messages. Their refusals
that leave the message unchanged are named instead (`refused`, never sent again), such as
`cant_update_message` or `edit_window_closed` as `forbidden` and `message_not_found` as
`not_found`; any other `ok: false` answer is still `outcome_unknown`. Every write needs an
approval, and an approval needs the approval clock to answer: when it gives no observation,
`connectors approvals clock-check` names why (`clock_refusal`). The release's
`docs/catalog-slack.md` (*Messages*) is the contract.

Since `v0.52.0` the approval clock answers from a quorum: the `[approval_clock]` table is
`format = "roughtime-quorum/1"` with a `quorum`, `max_rate_error_ppm` and two to eight
`[[approval_clock.sources]]`, each with its `address`, `public_key` and `version`. The
single-source `roughtime-clock/1` table is refused by `clock-check`, approval issuance and approved
writes until it is rewritten as the release's `docs/local-clock.md` shows; ordinary reads need no
clock. A Slack read answered `200` with `"ok": false` is now a refusal by name; writes keep the
rules above.

Since `v0.53.0` a write that creates a credential holds it in custody. OpenRouter's `key.create`
(the catalog provider's `openrouter` connector, required approval) sends the create once, stores
the returned key as a new connection of the alias and profile that
`[adapters.<alias>.permissions.custody]` names for the operation, and returns only that
connection's reference and the key's hash, never the key. A create with no custody target, or one
whose target disagrees with its declaration, is refused before anything is sent. A timeout or
crash after dispatch is `unknown` and is never re-sent; custody `failed` may still have published
the connection, so check `connections list` on the custody alias and the provider's `keys.list`
before creating again. The release's `docs/catalog-openrouter.md` (*Creating a key*) and
`docs/local-mutation-ledger.md` (*Created credentials*) are the contract. Since `v0.53.1` a Runpod
`pod.create` refused for lack of capacity is `refused` with code `capacity`, not `invalid_input`.

Since `v0.54.0` the catalog provider carries Zendesk ticket writes (`ticket.update`,
`ticket.note`, and `ticket.draft_reply`, a private note that never reaches the requester), Jira
issue writes (`issue.create`, `issue.comment.add`, `issue.edit`, `issue.link.add`,
`issue.delete`) and Slack `files.delete`, each under an issued approval. The Zendesk writes and
the Jira edit and delete take the target's update stamp read just before (`updated_stamp`) and
refuse a difference as `conflict` with nothing sent: read the target again and reassess, never
resend with a guessed stamp. Jira rich text is Atlassian Document Format that the caller writes;
plain text and Markdown are not converted. `issue.link.add` is not verified after the write. The
Zendesk writes are verified with the `zendesk.basic` profile only. The release's
`docs/catalog-zendesk.md` (*Writes*), `docs/catalog-jira.md` (*Issue writes*) and
`docs/catalog-slack.md` (*Files*) are the contract. Since `v0.55.0` a Jira `issue.create` or
`issue.comment.add` answer that names no new id is `unknown`, not `applied`: check the issue
before any new attempt.

## Explicit services

Since `v0.60.0` the CLI has no command for a standalone service: the top-level `describe
--endpoint`, `invoke --endpoint --token-file` and `serve --config` forms are removed and are parse
refusals (exit 2) that contact nothing. A supplied service endpoint is reached one of two ways:

- Give a local instance its endpoint with `connections connect` and a protected credential
  source ([setup.md](setup.md), step 3), then `invoke` it as the skill shows:
  `connectors connections connect '<connector>/<instance>' --endpoint '<url>' --profile '<profile>' --credential-prompt`. The endpoint is written into
  the configuration before a credential is asked for; a different endpoint on an existing instance
  is refused `lifecycle_conflict` naming `endpoint`.
- Call the service over HTTP (`GET /v1/describe`, `POST /v1/invoke`, and `POST /v1alpha2/invoke`
  for library clients), as the release's
  [`docs/running-services.md`](https://github.com/beyond10x/connectors/blob/v0.69.0/docs/running-services.md)
  shows, with the bearer in a protected file. Its operations' `input_schema` and `output_schema`
  are JSON Schema objects. Enable plaintext only when the deployment and task explicitly
  authorize it.

The federation host is a library (`connectors_host::federation`) a Rust program embeds; no
command starts it. `connectors-catalog-provider --config <file>` serves one catalog instance on
the HTTP host (since `v0.62.0`); start it only for an authorized service task and report the
process lifecycle. The current release has no hosted login mode, and its only MCP runtime is the
read-only local stdio server the skill describes; it serves no write. A served host's
`POST /v1alpha2/invoke` carries `audit_ref` and `audit_status`, and a write its recorded
`mutation` attempt; a write answered `outcome_unknown` is never sent again. Since `v0.64.0` a
write the provider definitely refuses (a 4xx the catalog classifies, such as 405, 409 or 422 to a
merge) answers `refused` with the provider's `message` (`forbidden` for 405, `invalid_input` for
409 and 422) and settles `Failed`; a 5xx, a cut connection and a timeout stay `outcome_unknown`.

Served-host deployment features are the operator's to configure and are documented in that
release file: admission by a Kubernetes service account token (`admission:
service-account-token`, with `principal`, `principals` or `service_accounts`), a GitHub App
installation credential (`github_app`), a git smart-HTTP relay that pushes with the instance's
own token after quarantine checks, a `catalog.repositories` list that keeps `owner`/`repo`
operations inside the listed repositories, and keyed writes under `standing_grants`, where every
write needs `idempotency_key` and a covering grant (`approval_required`, `rate_limited` or
`idempotency_conflict` otherwise, none sent). Never write or widen a grant, a principal or a
repository list to make a call work. Local placement uses `--config`/`--state-dir`; a service
is placed by its endpoint.
