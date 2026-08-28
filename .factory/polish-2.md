# Restore Drill — polish 2 finding map

Repair base: `1672b17b4fc47fb7193274da4b4c05b64a846185`.

Evidence shorthand used below:

- **Docker claim:** `real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup` on a GitHub-hosted real Docker daemon.
- **Browser suite:** Playwright desktop and 390 × 844 mobile projects, including Axe and offline tests.
- **Visual evidence:** `.factory/evidence/polish-2/`.

## Review 2 and reopened findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-2 | Replaced fixed HTML with a timed replay exported from the real CLI run. Added the unaltered signed report, demo-only session state, working Reset, and an install-section exit. | Docker claim; `demo exposes a real-run recording and inspectable signed report`; `@claim:demo-sandbox`; `demo-mobile.png`; live `/demo/?demo=1` |
| F-1-34 | Added a required real-daemon test for healthy and corrupt SQL. It verifies three rows, both signatures, and zero labelled resources; the command test asserts `network create --internal`. | Docker claim; `demo_uses_shipped_sample_in_a_fresh_temporary_directory`; public `demo/sample-report.json` |
| F-2-1 | Replaced horizontal `<pre>` overflow with a labelled, focusable terminal region whose lines wrap at 390 px. Removed the mobile Axe skip. | `all routes have no serious accessibility findings`; `all routes fit the viewport and visible controls meet touch size`; `demo-mobile.png` |
| F-2-2 | Registered the service worker on local production-equivalent tests. The claim now enumerates registrations, IndexedDB, Cache Storage, cookies, both Web Storage APIs, and every request origin. | `@claim:site-no-tracking uses only a public first-party cache`; offline reload test; live Privacy storage check |
| F-1-22 | Registered and tested JSON stdout, quiet stderr in JSON mode, and exit codes 0, 1, and 2. | `@claim:automation-contract` / `json_output_and_exit_codes_are_stable_for_automation` |
| F-1-24 | Expanded validation across public HTTP, loopback IP, localhost, HTTPS, undeclared internal, and declared internal hosts before Docker construction. | `@claim:production-boundary` / `rejects_public_loopback_and_undeclared_http_hosts_before_docker` |
| F-1-27 | Registered the distribution build and asserted an executable CLI plus every site route under `dist/`. | `@claim:distribution-build`; clean-clone `npm run build` |
| F-1-36 | Added exact `og:type`, route `og:url`, and full Twitter metadata to Demo, Privacy, Terms, and 404. | `every route has complete route-specific metadata and product chrome`; live route crawl |
| F-2-3 | Both site actions now target `#install-from-source`. README provides clone, `cd`, locked install, and demo commands. | `site setup links target the real README installation heading`; live GitHub anchor check |
| F-2-4 | Terms now says only source is available. It makes no binary-release claim. | `Terms — Restore Drill`; live `/terms/` copy check |
| F-2-5 | The recording is labelled as recorded output and replaces host timing with `<measured time>`. | `demo-recording.json`; `demo exposes a real-run recording and inspectable signed report` |
| F-2-6 | Narrowed privacy copy to implemented boundaries. Added sentinel input/output coverage and asserted no canary or secret reaches Docker arguments. | `@claim:local-io-boundary` / `drill_keeps_inputs_unchanged_and_outputs_in_configured_paths`; live `/privacy/` |
| F-2-7 | Added the missing-image branch: first inspect fails, pull occurs, second inspect succeeds, and pull precedes resource creation. | `@claim:image-pull` / `demo_pulls_a_missing_postgres_image_before_creating_resources` |
| F-2-8 | Full-document same-origin navigation focuses the destination h1 and announces it; Back does the same. Hash routes retain heading focus. | `hash and document navigation move focus and announce context` |
| F-2-9 | Standardized the browser path as “web walkthrough,” the executable path as “demo command,” and removed “safe.” | `.factory/copy-audit.md`; home and demo browser assertions |
| F-2-10 | Added a strict portable cron runner, crontab, and GitHub Actions example with weekly triggers, report preservation, real exit propagation, and secret references. | `@claim:weekly-scheduling` / `the scheduling examples preserve reports, failures, and credential boundaries` |

