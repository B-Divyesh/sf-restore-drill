# Restore Drill

Restore Drill is for small self-hosted teams that need recovery proof before an outage.

It rehearses a Postgres backup restore in an isolated Docker network.
It records a signed JSON report after the checks finish.

## Try the shipped sample

Docker Engine must be installed and reachable by the current user.

```sh
cargo install --path crates/restore-drill
restore-drill demo
```

`demo` copies the shipped SQL sample and configuration to a fresh system
temporary directory. It prints that directory when the drill completes. The
sample contains three fictional orders and checks that all three restore.

The browser walkthrough is available at
<https://restore-drill.sociobot.in/demo/?demo=1>.

## Run your own drill

Create a starter configuration and local credential file:

```sh
restore-drill init --output restore-drill.toml
$EDITOR .restore-drill.env
restore-drill check --config restore-drill.toml
restore-drill run --config restore-drill.toml --json
```

Use a copied backup and review the configuration before running it. The command
rejects HTTP checks that target public or undeclared hosts. Keep the report and
public signing key outside temporary CI storage.

`--json` writes the final result to standard output. Progress goes to standard
error. Exit code `0` means the drill passed. `1` means it ran and failed. `2`
means the configuration or prerequisites were invalid.

## Commands

```text
restore-drill demo
restore-drill init --output restore-drill.toml
restore-drill check --config restore-drill.toml
restore-drill run --config restore-drill.toml [--json] [--keep-on-failure]
restore-drill verify REPORT --public-key SIGNING_KEY.pub
```

Run `restore-drill --help` for each command's options.

## Develop, test, and deploy

```sh
npm ci
npm test
npm run check:types
npm run check:lint
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

`npm run build` writes the Linux binary to `dist/bin/` and the static site to
`dist/site/`. The factory deploys `dist/site/` as a static site; workers do not
change DNS, infrastructure, or billing.

The claim registry is [`.factory/claims.json`](.factory/claims.json). Run every
listed command from a clean clone before release.

## License and policies

Restore Drill is MIT-licensed. See [LICENSE](LICENSE),
[Privacy](https://restore-drill.sociobot.in/privacy/), and
[Terms](https://restore-drill.sociobot.in/terms/).
