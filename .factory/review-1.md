# Adversarial first-read review 1 — Restore Drill

**Verdict: FAIL**

**Reviewed:** 2026-08-28 UTC

**Candidate:** `4f84762554e3b693a064578a29d7178a0b93c211`

**Live URL:** <https://restore-drill.sociobot.in/>

There are blocking first-read, demo, claims, routing, checkout, and end-to-end
evidence failures. A PASS requires zero findings, so the passing build,
accessibility, privacy, and visual checks do not change the verdict.

## Cold first screen

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 without
scrolling or stored site data.

- **What I think it does:** restores a Postgres dump or volume in a disposable
  Docker network, checks the database and application, and keeps signed proof.
- **For whom:** I cannot answer from either first screen. The intended “small
  team self-hosting Postgres” audience is absent.
- **What I should click first:** I cannot answer confidently. “Install the CLI”
  is styled as primary, while “Run a 12-second preview” competes beside/below it.
  Neither is the required “Try it with sample data” entry.

Exact copy that failed: “A backup is a promise. Test the restore.”, “Restore a
Postgres dump or volume into a disposable Docker network, probe the database
and app, then keep signed proof it worked.”, “Install the CLI”, and “Run a
12-second preview”. The headline is a metaphor rather than the user's job; the
supporting sentence says what the tool does but not who needs it.

## Findings

### Blocking

#### F-1-1 — The first screen does not identify the user or one first action

- **Location/quote:** landing hero, both viewports: “A backup is a promise. Test
  the restore.”; “Install the CLI”; “Run a 12-second preview”.
- **Why this fails:** a cold visitor can infer the operation, but not whether the
  tool is for their team or which of two actions begins the evaluation. The hero
  also has only two safety fragments, not three plain privacy/offline/price
  facts.
- **Concrete fix:** use a job headline such as “Prove your Postgres backup
  restores”; follow with “For small self-hosted teams that need recovery proof
  before an outage”; make “Try it with sample data” the sole primary action and
  state what opens; add three short tested facts such as “Runs on your machine”,
  “Rejects production hosts”, and “Core CLI: free”.

#### F-1-2 — There is no one-click isolated demo for this CLI

- **Location/quote:** hero button “Run a 12-second preview”; `/demo`; CLI help;
  repository root.
- **Evidence:** the button scrolls to a scripted five-row animation. It has no
  “Demo — sample data, nothing is saved” banner, Reset demo, or Start for real.
  `GET /demo` returns the Azure Static Web Apps 404. Running
  `restore-drill demo` in a fresh temporary directory exits 2 with
  `unrecognized subcommand 'demo'`. There is no `examples/` directory and no
  `.factory/demo.md`.
- **Why this fails:** the animation does not execute the product's main job on a
  shipped backup and cannot prove the CLI works. A visitor cannot try the tool
  without setup.
- **Concrete fix:** ship a realistic sample dump/config under `examples/`, add a
  `restore-drill demo` command that uses an isolated temporary directory and
  reports its output path, publish a recording of that exact run, make `/demo`
  open it directly, and provide the persistent banner, reset, and start-real
  actions. Document the sandbox in `.factory/demo.md`.

#### F-1-3 — The required claims registry and tagged claim tests do not exist

- **Location:** `.factory/claims.json` and the full repository.
- **Evidence:** `.factory/claims.json` is absent and `rg '@claim'` returns no
  tests. Therefore there were no listed claim commands to run from the clean
  clone.
- **Why this fails:** no visitor-facing claim has the required stable mapping to
  a clean-sandbox outcome test. Existing broad tests cannot be audited claim by
  claim.
- **Concrete fix:** add `.factory/claims.json`; give every retained claim below
  exactly one `@claim:<id>` test; run each via the documented command from the
  shipped demo data.

Each row below is a separate unlisted-claim finding. Quotes combine duplicates
only where one observable test can cover the same promise.