## Review 1 cumulative audit

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the clear job headline, named audience, single sample action, action outcome, and three facts. | `home gives small self-hosted teams one clear sample entry`; `home-mobile.png` |
| F-1-2 | Fully superseded the old fake preview with the real-run walkthrough and CLI demo isolation. | Docker claim; `@claim:demo-sandbox`; live `/demo/?demo=1` |
| F-1-3 | Expanded `.factory/claims.json` to ten claims and enforced exactly one tag for each. | `every registered claim has exactly one tagged test`; clean-clone claim matrix |
| F-1-4 | The retained restore promise is now limited to the shipped sample and proven on real Postgres. | Docker claim and signed public sample report |
| F-1-5 | Retained only the tested public/loopback/HTTPS/undeclared-host boundary. | `@claim:production-boundary` |
| F-1-6 | Site privacy now states the tested no-tracking and public-cache boundary. | `@claim:site-no-tracking` |
| F-1-7 | Image ordering is now registered and tested directly. | `@claim:image-pull` |
| F-1-8 | Existing-network copy remains removed; undeclared hosts are tested before Docker. | `@claim:production-boundary`; copy audit |
| F-1-9 | Multi-format restoration marketing remains removed. | `.factory/copy-audit.md`; landing/README review |
| F-1-10 | Broad secret/read-only copy was replaced by the tested backup/output boundary. | `@claim:local-io-boundary` |
| F-1-11 | Broad SQL/HTTP scope marketing remains removed; the shipped SQL check is proved. | Docker claim; copy audit |
| F-1-12 | The sample report, signature, corrupt-run report, and cleanup are now real-daemon outcomes. | Docker claim; public signed report |
| F-1-13 | The new walkthrough declares and isolates demo-only state instead of claiming zero storage. | `@claim:demo-sandbox`; Privacy page |
| F-1-14 | Checkpoint-equivalence copy remains removed; the displayed transcript is exported from the run itself. | `demo-recording.json` source SHA; browser evidence test |
| F-1-15 | No release-binary promise appears. | Terms and README copy audit |
| F-1-16 | Broad independent-verification marketing remains removed; the downloadable report verifies locally. | `restore-drill verify site/public/demo/sample-report.json` |
| F-1-17 | The unsupported Team Kit offer remains removed. | live link crawl; no billing/checkout link |
| F-1-18 | The unsupported free-tier entitlement promise remains removed. | copy audit |
| F-1-19 | Merchant/refund wording remains removed with the paid offer. | copy audit |
| F-1-20 | License/offline-entitlement wording remains removed. | copy audit |
| F-1-21 | Docker prerequisite is plain; missing-image behavior is now tested. | README; `@claim:image-pull` |
| F-1-22 | Automation channels and all three exit codes are registered and asserted. | `@claim:automation-contract` |
| F-1-23 | Archive compatibility/security marketing remains removed. | README copy review |
| F-1-24 | Every retained HTTP boundary and the credential argument boundary are tested. | `@claim:production-boundary`; `@claim:local-io-boundary` |
| F-1-25 | Detailed schema/public-key promises remain removed; the sample report is directly inspectable. | live `/demo/sample-report.json` |
| F-1-26 | No truncated-dump promise appears. | README copy review |
| F-1-27 | Build output is now a registered claim. | `@claim:distribution-build` |
| F-1-28 | Overview stays limited to the shipped backup, Docker network, check, report, and cleanup. | Docker claim; copy audit |
| F-1-29 | Cleanup is proved after both healthy and corrupt real-daemon runs. | Docker claim; zero labelled resources assertion |
| F-1-30 | `--keep-on-failure` appears only in command syntax, not as a marketing promise. | README copy review |
| F-1-31 | MIT remains registered and package metadata is checked. | `@claim:mit-license` |
| F-1-32 | No paid primary path exists. | live link crawl |
| F-1-33 | Demo, legal pages, and designed 404 remain real documents; unknown live paths return the project 404 with status 404. | route suite; `404-mobile.png`; live unknown-path check |
| F-1-34 | Real Docker/Postgres healthy and corrupt runs now pass in hosted CI. | Docker claim |
| F-1-35 | Shared skip link, header, main, footer, legal links, factory credit, and build label remain on every HTML route. | metadata/chrome browser test |
| F-1-36 | Every route now has complete social metadata and local icons/art. | metadata browser test |
| F-1-37 | Hash, forward document, exit-to-install, and Back focus are tested and announced. | focus browser test; `@claim:demo-sandbox` |
| F-1-38 | All visible controls remain at least 44 × 44 px on every route at desktop and mobile. | touch/overflow browser test |
| F-1-39 | Home title remains the plain job statement. | home browser test |
| F-1-40 | README workflow prose remains split below 22 words. | `.factory/copy-audit.md` |
| F-1-41 | Exit behavior is short copy backed by one automation test. | copy audit; `@claim:automation-contract` |
| F-1-42 | H1 remains “Prove your Postgres backup restores.” | home browser test; `home-mobile.png` |
| F-1-43 | Abstract preview copy is replaced by “Replay a sample Postgres restore.” | demo browser test; `demo-mobile.png` |
| F-1-44 | The four concrete recovery steps remain. | landing copy audit |
| F-1-45 | Network copy explicitly says internal Docker network. | landing copy audit; Docker wrapper log assertion |
| F-1-46 | Report and cleanup language remains explicit. | landing copy audit; Docker claim |
| F-1-47 | Install section names the Docker host and now includes complete commands. | setup-anchor test; `home-mobile.png` |
| F-1-48 | Privacy boundary remains headed “What this does not do.” | landing copy audit |
| F-1-49 | Paid habit section remains absent. | live landing check |
| F-1-50 | No ambiguous copy button exists. | live control crawl |
| F-1-51 | No license button exists. | live control crawl |
| F-1-52 | Primary action remains “Try it with sample data.” | home browser test |
| F-1-53 | Terminology table fixes backup, web walkthrough, demo command, drill, network, and report. | `.factory/copy-audit.md` |
| F-1-54 | Public overview uses plain command/check/network/report language. | copy audit |
| F-1-55 | No unregistered paid bullet exists. | live landing check |
| F-1-56 | Catalog line is verb-first and 87 characters. | `catalog description is verb-first and no longer than 120 characters` |

## Evidence paths

- `.factory/evidence/polish-2/home-mobile.png`
- `.factory/evidence/polish-2/demo-mobile.png`
- `.factory/evidence/polish-2/demo-desktop.png`
- `.factory/evidence/polish-2/privacy-mobile.png`
- `.factory/evidence/polish-2/404-mobile.png`

Deployment and cold-live results are recorded in `.factory/handoff.md` after
the production upload. The deployed cold audit is
`.factory/evidence/live-polish-2/cold-check.json`; it covers every route,
offline reload, demo isolation/reset, the real report, storage, network, and
Axe. Lighthouse evidence is in `lighthouse.json` and scored 100 in all four
categories.
