# Restore Drill — polish 3 finding map

Candidate `996c188fccf7ceae5772e9358862cb5cf597c4df` was repaired in
`50ba2266b1aaf4326759f1fd4b16372133bbebf9`. This map deliberately includes
every historical finding, not only the four items reopened in review 3.

## Evidence key

- **C** — clean-clone claim matrix:
  `.factory/evidence/polish-3/clean-claim-tests.log`.
- **B** — full local unit, policy, browser, Axe, mobile, privacy, and offline
  suite: `.factory/evidence/polish-3/npm-test.log`.
- **Q** — type, lint, and production build logs in
  `.factory/evidence/polish-3/{types,lint,build}.log`.
- **D** — real Docker/Postgres hosted acceptance run:
  <https://github.com/B-Divyesh/sf-restore-drill/actions/runs/33178287693>.
- **L** — cold deployed browser audit and screenshots in
  `.factory/evidence/live-polish-3/`.

## Review 1 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the job headline, named small self-hosted teams, one primary sample action, action outcome, and three facts. | B home/mobile; L `home-cold-390.png` |
| F-1-2 | Kept `/demo/?demo=1`, the shipped CLI demo, recorded real run, sample banner, reset, and isolated namespace. | C `demo-sandbox`; D; L demo audit |
| F-1-3 | Kept the claims registry and exact-one-tag enforcement; added the missing offline claim. | B policy claim-tag test; C |
| F-1-4 | Limited restore proof to shipped three-order sample and real PostgreSQL acceptance. | D `real-docker-restore` |
| F-1-5 | Kept the tested public, loopback, HTTPS, and undeclared-host boundary. | C `production-boundary` |
| F-1-6 | Kept no-tracking site copy and local I/O boundary as observable claims. | C `site-no-tracking`, `local-io-boundary` |
| F-1-7 | Kept explicit missing-image pull-before-network coverage. | C `image-pull` |
| F-1-8 | Left untested existing-network copy removed; tested undeclared targets before Docker. | C `production-boundary` |
| F-1-9 | Left unsupported volume-format marketing removed. | B copy and route audit |
| F-1-10 | Kept input-unchanged/configured-output proof instead of broad secret copy. | C `local-io-boundary` |
| F-1-11 | Left broad SQL/HTTP marketing removed; the bundled SQL check is real-Docker tested. | D |
| F-1-12 | Kept signed healthy and failed reports plus managed-resource cleanup assertions. | D |
| F-1-13 | Kept demo-only session state and tests rather than a false zero-storage promise. | C `demo-sandbox` |
| F-1-14 | Kept the self-hosted recording exported from the real demo command. | B recording test; D |
| F-1-15 | Left unpublished-binary copy removed. | B copy audit |
| F-1-16 | Kept inspectable signed report and local verification in the real run. | D |
| F-1-17 | Left the unregistered paid offer removed. | L link crawl |
| F-1-18 | Left unregistered free-tier entitlement copy removed. | B copy audit |
| F-1-19 | Left unsupported merchant/refund copy removed. | B copy audit |
| F-1-20 | Left license entitlement copy removed; registered the separate offline-reading promise. | C `offline-web-walkthrough` |
| F-1-21 | Kept Docker prerequisite plain and image-pull behavior tested. | C `image-pull` |
| F-1-22 | Kept tested JSON stdout and 0/1/2 exit-code contract. | C `automation-contract` |
| F-1-23 | Left archive compatibility/security marketing removed. | B copy audit |
| F-1-24 | Kept host boundary and secret/canary argument coverage. | C `production-boundary`, `local-io-boundary` |
| F-1-25 | Left detailed report-schema marketing removed; the signed sample remains inspectable. | D; L demo report |
| F-1-26 | Left truncated-dump copy removed. | B copy audit |
| F-1-27 | Kept registered distribution build and made ordinary `npm test` clean-tree safe. | C `distribution-build`; B; Q |
| F-1-28 | Kept public scope limited to the bundled backup, network, check, report, and cleanup. | B copy audit; D |
| F-1-29 | Kept real cleanup checks after both healthy and corrupt runs. | D |
| F-1-30 | Kept `--keep-on-failure` out of public promise copy. | B copy audit |
| F-1-31 | Kept MIT claim and package assertion. | C `mit-license` |
| F-1-32 | Kept the dead checkout path absent. | L link crawl |
| F-1-33 | Kept real Demo/legal routes and project-owned 404 configuration. | B route test; L unknown-route check |
| F-1-34 | Ran the exact real-Docker claim on a clean Docker-capable hosted runner. | D |
| F-1-35 | Kept shared skip link, header, footer, legal links, credit, and build label. | B metadata/chrome test; L |
| F-1-36 | Kept route-specific canonical, OG, Twitter, favicon, and touch-icon metadata. | B metadata test; L |
| F-1-37 | Kept hash/document/back focus movement and polite announcements. | B navigation test; L |
| F-1-38 | Kept 44 px controls and no overflow at desktop and 390 px. | B touch/mobile test; L screenshots |
| F-1-39 | Kept the imperative home title. | B home test; L home title |
| F-1-40 | Kept README prose below the 22-word cap. | `.factory/copy-audit.md` |
| F-1-41 | Kept short exit-code documentation and automation proof. | C `automation-contract` |
| F-1-42 | Kept the plain job H1. | B home test; L home screenshot |
| F-1-43 | Kept the concrete sample-restore demo heading. | B demo test; L demo screenshot |
| F-1-44 | Kept four concrete recovery steps. | B copy audit; L home |
| F-1-45 | Kept explicit internal Docker-network wording. | C `image-pull`; B copy audit |
| F-1-46 | Kept report-and-cleanup wording bound to real Docker acceptance. | D; B copy audit |
| F-1-47 | Kept source installation commands and working GitHub anchor. | B setup-link test; L link crawl |
| F-1-48 | Kept the clear privacy-boundary heading. | B copy audit; L home |
| F-1-49 | Kept unsupported paid-habit content absent. | L home crawl |
| F-1-50 | Kept ambiguous copy control absent. | L control crawl |
| F-1-51 | Kept license control absent. | L control crawl |
| F-1-52 | Kept **Try it with sample data** as the sole primary action. | B home test; L home |
| F-1-53 | Kept the backup/web walkthrough/demo command/drill/network/report terminology table. | `.factory/copy-audit.md` |
| F-1-54 | Kept public overview language concrete and jargon-light. | `.factory/copy-audit.md` |
| F-1-55 | Kept unregistered paid bullets absent. | L home crawl |
| F-1-56 | Rewrote catalog description to verb-first: “Prove a Postgres backup restores in an isolated Docker network.” | B catalog test |