| ID | Exact claim and location | Concrete test or copy fix |
| --- | --- | --- |
| F-1-4 | Landing: “Restore a Postgres dump or volume into a disposable Docker network, probe the database and app, then keep signed proof it worked.” README: first two sentences. | Run the shipped dump and volume samples against a real Docker engine; assert database/app probes, signed report, and cleanup. Split the 29-word README sentence. |
| F-1-5 | Landing: “Production hosts rejected.” README: “It never performs production restores…” | Try public, loopback, production, and existing-network targets; assert rejection before Docker mutation. |
| F-1-6 | README: “does not upload backups, credentials, or reports”; “has no telemetry.” | Intercept network for init/check/run/verify and the full demo; allow only requested image pulls and assert no product telemetry/upload. |
| F-1-7 | Landing: “Pull images first, then create a new Docker `--internal` network.” | Record real Docker events; assert all pulls precede `network create --internal`. |
| F-1-8 | Landing: “Existing networks and undeclared HTTP hosts are refused.” | Seed a colliding network and an undeclared host; assert both fail without modifying the existing network. |
| F-1-9 | Landing: “Load a plain/custom Postgres dump or a validated volume tar.” README repeats the supported formats. | Restore one shipped sample of every stated format on a real Postgres container and assert its data. |
| F-1-10 | Landing: “The original stays read-only; secrets stay in local env files.” | Hash and permission-check the source before/after; inspect the process list, report, stdout, and stderr for secret absence. |
| F-1-11 | Landing: “Run exact SQL expectations and app-level HTTP probes from inside the isolated network—not against production.” | Run passing and failing SQL/HTTP sample assertions and prove the probe container joins only the disposable internal network. |
| F-1-12 | Landing: “Write hashes, image IDs, outcomes, and recovery time to Ed25519-signed JSON; remove containers, volume, and network.” | Assert every field/signature and zero labelled resources after success, failure, timeout, and Ctrl-C on real Docker. |
| F-1-13 | Landing: “This preview uses no server and touches no data.” | Intercept the entire preview and assert no post-load requests, cookies, local/session storage, IndexedDB, OPFS, or cache writes. The manual run found no browser storage change, but no claim test records it. |
| F-1-14 | Landing: “The real CLI records the same checkpoints from Docker.” | Compare every preview checkpoint with fields produced by `restore-drill demo`; remove this sentence until a real demo exists. |
| F-1-15 | Landing: “Release binaries are attached to tagged GitHub releases.” | Query a pinned public release and install its binary. No release link is presented; remove the sentence until a release exists. |
| F-1-16 | Landing: “A signed local report ties the exact artifact to the images, probes, and elapsed time used in the rehearsal. Verify it later without a service account.” | Run a real drill, verify with a separately retained public key while offline, then mutate each bound value and assert failure. |
| F-1-17 | Landing: “$39 once · yours for every environment your team operates” and four Team Kit entitlement bullets. | Complete sandbox checkout/license return and assert price, organization scope, all four downloads, and future-revision entitlement; otherwise remove the offer. |
| F-1-18 | Landing: “The core CLI, signed reports, safety checks, and JSON export stay free.” | Install without a license and exercise each named capability; assert no billing request or gate. |
| F-1-19 | Landing: “Sociobot/Dodo is the merchant of record. Refunds are handled there and revoke the license.” | Add billing sandbox tests for merchant display, refund, and automatic revocation. |
| F-1-20 | Offline status: “The docs and last verified Team Kit license remain available on this device.” | Prime the shell and license verdict, intercept the network, go offline, reload every promised page, and verify the cached entitlement. The docs-only manual offline reload passed. |
| F-1-21 | README: “Docker Engine must be installed and reachable…” and “Postgres and probe images are pulled before the isolated network is created.” | Run `check` with missing/unreachable Docker and run with an empty image cache; assert the stated diagnostics and event order. |
| F-1-22 | README: “There are no interactive prompts”; the `--json` stdout/stderr contract; the 0/1/2 exit-code sentence. | Exercise success, drill failure, invalid config, and closed stdin; parse exactly one stdout JSON object and assert channel/exit behavior. |
| F-1-23 | README: archive compatibility plus “rejects symlinks and path traversal before extraction.” | Test compatible/incompatible major versions and malicious symlink, hard-link, absolute, and traversal archives before extraction. |
| F-1-24 | README: HTTP host restrictions and “Secrets are passed to Docker with `--env-file`, never put on the command line or copied into reports.” | Inspect real Docker argv/config and all outputs while testing declared, public, loopback, and production hosts. |
| F-1-25 | README: reports contain duration, assertion evidence, SHA-256, image IDs, and Ed25519 signature; public-key verification behavior. | Schema-check a real report, verify it with the external public key, then test tampering and the documented no-key limitation. |
| F-1-26 | README: “A deliberately truncated dump should produce exit code `1`…” | Truncate the shipped dump, run on real Docker, assert exit 1, failed signed report, and cleanup. |
| F-1-27 | README: “`npm test` runs Rust unit/integration tests plus landing-page checks” and “`npm run build` produces…” | A clean-clone gate already observes this; register it as a claim and assert both `dist/bin/` and `dist/site/`. |
| F-1-28 | README: supported inputs/app containers and non-goals (“does not make backups, orchestrate PITR…or restore production”). | Add positive support tests and negative command/config tests for every boundary. |
| F-1-29 | README: labelled resources “are removed after success, failure, timeout, or Ctrl-C.” | Use a real daemon and enumerate labelled containers, volumes, and networks after all four paths. |
| F-1-30 | README: “`--keep-on-failure` exists for local diagnosis and prints the exact resources left behind.” | Force failure with the flag; assert the resources remain, names match output/report, and a documented cleanup command works. |
| F-1-31 | README: “Restore Drill is MIT-licensed…” | Register a packaging test that checks the distributed source/crate and binary bundle contain the MIT license, or state only that the repository source is MIT-licensed. |

