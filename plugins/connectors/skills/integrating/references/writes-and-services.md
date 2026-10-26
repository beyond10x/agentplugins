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
same request, `--approval-file` pointing to the proof and the required `--idempotency-key`.
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

For an existing supplied service endpoint, read `connectors describe --help` and
`connectors invoke --help`. These top-level commands form a separate compatibility interface;
only the global `--output` selection applies. Select the configured `--endpoint` and protected
`--token-file`, describe that service, then construct invocation from its descriptor and the
installed command's input options. Its operations' `input_schema` and `output_schema` are JSON
Schema objects; its `configuration_schema` is still JSON text. Keep service credentials in the
protected file.
Enable plaintext only when the deployment and task explicitly authorize it.

`connectors serve --help` describes the federation service; start it only for an authorized
service task and report the process lifecycle. The current release has no hosted login mode,
and its only MCP runtime is the read-only local stdio server the skill describes; it serves no
write. Since `v0.35.0` a service whose configuration names a private
`state` directory also answers `POST /v1alpha2/invoke` for library clients
(`connectors_client::Client::invoke_v1alpha2`): each admitted invocation carries `audit_ref` and
`audit_status`, and a write carries its recorded `mutation` attempt; a write answered
`outcome_unknown` is never sent again. The CLI's explicit `invoke` keeps the `/v1/invoke`
binding. Local placement uses `--config`/`--state-dir`; explicit service placement uses the
endpoint. Neither interface uses v1 target flags or description leases.
