# Restore Drill review 2 handoff

## Delivered

- Wrote `.factory/review-2.md` for candidate
  `07f624e2eee54bd2eeb1d8119c62df0bb6a23e4d`.
- Verdict: **FAIL**.
- No product code was modified.

## Verification performed

- Opened the live home cold in fresh Chromium contexts at 390 × 844 and
  1440 × 900.
- Exercised the live demo, Reset, browser storage, same-origin requests, service
  worker cache, offline reload, deep links, Back, 404, metadata, touch sizes,
  and Axe at desktop/mobile on every HTML route.
- Crawled every same-origin link and checked the two GitHub fragment links.
- Ran every `.factory/claims.json` command from clean clone
  `/tmp/restore-drill-review2-clean-oGI5ma/repo`; all command invocations passed.
- Ran `npm test` from that clone: 13 Playwright tests passed, 1 mobile Axe pass
  was intentionally skipped, and all Rust/Node tests passed.
- Ran `npm run build`; it produced `dist/bin/restore-drill-linux-x86_64` and
  `dist/site/`.
- Ran `npm run check:types`, `npm run check:lint`, and Cargo packaging; all
  passed.
- Ran `/opt/fleet/lib/verify-url.sh` against the live home; it passed.
- Ran the built `restore-drill demo` from a fresh temporary directory. It exited
  `2` at the Docker prerequisite because this worker has no Docker executable,
  daemon, or socket.

## Blocking gaps

- The one-click web demo is a fixed transcript, not a replay of a verified real
  run, and it does not expose the promised signed report.
- No real Docker/Postgres restore has been executed; the claim test uses a
  shell-script substitute.
- The mobile demo transcript fails Axe's serious
  `scrollable-region-focusable` rule.
- The no-tracking test omits its declared service-worker/IndexedDB assertions;
  the live site registers a service worker and Cache Storage.
- Several README claims remain outside the claims registry or are incompletely
  tested.
- Route social metadata is incomplete, both setup links target missing GitHub
  anchors, and Terms says binaries are available although no release exists.

See `.factory/review-2.md` for exact evidence, rewrites, the full copy audit,
and finding-by-finding verification of review 1.
