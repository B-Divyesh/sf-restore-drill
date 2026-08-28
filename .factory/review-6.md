# Adversarial first-read review 6 — Restore Drill

**Verdict: PASS**

Reviewed 2026-08-28 UTC against `8fb8b5fa611885df5e3e5836a6a4284e33a04430` and the live site at <https://restore-drill.sociobot.in/>.

There are no findings. This review used fresh Chromium contexts, a separate clean clone, and a fresh temporary working directory. The absence of a local Docker daemon was handled by the registered release gate; it verified the matching clean Docker-capable run rather than pretending the local machine performed a restore.

## Cold first screen

Fresh contexts at 390 × 844 and 1440 × 900 opened `/` without stored site data or scrolling.

- **What it does:** rehearses a Postgres backup restore and gives the team a signed report.
- **For whom:** small self-hosted teams needing recovery proof before an outage.
- **Click first:** **Try it with sample data**.

The first screen supplies this directly in: “Prove your Postgres backup restores.”, “For small self-hosted teams that need recovery proof before an outage.”, and “Replays a recorded sample restore and opens its signed report.” The sole prominent primary action is correctly named and fits on the mobile first screen. No cold-first-screen blocking finding was observed.

## Copy audit

Counts use visible alphanumeric words; hyphenated terms count once. URLs and command lines are counted as their visible word units. Code blocks are excluded. Every visible landing sentence-like unit and every README prose sentence is listed below. No unit exceeds 22 words. No banned marketing adjective appears. The remaining technical terms are necessary names in a Postgres/Docker CLI context and the overview explains their role. Terminology is consistent: **backup**, **web walkthrough**, **demo command**, **drill**, **internal Docker network**, and **report**.

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

### README

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
| 12 | Replay the verified demo at <https://restore-drill.sociobot.in/demo/?demo=1>. |
| 8 | Create a starter configuration and local credential file: |
| 11 | Use a copied backup and review the configuration before running it. |
| 13 | The command rejects public, loopback, HTTPS, and undeclared HTTP targets before Docker runs. |
| 11 | Keep the report and public signing key outside temporary automation storage. |
| 9 | With `--json`, standard output contains one final report object. |
| 5 | Other messages use standard error. |
| 14 | A passed drill exits `0`; a failed drill exits `1`; invalid input exits `2`. |
| 13 | The schedule examples include a portable cron runner and a GitHub Actions workflow. |
| 9 | Both preserve reports and return a failing exit code. |
| 11 | Credentials come from a mode-0600 file or an encrypted repository secret. |
| 7 | Run `restore-drill --help` for each command's options. |
| 13 | `npm run test:docker` uses a local Docker daemon when one is available. |
| 12 | Otherwise, it checks the matching clean hosted run and its evidence artifact. |
| 13 | The hosted runner executes `npm run test:docker:local` against Postgres 16 Alpine. |
| 19 | `npm run build` writes the executable Linux CLI to `dist/bin/` and the complete static site to `dist/site/`. |
| 9 | The factory deploys `dist/site/` as a static site. |
| 7 | The claim registry is `.factory/claims.json`. |
| 10 | Run every listed command from a clean clone before release. |
| 7 | A drill leaves the configured backup unchanged. |
| 12 | Reports and new signing keys are written only to configured output paths. |
| 9 | The documentation site uses no analytics or tracking cookies. |
| 9 | Its first-party offline cache contains only public site files. |
| 4 | Restore Drill is MIT-licensed. |
| 5 | See LICENSE, Privacy, and Terms. |

All controls name a result or direct action: **Try it with sample data**, **Read privacy details**, **Reset demo**, **View installation steps**, **Pause replay**, **Download signed report**, **Open report in this tab**, and **Install and run the demo**. No copy finding or rewrite is required.

## Demo and sandbox verification

The landing action reached `/demo/?demo=1` in one click. Before any interaction, the mobile screen showed the real command, a recording already progressing, a passed report with three restored rows and a signature, and realistic sample contents.

The persistent banner read “Demo — sample data, nothing is saved to your data.” It supplied **Reset demo** and **View installation steps**. In a fresh context:

- Network requests were same-origin public page, script, CSS, recording, and report files only.
- `localStorage` was empty; the only state was `sessionStorage['demo:restore-drill:playback']`.
- Its key used the `demo:restore-drill:` namespace.
- **Reset demo** returned the replay to its single command line and the namespaced value to `0`.
- Leaving to installation cleared the namespaced state.
- The registered offline test reloaded the walkthrough and passed report after network interception set the browser offline.

The CLI check was run from a fresh temporary directory. This sandbox has no `docker` executable, so `restore-drill demo --json` correctly reported that prerequisite and exited `2`; it did not create a working-directory file. The registered Docker claim command then verified the clean GitHub-hosted Docker/Postgres run for the matching source: healthy three-row restore, corrupt-backup failure, both signatures, and zero managed resources.

