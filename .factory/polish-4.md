# Restore Drill — polish 4 finding map

Candidate `06246d0f5da6b5019541faa97bb7afcd2ffbb321` was repaired in
`40b270e6d9004bbec0600d67cea3d2a308851cc2`. Every earlier finding was
rechecked against that code and the deployed site.

## Evidence key

- **D** — exact real-Docker claim on a clean hosted checkout of final commit
  `89d14a270eb19331881c628c9b358beafe3b4345`:
  <https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33186099697>.
- **C** — every non-Docker command in `.factory/claims.json`, run from clean
  checkout `/tmp/restore-drill-polish4-clean-6ZFRWO/repo` at `40b270e`.
- **B** — clean-checkout `npm test`: 4 Rust unit tests, 7 CLI tests, 3 Node
  claim tests, 7 policy tests, and 22 desktop/mobile Playwright tests.
- **Q** — clean-checkout type check, lint, production build, and crate package.
- **L** — deployed cold audit in `.factory/evidence/live-polish-4/`, including
  `cold-check.json`, `home-cold-390.png`, `demo-cold-390.png`, and
  `404-cold-390.png`.
- **V** — `/opt/fleet/lib/verify-url.sh` output under
  `.factory/evidence/live-polish-4/verify/`.
- **P** — Lighthouse report at
  `.factory/evidence/live-polish-4/lighthouse.json`.

## Review 1 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | The first screen names the restore job, small self-hosted teams, one sample action, its result, and three facts. | B `home gives small self-hosted teams one clear sample entry`; L `home-cold-390.png` |
| F-1-2 | The direct demo has the real-run replay, signed report, banner, reset, exit, isolated browser state, shipped CLI sample, and temporary CLI directory. | C `@claim:demo-sandbox`; D; L `demo-cold-390.png` |
| F-1-3 | Eleven claims are registered and the policy test enforces exactly one tag per claim. | B `every registered claim has exactly one tagged test`; C; D |
| F-1-4 | Public scope is limited to the shipped three-row Postgres restore proved on a real daemon. | D `real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup` |
| F-1-5 | Public, loopback, HTTPS, and undeclared targets are rejected before Docker. | C `rejects_public_loopback_and_undeclared_http_hosts_before_docker` |
| F-1-6 | Broad upload language remains removed; current local-I/O and no-tracking boundaries are registered. | C `local-io-boundary`; C `site-no-tracking` |
| F-1-7 | A missing Postgres image is pulled before resources are created. | C `demo_pulls_a_missing_postgres_image_before_creating_resources` |
| F-1-8 | Existing-network marketing remains removed; undeclared hosts are rejected before Docker. | C `production-boundary`; `.factory/copy-audit.md` |
| F-1-9 | Unsupported multi-format marketing remains absent. | `.factory/copy-audit.md`; B route/copy assertions |
| F-1-10 | The retained input/output boundary checks unchanged source bytes and configured destinations. | C `drill_keeps_inputs_unchanged_and_outputs_in_configured_paths` |
| F-1-11 | Broad SQL/HTTP marketing remains absent; the shipped SQL check restores three rows. | D; L signed report check |
| F-1-12 | Healthy and corrupt real runs produce signed reports and leave no labelled resources. | D |
| F-1-13 | The walkthrough declares demo-only session state and Reset behavior instead of claiming zero storage. | C `@claim:demo-sandbox`; L cache/storage audit |
| F-1-14 | The replay is exported by the real-Docker claim and names the actual demo command. | D artifact; B `demo exposes a real-run recording and inspectable signed report` |
| F-1-15 | No release-binary promise appears. | `.factory/copy-audit.md`; L link crawl |
| F-1-16 | The inspectable sample report and separately retained public key verify on the real run. | D; L report check |
| F-1-17 | The unavailable Team Kit offer remains removed. | L same-origin link crawl; `.factory/copy-audit.md` |
| F-1-18 | The unsupported free-tier entitlement promise remains removed. | `.factory/copy-audit.md` |
| F-1-19 | Unsupported merchant and refund wording remains removed. | `.factory/copy-audit.md` |
| F-1-20 | License-entitlement wording remains absent; the separate offline-reading claim passes. | C `@claim:offline-web-walkthrough`; L offline reload |
| F-1-21 | Docker is documented as a prerequisite and missing-image order is tested. | README; C `image-pull` |
| F-1-22 | JSON stdout, stderr separation, and exit codes 0, 1, and 2 are registered and tested. | C `json_output_and_exit_codes_are_stable_for_automation` |
| F-1-23 | Unsupported archive-compatibility marketing remains removed. | README; `.factory/copy-audit.md` |
| F-1-24 | Every retained host boundary and the credential/canary boundary are tested. | C `production-boundary`; C `local-io-boundary` |
| F-1-25 | Detailed schema marketing remains removed; the real signed sample report is inspectable. | D; L report check |
| F-1-26 | The old truncated-dump promise remains absent; corrupt SQL is covered by the real-engine claim. | D; README audit |
| F-1-27 | The registered build creates an executable CLI and every static route. | C `distribution-build`; Q |
| F-1-28 | Public scope stays limited to the shipped backup, internal network, data check, report, and cleanup. | D; `.factory/copy-audit.md` |
| F-1-29 | Real healthy and corrupt runs both assert zero labelled containers, volumes, and networks. | D |
| F-1-30 | `--keep-on-failure` remains command syntax, not an untested marketing promise. | README audit |
| F-1-31 | The repository and crate identify the MIT license. | C `the distributed repository includes the MIT license`; Q crate package |
| F-1-32 | No dead checkout or paid path is present. | L link crawl |
| F-1-33 | Demo and legal routes are real documents; unknown paths return the designed 404 with status 404. | B route tests; L `404-cold-390.png` |
| F-1-34 | The exact healthy/corrupt Postgres test passes on a clean runner with a real Docker daemon. | D |
| F-1-35 | Every route keeps the shared skip link, header, footer, legal links, factory credit, and build label. | B metadata/chrome test; L |
| F-1-36 | Every route has its own canonical, Open Graph/Twitter metadata, favicon, touch icon, and original image. | B metadata test; L route audit |
| F-1-37 | Hash, document, Back, and demo-exit navigation focus and announce the destination heading. | B `hash and document navigation move focus and announce context` |
| F-1-38 | Visible controls are at least 44 × 44 px with no 390 px overflow. | B viewport/touch test; L |
| F-1-39 | The home title uses the imperative “prove.” | B home test; L route title |
| F-1-40 | README workflow prose remains below 22 words per sentence. | `.factory/copy-audit.md` |
| F-1-41 | Exit behavior is short copy tied to the automation claim. | `.factory/copy-audit.md`; C `automation-contract` |
| F-1-42 | The H1 is “Prove your Postgres backup restores.” | B home test; L `home-cold-390.png` |
| F-1-43 | The walkthrough heading directly names the sample restore replay. | B demo test; L `demo-cold-390.png` |
| F-1-44 | The workflow is now accurately headed “Rehearse a restore in four steps.” | B focus/copy test; L live heading assertion |
| F-1-45 | Step one explicitly creates an isolated Docker network. | `.factory/copy-audit.md`; D |
| F-1-46 | Step four explicitly signs the report and removes test resources. | `.factory/copy-audit.md`; D |
| F-1-47 | The install heading names one binary and the Docker host. | B setup-link test; L home screenshot |
| F-1-48 | The privacy boundary is headed “What this does not do.” | `.factory/copy-audit.md`; L home screenshot |
| F-1-49 | The unsupported paid-habit section remains absent. | L landing audit |
| F-1-50 | No ambiguous Copy control exists. | L control crawl |
| F-1-51 | No license control exists. | L control crawl |
| F-1-52 | “Try it with sample data” remains the sole primary first-screen action. | B home test; L home screenshot |
| F-1-53 | Backup, web walkthrough, demo command, drill, network, and report use one meaning each. | `.factory/copy-audit.md` terminology table |
| F-1-54 | Public overview copy uses concrete command, check, network, and report words. | `.factory/copy-audit.md` |
| F-1-55 | No unregistered paid workflow bullet appears. | L landing audit |
| F-1-56 | Catalog copy is verb-first and 70 characters. | B `catalog description is verb-first and no longer than 120 characters` |

