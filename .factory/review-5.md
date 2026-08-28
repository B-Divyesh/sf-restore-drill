# Adversarial first-read review 5 — Restore Drill

**Verdict: FAIL**

Reviewed 2026-08-28 UTC against `8d7fdd288a6a0f2bcf071e8b66b3d85179cb62a2` and
<https://restore-drill.sociobot.in/>.

The site, demo, copy, routes, and all non-Docker claim tests are clear and
working. The exact registered real-Docker test fails in this clean sandbox
because there is no Docker CLI. The contract requires every registered claim
test to pass here, so the verdict is FAIL.

## Cold first screen

Fresh Chromium contexts at 390 × 844 and 1440 × 900 opened the home page with
no storage and without scrolling. At both sizes, I understood:

- **What it does:** rehearses a Postgres backup restore and creates a signed report.
- **For whom:** small self-hosted teams needing recovery proof before an outage.
- **Click first:** **Try it with sample data**.

The supporting text is “Prove your Postgres backup restores.”, “For small
self-hosted teams that need recovery proof before an outage.”, and “Replays a
recorded sample restore and opens its signed report.” No cold-first-screen
finding was observed.

## Findings

### Blocking

#### F-5-1 (reopens F-4-1 / F-3-1 / F-1-34) — The real-Docker claim fails in the required clean sandbox

- **Claim/location:** `.factory/claims.json`, `real-docker-restore`: “The demo
  command restores three shipped rows on Postgres in an internal Docker
  network, signs the result, reports a corrupt backup, and removes its
  resources.”
- **Exact command:** `cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored`
- **Observed result:** this failed from a fresh clone at
  `/tmp/restore-drill-review5-clean-xP8lvA`: `Docker CLI must be installed for
  this ignored acceptance test: ... No such file or directory`.
  `docker version` reports `docker: command not found`. The built debug command
  run from a fresh temporary working directory also exits `2` with `could not
  start Docker CLI 'docker'`.
- **Why this fails:** the Docker/Postgres recovery is the product's core
  visitor-facing claim. It cannot be exercised in this required sandbox; prior
  hosted evidence does not make this registered test pass here.
- **Concrete fix:** provide this release-review sandbox with a working Docker
  daemon and rerun the exact command from a fresh clone, retaining healthy and
  corrupt reports and zero-managed-resource evidence. If Docker cannot be
  supplied here, route the release gate to a Docker-capable clean runner and do
  not treat this sandbox as acceptance for the real-Docker claim.

No other finding was observed.

## Demo and sandbox check

The first-screen action opens `/demo/?demo=1` in one click. Its first view
shows the real command invocation and replaying terminal transcript, while the
banner says “Demo — sample data, nothing is saved to your data.” It includes
**Reset demo** and **View installation steps**. The transcript and report show
three sample orders and a signed result.

In a fresh context the demo made only same-origin requests. It wrote only
`sessionStorage['demo:restore-drill:playback']`; localStorage was empty. Reset
changed the namespaced value to `0` and did not create real-data state. The
registered demo, no-tracking, and offline tests passed at desktop and 390 px.
The privacy test observed one first-party public cache, no cookies, no
privacy-page Web Storage, and no IndexedDB. The offline test reloaded the demo
and signed-report summary after `context.setOffline(true)`.

## Claim results from a fresh clone

`npm ci` completed with zero vulnerabilities in a new clone. Every command in
`.factory/claims.json` was run exactly.

| Claim ID | Result | Evidence |
| --- | --- | --- |
| `real-docker-restore` | **FAIL** | Docker CLI missing; see F-5-1. |
| `demo-sandbox` | PASS | Two Playwright projects passed. |
| `production-boundary` | PASS | Tagged Rust test passed. |
| `automation-contract` | PASS | Tagged Rust test passed. |
| `local-io-boundary` | PASS | Tagged Rust test passed. |
| `image-pull` | PASS | Tagged Rust test passed. |
| `distribution-build` | PASS | Build made executable CLI and all static routes. |
| `weekly-scheduling` | PASS | Tagged Node test passed. |
| `mit-license` | PASS | Tagged Node test passed. |
| `site-no-tracking` | PASS | Two Playwright projects passed. |
| `offline-web-walkthrough` | PASS | Two Playwright projects passed. |

The ordinary `npm test` suite passed; its real-Docker acceptance test is
ignored by default. `npm run build` passed and produced `dist/bin/` and
`dist/site/`.

## Claim cross-check

The live landing page and README were reread after the test run. Restore,
signature, three-order, and cleanup statements map to `real-docker-restore`;
demo isolation maps to `demo-sandbox`; host safety maps to
`production-boundary`; CLI result behavior maps to `automation-contract`;
file behavior maps to `local-io-boundary`; the image statement maps to
`image-pull`; build, schedule, and license statements map to their matching
entries; site tracking and offline statements map to their two browser claims.
No unlisted claim was found.

## Copy audit

Visible language units are counted as alphanumeric words. Headings, actions,
facts, captions, and footer copy are included; commands and URLs are excluded.
No unit is over 22 words. No banned marketing term, inconsistent product term,
unexplained first-read jargon, nonsensical heading, or non-result button was
found. `Postgres`, `Docker`, `SQL`, `JSON`, and `cron` occur only in the
command-line context that needs them.

### Landing page

