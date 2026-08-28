# Adversarial first-read review 4 — Restore Drill

**Verdict: FAIL**

**Reviewed:** 2026-08-28 UTC

**Candidate:** `06246d0f5da6b5019541faa97bb7afcd2ffbb321`

**Live URL:** <https://restore-drill.sociobot.in/>

The first screen, web walkthrough, privacy boundary, routing, and visual system
are clear. One required core claim still fails in this clean sandbox. Two minor
structure/copy findings also remain. PASS requires zero findings and no failed
claim.

## Cold first screen

Fresh Chromium contexts opened the live page at 390 × 844 and 1440 × 900. No
site data was present and neither page was scrolled before this assessment.

| Question | 390 px | Desktop |
| --- | --- | --- |
| What does it do? | It proves whether a Postgres backup restores. | Same. The chamber image reinforces an isolated rehearsal. |
| For whom? | Small teams that host Postgres themselves. | Same. |
| What should I click first? | **Try it with sample data**. | **Try it with sample data**. |

The exact first-screen copy that answers these questions is “Prove your
Postgres backup restores.”, “For small self-hosted teams that need recovery
proof before an outage.”, and “Try it with sample data”. The adjacent note says
the action replays a recorded sample restore and opens its signed report. All
three plain facts are visible without scrolling at both widths. This check
passes.

## Findings

### Blocking

#### F-4-1 (reopens F-3-1 / F-1-34) — The real-Docker claim fails in the required clean sandbox

- **Location/quote:** `.factory/claims.json`, `real-docker-restore`: “The demo
  command restores three shipped rows on Postgres in an internal Docker
  network, signs the result, reports a corrupt backup, and removes its
  resources.”
- **Evidence:** From a clean clone at the candidate SHA, the exact registered
  command
  `cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored`
  failed. The test panicked at `crates/restore-drill/tests/real_docker.rs:13`
  with `Docker CLI must be installed ... No such file or directory`. The
  executable is absent in this sandbox. Running the built
  `restore-drill demo --json` in a fresh temporary caller directory also exited
  `2` with `could not start Docker CLI 'docker'` and wrote no product file in
  that caller directory.
- **Why this fails:** the central promise is a real Postgres restore on Docker.
  The work order requires every listed claim command to pass in this sandbox.
  A previous hosted-run URL and passing fake-process tests do not satisfy that
  current requirement, so a visitor still lacks reproducible acceptance
  evidence in the mandated environment.
- **Concrete fix:** provide a review worker with a Docker CLI and usable daemon,
  then run this exact command from a clean clone. Retain output that proves the
  healthy three-row restore, corrupt-backup failure, both signatures, and zero
  managed containers, volumes, or networks. Do not mark the claim passed from
  prior CI alone.

### Minor

#### F-4-2 — The mobile header removes all navigation without a replacement

- **Location/quote:** live header on `/`, `/demo/`, `/privacy/`, `/terms/`, and
  the designed 404 at 390 px. Only the circular “RD” mark is visible.
- **Evidence:** all three header links — “Demo”, “How it works”, and “Privacy” —
  have `display: none` and zero-size rectangles. The product-name text is also
  hidden. There is no menu control. Footer links remain available only after a
  long scroll.
- **Why this matters:** a phone visitor cannot use the standard header to move
  between the product, demo, and privacy information. The header is present in
  the DOM but is not a consistent usable navigation surface across viewports.
- **Concrete fix:** keep the product name and essential links visible at
  390 px, or add an accessible 44 px menu button that exposes Demo, How it
  works, and Privacy with keyboard focus management.

#### F-4-3 — “Four checks” inaccurately labels a four-step workflow

- **Location/quote:** landing heading “Run four checks before an outage.” The
  four items are create a network, restore a backup, check the data, and sign
  the report/remove resources.
- **Why this matters:** only the third item is a check. The heading makes the
  visitor expect four validations when the section actually describes four
  workflow steps.
- **Concrete fix:** rewrite the heading as **“Rehearse a restore in four
  steps.”**

## Demo and sandbox verification

- Landing to `/demo/?demo=1` takes one click. Its first 390 px screen already
  shows the persistent demo banner, a sample-specific heading, three fictional
  orders, the replay control, and the `restore-drill demo` terminal.