## Claims from a clean clone

A new clone at `/tmp/restore-drill-review6-MEiegB/repo` received `npm ci` (zero audit vulnerabilities). Every command listed in `.factory/claims.json` passed.

| Claim ID | Result | Verification |
| --- | --- | --- |
| `real-docker-restore` | PASS | `npm run test:docker` validated the matching clean hosted Docker/Postgres run and evidence artifact. |
| `demo-sandbox` | PASS | Desktop and mobile tagged Playwright checks passed. |
| `production-boundary` | PASS | Tagged Rust host-rejection test passed. |
| `automation-contract` | PASS | Tagged Rust JSON/exit-code test passed. |
| `local-io-boundary` | PASS | Tagged Rust input/output boundary test passed. |
| `image-pull` | PASS | Tagged Rust pull-before-resource test passed. |
| `distribution-build` | PASS | `npm run build` made an executable CLI and all static route documents. |
| `weekly-scheduling` | PASS | Tagged Node schedule-contract test passed. |
| `mit-license` | PASS | Tagged Node license and crate-metadata test passed. |
| `site-no-tracking` | PASS | Desktop and mobile tagged Playwright checks passed. |
| `offline-web-walkthrough` | PASS | Desktop and mobile tagged Playwright offline reload checks passed. |

The live landing page and README were cross-checked after testing. Restore, three-order, network, signing, cleanup, demo isolation, host boundary, automation, local I/O, image order, build, schedule, license, tracking, and offline statements map to the above entries. No unlisted claim was found.

## Structure, access, and identity

Live checks covered home, demo, privacy, terms, and an unknown route at 390 px; home was also checked at 1440 px.

- All routes return their expected status (200 except the designed 404, which returns 404), have `lang="en"`, one `<main>`, one `<h1>`, descriptions, canonical URLs, route titles, OG/Twitter data, favicon, and touch icon.
- Titles follow the required route pattern: “Restore Drill — prove your Postgres backup restores”, “Demo — Restore Drill”, “Privacy — Restore Drill”, “Terms — Restore Drill”, and “Page not found — Restore Drill”.
- The designed 404 retains the field-report visual language and offers **Return home** and **Open the demo**.
- All crawled same-origin and GitHub setup links returned 200. Header, skip link, footer, Privacy, Terms, factory credit, and build label are present on every route.
- Hash navigation, document navigation, Back, and demo exit focus the destination heading and populate the polite route announcement. A repeated live Back check returned focus to `#hero-title` and announced the home context.
- No unexpected console errors occurred on the normal route flow. The local route suite’s Axe scans found no serious or critical issues, and the mobile controls fit without horizontal overflow.
- The warm paper, forest ink, vermilion stamp, monospace evidence, and original test-chamber art are visibly a recovery field report, not a generic SaaS template. The asset provenance and motion/reduced-motion policy match `.factory/design.md`.

## History validation

Every earlier review, polish record, demo document, design record, and handoff was read. The live site and current code were rechecked rather than accepting an earlier “fixed” label.

| Earlier findings | Current confirmation |
| --- | --- |
| F-1-1–F-1-3 | Job/audience/action, isolated demo, claims registry, and one tagged test per claim are present. |
| F-1-4–F-1-31 | Unsupported broad promises remain removed or are narrowed to the registered, passing sample/host/I-O/automation/image/build/schedule/license claims. |
| F-1-32–F-1-39 | No dead paid route; demo and 404 are product routes; hosted real-Docker evidence passes the current gate; shared chrome, metadata, focus, targets, and title all verify. |
| F-1-40–F-1-56 | Copy is short and concrete; terms are consistent; the sample action is sole primary; old paid/ambiguous controls are absent; catalog summary remains valid. |
| F-2-1–F-2-10 | Mobile terminal, privacy/cache isolation, setup links, Terms scope, variable recording timing, local I/O, image order, focus, demo terminology, and weekly scheduling verify. |
| F-3-1–F-3-4 | The current Docker release gate passes in this sandbox; offline reading is registered; the exit control names its destination; public copy avoids unexplained “tamper-evident”. |
| F-4-1–F-4-3 | The Docker gate passes; 390 px header navigation stays visible; the process is correctly called four steps. |
| F-5-1 | `npm run test:docker` passes from this clean Docker-less sandbox by verifying the matching clean Docker-capable run. |

No earlier finding is unfixed, half-fixed, or regressed.

## Missed leverage

No missing AI, import/export, or sync feature is implied by the brief. The appropriate valuable extensions—machine-readable JSON report, downloadable signed sample report, and tested weekly cron/GitHub Actions scheduling—already exist. An AI feature would add a data path without helping the stated recovery-proof job.

## What would make this perfect

Nothing is currently required. Continue rerunning the exact clean-clone claim set, particularly the Docker release gate, whenever source or deployment changes.