#### F-1-32 — The paid primary path is dead

- **Location/quote:** “Buy the Team Kit · $39” links to
  `https://api.sociobot.in/api/v1/products/restore-drill/checkout`.
- **Evidence:** fresh HEAD and GET requests both return HTTP 404; GET body is
  `{"error":"enabled factory product","status":404}`. The earlier handoff also
  says the product still needs factory registration.
- **Why this fails:** the site offers a paid product that cannot be purchased.
- **Concrete fix:** register and enable `restore-drill` in the Sociobot billing
  API, configure the return URL, and add a sandboxed checkout-to-download test.

#### F-1-33 — `/demo` and unknown paths use a generic third-party 404

- **Location:** `/demo` and `/does-not-exist`.
- **Evidence:** both return HTTP 404 with title “Azure Static Web Apps - 404:
  Not found”, no `main` or `h1`, Azure branding, and requests to
  `ajax.aspnetcdn.com` and `appservice.azureedge.net`.
- **Why this fails:** the required demo deep link is broken, the 404 has no way
  back in this product's identity, and the error page violates the no-third-
  party-script/font policy.
- **Concrete fix:** add a real `/demo` route; configure a Restore Drill 404 with
  its own title, one h1, main landmark, header/footer, and home/demo links; make
  the host serve it without Azure CDN assets.

#### F-1-34 — The core restore has not been verified on a real Docker engine

- **Location:** `.factory/handoff.md` known gaps and both earlier verification
  reports.
- **Evidence:** the repository test uses a Docker-compatible fake. The previous
  handoff explicitly says no real representative or corrupt restore has run.
- **Why this fails:** the real job-to-be-done is Docker/Postgres recovery. A fake
  command harness is useful but cannot prove image, network, volume, Postgres,
  signal, or teardown behavior end to end.
- **Concrete fix:** on a clean Docker host, run one representative shipped dump
  and one corrupt copy, retain signed reports/event logs, verify cleanup and exit
  codes, and make these claim tests repeatable in CI.

### Non-blocking findings

#### F-1-35 — Legal routes do not use the consistent site header and footer

- **Location:** `/privacy/` and `/terms/`; each has zero `header` and zero
  `footer` elements.
- **Why this matters:** visitors lose the Demo/navigation links, product
  one-liner, Privacy/Terms pair, factory credit, and version/build id required by
  the shared skeleton.
- **Concrete fix:** render the same header, skip link, and complete footer on all
  routes; include “Built by Param Factory” and a build/version identifier.

