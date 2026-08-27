# Restore Drill v0.1.0 handoff

## What shipped

- A Rust single-binary CLI with `init`, `check`, `run`, and `verify` commands,
  non-interactive operation, documented exit codes, human output, and `--json`.
- Docker orchestration that pulls images before creating a new explicitly named
  `--internal` network, rejects undeclared HTTP hosts, restores Postgres plain or
  custom dumps and validated physical-volume tar archives, starts optional app
  containers, and probes SQL plus HTTP behavior.
- Default teardown of every labeled container, temporary volume, and network on
  success or failure. `--keep-on-failure` is an explicit diagnostic escape hatch
  and records retained resource names in the failed report.
- Local SHA-256 artifact evidence with a before/after mutation check, resolved
  image IDs, per-assertion durations, measured recovery time, Ed25519 signatures,
  collision-safe report creation, and optional verification against a separately
  retained public key.
- Credential files are resolved locally, passed through Docker `--env-file`,
  excluded from reports, required to contain a non-empty password, and required
  to be mode `0600` on Unix. `init` creates them with that mode.
- A responsive static landing/docs site at `dist/site`, including the interactive
  healthy/broken drill preview, offline shell, install/run documentation, legal
  pages, security/cache headers, and a $39 one-time Team Kit unlock.
- The paid flow uses only Sociobot's product-slug checkout and verify endpoints,
  stores `sb_license:restore-drill`, strips returned tokens from the URL, caches
  successful verification for at most one day, preserves a prior valid verdict
  offline, and supports pasted-license restoration. The core CLI, safety checks,
  signed reports, and JSON output remain free.
- Three locally generated Team Kit downloads: a CI workflow, four-week scorecard,
  and drill checklist.

## Visual system and asset

The visual thesis is recorded in `.factory/design.md`. It uses a single-mode
warm-paper, forest-ink, vermilion field-report system with Georgia editorial type,
system monospace evidence type, an 8 px rhythm, one-shot registration motion, and
a complete reduced-motion fallback.

The original hero was generated with `/opt/fleet/lib/gen-image.sh` using the
factory-image deployment and the prompt recorded in
`site/public/restore-chamber.provenance.json`. The published WebP is 1280×853 and
181,360 bytes (below the 300 KB budget). No stock assets, third-party fonts,
runtime CDNs, analytics, or tracking are present.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

Build output:

- Static deploy root: `dist/site/` (`index.html` is at that root)
- Release binary: `dist/bin/restore-drill-linux-x86_64`
- Ready-to-publish crate: `target/package/restore-drill-0.1.0.crate`

Verification completed on 2026-08-27:

- Rust: 7 tests passed (config safety, unsafe archive rejection, production URL
  rejection, signature tampering, CLI help/init, full simulated-engine success,
  deliberately broken restore, signed report verification, and teardown checks).
- Playwright 1.58.2: 9 passed, 1 intentional duplicate mobile axe pass skipped;
  desktop Chromium and a 390×844 touch viewport were exercised.
- Axe/WCAG 2 A/AA/2.1 AA: no serious or critical findings.
- `/opt/fleet/lib/verify-url.sh`: HTTP 200, title/lang/main present, one `h1`, no
  missing alt text, no unlabeled buttons, and zero console/page errors.
- Lighthouse 13 mobile: performance **99**, accessibility **100**, best practices
  **100**, SEO **100**; FCP 0.9 s, LCP 2.0 s, CLS 0, TBT 0 ms.
- Production payload: 6.63 KB JS, 11.00 KB CSS, no font payload, 181.36 KB hero.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `cargo package`: passed, 25.6 KB compressed crate.

## Known gaps and release steps

- The disposable builder image has no Docker executable, daemon, or socket, so a
  live Postgres container drill could not run here. A deterministic Docker-CLI
  integration harness exercised the complete successful and corrupted-backup
  command sequences, reports, exit codes, and teardown. Run one live dump drill
  on a Docker host before tagging the release.
- The factory must register the `restore-drill` product and return URL in the
  Sociobot billing engine. No provider credentials or product IDs are embedded.
- The produced binary targets this worker's Linux x86-64 platform. Build release
  binaries for other supported targets from the packaged crate.

Suggested first operational acceptance: restore a representative dump, truncate
a copy and confirm exit `1`, pin `signing.key.pub` outside the runner, then schedule
the healthy artifact weekly and review four consecutive recovery-time reports.
