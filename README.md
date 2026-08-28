# Restore Drill

Restore Drill is for small self-hosted teams that need recovery proof before an outage.

It restores a Postgres backup in an internal Docker network. It records a
signed report file you can check later.

## Install from source

Docker Engine and a Rust toolchain must be installed. Copy and run these
commands from any working directory:

```sh
git clone https://github.com/B-Divyesh/sf-restore-drill.git
cd sf-restore-drill
cargo install --locked --path crates/restore-drill
restore-drill demo
```

The demo command copies the shipped SQL sample and configuration to a fresh
system temporary directory. It prints that directory when the drill completes.
The sample contains three fictional orders and checks that all three restore.

Replay the verified demo at
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
rejects public, loopback, HTTPS, and undeclared HTTP targets before Docker runs.
Keep the report and public signing key outside temporary automation storage.

With `--json`, standard output contains one final report object. Other messages
use standard error. A passed drill exits `0`; a failed drill exits `1`; invalid
input exits `2`.

## Schedule a weekly drill

The [schedule examples](examples/schedule/) include a portable cron runner and
a GitHub Actions workflow. Both preserve reports and return a failing exit code.
Credentials come from a mode-0600 file or an encrypted repository secret.

## Commands

```text
restore-drill demo [--json]
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
npm run test:docker
npm run check:types
npm run check:lint
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

`npm run test:docker` needs a real Docker daemon and pulls Postgres 16 Alpine.
`npm run build` writes the executable Linux CLI to `dist/bin/` and the complete
static site to `dist/site/`.

The factory deploys `dist/site/` as a static site. The claim registry is
[`.factory/claims.json`](.factory/claims.json). Run every listed command from a
clean clone before release.

## Privacy

A drill leaves the configured backup unchanged. Reports and new signing keys
are written only to configured output paths. The documentation site uses no
analytics or tracking cookies. Its first-party offline cache contains only
public site files.

## License and policies

Restore Drill is MIT-licensed. See [LICENSE](LICENSE),
[Privacy](https://restore-drill.sociobot.in/privacy/), and
[Terms](https://restore-drill.sociobot.in/terms/).
