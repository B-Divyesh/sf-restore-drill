# Restore Drill independent verification — FAIL

**Candidate:** `51be449960dbc92e7c4aa1b37cb484c7516756e6`  
**Live URL:** https://restore-drill.sociobot.in/  
**Verified:** 2026-08-28 UTC  
**Verdict:** **FAIL** — the deployed product bytes match the candidate and the
functional checks pass, but the live host does not apply the candidate's
security or immutable-cache response policies. This fails the product's
deployment quality contract.

## Evidence

### Clean checkout, quality gates, and distribution

- Checkout was clean and `git rev-parse HEAD` returned the candidate SHA.
- `npm ci` completed with 0 audit vulnerabilities.
- `npm test` passed: Rust unit/integration suite (7 tests) plus Playwright
  desktop and 390x844 mobile suite (9 passed; one deliberate duplicate mobile
  axe pass skipped).
- Exact production build, `npm run build`, passed and produced
  `dist/bin/restore-drill-linux-x86_64` (2,001,400 bytes) and `dist/site/`.
  Initial payloads are 6,625-byte JS, 10,999-byte CSS, no font payload, and a
  181,360-byte WebP hero: all stated asset budgets pass.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  and `npm audit --audit-level=high` all passed. No separate TypeScript type or
  lint script is configured; Vite compiled the TypeScript during the production
  build.
- `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`
  passed (25.6 KiB compressed). The resulting crate was extracted into a clean
  consumer directory and installed with `cargo install --path ... --locked`.
  The installed `restore-drill 0.1.0` binary had the documented help and
  commands. `init` created its credential file at mode 0600; a duplicate init
  exited 2; `check` against the starter's missing backup exited 2 with the
  actionable missing-file error.

### CLI behaviour and safety boundaries

- Authored integration tests exercised a successful simulated Docker engine
  drill with a signed report and independent public-key verification, then a
  deliberately broken restore that returned exit 1, emitted failed JSON, and
  removed labelled containers, volume, and network.
- Config tests accept a declared internal HTTP service and reject an HTTPS/
  production URL; archive validation rejects unsafe traversal/link inputs.
  Static review confirms the required `restore-drill-` disposable network and
  container prefixes, `docker network create --internal`, declared-service-only
  HTTP probes, local credential files passed as `--env-file`, artifact hashing
  before/after the drill, report signatures, and cleanup on normal failures.
- A real Docker/Postgres restore was not run: this verifier container has no
  `docker` executable, daemon, or socket. This is an environment limitation,
  not a passing substitute for operational acceptance; run one representative
  live dump and one intentionally corrupt dump on a Docker host before release.

### Live deployment identity, browser, privacy, PWA, and accessibility

- Freshly built and live artifacts match byte-for-byte:
  `index.html`, `assets/main-Dae_hWeU.js`, `assets/style-DJLIgwO5.css`, and
  `restore-chamber.webp` have identical SHA-256 values. The live response is
  therefore the candidate's static product, not a stale deployment.
- Chromium checks at desktop and 390x844 mobile found no page errors, console
  errors, failed requests, horizontal overflow, or third-party requests on a
  fresh load. The landing page has one `h1`, `main`, title, language, meaningful
  image alt text, privacy/terms pages, and no runtime CDN or analytics request.
- Keyboard starts on the visible 3 px skip-link focus ring. Healthy preview,
  deliberately broken-preview recovery path, and reduced-motion preview all
  worked; the reduced-motion preview completed in 255 ms. Axe WCAG 2 A/AA and
  2.1 A/AA returned zero serious or critical violations.
- The live service worker became controlling and an offline reload retained the
  title and one `h1`; the cached shell is usable offline. License storage and
  verification are local-first and no license request is made on a fresh load.
- Lighthouse 13 could not be run in this container because its Chrome launcher
  could not connect to the preinstalled Playwright Chromium, even with
  `CHROME_PATH` and `--no-sandbox`. This is recorded as a test-tool limitation;
  asset-budget and live browser evidence above were obtained independently.

## Defects

### Medium — production response policies and immutable caching are not deployed

`site/public/_headers` declares CSP, `Permissions-Policy`,
`frame-ancestors 'none'`, and one-year immutable caching for `/assets/*` and
the hero. Fresh `curl -I` evidence from the live host instead shows only:

```
Cache-Control: public, must-revalidate, max-age=30
Referrer-Policy: strict-origin-when-cross-origin
X-Content-Type-Options: nosniff
```

The HTML, JS, CSS, and WebP responses omit `Content-Security-Policy`,
`Permissions-Policy`, and frame-embedding protection; immutable cache headers
are missing from hashed assets and the hero. This is a deployment/configuration
defect, not a source-byte mismatch. Configure the actual static host to honour
the equivalent rules, then re-verify live headers and cache behaviour.

### No critical or high defects found

## Release conditions

1. Apply the response-policy and cache configuration at the serving layer and
   re-run the live header check.
2. On a Docker-capable runner, perform one real representative dump restore and
   one corrupt-artifact run, retaining the signed reports and verifying their
   exit codes/cleanup.
3. Re-run Lighthouse on a Chrome-compatible runner if the factory requires the
   numeric score gate.