| Words | Copy |
| ---: | --- |
| 2 | Restore Drill |
| 1 | Demo |
| 3 | How it works |
| 1 | Privacy |
| 3 | Postgres restore rehearsal |
| 5 | Prove your Postgres backup restores. |
| 11 | For small self-hosted teams that need recovery proof before an outage. |
| 5 | Try it with sample data |
| 10 | Replays a recorded sample restore and opens its signed report. |
| 6 | Runs with your local Docker engine |
| 7 | Sample files stay in a temporary directory |
| 3 | MIT-licensed command-line tool |
| 4 | Plate 01 / isolated rehearsal |
| 14 | A sealed backup enters an isolated test chamber and emerges as a healthy database |
| 3 | Sample backup in. |
| 3 | Signed report out. |
| 4 | How the command works |
| 6 | Rehearse a restore in four steps. |
| 5 | Create an isolated Docker network. |
| 10 | The command uses a new internal network for the rehearsal. |
| 5 | Restore a shipped sample backup. |
| 14 | The demo uses a SQL file with three sample orders in a temporary directory. |
| 4 | Check the restored data. |
| 12 | The sample confirms that all three orders are present after the restore. |
| 7 | Sign the report and remove test resources. |
| 11 | Keep the report path printed by the command for later inspection. |
| 3 | Start for real |
| 7 | Install one binary on your Docker host. |
| 12 | Read the configuration before you run a drill against your own backup. |
| 5 | Read the source (opens GitHub) |
| 6 | Read the setup guide (opens GitHub) |
| 5 | What this does not do |
| 4 | It rehearses a restore. |
| 5 | It does not restore production. |
| 10 | Use a copied backup and a Docker host you control. |
| 10 | Read the privacy policy for site and command data handling. |
| 3 | Read privacy details |
| 2 | 3 orders |
| 1 | internal |
| 6 | signed report you can check later |
| 2 | restore-drill demo |
| 6 | Recovery proof for self-hosted Postgres teams. |
| 4 | Built by Param Factory |

### README prose

| Words | Sentence |
| ---: | --- |
| 14 | Restore Drill is for small self-hosted teams that need recovery proof before an outage. |
| 10 | It restores a Postgres backup in an internal Docker network. |
| 10 | It records a signed report file you can check later. |
| 9 | Docker Engine and a Rust toolchain must be installed. |
| 9 | Copy and run these commands from any working directory: |
| 16 | The demo command copies the shipped SQL sample and configuration to a fresh system temporary directory. |
| 8 | It prints that directory when the drill completes. |
| 12 | The sample contains three fictional orders and checks that all three restore. |
| 5 | Replay the verified demo at <https://restore-drill.sociobot.in/demo/?demo=1>. |
| 8 | Create a starter configuration and local credential file: |
| 11 | Use a copied backup and review the configuration before running it. |
| 13 | The command rejects public, loopback, HTTPS, and undeclared HTTP targets before Docker runs. |
| 11 | Keep the report and public signing key outside temporary automation storage. |
| 9 | With `--json`, standard output contains one final report object. |
| 5 | Other messages use standard error. |
| 14 | A passed drill exits `0`; a failed drill exits `1`; invalid input exits `2`. |
| 12 | The schedule examples include a portable cron runner and a GitHub Actions workflow. |
| 9 | Both preserve reports and return a failing exit code. |
| 11 | Credentials come from a mode-0600 file or an encrypted repository secret. |
| 6 | Run `restore-drill --help` for each command's options. |
| 13 | `npm run test:docker` needs a real Docker daemon and pulls Postgres 16 Alpine. |
| 14 | `npm run build` writes the executable Linux CLI to `dist/bin/` and the complete static site to `dist/site/`. |
| 9 | The factory deploys `dist/site/` as a static site. |
| 7 | The claim registry is `.factory/claims.json`. |
| 10 | Run every listed command from a clean clone before release. |
| 7 | A drill leaves the configured backup unchanged. |
| 12 | Reports and new signing keys are written only to configured output paths. |
| 9 | The documentation site uses no analytics or tracking cookies. |
| 10 | Its first-party offline cache contains only public site files. |
| 5 | Restore Drill is MIT-licensed. |
| 5 | See LICENSE, Privacy, and Terms. |

## Structure, accessibility, and routing

Home, demo, privacy, terms, and an unknown route were checked live at 390 px;
home was also checked at 1440 px. Each has one `h1`, one `main`, `lang="en"`,
a route title and description, canonical URL, Open Graph/Twitter metadata,
local favicon and touch icon, shared header/footer, skip link, Privacy/Terms,
factory credit, and build label. The unknown route returns the designed product
404 and an HTTP 404. Sitemap and robots are present. The route suite crawled
every same-origin link successfully.

Hash navigation, document navigation, Back, and demo exit focus the new
heading and announce it. Axe found no serious or critical problem at desktop or
390 px. Reduced motion has an explicit path. The warm-paper, forest-ink,
vermilion field-report identity and original test-chamber art match the design
brief and are not a generic SaaS template.

## History validation

Every earlier review, polish report, demo document, design document, and
handoff was read. Live and code verification confirms the earlier hero, demo,
claims, removed paid and unsupported copy, metadata, shared chrome, focus,
mobile navigation, touch-target, copy, terminology, scheduling, privacy, and
offline repairs remain present (F-1-1 through F-1-56; F-2-1 through F-2-10;
F-3-2 through F-3-4; F-4-2 and F-4-3). The sole repeated exception is the
real-Docker acceptance condition, reissued above as F-5-1.

## Missed leverage

No extra AI, import/export, or sync feature is implied by the brief. The CLI
already provides JSON reports, a downloadable signed sample report, and weekly
scheduling examples. An AI feature would introduce an unnecessary data path.

## What would make this perfect

Run the real-Docker claim successfully in the same Docker-capable clean
acceptance sandbox used for review. With that command green, no remaining
copy, demo, privacy, claims, routing, accessibility, or visual finding was
observed.
