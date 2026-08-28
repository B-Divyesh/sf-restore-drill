# Restore Drill — polish 3 handoff

## Outcome

**PASS.** All findings in `.factory/review-1.md`, `.factory/review-2.md`, and
`.factory/review-3.md` are resolved and mapped in `.factory/polish-3.md`.
Restore Drill is live at <https://restore-drill.sociobot.in/>; the deployed
static release is `dc1b99bd-7acb-4805-9208-3c712c6d764d`.

## Delivered

- Registered and tested the privacy-page offline-reading promise.
- Kept the direct Docker/Postgres restore acceptance test and verified it on a
  clean hosted runner with a real Docker daemon.
- Replaced the vague demo-exit label with **View installation steps** and
  replaced jargon with **signed report you can check later**.
- Fixed a clean-tree test-order issue: the distribution assertion now waits for
  its documented build command instead of requiring a pre-existing `dist/`.
- Rechecked the isolated `?demo=1` walkthrough, reset, route titles, metadata,
  404, legal links, focus behavior, mobile layout, privacy boundaries, and the
  halftone field-report visual system.
- Corrected the shared footer build marker to `build polish3` on every route.

## Verification

### Claims from a clean checkout

A fresh checkout at `50ba2266b1aaf4326759f1fd4b16372133bbebf9` ran `npm ci`
and every non-Docker command in `.factory/claims.json`. The exact transcript is
`.factory/evidence/polish-3/clean-claim-tests.log`:

- `production-boundary`
- `automation-contract`
- `local-io-boundary`
- `image-pull`
- `distribution-build`
- `weekly-scheduling`
- `mit-license`
- `demo-sandbox`
- `site-no-tracking`
- `offline-web-walkthrough`

The remaining registered claim, `real-docker-restore`, passed on GitHub's clean
Ubuntu Docker runner at
<https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33178287693>. It
ran the exact command below against a real Docker daemon and checked a healthy
three-row restore, corrupt-backup failure, report signature, and cleanup:

```sh
cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored
```

### Local quality gates

- `npm test`: passed — 4 Rust unit tests, 7 CLI integration tests, 3 Node
  claim tests, 7 policy tests, and 20 desktop/mobile Playwright tests.
- `npm run check:types`, `npm run check:lint`, and `npm run build`: passed.
- `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`:
  passed; 112.3 KiB unpacked and 28.8 KiB compressed.
- Static output: 4.31 kB JavaScript (1.92 kB gzip) and 14.24 kB CSS (3.93 kB
  gzip).

Final command logs are in `.factory/evidence/polish-3/`.

### Cold production audit

`npm run test:live -- https://restore-drill.sociobot.in/
.factory/evidence/live-polish-3` and `/opt/fleet/lib/verify-url.sh` both
passed after the final deployment. They covered Home, Demo, Privacy, Terms,
the designed 404 response, route metadata, one H1/main landmark, responsive
390 px rendering, keyboard/focus behavior, sample reset, isolated demo storage,
service-worker cache, offline reload, report signature, same-origin requests,
and console output. Axe reported zero serious or critical issues on every
tested route.

Lighthouse 12.8.2 (mobile, production) scored Performance 100, Accessibility
100, Best Practices 100, and SEO 100. FCP was 0.8 s, LCP 1.7 s, CLS 0, and TBT
10 ms. Screenshots, JSON reports, and the live audit transcript are in
`.factory/evidence/live-polish-3/`.

## Run, package, and deploy

```sh
npm ci
npm test
npm run check:types
npm run check:lint
npm run build
npm run test:live -- https://restore-drill.sociobot.in/ .factory/evidence/live-polish-3
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

For a Docker-capable machine, run the real restore claim shown above. The
publish-ready crate is produced by `cargo package`; do not publish from this
worktree. Static deployment uses:

```sh
npm run build:site
/opt/fleet/lib/deploy-static.sh restore-drill dist/site
```

## Known gaps

None. The local nested container cannot run Docker due kernel privileges, but
the exact Docker acceptance command passed in the hosted clean Docker runner.
