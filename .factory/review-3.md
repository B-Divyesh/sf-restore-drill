# Adversarial first-read review 3 — Restore Drill

**Verdict: FAIL**

**Reviewed:** 2026-08-28 UTC  
**Candidate:** `996c188fccf7ceae5772e9358862cb5cf597c4df`  
**Live URL:** <https://restore-drill.sociobot.in/>

PASS requires zero findings and every registered claim to pass from this clean
sandbox. This review has two blocking findings and two minor copy findings.

## Cold first screen

Fresh Chromium contexts opened the live home page at 390 × 844 and 1440 × 900.
No site data was present and neither page was scrolled.

| Question | Result at both widths |
| --- | --- |
| What does it do? | It helps prove that a Postgres backup restores. |
| For whom? | Small teams that self-host Postgres. |
| What should I click first? | **Try it with sample data**. |

The exact visible copy that establishes this is “Prove your Postgres backup
restores.”, “For small self-hosted teams that need recovery proof before an
outage.”, and “Try it with sample data”. The button and all three plain facts
were visible at both widths, with no horizontal overflow. This first-read check
passes.

## Findings

### Blocking

#### F-3-1 (reopens F-1-34) — The real-Docker claim fails in the required clean sandbox

- **Location/quote:** `.factory/claims.json`, `real-docker-restore`: “The demo
  command restores three shipped rows on Postgres in an internal Docker network,
  signs the result, reports a corrupt backup, and removes its resources.”
- **Evidence:** In the clean clone, the exact registered command
  `cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored`
  failed. Its sole test panicked with `Docker CLI must be installed for this
  ignored acceptance test: ... No such file or directory`. `docker` is absent
  in this sandbox. Running `restore-drill demo --json` from a fresh temporary
  directory also exited 2 with `could not start Docker CLI 'docker'`; it left
  zero caller-directory files.
- **Why this fails:** The review instructions require every registered claim
  command to pass in this sandbox. The product's core promise is a real
  Docker/Postgres restore, so a fake-Docker unit test and a prior hosted-CI URL
  do not turn this failed local claim into current evidence.
- **Concrete fix:** make the release verification environment Docker-capable
  (Docker CLI plus daemon), retain this real healthy and corrupt sample test,
  and do not report the claim as passed until its exact command succeeds there.
  Preserve signed healthy/failed reports and zero-resource assertions.

#### F-3-2 — The privacy page makes an unlisted offline claim

- **Location/quote:** live `/privacy/`: “A first-party service worker caches
  public site files for offline reading.”
- **Evidence:** `.factory/claims.json` has no `offline` claim. The repository
  has an untagged Playwright test, `demo reloads offline after its first visit`,
  and this review independently confirmed the cached demo reloads offline.
  Neither supplies the required one-to-one registered `@claim:` test.
- **Why this fails:** A visitor can rely on offline reading, but release review
  cannot discover or run its proof through the claim registry. This is an
  unlisted claim even though the behavior appears to work.
- **Concrete fix:** add an `offline-web-walkthrough` claim with this Privacy
  location, tag the existing test `@claim:offline-web-walkthrough`, and state
  the fresh-context, first-visit, intercepted-network sandbox. Or remove “for
  offline reading” from the page.

### Minor

#### F-3-3 — “Start for real” does not name the result of the control

- **Location/quote:** demo banner link: “Start for real”.
- **Why this matters:** It opens `/#install-from-source`, but a cold visitor
  cannot tell that the result is installation instructions.
- **Concrete fix:** change it to **View installation steps**. The adjacent
  install-section eyebrow may remain “Start for real” if it is not a control.

#### F-3-4 — “Tamper-evident” is unexplained technical language

- **Location/quote:** landing evidence slip output: “tamper-evident report”.
- **Why this matters:** The first screen otherwise uses direct language. This
  cryptographic term does not tell a first-time visitor what they can do with
  the report.
- **Concrete fix:** use **signed report you can check later**. Keep the signed
  report evidence mapped to `real-docker-restore`.

## Copy audit

Counts use visible alphanumeric words. Commands in fenced code blocks are not
sentences. No audited sentence exceeds 22 words and no banned marketing word
appears. F-3-3 and F-3-4 are the only copy flags.

### Landing page

| Words | Sentence or visible copy unit |
| ---: | --- |
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
| 2 | 3 orders |
| 1 | internal |
| 2 | tamper-evident report |
| 2 | restore-drill demo |
| 6 | Recovery proof for self-hosted Postgres teams. |
| 4 | Built by Param Factory |