#### F-1-36 — Social metadata and required icon variants are absent

- **Location:** `<head>` on `/`, `/privacy/`, and `/terms/`.
- **Evidence:** no Open Graph title/description/image, Twitter card, or
  apple-touch icon exists. Legal routes also have no favicon. Home has only a
  data-URL SVG favicon.
- **Concrete fix:** add route-specific OG/Twitter metadata, a real original
  1200 × 630 field-report image, a served SVG favicon, and a 180 px apple-touch
  icon to every page.

#### F-1-37 — Hash navigation does not move or announce focus

- **Location:** “How it works” and “Evidence” header links.
- **Evidence:** after activating “How it works”, the URL and scroll position
  change but `document.activeElement` becomes `body`; no route announcement is
  made. Back returns to the prior URL/scroll position.
- **Concrete fix:** focus the destination heading with `tabindex="-1"` and
  announce it in a polite live region after in-page navigation.

#### F-1-38 — Several mobile targets are smaller than 44 px

- **Location/evidence at 390 px:** home logo 36 × 36; “Read the source” and
  “Open the runbook” are 25 px high; inline Privacy/Terms are 16 px high; one
  Terms link is 42 px wide.
- **Concrete fix:** add padding/minimum inline size so every interactive target
  is at least 44 × 44 CSS px without changing the visible type size.

#### F-1-39 — The home title is grammatically unclear

- **Location/quote:** `<title>Restore Drill — proof your Postgres backup
  restores</title>`.
- **Why this matters:** “proof” reads as a noun where the title needs a plain
  description.
- **Concrete fix:** “Restore Drill — prove your Postgres backup restores”.

#### F-1-40 — README sentence exceeds 22 words

- **Quote (29 words):** “It creates a disposable internal Docker network,
  restores a dump or volume archive, runs SQL and application HTTP assertions,
  writes a signed JSON report, and tears the environment down.”
- **Concrete rewrite:** “It restores your backup in a disposable Docker
  network. It runs SQL and HTTP checks, signs the result, then removes the test
  environment.”

#### F-1-41 — README exit-code sentence exceeds 22 words

- **Quote (24 words):** “Exit code `0` means every restore and assertion passed,
  `1` means the drill ran and failed, and `2` means configuration or
  prerequisites were invalid.”
- **Concrete rewrite:** “Exit code `0` means the drill passed. `1` means it ran
  and failed. `2` means the configuration or prerequisites were invalid.”

The following copy flags are separate findings:

| ID | Exact copy/location | Problem | Concrete rewrite |
| --- | --- | --- | --- |
| F-1-42 | H1: “A backup is a promise.” | Metaphor does not name the job out of context. | “Prove your Postgres backup restores.” |
| F-1-43 | H2: “Watch the proof assemble.” | “Proof” is abstract and the animation is not the product. | “See the checks in a restore report.” |
| F-1-44 | H2: “Small enough for cron. Strict enough to trust.” | Vague marketing claim and unexplained `cron`. | “Run four restore checks on a schedule.” |
| F-1-45 | H3: “Seal the room” | Metaphor hides the network step. | “Create an isolated Docker network.” |
| F-1-46 | H3: “Sign and clear” | “Clear” does not say what is removed. | “Sign the report and remove test resources.” |
| F-1-47 | H2: “One binary. Your Docker engine.” | Fragment does not describe an action or result. | “Install one binary on your Docker host.” |
| F-1-48 | Eyebrow: “Evidence, not optimism” | Slogan does not name the section. | “Signed restore report.” |
| F-1-49 | H2: “Turn a command into a recovery habit.” | Marketing phrasing hides the paid contents. | “Schedule drills and track four weekly results.” |
| F-1-50 | Button: “Copy” | The result is ambiguous out of context. | “Copy install command.” |
| F-1-51 | Button: “Have a license?” | A question is not a result-naming verb. | “Enter license token.” |
| F-1-52 | Button: “Run a 12-second preview” | It promises a duration, does not name sample data, and competes with Install. | “Try it with sample data.” |
| F-1-53 | Landing and README: `backup`, `dump`, `volume`, `archive`, and `artifact` for the input | The changing noun makes the safety boundary harder to follow. | Define “backup” once; use “dump” and “volume archive” only as its two formats. |
| F-1-54 | Landing: `CLI`, `cron`, `probe`, `RTO`, `SHA-256`, `Ed25519`; README additionally uses `PITR`, `stdout`, `stderr`, `env-file` | Unexpanded jargon slows a cold read; `RTO` is never defined. | Use “command-line tool”, “schedule”, and “check” in overview copy; expand “recovery time objective (RTO)” and keep protocol/format terms only beside examples. |
| F-1-55 | Team Kit bullet: “Ready-to-edit GitHub Actions and cron schedules” | “Ready-to-edit” is a marketing adjective and the offer cannot currently be tested. | “GitHub Actions workflow and weekly cron example.” |
| F-1-56 | `.factory/brief.json` has no `summary` field | The factory catalog has no auditable ≤120-character plain-words description. | Add `"summary": "Prove a Postgres backup restores in an isolated Docker network."` after claims tests cover it. |