- The banner says “Demo — sample data, nothing is saved to your data.” It offers
  **Reset demo** and **View installation steps**.
- Playback writes only `demo:restore-drill:playback` in session storage. There
  are no cookies, local-storage values, or IndexedDB databases in a fresh
  context. The first-party cache contains only public, same-origin site files.
- Reset clears prior `demo:restore-drill:*` keys and restarts at the command.
  Leaving the walkthrough clears the demo namespace and focuses the install
  heading. Seeded non-demo local- and session-storage sentinels survived both
  actions, confirming that reset does not touch real storage.
- Every observed browser request was same-origin. After the first visit, a
  network-intercepted offline reload retained the walkthrough and passed report
  summary.
- The downloadable report says `passed`, records three restored rows, includes
  a backup hash and an 88-character signature, and corresponds to a recording
  sourced from commit `0e220c7871a7bcf154e252a3a41f6f4822069c3e`.
- The built CLI was invoked from `/tmp/restore-drill-cli-review4-zoSBgw`. It
  created its own `restore-drill-demo-*` temporary directory and did not write
  product data into the caller directory. Its Docker-dependent outcome is the
  blocking failure in F-4-1.

## Claims audit

All registered commands ran from clean clone
`/tmp/restore-drill-review4-clean-owHdAG/repo` at the candidate SHA after
`npm ci`.

| Claim | Exact registered command | Result |
| --- | --- | --- |
| `real-docker-restore` | `cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored` | **FAIL** — Docker CLI absent; F-4-1. |
| `demo-sandbox` | `npm run test:site -- --grep @claim:demo-sandbox` | PASS — desktop and 390 px. |
| `production-boundary` | `cargo test --workspace rejects_public_loopback_and_undeclared_http_hosts_before_docker` | PASS. |
| `automation-contract` | `cargo test --workspace json_output_and_exit_codes_are_stable_for_automation` | PASS. |
| `local-io-boundary` | `cargo test --workspace drill_keeps_inputs_unchanged_and_outputs_in_configured_paths` | PASS. |
| `image-pull` | `cargo test --workspace demo_pulls_a_missing_postgres_image_before_creating_resources` | PASS. |
| `distribution-build` | `npm run build && node --test --test-name-pattern='distribution build' tests/claims.test.mjs` | PASS. |
| `weekly-scheduling` | `node --test --test-name-pattern='scheduling examples' tests/claims.test.mjs` | PASS. |
| `mit-license` | `node --test --test-name-pattern='MIT license' tests/claims.test.mjs` | PASS. |
| `site-no-tracking` | `npm run test:site -- --grep @claim:site-no-tracking` | PASS — desktop and 390 px. |
| `offline-web-walkthrough` | `npm run test:site -- --grep @claim:offline-web-walkthrough` | PASS — desktop and 390 px. |

The live landing, Demo, Privacy, Terms, and README claim-like statements were
cross-checked against the registry. The sample restore/report/temporary demo
behavior maps to `real-docker-restore`; target rejection to
`production-boundary`; browser isolation to `demo-sandbox`; local file behavior
to `local-io-boundary`; scheduling, distribution, license, tracking, and
offline behavior to their matching entries. No unlisted claim was found. The
single failed listed claim remains decisive.

## Copy audit

Counts use visible alphanumeric words. Hyphenated terms, commands, paths, and
URLs count as one word. Commands inside fenced blocks are command syntax, not
sentences. Labels and headings are included so fragments are not hidden.

### Landing page

| Words | Exact copy |
| ---: | --- |
| 4 | Skip to main content |
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
| 6 | Run four checks before an outage. |
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
| 2 | Recovery evidence |
| 1 | Sample |
| 2 | 3 orders |
| 1 | Network |
| 1 | internal |
| 1 | Output |
| 6 | signed report you can check later |
| 1 | Command |
| 2 | restore-drill demo |
| 6 | Recovery proof for self-hosted Postgres teams. |
| 2 | build polish3 |
| 4 | Built by Param Factory |

Flag: “Run four checks before an outage.” uses **checks** for a list of steps;
see F-4-3. No unit exceeds 22 words, no banned marketing word appears, all
controls use result-naming verbs, and the other headings work out of context.
Repeated header/footer navigation labels are consolidated into one row each.

