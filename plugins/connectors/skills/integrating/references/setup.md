# Setup and acquisition

Read this for a requested setup, connection or repair. First inspect the corresponding
`connectors setup --help` and `connectors connections --help` on the installed release.

1. Run `connectors --output json setup check`. For an uninitialized local deployment,
   `connectors --output json setup init` creates private configuration and state without starting
   services. Existing configuration is preserved; inspect the refusal instead of replacing it.
2. Select the adapter's current upstream setup guide and actual installed executable artifacts.
   Configure owner-only files with the real artifact digest, private protocol and bootstrap
   metadata that guide requires. An example's fictional paths or digests are not runnable
   artifacts. `b10x` installs the CLI, not every adapter. `setup check` must show its artifact and
   state prerequisites before continuing.
3. Read `adapters describe --adapter '<alias>'` and the adapter's supported acquisition profile.
   For an authorized connection, inspect `connections connect --help`, then use the selected
   `--adapter` and `--profile` with one protected entry option: `--credential-prompt`,
   `--credential-file` or `--credential-stdin`. The operator supplies the documented credential
   document directly to that protected channel. Never read secret bytes into model context or
   put them in chat, command arguments, logs or configuration. OAuth acquisition uses the
   adapter's returned safe next action; do not invent an authorization URL or copy tokens. A
   profile using `oauth2_client_credentials` takes a `{client_id, client_secret}` document through
   the same protected channel; the CLI requests and renews the access token itself.
4. Observe the returned connection or acquisition with `connections status --adapter '<alias>'`
   and exactly one of `--connection` or `--acquisition`. Report ready only when observed state
   says so. `connections revalidate` checks a saved credential without re-entry; it contacts
   the provider and requires the current `--expected-revision`. After the adapter's
   configuration changes (a rebuilt bundle or selection), revalidate: the connection follows the
   new configuration when its provider authority, profile and identity are unchanged, and
   otherwise refuses with `next_action: create_connection`. That refusal keeps the credential
   and is recorded on the connection, whose reads then name `create_connection` too (a repair
   clears it). Connect again: under the same `instance` id the new connection is admitted under
   the configured revision, while a changed provider host or profile needs a new instance id.
   Since `v0.34.0` no such connection refuses with `retry_status`. A release whose
   notes say an instance's configuration revision moved (in `v0.32.0`, every instance using the
   shipped GitLab selection set) leaves that adapter unable to start and its connections
   `pending` until the operator prints the bootstrap again and copies its
   `configuration_revision` into the adapter entry; then revalidate each connection and issue
   the instance's approval policies again (the release's `docs/local-catalog-provider.md`).
   Reading the GitLab feed also needs `feed.containers` and `feed.items` among the adapter's
   permitted operations; add them only when the operator authorizes that access. A profile's access
   read answered 401 or 403 refuses the connection as `insufficient_scope`: the credential lacks a
   scope the profile needs, and only a credential that has it fixes that.
5. For repair, preserve connection identity: `connections repair` takes `--adapter`,
   `--connection`, `--expected-revision` and one protected credential source. For a requested
   terminal revocation, inspect `connections revoke --help` and use the current revision.
   A conflicting revision requires a fresh observation, never a guessed increment.
6. To hand one saved connection to a local program, inspect `connections launch --help` and the
   release's `docs/local-consumer-launch.md`. The operator declares the consumer under
   `[consumers]` (configuration `connectors-local/3`) with a pinned executable digest and the
   adapters it may use; never add or widen a declaration to make a launch work. The consumer
   receives the protected document on file descriptor 3 and nothing you can read holds it.