## Copy audit

Counts use visible alphanumeric words; hyphenated terms count as one and slash-
separated terms as separate words. Code blocks are excluded because they are
commands/configuration, not sentences. No supplied banned word appears. The
only >22-word sentences are F-1-40 and F-1-41.

### Landing-page sentences and list statements

| Words | Copy |
| ---: | --- |
| 5 | A backup is a promise. |
| 3 | Test the restore. |
| 22 | Restore a Postgres dump or volume into a disposable Docker network, probe the database and app, then keep signed proof it worked. |
| 3 | Internal network required. |
| 3 | Production hosts rejected. |
| 3 | Backup artifact in. |
| 4 | Measured recovery evidence out. |
| 9 | This preview uses no server and touches no data. |
| 9 | The real CLI records the same checkpoints from Docker. |
| 10 | Pull images first, then create a new Docker `--internal` network. |
| 8 | Existing networks and undeclared HTTP hosts are refused. |
| 11 | Load a plain/custom Postgres dump or a validated volume tar. |
| 10 | The original stays read-only; secrets stay in local env files. |
| 16 | Run exact SQL expectations and app-level HTTP probes from inside the isolated network—not against production. |
| 16 | Write hashes, image IDs, outcomes, and recovery time to Ed25519-signed JSON; remove containers, volume, and network. |
| 4 | Build from source today. |
| 8 | Release binaries are attached to tagged GitHub releases. |
| 19 | A signed local report ties the exact artifact to the images, probes, and elapsed time used in the rehearsal. |
| 7 | Verify it later without a service account. |
| 6 | Ready-to-edit GitHub Actions and cron schedules. |
| 4 | Four-week recovery scorecard template. |
| 5 | Failure injection and retention checklist. |
| 5 | All future Team Kit revisions. |
| 12 | The core CLI, signed reports, safety checks, and JSON export stay free. |
| 7 | Sociobot/Dodo is the merchant of record. |
| 8 | Refunds are handled there and revoke the license. |
| 6 | No license stored on this device. |
| 3 | Team Kit unlocked. |
| 4 | Generate each template locally. |
| 1 | Offline. |
| 11 | The docs and last verified Team Kit license remain available on this device. |

### Landing headings, controls, labels, and dynamic states

