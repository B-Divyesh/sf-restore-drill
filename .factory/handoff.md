# Restore Drill v0.1.0 handoff

> ## Repair verification — **PASS** (2026-08-28 UTC)
>
> Repair commit `8dd45ba0fadb0108d34d0a1366eb1c74f503d7eb` resolves the one
> independent-verifier release blocker in candidate
> `51be449960dbc92e7c4aa1b37cb484c7516756e6`. Azure Static Web Apps does not
> read the `_headers` convention, so the deployable site now contains its native
> `staticwebapp.config.json`. The public host now returns the required CSP,
> Permissions-Policy, `X-Frame-Options: DENY`, and immutable cache controls for
> fingerprinted assets and the hero. The original CLI and landing-page behavior
> remain covered and passing. A Docker executable and daemon are still absent in
> this worker, so the existing requirement for one live representative restore
> and corrupt-artifact run on a Docker-capable host remains operational
> acceptance work, not a release-policy defect.

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
  pages, security/cache headers, and a $39 one-time Team Kit unlock. The static
  root includes Azure-native `staticwebapp.config.json`; `_headers` remains as a
  portable fallback for hosts that support it.
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
npm run check:types
npm run check:lint
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

Build output:

- Static deploy root: `dist/site/` (`index.html` is at that root)
- Release binary: `dist/bin/restore-drill-linux-x86_64`
- Ready-to-publish crate: `target/package/restore-drill-0.1.0.crate`

Repair verification completed on 2026-08-28:

- Clean `npm ci`, `npm test`, `npm run check:types`, `npm run check:lint`,
  `npm run build`, `cargo package --allow-dirty`, and
  `npm audit --audit-level=high` passed. Rust: 7 tests; policy regression suite:
  3 tests; Playwright 1.58.2: 9 passed and 1 intentional duplicate mobile axe
  pass skipped.
- The new policy regression suite asserts Azure's exact global CSP,
  Permissions-Policy, referrer/content-type/frame headers; both immutable cache
  routes; no immutable HTML route; and no divergence from `_headers`.
- The packaged `restore-drill-0.1.0.crate` was extracted into a clean consumer
  directory and installed with `cargo install --path ... --locked`. Its help
  exposes the documented commands, `init` created the credential file at mode
  `0600`, duplicate `init` exited `2`, and `check` against the starter's missing
  backup exited `2`.
- Live identity: the deployed `index.html` SHA-256 is
  `364732a37da4df3944d449b7faa56149821029af65bb4a95b142481470fbc8a7` and the
  hero SHA-256 is
  `d14467c5d1d117d8a9ff3b46d8766473fe4498e765c50f70b983e4b17af71fe4`, matching
  this build byte-for-byte.
- Live response policy: `/` returns the CSP, Permissions-Policy, referrer
  policy, `nosniff`, and `X-Frame-Options: DENY`; the fingerprinted 6,625-byte JS
  and 181,360-byte hero return
  `Cache-Control: public, max-age=31536000, immutable` plus the same security
  policy. HTML remains revalidatable at `max-age=30`.
- Live `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, 762 ms load, title/lang,
  one `h1`, `main`, no missing image alt text or unlabeled buttons, and zero page
  or console errors. A direct live Playwright check found no third-party
  requests, no 390×844 overflow, a focused skip link on first Tab, a controlling
  service worker with a working offline reload, and zero serious/critical Axe
  WCAG 2 A/AA/2.1 AA violations.
- Production payload: 6.63 KB JS, 11.00 KB CSS, no font payload, and a 181.36 KB
  hero. `cargo package` produced a 25.6 KB compressed crate.
- Lighthouse 13.4.1 was retried with the preinstalled Chromium. It still cannot
  complete in this container because Chrome's tab crashes; browser, payload, and
  accessibility checks above completed independently. The prior verifier run
  recorded 99 performance, 100 accessibility, 100 best practices, and 100 SEO.

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
