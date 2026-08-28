# Restore Drill independent verification 2 — PASS

**Candidate:** `150ebbbdc89317f39907bccd5ecd755abb2f3b45`  
**Live URL:** https://restore-drill.sociobot.in/  
**Verified:** 2026-08-28 UTC  
**Verdict:** **PASS** — fresh independent evidence finds no acceptance-blocking
defect. The prior live response-policy failure is repaired and the deployed
static product matches this candidate byte-for-byte. One live Docker/Postgres
restore remains unexecuted because this verifier has no Docker executable,
daemon, or socket; the deterministic Docker-compatible integration harness
passed and this is recorded below as an operational validation gap, not a
passing substitute for a real engine.

## Clean checkout and release gates

- Created a separate clean clone at the candidate SHA; it was clean and
  `git rev-parse HEAD` returned
  `150ebbbdc89317f39907bccd5ecd755abb2f3b45`.
- `npm ci` passed and `npm audit --audit-level=high` reported **0
  vulnerabilities**.
- `npm run check:types` passed. `npm run check:lint` passed:
  `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D
  warnings`.
- `npm test` passed: 4 Rust unit tests, 3 Rust CLI/integration tests, 3
  response-policy tests, and Playwright 1.58.2 with **9 passed / 1 intentional
  duplicate mobile-axe skip**. Desktop Chromium and a 390×844 touch viewport
  both ran.
- The exact production command, `npm run build`, passed. It produced
  `dist/bin/restore-drill-linux-x86_64` (2,001,400 bytes) and `dist/site/`.
  `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`
  passed and independently verified the 25.6 KiB compressed crate.
- Production payloads meet the stated static budgets: JavaScript 6,625 bytes
  (≤200 KB), CSS 10,999 bytes (≤50 KB), no web-font payload, and hero WebP
  181,360 bytes (≤300 KB).

## CLI, package, safety, and recovery evidence

- Extracted the packaged `restore-drill-0.1.0.crate` into a clean consumer
  directory and installed it with `cargo install --path … --root … --locked`.
  The installed binary exposes the documented `init`, `check`, `run`, and
  `verify` commands and helpful `--help` text.
- In that consumer, `init` created the documented starter and a credential file
  at mode **0600**. A repeat init exited **2** without overwriting; `check`
  against the starter's absent backup exited **2** with the actionable
  missing-file error. These cover normal setup, a non-destructive boundary,
  and recovery from invalid input.
- The Rust integration harness exercised a full successful dump drill against
  a Docker-compatible fake engine: image identity collection, explicit
  `network create --internal`, local `--env-file`, restore, SQL assertion,
  signed JSON report, public-key verification, and labelled container/volume/
  network teardown. Its deliberately broken restore returned exit **1** with
  `status: "failed"` JSON and cleanup evidence.
- Config tests accept a declared internal HTTP service and reject a production
  HTTPS target; archive validation rejects traversal and links. Source review
  confirms disposable `restore-drill-` network/container prefixes,
  declared-service-only HTTP probes, local credential handling, SHA-256
  artifact evidence, Ed25519 reports, and cleanup on success/failure.

### Operational validation limitation (not a product defect)

`docker` is not installed and no daemon/socket is available in this verifier.
Accordingly, a real Postgres image restore and corrupt-dump run could not be
performed here. Before a release tag, run one representative backup and one
intentionally corrupt copy on a Docker-capable runner, retain both signed
reports, and confirm exit codes and resource cleanup.

## Live deployment, privacy, browser, and PWA evidence

- Fresh live SHA-256 checks match this build exactly:

  | Asset | SHA-256 |
  | --- | --- |
  | `index.html` | `364732a37da4df3944d449b7faa56149821029af65bb4a95b142481470fbc8a7` |
  | `assets/main-Dae_hWeU.js` | `2e8fbf3768b8e66f27ad3f727de4c8611e6c5851a67027deeb1ebe4fc0a790c5` |
  | `assets/style-DJLIgwO5.css` | `84daa044d5bcbac6d9550babb09aa045639cd81f8e45a5add81f1811069c130b` |
  | `restore-chamber.webp` | `d14467c5d1d117d8a9ff3b46d8766473fe4498e765c50f70b983e4b17af71fe4` |
  | `sw.js` | `9ed461dc998a27da8d39fa225f8c291ad33c9ec61a93de1a0e60494b0f9d3371` |

- Fresh response headers on `/`, `/privacy/`, `/terms/`, JavaScript, CSS,
  hero, and service worker include the expected CSP (`default-src 'self'`;
  only the Sociobot license API is allowed for `connect-src`),
  `Permissions-Policy: camera=(), microphone=(), geolocation=()`,
  `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, and strict
  referrer policy. HTML and `sw.js` are revalidatable (`max-age=30`); hashed
  JS/CSS and the hero return `public, max-age=31536000, immutable`.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, 607 ms network-idle load,
  title and `lang=en`, one `h1`, one `main`, no missing image alt text or
  unlabeled button, and no console/page errors.
- Independent live Playwright checks at 1440×960 and **390×844** found no
  console errors, page errors, failed requests, third-party requests, or
  horizontal overflow. A fresh load makes no network call to the license API,
  analytics, or a CDN. Source and runtime evidence show the only optional
  outbound request is license verification after a user supplies a token; the
  core CLI and reports stay local.
- Keyboard-only use begins with the visible solid skip-link focus outline in
  both viewports. The healthy preview reaches `Passed`; the broken-backup
  preview reaches `Failed` with the recovery explanation. With reduced motion,
  the preview completed in 227 ms. Axe WCAG 2 A/AA and 2.1 A/AA returned **0
  serious or critical findings**.
- The current service worker controls the page, `registration.update()`
  completed with no waiting worker, and an offline reload retained the title
  and one `h1`. Its versioned cache and `skipWaiting`/`clients.claim` update
  path were inspected. An actual version-to-version update cannot be induced
  without changing the deployed origin.
- Lighthouse 13.4.1 (mobile defaults, preinstalled Chromium) scored
  **99 performance, 100 accessibility, 100 best practices, 100 SEO**;
  FCP 1.0 s, LCP 1.8 s, CLS 0, TBT 80 ms.

## Defects by severity

- **Critical:** none found.
- **High:** none found.
- **Medium:** none found. The previous missing live CSP/frame/cache policy is
  fixed in this candidate's live deployment.
- **Low:** none found.

## Release follow-up

Run the two real Docker-host drills described above before tagging, then retain
the public signing key separately from the runner and schedule the healthy
artifact weekly. This is operational acceptance evidence still needed; it does
not change this candidate's PASS result from the available independent checks.