| Words | Copy |
| ---: | --- |
| 4 | Skip to main content |
| 2 | Restore Drill |
| 3 | How it works |
| 1 | Evidence |
| 3 | Team Kit · $39 |
| 4 | Restore verification / v0.1 |
| 3 | Install the CLI |
| 4 | Run a 12-second preview |
| 4 | Recorded drill / local simulation |
| 4 | Watch the proof assemble. |
| 4 | Try a broken backup |
| 3 | The drill boundary |
| 8 | Small enough for cron. Strict enough to trust. |
| 3 / 3 / 2 / 3 | Seal the room / Restore the artifact / Probe usability / Sign and clear |
| 3 | Install / Linux x86-64 |
| 5 | One binary. Your Docker engine. |
| 1 / 3 / 3 | Copy / Read the source / Open the runbook |
| 3 | Evidence, not optimism |
| 7 | Know the recovery time before the incident. |
| 1 / 2 / 1 / 1 / 1 | Status / Recovery time / Artifact / Assertions / Signature |
| 3 / 7 | One-time operator upgrade / Turn a command into a recovery habit. |
| 9 | $39 once · yours for every environment your team operates |
| 5 / 3 / 2 | Buy the Team Kit · $39 / Have a license? / Verify license |
| 3 / 2 / 3 | Download CI workflow / Download scorecard / Download drill checklist |
| 6 | Restore Drill · Recovery proof for self-hosters. |
| 4 | Restore drill preview started. |
| 6 / 5 | Broken backup detected during isolated restore. / Later probes were not run. |
| 3 / 7 | Restore drill passed. / Five checkpoints complete in 12.044 seconds. |
| 2 / 1 / 2 | Select command / Copied / Checking license… |
| 5 | License verified on this device. |
| 4 / 8 | License no longer active. / You can purchase a new Team Kit license. |
| 2 / 6 | License verified. / Team Kit unlocked on this device. |
| 7 | Check the token or purchase a new license. |
| 6 / 5 | Offline—using the last verified license. / We will check again later. |
| 6 / 7 | Could not reach license verification. / Check your connection and try again. |
| 9 | Paste the full license token, then verify again. |

### README sentences and headings

| Words | Copy |
| ---: | --- |
| 2 | Restore Drill |
| 22 | Restore Drill is a Docker-first CLI for small teams that need evidence a Postgres backup restores—not another green “backup completed” log. |
| **29** | It creates a disposable internal Docker network, restores a dump or volume archive, runs SQL and application HTTP assertions, writes a signed JSON report, and tears the environment down. |
| 13 | It never performs production restores and does not upload backups, credentials, or reports. |
| 8 | Restore Drill is MIT-licensed and has no telemetry. |
| 1 | Install |
| 12 | Download a release binary, or build from source with Rust 1.85+: |
| 11 | Docker Engine must be installed and reachable by the current user. |
| 12 | Postgres and probe images are pulled before the isolated network is created. |
| 1 | Usage |
| 9 | Create a starter configuration and a gitignored credential file: |
| 7 | Run the drill from cron or CI. |
| 5 | There are no interactive prompts: |
| 12 | `--json` writes one machine-readable result to stdout; human progress goes to stderr. |
| **24** | Exit code `0` means every restore and assertion passed, `1` means the drill ran and failed, and `2` means configuration or prerequisites were invalid. |
| 3 | Minimal `restore-drill.toml`: |
| 11 | For a physical volume archive created with `tar`, select `volume_tar`: |
| 15 | The archive must contain a Postgres data directory compatible with the selected image major version. |
| 9 | Restore Drill rejects symlinks and path traversal before extraction. |
| 12 | HTTP probes run from a curl container on Docker's `--internal` network. |
| 17 | Their hostnames must exactly match a declared service; public, loopback, and production URLs are rejected during validation. |
| 17 | Secrets are passed to Docker with `--env-file`, never put on the command line or copied into reports. |
| 18 | Reports include recovery duration, assertion evidence, the SHA-256 backup hash, the resolved image IDs, and an Ed25519 signature. |
| 5 | Verify a report later with: |
| 13 | The public key is saved beside the signing key as `signing.key.pub`. |
| 22 | Retain it separately and pass `--public-key` for independent signer verification; without that option, `verify` checks only that the report is internally intact. |
| 1 | Scheduling |
| 4 | Example weekly cron entry: |
| 12 | Run once manually first and keep the reports outside ephemeral CI storage. |
| 17 | A deliberately truncated dump should produce exit code `1`; test that failure path before trusting the schedule. |
| 3 | Develop and verify |
| 10 | `npm test` runs Rust unit/integration tests plus landing-page checks. |
| 17 | `npm run build` produces the binary in `dist/bin/` and the deployable static site in `dist/site/`. |
| 8 | Run the site locally with `npm run dev`. |
| 3 | Scope and safety |
| 14 | Restore Drill supports Postgres dump files and tarred data volumes plus optional application containers. |
| 17 | It does not make backups, orchestrate PITR, operate on an existing network or container, or restore production. |
| 15 | Docker resources use the `restore-drill` label and are removed after success, failure, timeout, or Ctrl-C. |
| 12 | `--keep-on-failure` exists for local diagnosis and prints the exact resources left behind. |
| 10 | Security reports are welcome through GitHub's private vulnerability reporting. |
| 14 | See the website's privacy and terms pages for the one-time Team Kit purchase. |

