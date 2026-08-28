# Restore Drill demo

## Entry points

- Web: `https://restore-drill.sociobot.in/demo/?demo=1`
- CLI: `restore-drill demo`

The web route shows the exact command, its sample data, and the expected report
path. Its persistent banner identifies the sample and offers **Reset demo** and
**Start for real**.

## Isolation

`restore-drill demo` creates a directory named `restore-drill-demo-*` below the
system temporary directory. It writes a copy of `examples/sample-backup.sql`, a
sample configuration, credentials, signing key, and report only there. It never
reads a caller configuration or backup. Docker resources use the usual
`restore-drill-*` disposable names and are removed by the normal drill cleanup.

The command prints the temporary directory. Removing that directory discards all
demo files. Resetting the web walkthrough reloads its static sample state; the
site does not store demo data in browser storage.

## Sample

The sample is a plain Postgres SQL backup containing three fictional orders. Its
SQL assertion expects a count of three after restoration. The source files ship
under `examples/` and the Rust integration test exercises the same bundled
files through the normal drill pipeline.
