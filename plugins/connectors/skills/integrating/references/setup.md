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

Use the [current local foundation](https://github.com/beyond10x/connectors/blob/v0.43.0/docs/local-runtime-foundation.md)
and the adapter's linked guide for deployment-specific prerequisites. The local CLI targets Linux
keyring custody. A source installation of `v0.43.0` requires Rust 1.91 or newer and builds package
`connectors`; its release carries no prebuilt archives. `b10x` selects the exact release tag and
Cargo's locked dependency graph. Build failures retain the previous installed binary.

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

For a 0.7.x deployment, preserve the old configuration/state and identify each consumer's contract
before replacement. Current local groups use a different configuration and state model; there is
no automatic v1 state or credential migration. Re-establish connections through protected
acquisition and validate the required operations after upgrading. Do not promise CLI or daemon
compatibility from a version check alone.
