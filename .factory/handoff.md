# Restore Drill — review 6 handoff

## Outcome

Completed the requested adversarial first-read review without changing product code. The review is recorded in `.factory/review-6.md`; verdict: **PASS**. The working source reviewed was `8fb8b5fa611885df5e3e5836a6a4284e33a04430`.

## What was verified

- Cold live site checks at 390 px and 1440 px: clear job, audience, and one sample action.
- Live demo: immediate three-order transcript/report, persistent isolation banner, namespaced session state, Reset, exit cleanup, same-origin requests, and offline reload.
- CLI demo command from a fresh temporary directory: documented Docker-missing prerequisite path exits 2 without writing there.
- All 11 `.factory/claims.json` commands from a separate clean clone passed. The Docker claim used its intentional clean hosted-run validation because this sandbox has no Docker executable.
- Route metadata, 404, crawled links, shared chrome, mobile layout, focus/Back behaviour, and visual identity were checked live.
- Every prior review/polish/handoff finding was reread and confirmed against current live/code behaviour.

## Reproduce

```sh
npm ci
npm run test:docker
npm run test:site -- --grep @claim:demo-sandbox
cargo test --workspace rejects_public_loopback_and_undeclared_http_hosts_before_docker
cargo test --workspace json_output_and_exit_codes_are_stable_for_automation
cargo test --workspace drill_keeps_inputs_unchanged_and_outputs_in_configured_paths
cargo test --workspace demo_pulls_a_missing_postgres_image_before_creating_resources
npm run build
node --test --test-name-pattern='distribution build|scheduling examples|MIT license' tests/claims.test.mjs
npm run test:site -- --grep '@claim:(site-no-tracking|offline-web-walkthrough)'
```

## Known gaps

None. A real local CLI demo needs Docker Engine by design; the registered release gate verifies the matching clean Docker-capable run when Docker is unavailable.
