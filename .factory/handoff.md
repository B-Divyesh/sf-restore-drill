# Restore Drill — polish 4 handoff

## Outcome

**PASS.** Every finding in `.factory/review-1.md` through
`.factory/review-4.md` is resolved and mapped in `.factory/polish-4.md`.
Restore Drill is deployed at <https://restore-drill.sociobot.in/>. The static
deployment ID is `b351562d-a2ea-430e-8425-94e4504f6956`.

## Delivered

- Restored usable 390 px header navigation on every route. The product name,
  Demo, How it works, and Privacy now remain visible with 44 px targets.
- Renamed the workflow heading to **Rehearse a restore in four steps.**
- Added mobile-navigation and deployed-live assertions so both review-4 UI
  findings stay fixed.
- Revalidated the exact healthy and corrupt Postgres claim on a clean checkout
  with a real Docker daemon.
- Updated the catalog line to the 70-character, verb-first sentence: “Prove
  your Postgres backup restores inside an isolated Docker network.”
- Corrected demo provenance documentation to describe the real test capture
  path instead of a nonexistent script.
- Preserved the warm-paper, forest-ink, vermilion, halftone field-report visual
  system and the original recovery-chamber art.

## Exact verification evidence

### Every claim from clean checkouts

A fresh checkout at `40b270e6d9004bbec0600d67cea3d2a308851cc2`
was created at `/tmp/restore-drill-polish4-clean-6ZFRWO/repo`. `npm ci` found
zero vulnerabilities. Every non-Docker command listed in
`.factory/claims.json` passed there:

- `demo-sandbox`: 2 Playwright projects passed.
- `production-boundary`: the tagged Rust configuration test passed.
- `automation-contract`: the tagged Rust CLI test passed.
- `local-io-boundary`: the tagged Rust filesystem-boundary test passed.
- `image-pull`: the tagged Rust image-order test passed.
- `distribution-build`: the build and distribution assertion passed.
- `weekly-scheduling`: the tagged Node test passed.
- `mit-license`: the tagged Node test passed.
- `site-no-tracking`: 2 Playwright projects passed.
- `offline-web-walkthrough`: 2 Playwright projects passed.

The remaining claim, `real-docker-restore`, ran from a separate fresh checkout
of the same commit on GitHub's Docker-capable Ubuntu runner:
<https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33185239060>.
The exact registered command passed:

```sh
cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored
```

It restored the three shipped rows on Postgres 16 Alpine, verified the healthy
report, rejected corrupt SQL with exit 1, verified that failed report, and
asserted zero labelled containers, volumes, or networks after both runs.

### Full clean-checkout suite

- `npm test`: passed — 4 Rust unit tests, 7 CLI integration tests, 3 Node
  claim tests, 7 policy/structure tests, and 22 desktop/mobile Playwright tests.
- `npm run check:types`: passed.
- `npm run check:lint`: passed (`cargo fmt` and Clippy with warnings denied).
- `npm run build`: passed and produced `dist/bin/` plus `dist/site/`.
- `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`:
  passed; 112.3 KiB unpacked and 28.8 KiB compressed.
- Production payload: 4.31 kB JavaScript (1.92 kB gzip), 14.32 kB CSS
  (3.94 kB gzip), no web fonts, and a 181 kB hero image.

The Playwright suite covers the first screen, one-click demo, reset and exit,
storage isolation, same-origin traffic, offline reload, signed sample report,
titles, metadata, links, route focus, 404, 390 px overflow, 44 px targets,
visible mobile navigation, and Axe on every route at both viewports.

### Cold deployed audit

After deployment, a new 390 × 844 browser context opened Home, Demo, Privacy,
Terms, and a missing route. `npm run test:live` passed with:

- correct route titles, one H1, one main landmark, canonical/Open Graph/Twitter
  metadata, shared product chrome, and the designed HTTP 404;
- visible mobile product name and all three primary links at 44 px or larger;
- the exact four-step heading;
- zero serious or critical Axe findings;
- a passing signed report with three restored rows and an 88-character
  signature;
- only `demo:restore-drill:playback` session state, one first-party service
  worker, no IndexedDB, public same-origin cache entries, and a passing offline
  reload;
- no external browser requests and no unexpected console errors.

Evidence is under `.factory/evidence/live-polish-4/`:

- `cold-check.json`
- `home-cold-390.png`
- `demo-cold-390.png`
- `404-cold-390.png`
- `verify/verify.json` and verify screenshots
- `lighthouse.json`

`/opt/fleet/lib/verify-url.sh` passed at 667 ms network idle with the correct
title, `lang=en`, one H1, one main, complete alt text, labelled buttons, and no
console errors. Lighthouse 13.4.1 scored **100 Performance, 100 Accessibility,
100 Best Practices, and 100 SEO**. FCP was 0.8 s, LCP 1.7 s, CLS 0, and TBT
30 ms.

Local polish screenshots are
`.factory/evidence/polish-4/home-mobile.png` and
`.factory/evidence/polish-4/demo-mobile.png`.

## Run, package, and deploy

```sh
npm ci
npm test
npm run test:docker
npm run check:types
npm run check:lint
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
npm run test:live -- https://restore-drill.sociobot.in/ .factory/evidence/live-polish-4
```

The crate is ready for the factory publishing workflow; it was not published
from this worker. Static deployment used:

```sh
/opt/fleet/lib/deploy-static.sh restore-drill dist/site
```

## Known gaps and next steps

None. No review finding or acceptance task remains open.