Use the [current local foundation](https://github.com/beyond10x/connectors/blob/v0.48.0/docs/local-runtime-foundation.md)
and the adapter's linked guide for deployment-specific prerequisites. Credential custody is fixed
when the binary is built: the Secret Service on Linux, the login keychain on macOS (since
`v0.48.0`), and `setup check` reports it as `custody_backend` beside `os`. No backend falls back
to a file or an environment variable. Since `v0.48.0` each release carries checksummed prebuilt
archives for x86-64 and ARM64 Linux and macOS, which `b10x` installs by default. Since `v0.49.0`
the CLI also builds and runs on Windows, with custody in the Windows Credential Manager and
weaker platform guarantees that the release's CLI semantics name; there is no Windows release
binary, so it is built from source there. An older, source-only release, or `--method cargo`, builds package `connectors` at
the exact tag with Cargo's locked dependency graph and needs Rust 1.91 or newer. A failed
download, checksum or build retains the previous installed binary. The macOS binary is not signed
yet: a browser download needs `xattr -d com.apple.quarantine connectors` before it runs, and an
upgraded binary cannot read keychain items an earlier build wrote, so after an upgrade on macOS
the operator stores each credential again through a protected channel (step 3 or 5).

Since `v0.33.0`, `setup init` creates a metadata store with durable open checkpoints, and
`v0.32.0` and earlier refuse to open such a store. Upgrade every `connectors` binary and any other
tool that opens the store before using one this release created. An existing store keeps working
with older releases until its owner runs `setup checkpoints-enable --confirm one-way`; that is
one-way, cannot be undone by any command, and refuses `lifecycle_conflict` (`next_action =
stop_owner`) while an owner runs. Run it only when the operator asks for it.

Since `v0.39.0` a catalog selection may name `credential`: parameters through which the
provider's document passes the credential the connection already sends, such as Slack's `token`.
They are never declared, required or sent. A release before `v0.39.0` refuses a selection file
that names one, so upgrade every `connectors` binary that loads the adapter before using it.

From `v0.41.0` to `v0.43.0` the shipped selection sets use members that earlier catalog
providers refuse: `body_types` and `body_required`, then `body_fixed`, `further_preflights`,
`postflight.read` and `postflight.any_of`, then a `bounds` value set and `postflight.absent`. An
earlier bundle build likewise refuses `v0.43.0`'s GitLab amendment file (`correct_path`). Upgrade
every binary that builds or loads the adapter before using these files. The GitLab, Jira and SQL
descriptors gain operations in these releases. An existing adapter entry keeps the operations it
permits; add a new one to its `operations` only when the operator authorizes that access. An
instance using a shipped selection set or GitLab bundle from the new release has a new
configuration revision: recover as step 4 describes.

Selection sets grow again: GitLab's from 42 to 51 operations in `v0.44.0`, Slack's bot set from 7
to 10 and Jira's from 8 to 9 in `v0.45.0`. Each changes that descriptor and its configuration
revision: copy the `configuration_revision` from the adapter executable's
`--print-local-bootstrap` output again and recover as step 4 describes. Add a new operation to the
adapter's `operations` only when the operator authorizes that access. Since `v0.44.0` a
`connectors-source-amendments/1` entry may hold `correct_media_type` (GitLab's amendment file uses
it), and since `v0.45.0` a catalog selection may read a binary response or a `download` URL;
upgrade every binary that builds or loads the adapter before using such files. A catalog
connection may now admit further origins in `hosts`, which some binary reads need (the adapter's
guide names them, such as Slack file downloads or Jira attachment redirects); adding `hosts`
changes the configuration revision.

Since `v0.45.0` a Kubernetes configuration may set `pod_logs` and `kubeconfig` (reads) and
`pod_exec`; each is off when absent and then leaves the revision unchanged. `pod_exec` advertises
`pods.exec`, a write on the `connectors-private/2` exchange: it needs that `private_protocol`, the
operation among the adapter's permitted operations and an approval policy, and every call goes
through [writes-and-services.md](writes-and-services.md). Enable it only when the operator
authorizes running commands in pods.

Since `v0.47.0` a catalog guard check may expect a JSON boolean, `{"boolean": true}` or `false`,
and a selection may hold string body keys non-empty (`body_nonempty`); the release's
`docs/local-catalog-provider.md` defines both. The same release refuses, when the selection
loads, a guard check written `{"literal": "true"}` or `{"literal": "false"}`: a configuration
that carries its own selection file with either no longer loads until each is rewritten as
`{"boolean": true}` or `{"boolean": false}`. Upgrade every binary that loads the adapter before
using such a file. The shipped GitLab and Jira guards now compare JSON booleans, so a string
`"true"` from the provider no longer satisfies them. Slack's bot selection gains two writes,
`conversations.open` and `chat.postMessage` (scopes `im:write`, `mpim:write` for a group, and
`chat:write`; a direct message is open, then post): a changed descriptor, so recover as step 4
describes, and add either to the adapter's `operations` only when the operator authorizes
sending messages. Both go through [writes-and-services.md](writes-and-services.md); the release's
`docs/catalog-slack.md` (*Messages*) is their contract.

`v0.51.0` adds `chat.update` and `chat.delete` to Slack's bot selection: a changed descriptor
again, so recover as step 4 describes, and add either to the adapter's `operations` only when the
operator authorizes editing or deleting messages. A local install that still holds the
credential-only `slack` adapter beside `slack-bot` moves to `slack-bot` and retires `slack` with
the steps in the release's `docs/catalog-slack.md`, *Moving a local install off a
credential-only adapter*. Release archives now carry `connectors-catalog-provider` (the executable
the catalog provider guide binds to the CLI) beside `connectors`. `b10x` installs `connectors`
alone; where the catalog provider runs, install `connectors-catalog-provider` from the same
archive as the release's README (*install a release binary*) shows, so both are one release.

`v0.52.0` changes the approval clock to a quorum of Roughtime sources (`roughtime-quorum/1`): an
install that issues approvals or makes approved writes rewrites its `[approval_clock]` table first,
as [writes-and-services.md](writes-and-services.md) and the release's `docs/local-clock.md` say,
then runs `connectors approvals clock-check --adapter <alias>`. The same release adds a
`confluence-v1` provider (seven Confluence REST v1 reads, for a token with classic scopes whose v2
reads answer `401`), configured as its own adapter beside `confluence` as the release's
`docs/catalog-confluence.md` shows; Slack history and thread reads in the `slack.user` selection,
a changed descriptor, so recover as step 4 describes; and Kubernetes reads of statefulsets, nodes
(`node_reads`) and configured custom resources, with connections through an `exec` credential
plugin on Linux. A Kubernetes binding that sets neither `node_reads` nor `custom_resources` keeps
its configuration revision.

`v0.53.0` adds OpenRouter management through the catalog provider (`openrouter`, profile
`openrouter.api-key`: reads `key.get`, `workspaces.list`, `keys.list`, and the approved write
`key.create`, whose created key goes into custody as
[writes-and-services.md](writes-and-services.md) says), Runpod's `billing.pods` read, and GitLab's
`file.raw` and `project.languages` reads. An install that already has an OpenRouter alias adds
`key.create`, the custody table and the custody alias, with the new revisions and provider SHA-256,
as the release's `docs/catalog-openrouter.md` (*Moving an existing install*) shows; every catalog
provider alias renews its revision and SHA-256 as the upgrade skill says.

For a 0.7.x deployment, preserve the old configuration/state and identify each consumer's contract
before replacement. Current local groups use a different configuration and state model; there is
no automatic v1 state or credential migration. Re-establish connections through protected
acquisition and validate the required operations after upgrading. Do not promise CLI or daemon
compatibility from a version check alone.