Headings make sense out of context. `Try it with sample data`, `Read privacy
details`, and the source/setup links name a result. “Start for real” is the
exception in F-3-3. “Tamper-evident” is the jargon exception in F-3-4. The
intended terms otherwise remain consistent: **backup**, **web walkthrough**,
**demo command**, **drill**, **internal Docker network**, and **report**.

### README

| Words | Sentence or visible copy unit |
| ---: | --- |
| 14 | Restore Drill is for small self-hosted teams that need recovery proof before an outage. |
| 11 | It rehearses a Postgres backup restore in an isolated Docker network. |
| 10 | It records a tamper-evident report file after the checks finish. |
| 4 | Install from source |
| 9 | Docker Engine and a Rust toolchain must be installed. |
| 10 | Copy and run these commands from any working directory: |
| 17 | The demo command copies the shipped SQL sample and configuration to a fresh system temporary directory. |
| 10 | It prints that directory when the drill completes. |
| 12 | The sample contains three fictional orders and checks that all three restore. |
| 5 | Replay the verified demo at |
| 4 | Run your own drill |
| 8 | Create a starter configuration and local credential file: |
| 10 | Use a copied backup and review the configuration before running it. |
| 15 | The command rejects public, loopback, HTTPS, and undeclared HTTP targets before Docker runs. |
| 11 | Keep the report and public signing key outside temporary automation storage. |
| 9 | With `--json`, standard output contains one final report object. |
| 6 | Other messages use standard error. |
| 16 | A passed drill exits `0`; a failed drill exits `1`; invalid input exits `2`. |
| 6 | Schedule a weekly drill |
| 14 | The schedule examples include a portable cron runner and a GitHub Actions workflow. |
| 10 | Both preserve reports and return a failing exit code. |
| 11 | Credentials come from a mode-0600 file or an encrypted repository secret. |
| 1 | Commands |
| 8 | Run `restore-drill --help` for each command's options. |
| 5 | Develop, test, and deploy |
| 11 | `npm run test:docker` needs a real Docker daemon and pulls Postgres 16 Alpine. |
| 15 | `npm run build` writes the executable Linux CLI to `dist/bin/` and the complete static site to `dist/site/`. |
| 14 | The factory deploys `dist/site/` as a static site. |
| 7 | The claim registry is `.factory/claims.json`. |
| 10 | Run every listed command from a clean clone before release. |
| 2 | Privacy |
| 8 | A drill leaves the configured backup unchanged. |
| 12 | Reports and new signing keys are written only to configured output paths. |
| 11 | The documentation site uses no analytics or tracking cookies. |
| 11 | Its first-party offline cache contains only public site files. |
| 3 | License and policies |
| 3 | Restore Drill is MIT-licensed. |
| 9 | See LICENSE, Privacy, and Terms. |

Targeted CLI words such as Docker, Postgres, Rust, cron, and GitHub Actions
are appropriate to the named audience. The README's “tamper-evident” wording
is the same jargon issue as F-3-4.

## Demo and sandbox checks

- Landing → `/demo/?demo=1` took one click. The first page immediately showed
  a recorded terminal run, three fictional orders, a report panel, and a
  downloadable signed JSON report.
- The persistent banner says “Demo — sample data, nothing is saved to your
  data.” Reset cleared `demo:restore-drill:playback` and restarted playback.
  Start for real cleared that key and focused the install heading.
- Fresh demo storage had no cookies, localStorage, or IndexedDB. The only
  session key was `demo:restore-drill:playback`; requests were same-origin.
  The service worker cache was `restore-drill-shell-v3` and contained public
  same-origin site files. An offline reload after first visit retained the demo
  headline and passed report summary.
- The CLI demo command was executed from a new `/tmp` directory. It could not
  start because Docker is absent, returned 2, and wrote no caller-directory
  file. This is F-3-1, not evidence that demo data leaked.

## Claim execution from a clean clone

Clean clone: `/tmp/restore-drill-review3-gFMtcH/repo` at the candidate SHA.
`npm ci` completed with zero reported vulnerabilities.

