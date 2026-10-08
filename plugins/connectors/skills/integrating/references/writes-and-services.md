# Governed writes and explicit service endpoints

## Local writes

An operation must be admitted by the adapter and covered by the user's existing authorization.
Inspect `connectors approvals --help` and the selected operation's current write requirements.
Use `approvals prepare` with the exact `--adapter`, `--connection`, `--operation`, `--schema`,
`--revision` and input document intended for invocation. This produces the reconstructed subject
for review. Missing policy, signing custody or clock prerequisites need their documented setup;
a refused invocation does not authorize changing them.

When the exact effect and target are authorized, `approvals issue` takes the same request,
`--approve-subject` from preparation and a new `--proof-output` protected path. Invoke with that
same request, `--approval-file` pointing to the proof and the required `--idempotency-key`.
Keep proof bytes out of model context. Never invent approval evidence, signing keys, a subject or
a revision. The current CLI rejects approval files and idempotency keys on read operations.

An approval is bound to exact request bytes and current authority. A changed input, description
or connection must be reassessed and prepared again. After an uncertain write, preserve its
idempotency key and follow the operation's documented observation/recovery contract; a new key
would describe another attempt. Report ambiguity rather than automatically resending.

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
service task and report the process lifecycle. The current release has no implemented MCP
runtime or hosted login mode. Local placement uses `--config`/`--state-dir`; explicit service
placement uses the endpoint. Neither interface uses v1 target flags or description leases.
