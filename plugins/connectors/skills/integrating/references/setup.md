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
3. Read `instances describe --connector '<connector>' --instance '<instance>'` and the
   instance's supported acquisition profile. An instance holds at most one connection: `connect`
   on one that already holds a live or stale connection is refused before it asks for a
   credential, so a second account is a second instance. For an authorized connection, inspect
   `connections connect --help`, then use the selected `--connector`, `--instance` and
   `--profile` with one protected entry option: `--credential-prompt`,
   `--credential-file` or `--credential-stdin`. The operator supplies the documented credential
   document directly to that protected channel. Never read secret bytes into model context or
   put them in chat, command arguments, logs or configuration. OAuth acquisition uses the
   adapter's returned safe next action; do not invent an authorization URL or copy tokens. A
   profile using `oauth2_client_credentials` takes a `{client_id, client_secret}` document through
   the same protected channel; the CLI requests and renews the access token itself. Since
   `v0.58.0` a profile using `oauth2_grant` signs in instead: `connections connect` runs the
   authorization code grant with PKCE (it prints the authorization address and takes the redirect
   on a loopback port, or the profile has the operator paste the code), or with `--device` the
   device grant, showing a user code to enter in any browser within the window the release
   documents. Only the operator signs in; nothing is stored when the grant fails.
   `connections token --connector '<connector>' --instance '<instance>' --connection '<id>'`
   then prints the access token's expiry, scopes and issue, refreshing it first when it is
   about to expire. `--reveal` prints the token itself: that is the operator's command for their
   own tool, never one you run.
4. Observe the returned connection or acquisition with
   `connections status --connector '<connector>' --instance '<instance>'` and exactly one of
   `--connection` or `--acquisition`. Report ready only when observed state says so.
   `connections revalidate` checks a saved credential without re-entry; it contacts the provider
   and requires the current `--expected-revision`. Since `v0.54.0` a catalog connection is bound
   to the credential side of its configuration and to the provider executable, not to the
   operation list: a bundle, source or selection change leaves it `ready`, and an added read
   works with no revalidation. A provider executable change validates the stored credential once
   on the next call and refuses as `identity_mismatch` when the identity differs; republish the
   approval policy after it. A CA file, token CA, admitted host or request prefix change still
   gives `pending` until revalidation. A connection saved before `v0.54.0`, or under an entry
   other than the catalog provider's, keeps the rule that follows until its first revalidation.
   After the instance's configuration changes (a rebuilt bundle or selection), revalidate: the connection follows the
   new configuration when its provider authority, profile and identity are unchanged, and
   otherwise refuses with `next_action: create_connection`. That refusal keeps the credential
   and is recorded on the connection, whose reads then name `create_connection` too (a
   reconnect clears it). Connect again: under the same `instance` id the new connection is admitted under
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
5. For repair, preserve connection identity: `connections reconnect` (since `v0.54.0`; its
   deprecated old name runs the same command) takes `--connector`, `--instance`,
   `--expected-revision`, one protected credential source and, optionally, `--connection` (the
   instance's one connection when left out). It turns a `stale` connection, whose credential the
   provider refused, `live` again under the same identity. For a requested
   terminal revocation, inspect `connections revoke --help` and use the current revision.
   A conflicting revision requires a fresh observation, never a guessed increment.
6. To hand one saved connection to a local program, inspect `connections launch --help` and the
   release's `docs/local-consumer-launch.md`. The operator declares the consumer under
   `[consumers]` (configuration `connectors-local/3`) with a pinned executable digest and the
   adapters it may use; never add or widen a declaration to make a launch work. The consumer
   receives the protected document on file descriptor 3 and nothing you can read holds it. For
   an `oauth2_grant` connection (since `v0.58.0`) descriptor 3 carries only the current access
   token, never the stored entry or its refresh token.

Use the [current local foundation](https://github.com/beyond10x/connectors/blob/v0.59.0/docs/local-runtime-foundation.md)
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
then runs `connectors approvals clock-check --connector <connector> --instance <instance>`. The same release adds a
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

`v0.54.0` adds connector instances. A `connectors-local/5` configuration enables connectors by
name under `[connectors.<connector>]` and names their instances under
`[connectors.<connector>.instances.<instance>]`, each with only its own `endpoint`, optional
`description` and, where its connector declares more than one auth profile, `profile`; the
executable, its SHA-256, the configuration revision and the operations come from the
`connectors-catalog-provider` installed beside `connectors`, and `default_instance` selects one
for a command that names none. `setup init` still writes the older format; the release's README
(*Configure instances*) shows the file. A legacy `[adapters.<alias>]` entry keeps working and is
moved by `connectors setup migrate`, as `connectors:upgrade` says. The same release adds
approved Zendesk ticket writes, Jira issue writes with `issue.create_meta.fields`, and Slack
`users.info`, `users.lookupByEmail` and `files.delete`: use one only when the operator authorizes
that access, and send every write through
[writes-and-services.md](writes-and-services.md). `v0.55.0` changes no setup step.

`v0.56.0` lets one pinned, read-only catalog instance carry `bindings` in its
`connectors-catalog-local/2` native configuration: 1 to 8 alternative APIs, each with its own
bundle, selection set and access read, tried in order on the 401 or 403 its `next` names (the
release's `docs/catalog-confluence.md`, *One instance for either API*). `connections connect`,
`describe` and `revalidate` then name the `binding` the connection uses, and a revalidation that
changes it adds `"rebound": {"from", "to"}`. When every binding refuses, the result is
`insufficient_scope` naming each answer; a wrong password is `invalid_credential` and tries no
other binding. No shipped connector declares `bindings`, so only an operator-pinned instance has
them, and the change needs no setup step otherwise.

`v0.57.0` ships the Jira issue feed (`jira-issues/1`) and the Confluence REST v1 page feed
(`confluence-pages/1`) in their selections, so a `jira` or `confluence-v1` instance has
`feed.containers` and `feed.items`, and Jira gains `projects.list` and `project.get`. Both feeds
are verified against fixtures only, not live. An instance derived from the shipped declaration
takes the new operations from the installed `connectors-catalog-provider`; a pinned instance or a
legacy `[adapters.<alias>]` entry adds one to its `operations` only when the operator authorizes
that access.

`v0.58.0` adds `oauth2_grant` profiles (step 3) and two catalog connectors that sign in with a
person's own subscription: `chatgpt` (profile `chatgpt.subscription`, browser or `--device`, read
`models.list`) and `claude-subscription` (profiles `claude.subscription` and
`claude.subscription-pasted`, read `account.profile`). Neither ships a client id: the operator pins
an instance whose native configuration names its own `oauth2_client_id`, and an instance without
one is refused as `InvalidConfiguration`. Whether a client may use a plan is the provider's terms'
question, which the guides quote ([`chatgpt`](https://github.com/beyond10x/connectors/blob/v0.59.0/website/docs/connectors/chatgpt.md),
[`claude-subscription`](https://github.com/beyond10x/connectors/blob/v0.59.0/website/docs/connectors/claude-subscription.md)).
Both sign-ins are verified against a local fake authorization server only, never live.
`v0.59.0` changes no setup step.

For a 0.7.x deployment, preserve the old configuration/state and identify each consumer's contract
before replacement. Current local groups use a different configuration and state model; there is
no automatic v1 state or credential migration. Re-establish connections through protected
acquisition and validate the required operations after upgrading. Do not promise CLI or daemon
compatibility from a version check alone.
