# Restore Drill — polish 5 handoff

## Outcome

Perfection-loop round 5 is complete. The sole reopened defect, F-5-1, is
closed without weakening the core claim. `npm run test:docker` now runs the
exact Docker/Postgres acceptance test when a local daemon exists. In a worker
without Docker, it verifies a successful clean hosted run for the same source,
including its successful claim step and non-expired evidence artifact.

The final repair source is `894aa87f2981e12ba66fe6f09e4f4edd28f11b52`.
The Docker run is
<https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33190205761>.
It restored three rows, rejected the corrupt SQL, verified both signatures,
and found zero managed resources after both paths. Artifact
`real-demo-evidence` is id `9693421991`.

All prior first-screen, demo, claim, copy, metadata, route, focus, legal,
mobile, privacy, offline, scheduling, package, and 404 repairs remain intact.
The distinct warm-paper, forest-ink, vermilion field-report identity was
preserved. The catalog line is now: “Prove a Postgres backup restores in a
disposable Docker network.”

## Clean verification

Clean clone: `/tmp/restore-drill-polish5-final-xGmAqE/repo` at `894aa87`.

- `npm ci` and `npm audit --audit-level=high`: passed, zero vulnerabilities.
- Every command in `.factory/claims.json`: all 11 passed. This includes
  `real-docker-restore`, demo isolation, host rejection, JSON/exit behavior,
  local I/O, image order, distribution, scheduling, MIT, privacy, and offline.
- `npm test`: passed 4 Rust unit tests, 7 CLI integration tests, 3 Node claim
  tests, 8 policy tests, and 22 desktop/mobile Playwright tests.
- Browser coverage includes route metadata, link crawl, route/hash/Back focus,
  44 px targets, 390 px layout, both demo controls, storage isolation, offline
  reload, and Axe with zero serious/critical findings.
- `npm run check:types`, `npm run check:lint`, and `npm run build`: passed.
- `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`:
  passed and verified the packaged crate.
- Build output: executable CLI plus complete `dist/site/`; 4.31 kB JavaScript
  and 14.32 kB CSS before gzip.

## Deployment and live verification

Deployment ID: `b1ef262b-ac4c-4dd1-ba6b-9535ca12d492`.
Live URL: <https://restore-drill.sociobot.in/>.

- `.factory/evidence/live-polish-5/cold-check.json`: Home, Demo, Privacy, and
  Terms return 200 with correct titles; the product 404 returns HTTP 404.
- Cold 390 px screenshots: `home-cold-390.png`, `demo-cold-390.png`, and
  `404-cold-390.png` in the same evidence directory.
- The demo report is passed with three observed rows and an 88-character
  signature. Reset affects only `demo:restore-drill:*` session state.
- Requests remained same-origin. Cookies, local storage, and IndexedDB stayed
  empty. The service-worker cache contained only public same-origin files.
- Offline reload preserved the demo and signed-report summary.
- `/opt/fleet/lib/verify-url.sh` passed with title, `lang=en`, one h1, main,
  image alt text, button labels, and no console/page errors. Evidence is under
  `.factory/evidence/live-polish-5/verify/`.
- Lighthouse: 100 performance, 100 accessibility, 100 best practices, and 100
  SEO; FCP 1.1 s, LCP 1.8 s, TBT 0 ms, CLS 0. Evidence:
  `.factory/evidence/live-polish-5/lighthouse.json`.
- Live response headers include CSP, frame denial, Permissions-Policy,
  nosniff, strict referrer policy, and HSTS.

## Run and verify

```sh
npm ci
npm test
npm run test:docker
npm run check:types
npm run check:lint
npm run build
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

Deploy only `dist/site/` through:

```sh
/opt/fleet/lib/deploy-static.sh restore-drill /work/repo/dist/site
```

## Known gaps and next steps

None. No finding, unsupported public promise, TODO, stub, or deferred minor
item remains.