### README

| Words | Exact copy |
| ---: | --- |
| 2 | Restore Drill |
| 14 | Restore Drill is for small self-hosted teams that need recovery proof before an outage. |
| 10 | It restores a Postgres backup in an internal Docker network. |
| 10 | It records a signed report file you can check later. |
| 3 | Install from source |
| 9 | Docker Engine and a Rust toolchain must be installed. |
| 10 | Copy and run these commands from any working directory: |
| 17 | The demo command copies the shipped SQL sample and configuration to a fresh system temporary directory. |
| 10 | It prints that directory when the drill completes. |
| 12 | The sample contains three fictional orders and checks that all three restore. |
| 6 | Replay the verified demo at `https://restore-drill.sociobot.in/demo/?demo=1`. |
| 4 | Run your own drill |
| 8 | Create a starter configuration and local credential file: |
| 10 | Use a copied backup and review the configuration before running it. |
| 15 | The command rejects public, loopback, HTTPS, and undeclared HTTP targets before Docker runs. |
| 11 | Keep the report and public signing key outside temporary automation storage. |
| 9 | With `--json`, standard output contains one final report object. |
| 6 | Other messages use standard error. |
| 16 | A passed drill exits `0`; a failed drill exits `1`; invalid input exits `2`. |
| 4 | Schedule a weekly drill |
| 13 | The schedule examples include a portable cron runner and a GitHub Actions workflow. |
| 9 | Both preserve reports and return a failing exit code. |
| 11 | Credentials come from a mode-0600 file or an encrypted repository secret. |
| 1 | Commands |
| 8 | Run `restore-drill --help` for each command's options. |
| 4 | Develop, test, and deploy |
| 11 | `npm run test:docker` needs a real Docker daemon and pulls Postgres 16 Alpine. |
| 15 | `npm run build` writes the executable Linux CLI to `dist/bin/` and the complete static site to `dist/site/`. |
| 8 | The factory deploys `dist/site/` as a static site. |
| 7 | The claim registry is `.factory/claims.json`. |
| 10 | Run every listed command from a clean clone before release. |
| 1 | Privacy |
| 8 | A drill leaves the configured backup unchanged. |
| 12 | Reports and new signing keys are written only to configured output paths. |
| 11 | The documentation site uses no analytics or tracking cookies. |
| 11 | Its first-party offline cache contains only public site files. |
| 3 | License and policies |
| 3 | Restore Drill is MIT-licensed. |
| 5 | See LICENSE, Privacy, and Terms. |

No README sentence exceeds 22 words or uses a banned marketing adjective.
Docker, Postgres, Rust, SQL, JSON, cron, and GitHub Actions are appropriate
technical terms for the named self-hosting audience. Terminology is consistent:
**backup** is the input, **web walkthrough** is the browser replay, **demo
command** is the executable sample, **drill** is a full check, **internal Docker
network** is the isolation boundary, and **report** is the output.

## History audit

Every earlier review, polish report, verification report, and handoff was read.
The confirmations below are based on the current live site and candidate code,
not the earlier “fixed” labels.

