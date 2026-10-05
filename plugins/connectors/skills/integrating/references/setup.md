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
   adapter's returned safe next action; do not invent an authorization URL or copy tokens.
4. Observe the returned connection or acquisition with `connections status --adapter '<alias>'`
   and exactly one of `--connection` or `--acquisition`. Report ready only when observed state
   says so. `connections revalidate` checks a saved credential without re-entry; it contacts
   the provider and requires the current `--expected-revision`.
5. For repair, preserve connection identity: `connections repair` takes `--adapter`,
   `--connection`, `--expected-revision` and one protected credential source. For a requested
   terminal revocation, inspect `connections revoke --help` and use the current revision.
   A conflicting revision requires a fresh observation, never a guessed increment.

Use the [current local foundation](https://github.com/beyond10x/connectors/blob/v0.28.0/docs/local-runtime-foundation.md)
and the adapter's linked guide for deployment-specific prerequisites. The local CLI targets Linux
keyring custody. A source installation of `v0.28.0` requires Rust 1.91 or newer and builds package
`connectors`; its release carries no prebuilt archives. `b10x` selects the exact release tag and
Cargo's locked dependency graph. Build failures retain the previous installed binary.

For a 0.7.x deployment, preserve the old configuration/state and identify each consumer's contract
before replacement. Current local groups use a different configuration and state model; there is
no automatic v1 state or credential migration. Re-establish connections through protected
acquisition and validate the required operations after upgrading. Do not promise CLI or daemon
compatibility from a version check alone.