| Claim | Exact registered command | Result |
| --- | --- | --- |
| `real-docker-restore` | `cargo test --workspace real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup -- --ignored` | **FAIL** — Docker CLI absent; see F-3-1. |
| `demo-sandbox` | `npm run test:site -- --grep @claim:demo-sandbox` | PASS (desktop and 390 px). |
| `production-boundary` | `cargo test --workspace rejects_public_loopback_and_undeclared_http_hosts_before_docker` | PASS. |
| `automation-contract` | `cargo test --workspace json_output_and_exit_codes_are_stable_for_automation` | PASS. |
| `local-io-boundary` | `cargo test --workspace drill_keeps_inputs_unchanged_and_outputs_in_configured_paths` | PASS. |
| `image-pull` | `cargo test --workspace demo_pulls_a_missing_postgres_image_before_creating_resources` | PASS. |
| `distribution-build` | `npm run build && node --test --test-name-pattern='distribution build' tests/claims.test.mjs` | PASS. |
| `weekly-scheduling` | `node --test --test-name-pattern='scheduling examples' tests/claims.test.mjs` | PASS. |
| `mit-license` | `node --test --test-name-pattern='MIT license' tests/claims.test.mjs` | PASS. |
| `site-no-tracking` | `npm run test:site -- --grep @claim:site-no-tracking` | PASS (desktop and 390 px). |

`npm test` also passed: 4 Rust unit tests, 7 CLI tests, 3 Node claim tests, 7
policy tests, and 20 Playwright checks. Its real-Docker test is intentionally
ignored, so the separately required claim command remains decisive. `npm run
build` passed and produced the CLI plus static site.

## History audit

All prior review, polish, verification, and handoff files were read. This table
records current live/code confirmation rather than relying on a prior “fixed”
label.

| Earlier finding | Current confirmation |
| --- | --- |
| F-1-1 | Fixed: job, audience, sole sample action, and three facts are live at both widths. |
| F-1-2 | Fixed: direct demo route, recorded run, signed report, banner, Reset, and CLI command exist. |
| F-1-3 | Fixed: registry exists and every ID has one tagged test. |
| F-1-4 through F-1-33 | Fixed or removed: broad old copy, billing, release-binary, fake preview, dead routes, metadata, navigation, targets, and old copy are absent; current routes/links verify. |
| F-1-34 | **Reopened as F-3-1:** the exact real-Docker claim command fails in this sandbox. |
| F-1-35 through F-1-56 | Fixed: shared chrome, assets, focus behavior, titles, short copy, terminology, and catalog summary were confirmed. |
| F-2-1 | Fixed: terminal is focusable, wraps at 390 px, and current Axe scans have zero serious/critical issues. |
| F-2-2 | Fixed for the registered no-tracking/cache claim; F-3-2 identifies the separate missing offline registration. |
| F-2-3 | Fixed: public setup links use the live `#install-from-source` README anchor. |
| F-2-4 | Fixed: Terms promises source, not published binaries. |
| F-2-5 | Fixed: recording uses `<measured time>`, not a fixed duration. |
| F-2-6 | Fixed: local I/O assertion is registered and passing. |
| F-2-7 | Fixed: missing-image ordering is registered and passing. |
| F-2-8 | Fixed: forward, Back, and hash navigation focus a heading and announce context. |
| F-2-9 | Fixed: current code distinguishes web walkthrough from demo command. |
| F-2-10 | Fixed: cron and GitHub Actions examples are present and the claim passes. |

## Structure, accessibility, links, and product fit

- Live Home, Demo, Privacy, Terms, and the unknown-route 404 have correct
  titles, one h1, main landmark, description, canonical, local favicon/touch
  icon, complete OG/Twitter metadata, and shared header/footer.
- The unknown route returns the designed product 404 with HTTP 404. Deep links,
  hash navigation, Back focus, announcements, same-origin links, and in-page
  targets work. The GitHub installation anchor resolves.
- Desktop and 390 px Axe scans have zero serious/critical issues. There is no
  horizontal overflow and visible controls meet the 44 px target rule.
- Requests during the browser flow stayed same-origin. The expected unknown
  route produced the only console resource error.
- The warm-paper, forest-ink, vermilion, halftone field-report surface, original
  chamber art, serif/monospace pairing, and ruled evidence layout match the
  design thesis and are distinct from a generic SaaS template.
- The brief implies recurring proof, importable input, and report output. The
  weekly scheduling examples, bundled SQL sample, and signed report address
  that leverage. AI would be decorative for this deterministic restore check;
  no AI feature or provider key is present.

## What would make this perfect

Run the real-Docker claim successfully in the required verification sandbox,
then register the offline promise with its existing observable test. Replace
the vague demo-exit control and unexplained report label. Re-run this whole
review from fresh browser and clean-clone contexts. PASS requires no remaining
finding and no failing or unlisted claim.