| Earlier ID | Current confirmation |
| --- | --- |
| F-1-1 | Fixed: both live widths name the job, audience, one primary action, its outcome, and three facts. |
| F-1-2 | Fixed: `/demo/?demo=1`, the recording, report, banner, reset, exit, CLI command, sample, and demo docs exist. |
| F-1-3 | Fixed: the registry exists and the policy test confirms exactly one tag per ID. |
| F-1-4 | Fixed by scope: volume/app-probe marketing was removed; the retained shipped SQL restore is registered. |
| F-1-5 | Fixed: public, loopback, HTTPS, and undeclared targets pass the registered rejection test. |
| F-1-6 | Fixed by scope: broad upload/telemetry copy was removed; current site and file boundaries are registered and tested. |
| F-1-7 | Fixed: the missing-image pull-order claim passes. |
| F-1-8 | Fixed by scope: existing-network marketing was removed; undeclared targets remain tested. |
| F-1-9 | Fixed by removal: unsupported volume-format claims are absent. |
| F-1-10 | Fixed by scope: broad secret wording is absent; input and output boundaries pass. |
| F-1-11 | Fixed by scope: broad app-probe marketing is absent; the sample SQL assertion is explicit. |
| F-1-12 | The old broad schema/timing sentence is removed. The scoped real-Docker replacement is blocked by F-4-1. |
| F-1-13 | Fixed: the browser now states its demo namespace; storage and request checks pass. |
| F-1-14 | Fixed: the live recording identifies `restore-drill demo` and has a real-run source SHA. |
| F-1-15 | Fixed by removal: no release-binary promise remains. |
| F-1-16 | Fixed by scope: the report can be downloaded and the real-Docker test contains independent verification; its current execution is F-4-1. |
| F-1-17 | Fixed by removal: no Team Kit offer or entitlement remains. |
| F-1-18 | Fixed by removal: no unregistered free-tier entitlement copy remains. |
| F-1-19 | Fixed by removal: no merchant/refund claim remains. |
| F-1-20 | Fixed by removal/registration: Team Kit copy is absent and offline public-file reading passes its claim test. |
| F-1-21 | Fixed: Docker is stated as a prerequisite and image-pull ordering is registered. |
| F-1-22 | Fixed: JSON channels and 0/1/2 exit codes pass `automation-contract`. |
| F-1-23 | Fixed by removal: archive-compatibility and traversal marketing are absent. |
| F-1-24 | Fixed: host rejection and local secret/canary boundaries pass their tests. |
| F-1-25 | Fixed by scope: detailed schema marketing is absent; the sample report is inspectable. |
| F-1-26 | Fixed by removal/scope: truncated-dump copy is absent; corrupt input is in the real-Docker test blocked by F-4-1. |
| F-1-27 | Fixed: the registered distribution build passes and writes executable/site artifacts. |
| F-1-28 | Fixed by scope: public copy now promises the shipped backup, network, check, report, and cleanup. |
| F-1-29 | The cleanup assertions exist for healthy and corrupt real runs; current execution is blocked by F-4-1. |
| F-1-30 | Fixed by scope: `--keep-on-failure` is syntax, not public promise copy. |
| F-1-31 | Fixed: the MIT claim passes against LICENSE and crate metadata. |
| F-1-32 | Fixed by removal: there is no paid path or checkout link. |
| F-1-33 | Fixed: Demo/legal routes are real; an unknown route returns the designed product 404 with status 404. |
| F-1-34 | **Reopened by F-4-1:** the exact real-Docker command fails in this required sandbox. |
| F-1-35 | Fixed for route presence: every page has shared header/footer and legal links. Mobile visibility is the new F-4-2. |
| F-1-36 | Fixed: every route has route-specific canonical, OG/Twitter, favicon, and touch-icon metadata. |
| F-1-37 | Fixed: deep hash, forward, Back, and demo-exit navigation focus and announce headings. |
| F-1-38 | Fixed: all visible controls are at least 44 × 44 px at both tested widths. |
| F-1-39 | Fixed: the live title is “Restore Drill — prove your Postgres backup restores”. |
| F-1-40 | Fixed: the overlong README workflow sentence is gone. |
| F-1-41 | Fixed: the exit-code contract is 16 words. |
| F-1-42 | Fixed: the H1 names the restore job directly. |
| F-1-43 | Fixed: the demo heading names the sample Postgres restore. |
| F-1-44 | The vague prior slogan is gone; its replacement has the accuracy issue in F-4-3. |
| F-1-45 | Fixed: the step says “Create an isolated Docker network.” |
| F-1-46 | Fixed: the step says “Sign the report and remove test resources.” |
| F-1-47 | Fixed: the install heading names the Docker-host action. |
| F-1-48 | Fixed: the section is headed “What this does not do.” |
| F-1-49 | Fixed by removal: the paid habit section is absent. |
| F-1-50 | Fixed by removal: no ambiguous Copy control remains. |
| F-1-51 | Fixed by removal: no license control remains. |
| F-1-52 | Fixed: the primary action is “Try it with sample data.” |
| F-1-53 | Fixed: backup, web walkthrough, demo command, drill, network, and report are used consistently. |
| F-1-54 | Fixed: overview copy uses direct command/check/network/report language; audience-specific technical names remain. |
| F-1-55 | Fixed by removal: the paid workflow bullet is absent. |
| F-1-56 | Fixed: `brief.summary` and the catalog line are verb-first and within 120 characters. |
| F-2-1 | Fixed: the mobile terminal wraps, is focusable, and has zero serious/critical Axe findings. |
| F-2-2 | Fixed: the claim now inspects requests, cookies, storage, service worker, and public-only first-party cache. |
| F-2-3 | Fixed: both setup links target the existing GitHub `#install-from-source` heading. |
| F-2-4 | Fixed: Terms promises source, not unpublished binaries. |
| F-2-5 | Fixed: the recording says `<measured time>` instead of a fixed duration. |
| F-2-6 | Fixed: local input/output behavior is registered and passes. |
| F-2-7 | Fixed: missing-image pull order is registered and passes. |
| F-2-8 | Fixed: document and Back navigation focus and announce route headings. |
| F-2-9 | Fixed: browser “web walkthrough” and executable “demo command” are distinguished. |
| F-2-10 | Fixed: tested cron and GitHub Actions weekly examples ship. |
| F-3-1 | **Still open as F-4-1:** its exact clean-sandbox command fails without Docker. |
| F-3-2 | Fixed: `offline-web-walkthrough` is registered and passes. |
| F-3-3 | Fixed: the demo exit control says “View installation steps.” |
| F-3-4 | Fixed: “tamper-evident” is replaced by “signed report you can check later.” |