## Demo and sandbox evidence

- Fresh browser storage before preview: no local storage, session storage, or
  cookies. After a healthy preview: still none.
- The first screen after the click does show a realistic-looking `weekly-orders`
  result with hash, 8.4 GB restore, SQL, HTTP, signature, and 12.044-second
  values. Those values are hard-coded in `site/src/main.ts`; no backup is used.
- Healthy and broken animations reach Passed/Failed. There is no reset control;
  re-running merely rewrites the same in-memory rows.
- The preview made no external request. A fresh online load followed by offline
  reload succeeded from the service worker with only same-origin requests.
- `/demo` is not cached or routed. The 404 itself loads third-party Microsoft
  CDN scripts/styles/images.
- CLI demo command in a temporary directory: exit 2, unrecognized subcommand.

## Claims and clean-clone gates

Because `.factory/claims.json` is absent, **zero claims were listed and zero
claim-tagged tests could be run**. This is an untested-claim failure even though
the general suite passes.

From a separate `git clone --no-local /work/repo` at the candidate commit:

- `npm ci`: pass; 0 vulnerabilities reported by npm.
- `npm test`: pass; 7 Rust tests, 3 response-policy tests, and Playwright 9
  passed / 1 intentionally skipped duplicate mobile axe test.
- `npm run check:types`: pass.
- `npm run check:lint`: pass.
- `npm run build`: pass; produced `dist/bin/restore-drill-linux-x86_64` and
  `dist/site/`.
- Live built `index.html`, JavaScript, and hero hashes match the clean build.

These tests rely on a fake Docker-compatible executable for the restore path and
do not cure F-1-34.

## History audit

No earlier `.factory/review-*.md` or `.factory/polish-*.md` exists.

- The response-header defect in `.factory/verification.md` is genuinely fixed:
  live responses now include CSP, Permissions-Policy, frame denial, nosniff,
  referrer policy, and the tested cache rules. It is not repeated.
- The handoff's real-Docker operational gap remains open and is repeated as
  F-1-34.
- The handoff's billing-registration gap remains open and is repeated as
  F-1-32.

## Structure, accessibility, privacy, and links

Confirmed passes:

- Home, Privacy, and Terms return 200, set `lang="en"`, have a route-specific
  title, canonical URL, meta description, one h1, and one main landmark.
- Home has one meaningful image alt; no console/page errors occurred.
- `robots.txt` and `sitemap.xml` exist and list the three working routes.
- Home links to GitHub, Privacy, and Terms return 200; all in-page targets exist.
- Browser back restores the pre-hash URL and scroll position.
- First Tab focuses the skip link; keyboard preview controls work.
- Live Playwright axe scans at 390 × 844 and 1440 × 900 found zero WCAG 2 A/AA
  or 2.1 AA violations.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, title/lang/main/alt/button
  checks, and zero console errors.
- The warm-paper, forest-ink, vermilion, halftone field-report identity is
  recognizably product-specific and matches `.factory/design.md`; it is not a
  generic SaaS template.

Failures are F-1-32 through F-1-39. The complete crawl also found the checkout
404; `/demo` and an arbitrary unknown path return the generic 404.

## What would make this perfect

Nothing is optional for a clean next review: replace the scripted preview with
the shipped one-command sandbox; write and run every claim test on real Docker;
repair checkout and 404 routing; rewrite the first screen and flagged copy;
complete metadata and consistent route chrome; fix route focus and touch
targets; then rerun this entire checklist from a fresh browser and clean clone.
A perfect result has no open row above and no public sentence without a passing
claim test.
