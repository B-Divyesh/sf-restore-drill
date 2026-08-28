# Restore Drill — review 5 handoff

## Outcome

Review 5 added [`.factory/review-5.md`](review-5.md) and made no product-code
changes. **Verdict: FAIL.** The sole finding is F-5-1: the real Docker/Postgres
claim cannot run in this disposable sandbox because `docker` is absent.

## Verification

- Fresh live contexts at 390 px and desktop checked the landing page, demo,
  storage/reset, metadata, 404, links, routing, and accessibility.
- A fresh clone ran `npm ci`, every registered claim command, `npm test`, and
  `npm run build`.
- Ten claim commands passed. The real-Docker test failed only because no Docker
  CLI/daemon is available here; normal `npm test` skips that ignored acceptance
  test.

## Required next step

Use a Docker-capable clean runner and run:

```sh
cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored
```

It must pass the healthy and corrupt paths, report signatures, and cleanup
checks before the review can pass.
