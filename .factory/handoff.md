# Restore Drill polish 2 handoff

## Outcome

All findings in `.factory/review-1.md`, `.factory/polish-1.md`, and
`.factory/review-2.md` are resolved. The complete 66-finding mapping is in
`.factory/polish-2.md`. No known gaps remain.

The release is live at <https://restore-drill.sociobot.in/>. Static deployment
ID: `0b00eaac-8e8b-4bfe-8cfa-e3be42c87c0e`.

## Delivered

- Rewrote the first screen around the restore job, the small-team audience,
  one sample action, its outcome, and three short facts.
- Replaced the fixed mock terminal with a timed walkthrough exported from an
  actual Docker/Postgres run. `/demo/?demo=1` starts it directly.
- Added a persistent demo banner, demo-only `sessionStorage`, Reset, Start for
  real, an inspectable signed report, and a report download.
- Added a real-Docker claim test covering a successful three-row restore, a
  corrupt backup failure, signature verification, and cleanup after both.
- Fixed the Postgres readiness race found by that real-daemon test.
- Added ten claims to `.factory/claims.json`, each with exactly one tagged
  observable test.
- Added JSON/exit-code, boundary, local-I/O, image-pull, distribution, weekly
  schedule, MIT-license, and no-tracking coverage.
- Completed route-specific titles, descriptions, canonical/Open Graph/Twitter
  metadata, focus announcements, Back navigation, legal links, and the real
  styled 404 response.
- Fixed 390 px terminal wrapping, 44 px targets, mobile Axe coverage, offline
  reload, and storage/privacy assertions.
- Added complete source installation and scheduling examples without claiming
  that release binaries exist.
- Preserved the halftone field-report identity and its ink, paper, and
  vermilion palette.
- Updated the README, CHANGELOG, demo/design/copy documentation, catalog line,
  and this handoff.

## Verification evidence

### Real Docker acceptance

GitHub Actions run
<https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33173322960>
passed on commit `0e220c7871a7bcf154e252a3a41f6f4822069c3e`.
It ran the exact `real-docker-restore` claim command on a real Docker daemon.
The exported report records `status: passed`, `observed: 3`, and an 88-character
signature. The same report verifies with the release CLI.

### Clean checkout

The clean checkout at `0e220c7871a7bcf154e252a3a41f6f4822069c3e`
passed every command in `.factory/claims.json`. Exact output is stored in
`.factory/evidence/polish-2/clean-claim-tests.log`.

It also passed:

- `npm test`: 4 Rust unit tests, 7 CLI integration tests, 3 Node claim tests,
  7 policy tests, and 20 Playwright tests across desktop and 390 px mobile.
- `npm run check:types`.
- `npm run check:lint`, including `cargo fmt` and Clippy with warnings denied.
- `npm run build`, producing `dist/bin/restore-drill-linux-x86_64` and
  `dist/site/`.
- `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`:
  112.3 KiB unpacked and 28.8 KiB compressed.

Full output is in `.factory/evidence/polish-2/clean-full-suite.log`.

### Deployed site

`npm run test:live -- https://restore-drill.sociobot.in/
.factory/evidence/live-polish-2` passed cold against production. It checked
Home, Demo, Privacy, Terms, and an unknown route; route titles and metadata;
one h1 and main landmark; responsive width; serious/critical Axe results;
demo playback and Reset; isolated session state; report data; service-worker
cache; offline reload; external requests; and console output.

Results:

- Home, Demo, Privacy, and Terms: HTTP 200 and zero serious/critical Axe issues.
- Unknown route: HTTP 404 with the designed Restore Drill page.
- External requests during the tested flow: zero.
- IndexedDB databases: zero; service workers: one; cache:
  `restore-drill-shell-v3`; demo state exists only in `sessionStorage`.
- Expected browser console entry: the deliberately requested unknown route's
  HTTP 404 only.
- `/opt/fleet/lib/verify-url.sh`: title, language, h1, main, image alt,
  button names, and console checks passed in 851 ms.

Lighthouse 12.8.2 mobile scores against production:

- Performance: 100
- Accessibility: 100
- Best practices: 100
- SEO: 100
- FCP: 0.9 s; LCP: 1.7 s; CLS: 0; total blocking time: 50 ms

The production build ships 4.31 kB JavaScript and 14.24 kB CSS before gzip
(1.92 kB and 3.93 kB gzip). Live JSON, screenshots, and Lighthouse output are
under `.factory/evidence/live-polish-2/`. Review screenshots are under
`.factory/evidence/polish-2/`.

## Run and verify

```sh
npm ci
npm test
npm run check:types
npm run check:lint
npm run build
npm run test:live -- https://restore-drill.sociobot.in/ .factory/evidence/live-polish-2
```

The real Docker claim needs a Docker daemon and may pull
`postgres:16-alpine`:

```sh
cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored
```

Deployment used the work-order commands:

```sh
npm ci
npm run build:site
/opt/fleet/lib/deploy-static.sh restore-drill dist/site
```

## Known gaps

None.
