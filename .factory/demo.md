# Restore Drill demo

## Entry points

- Web: `https://restore-drill.sociobot.in/demo/?demo=1`
- CLI: `restore-drill demo`

The web route immediately replays a recorded run of the real binary. It links
the signed JSON report created by that run. The persistent banner identifies
sample mode and provides **Reset demo** and **View installation steps**.

## Browser isolation

Playback progress uses only the `demo:restore-drill:playback` session-storage
key. The page never reads or writes application data. **Reset demo** clears all
`demo:restore-drill:*` session keys and restarts the recording. **View
installation steps** clears the same namespace and opens the source-install
section.

The recording and report are public, read-only files under `site/public/demo/`.
They are available offline after the first visit through the first-party site
cache. That cache contains public site assets only.

## CLI isolation

`restore-drill demo` creates `restore-drill-demo-*` below the system temporary
directory. It writes the shipped backup, configuration, credentials, signing
key, and report only there. It never reads a caller configuration or backup.
Docker resources carry `in.sociobot.restore-drill=managed` and are removed by
the normal cleanup path after both passing and failing drills.

## Sample and recording provenance

The sample is a plain Postgres SQL backup with three fictional orders. Its SQL
check expects a count of three after restoration. The source files ship under
`examples/` and `crates/restore-drill/examples/`.

The `real-docker-restore` claim runs the healthy and corrupt samples against
`postgres:16-alpine` on a real Docker daemon. When
`RESTORE_DRILL_CAPTURE_DIR` is set, that same test exports the successful
transcript and signed report. Temporary paths and the host-dependent elapsed
time are replaced in the transcript. The report stays unaltered so its
Ed25519 signature remains verifiable.