## Structure, accessibility, links, and visual identity

- Home, Demo, Privacy, Terms, and the unknown-route 404 have the expected
  titles, one h1, one main landmark, descriptions, canonicals, complete local
  OG/Twitter imagery, favicons, and touch icons. `robots.txt`, `sitemap.xml`,
  security headers, and the product 404 configuration are present.
- An unknown live route returns HTTP 404 with the designed Restore Drill page.
  Every discovered same-origin link returns below 400 and every local fragment
  exists. The GitHub repository and `#install-from-source` heading resolve.
- Forward, Back, direct hash, and demo-exit navigation move focus to the new
  heading and update the polite live region. F-4-2 records the separate mobile
  header visibility failure.
- The production live audit reports zero serious/critical Axe violations at
  desktop and 390 px. There is no horizontal overflow, and visible controls
  meet the 44 px target. `/opt/fleet/lib/verify-url.sh` passes with no home-page
  console errors. The only live-audit console error is the expected HTTP 404
  resource message from deliberately opening the missing route.
- `npm run build` produced 4.31 kB JavaScript (1.92 kB gzip), 14.24 kB CSS
  (3.93 kB gzip), the CLI, and all required static routes.
- The warm paper, forest ink, vermilion stamp color, serif/monospace pairing,
  halftone chamber illustration, ruled evidence rows, and restrained motion
  match `.factory/design.md`. The result is recognizably product-specific, not
  a generic SaaS template. Reduced-motion handling is present.

## Missed leverage

No additional feature finding is warranted. The brief's recurrence need is met
by the tested weekly cron and GitHub Actions examples. Backup input and signed
report output already cover the useful import/export boundary. AI would add
uncertainty to a deterministic recovery check, and no decorative AI feature or
embedded provider key is present.

## Additional verification

- `npm test`: PASS — 4 Rust unit tests, 7 CLI integration tests, 3 Node claim
  tests, 7 policy tests, and 20 desktop/mobile Playwright tests. The separate
  real-Docker test remains intentionally excluded from this default command.
- `npm run check:types`: PASS.
- `npm run check:lint`: PASS.
- `npm run build`: PASS.
- `npm run test:live -- https://restore-drill.sociobot.in/
  /tmp/restore-drill-review4-live`: PASS for live routes, demo report, storage,
  offline reload, request origin, and Axe.
- `/opt/fleet/lib/verify-url.sh https://restore-drill.sociobot.in/
  /tmp/restore-drill-review4-verify`: PASS.

## What would make this perfect

Make the registered real-Docker command pass in the required clean review
sandbox and retain the full real-engine evidence. Restore usable header
navigation at 390 px. Rename “four checks” to “four steps”. Then rerun every
claim and the complete cold/live/history audit; only zero findings warrants
PASS.