## Review 2 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 | The terminal wraps, is focusable, and passes Axe at 390 px. | B Axe/mobile tests; L demo screenshot |
| F-2-2 | The privacy claim inspects requests, cookies, both Web Storage APIs, IndexedDB, service worker, and public cache contents. | C `@claim:site-no-tracking`; L `cold-check.json` |
| F-2-3 | Both setup actions target the real `#install-from-source` README heading. | B setup-link test; L link crawl |
| F-2-4 | Terms promises available source, not unpublished binaries. | B route test; L `/terms/` |
| F-2-5 | The recording labels host timing as variable instead of claiming 3.24 seconds. | B recording test; L demo screenshot |
| F-2-6 | Named inputs, unchanged backup bytes, configured outputs, and canary exclusion are registered. | C `local-io-boundary` |
| F-2-7 | The missing-image pull path and order are registered. | C `image-pull` |
| F-2-8 | Full-document and Back navigation focus and announce the destination. | B navigation test |
| F-2-9 | Browser “web walkthrough” and executable “demo command” remain distinct. | `.factory/copy-audit.md` |
| F-2-10 | Weekly cron and GitHub Actions examples preserve reports, failures, and credential boundaries. | C `weekly-scheduling` |

## Review 3 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Revalidated the exact real-Docker/Postgres claim on the repair commit. | D |
| F-3-2 | Offline reading remains registered and passes in a fresh browser context. | C `@claim:offline-web-walkthrough`; L offline reload |
| F-3-3 | The demo exit control says “View installation steps.” | C `@claim:demo-sandbox`; L demo screenshot |
| F-3-4 | Public copy says “signed report you can check later,” not “tamper-evident.” | B home test; `.factory/copy-audit.md` |

## Review 4 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-4-1 | Ran the exact registered command on a fresh checkout with Docker and Postgres. It passed healthy rows, corrupt failure, both signatures, and cleanup. | D |
| F-4-2 | Replaced the hidden mobile navigation with a visible two-row header containing the product name and all three primary links. | B `mobile headers keep the product name and primary navigation visible`; `.factory/evidence/polish-4/home-mobile.png`; L live mobile assertions |
| F-4-3 | Replaced “Run four checks” with “Rehearse a restore in four steps.” | B navigation/copy assertion; L live heading assertion and `home-cold-390.png` |

## Final release evidence

The static deployment ID is `b351562d-a2ea-430e-8425-94e4504f6956` at
<https://restore-drill.sociobot.in/>. The cold production audit found no
external requests, no unexpected console errors, no serious/critical Axe
issues, correct titles and metadata, a working 404, isolated/resettable demo
state, and a successful offline reload. Lighthouse scored 100 in all four
categories with 1.7 s LCP, 30 ms TBT, and zero CLS.
