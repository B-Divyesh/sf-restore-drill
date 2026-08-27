# Restore Drill

Restore Drill is a Docker-first CLI for small teams that need evidence a
Postgres backup restores—not another green “backup completed” log. It creates a
disposable internal Docker network, restores a dump or volume archive, runs SQL
and application HTTP assertions, writes a signed JSON report, and tears the
environment down.

It never performs production restores and does not upload backups, credentials,
or reports. Restore Drill is MIT-licensed and has no telemetry.

## Install

Download a release binary, or build from source with Rust 1.85+:

```sh
cargo install --path crates/restore-drill
restore-drill --help
```

Docker Engine must be installed and reachable by the current user. Postgres and
probe images are pulled before the isolated network is created.

## Usage

Create a starter configuration and a gitignored credential file:

```sh
restore-drill init --output restore-drill.toml
$EDITOR .restore-drill.env
restore-drill check --config restore-drill.toml
```

Run the drill from cron or CI. There are no interactive prompts:

```sh
restore-drill run --config restore-drill.toml
restore-drill run --config restore-drill.toml --json
```

`--json` writes one machine-readable result to stdout; human progress goes to
stderr. Exit code `0` means every restore and assertion passed, `1` means the
drill ran and failed, and `2` means configuration or prerequisites were invalid.

Minimal `restore-drill.toml`:

```toml
version = 1

[drill]
name = "weekly-orders"
network = "restore-drill-weekly-orders"
timeout_seconds = 300
report_dir = ".restore-drill/reports"
signing_key = ".restore-drill/signing.key"

[postgres]
image = "postgres:16-alpine"
container = "restore-drill-db"
database = "app"
user = "restore_drill"
credential_file = ".restore-drill.env"

[source]
kind = "dump"
path = "backups/latest.dump"
format = "auto"

[[assertions.sql]]
name = "orders survived"
query = "SELECT count(*) > 0 FROM orders"
expect = "t"

[[assertions.http]]
name = "application health"
url = "http://restore-drill-app:3000/health"
status = 200
body_contains = "ok"

[[services]]
name = "restore-drill-app"
image = "ghcr.io/example/app:stable"
env_file = ".restore-drill-app.env"
```

For a physical volume archive created with `tar`, select `volume_tar`:

```toml
[source]
kind = "volume_tar"
path = "backups/postgres-data.tar.gz"
```

The archive must contain a Postgres data directory compatible with the selected
image major version. Restore Drill rejects symlinks and path traversal before
extraction.

HTTP probes run from a curl container on Docker's `--internal` network. Their
hostnames must exactly match a declared service; public, loopback, and production
URLs are rejected during validation. Secrets are passed to Docker with
`--env-file`, never put on the command line or copied into reports.

Reports include recovery duration, assertion evidence, the SHA-256 backup hash,
the resolved image IDs, and an Ed25519 signature. Verify a report later with:

```sh
restore-drill verify .restore-drill/reports/weekly-orders-2026-08-27T120000Z.json \
  --public-key .restore-drill/signing.key.pub
```

The public key is saved beside the signing key as `signing.key.pub`. Retain it
separately and pass `--public-key` for independent signer verification; without
that option, `verify` checks only that the report is internally intact.

## Scheduling

Example weekly cron entry:

```cron
0 3 * * 0 cd /srv/app && /usr/local/bin/restore-drill run --config restore-drill.toml --json >> .restore-drill/cron.jsonl 2>&1
```

Run once manually first and keep the reports outside ephemeral CI storage. A
deliberately truncated dump should produce exit code `1`; test that failure path
before trusting the schedule.

## Develop and verify

```sh
npm ci
npm test
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

`npm test` runs Rust unit/integration tests plus landing-page checks.
`npm run build` produces the binary in `dist/bin/` and the deployable static site
in `dist/site/`. Run the site locally with `npm run dev`.

## Scope and safety

Restore Drill supports Postgres dump files and tarred data volumes plus optional
application containers. It does not make backups, orchestrate PITR, operate on
an existing network or container, or restore production. Docker resources use
the `restore-drill` label and are removed after success, failure, timeout, or
Ctrl-C. `--keep-on-failure` exists for local diagnosis and prints the exact
resources left behind.

Security reports are welcome through GitHub's private vulnerability reporting.
See the website's [privacy](https://restore-drill.sociobot.in/privacy/) and
[terms](https://restore-drill.sociobot.in/terms/) pages for the one-time Team
Kit purchase.