## Review 2 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 | Kept focusable, wrapping terminal evidence and mobile Axe coverage. | B Axe/mobile test; L demo |
| F-2-2 | Kept full service-worker, cache, storage, cookie, and same-origin checks. | C `site-no-tracking` |
| F-2-3 | Kept both setup links on `#install-from-source`. | B setup-link test; L link crawl |
| F-2-4 | Kept Terms limited to source availability. | B route/copy audit; L terms |
| F-2-5 | Kept recording timing variable rather than a hard-coded duration. | B recording test; L demo |
| F-2-6 | Kept local input/output and canary boundaries registered. | C `local-io-boundary` |
| F-2-7 | Kept missing-image pull ordering registered. | C `image-pull` |
| F-2-8 | Kept full-document and Back focus/announcement behavior. | B navigation test; L |
| F-2-9 | Kept browser path as web walkthrough and executable path as demo command. | `.factory/copy-audit.md`; L |
| F-2-10 | Kept tested weekly cron and GitHub Actions examples. | C `weekly-scheduling` |

## Review 3 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 / F-1-34 | Preserved the direct real Docker/Postgres test and executed it on the clean hosted runner with Docker daemon, healthy and corrupt samples, signatures, and cleanup. | D; hosted job log |
| F-3-2 | Added `offline-web-walkthrough` to `claims.json` and tagged its fresh-context offline-reload test. | C `offline-web-walkthrough`; L offline audit |
| F-3-3 | Renamed the demo-exit link to **View installation steps** and updated its focus/reset test and demo docs. | C `demo-sandbox`; L demo |
| F-3-4 | Replaced “tamper-evident report” with **signed report you can check later** on landing and README. | B home copy test; `.factory/copy-audit.md`; L home |

## Release evidence

The final cold production audit, screenshots, and Lighthouse JSON are recorded
under `.factory/evidence/live-polish-3/`. The final deployment URL is
<https://restore-drill.sociobot.in/>.

## Additional cold-check correction

| Finding | Change made | Evidence |
| --- | --- | --- |
| Cold-check-1 | Replaced the stale `build 1672b17+polish2` footer marker with `build polish3` on Home, Demo, Privacy, Terms, and 404; added a shared-chrome assertion and redeployed. | `.factory/evidence/polish-3/site-after-footer.log`; `.factory/evidence/polish-3/live-audit-final.log`; live footer check |
