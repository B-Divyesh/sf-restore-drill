# Adversarial first-read review 2 — Restore Drill

**Verdict: FAIL**

**Reviewed:** 2026-08-28 UTC

**Candidate:** `07f624e2eee54bd2eeb1d8119c62df0bb6a23e4d`

**Live URL:** <https://restore-drill.sociobot.in/>

The cold landing screen is clear and the declared test commands pass. The
product still has blocking demo, real-Docker evidence, claim coverage,
accessibility, metadata, and setup-link failures. PASS requires zero findings.

## Cold first screen

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 with no site
data. Nothing was scrolled before this assessment.

| Question | 390 px | Desktop |
| --- | --- | --- |
| What does it do? | It checks whether a Postgres backup can be restored. | Same. The chamber image reinforces an isolated test. |
| For whom? | Small teams that host Postgres themselves. | Same. |
| What should I click first? | **Try it with sample data**. | **Try it with sample data**. |

The exact first-screen copy that answers those questions is “Prove your
Postgres backup restores.”, “For small self-hosted teams that need recovery
proof before an outage.”, and “Try it with sample data”. This part passes.

## Findings

### Blocking

#### F-1-2 — The supposed one-click demo is still a fixed transcript

- **Location/quote:** landing action “Try it with sample data”; action note
  “Opens a safe sample drill and its signed report.”; `/demo/?demo=1` static
  `<pre>` ending “✓ passed in 3.24s”.
- **Evidence:** one click opens a prewritten HTML transcript. It does not run or
  replay the binary, and it does not show or link the signed JSON report. Reset
  reloads the same immutable page. “Start for real” returns to `/` instead of an
  install step. The CLI-specific demo contract requires a self-hosted recording
  of the real binary as well as the bundled `restore-drill demo` command.
- **Why this fails:** the first screen after the click looks like a result but
  provides no way to distinguish recorded output from invented output. A
  visitor cannot inspect the promised report or experience a state that Reset
  meaningfully resets. This is the same half-fixed demo failure from review 1.
- **Concrete fix:** record a successful real-Docker `restore-drill demo` run,
  self-host the terminal playback, expose its generated sample report for
  inspection, make Reset restart the playback, and point Start for real to a
  working installation section. Rewrite the note to “Replays a sample restore
  and opens its signed report.”

#### F-1-34 — The real restore job remains unverified on Docker/Postgres

- **Location:** `.factory/handoff.md`; `crates/restore-drill/tests/cli.rs` test
  `demo_uses_shipped_sample_in_a_fresh_temporary_directory`.
- **Evidence:** the handoff still states that no Docker executable, daemon, or
  socket was available. The claim test replaces Docker with a shell script that
  returns `3` for the SQL query without loading Postgres or executing
  `sample-backup.sql`. In this review, the built CLI was invoked from a fresh
  temporary directory and exited `2`: “could not start Docker CLI 'docker': No
  such file or directory”.
- **Why this fails:** the product's job is to prove a real backup restores. A
  scripted process boundary verifies command composition, not Docker network
  isolation, Postgres readiness, SQL restoration, or cleanup by a daemon. This
  exact operational gap was blocking in review 1 and is still open.
- **Concrete fix:** add a Docker-enabled claim test that runs the bundled sample
  against real Postgres, checks the three rows, verifies the report signature,
  and confirms zero labelled resources after success. Add a corrupt-dump run
  that confirms exit `1`, a signed failed report, and cleanup.

#### F-2-1 — The mobile demo evidence cannot be reached by keyboard

- **Location:** `/demo/?demo=1`, the horizontally scrollable `<pre>` at 390 px.
- **Evidence:** Axe 4.10.2 reports serious rule
  `scrollable-region-focusable`: “Element should be focusable.” The transcript
  is visibly clipped on the right and contains no focusable descendant.
- **Why this fails:** a keyboard user cannot reach or horizontally scroll the
  demo's only result evidence. The repository test skips the mobile Axe pass as
  “identical”, but responsive overflow makes the mobile document materially
  different.
- **Concrete fix:** make the transcript focusable with an accessible label and
  visible focus state, or wrap long lines without losing their meaning. Run Axe
  at both desktop and 390 px without skipping mobile.

#### F-2-2 — The no-tracking claim test omits a declared sandbox assertion

- **Location:** `.factory/claims.json` `site-no-tracking`; live `/privacy/`;
  `tests/site/site.spec.ts:13`.
- **Evidence:** the registry says the test will assert no service-worker
  registration before closing the page. The test never calls
  `getRegistrations()` and only checks that the IndexedDB API is available,
  rather than checking for databases. It runs on localhost, where `main.ts`
  deliberately suppresses service-worker registration. A fresh live privacy
  visit registers `/sw.js` and creates Cache Storage
  `restore-drill-shell-v2`.
- **Why this fails:** the listed command is green because it avoids the deployed
  behavior that its sandbox says it checks. The tracking claim itself may be
  true, but the declared evidence is not.
- **Concrete fix:** either remove the incorrect no-service-worker condition
  from the sandbox or test the production-equivalent registration explicitly.
  Enumerate IndexedDB and Cache Storage, assert every request is same-origin,
  and state that the first-party offline cache is permitted and contains only
  public site assets.

#### F-1-22 — README output and exit-code claims remain outside the registry

- **Location/quote:** README: “`--json` writes the final result to standard
  output. Progress goes to standard error. Exit code `0` means the drill
  passed. `1` means it ran and failed. `2` means the configuration or
  prerequisites were invalid.”
- **Evidence:** none of the four `.factory/claims.json` entries covers these
  observable contracts. General CLI tests do not replace the required tagged
  claim entry.
- **Why this fails:** automation can depend on these channels and exit codes.
  The same claim-coverage finding from review 1 was shortened, not completed.
- **Concrete fix:** add one registry entry and one tagged test that exercises
  success, drill failure, and invalid configuration; parse stdout as exactly
  one JSON value and assert progress is confined to stderr.

#### F-1-24 — The retained host-rejection claim is only partially tested

- **Location/quote:** README: “The command rejects HTTP checks that target
  public or undeclared hosts.”
- **Evidence:** `@claim:production-boundary` tries only
  `https://api.example.com/health` and passes because HTTPS is disallowed. It
  does not try a public `http://` URL, loopback, or an undeclared internal host.
- **Why this fails:** the test can pass without proving the two boundaries the
  README names. This is the retained part of review-1 finding F-1-24.
- **Concrete fix:** make the tagged test table-driven over public HTTP,
  loopback, undeclared internal, and declared internal targets. Confirm every
  rejected case fails before Docker is invoked.

#### F-1-27 — The documented distribution build is an unlisted claim

- **Location/quote:** README: “`npm run build` writes the Linux binary to
  `dist/bin/` and the static site to `dist/site/`.”
- **Evidence:** the command passed in this review and produced both paths, but
  `.factory/claims.json` has no entry for this public distribution promise.
- **Why this fails:** the outcome is currently tested only by reviewer habit,
  not by the claim registry the README tells releasers to trust. Review 1 asked
  for registration; polish 1 did not add it.
- **Concrete fix:** register `distribution-build` and add a tagged test that
  runs the build and checks an executable binary plus the required site files.

#### F-1-36 — Route metadata is still incomplete

- **Location:** live `/demo/`, `/privacy/`, `/terms/`, and the designed 404.
- **Evidence:** `/demo/` lacks `og:url`. Privacy, Terms, and 404 lack `og:type`
  and `og:url`, and expose only `twitter:card` without Twitter title,
  description, or image. Home is complete.
- **Why this fails:** review 1 required route metadata on every page. The polish
  added only part of it, so the same finding is blocking again under the review
  history rule.
- **Concrete fix:** add route-specific `og:type`, `og:url`, `twitter:title`,
  `twitter:description`, and `twitter:image` to every route and test their
  values, not merely the presence of `og:image`.

#### F-2-3 — Both setup links land on nonexistent GitHub sections

- **Location/quote:** home “Read the setup guide (opens GitHub)” links to
  `#usage`; demo “Install and run the demo (opens GitHub)” links to `#install`.
- **Evidence:** GitHub returns 200 for the repository page, but its README has
  neither `usage` nor `install` anchor. The relevant current heading is
  `#try-the-shipped-sample`. GitHub has no releases, and the shown
  `cargo install --path crates/restore-drill` command only works after a source
  checkout that the site never instructs the visitor to make.
- **Why this fails:** the only path from the web walkthrough to a real run does
  not reach the promised instructions. This is broken routing on the primary
  product path.
- **Concrete fix:** link both actions to a real stable anchor. Give complete
  copy-paste steps (`git clone`, `cd`, install, demo), or publish a tagged binary
  and link the exact asset with checksum verification.

#### F-2-4 — The Terms page says binaries are available when none are published

- **Location/quote:** `/terms/`: “Restore Drill source and binaries are
  available under the MIT License in this repository.”
- **Evidence:** the repository contains source but no tracked distribution
  binary. The GitHub releases API returned an empty list and `/releases/latest`
  returned 404.
- **Why this fails:** a visitor is told a binary is available but can only build
  one from source.
- **Concrete fix:** publish signed release binaries and link them, or rewrite to
  “Restore Drill source is available under the MIT License in this repository.”

### Non-blocking

#### F-2-5 — The demo makes an unlisted fixed performance claim

- **Location/quote:** `/demo/`: “✓ passed in 3.24s”.
- **Why this matters:** this looks like measured output, but it is hard-coded
  HTML and no claim entry measures 3.24 seconds. Restore time depends on the
  host and image cache.
- **Concrete fix:** label the transcript “example output” and replace the number
  with a variable such as `<measured time>`, or add a bounded quantitative claim
  and measure it in the stated sandbox.

#### F-2-6 — File and network behavior on the Privacy page is unlisted

- **Location/quotes:** `/privacy/`: “The command reads the backup,
  configuration, and credential files you name.”; “It writes reports and
  signing keys to paths in your configuration.”; “This static site does not
  send your backup, configuration, report, or credentials to Restore Drill.”
- **Why this matters:** these are concrete privacy boundaries that users can
  rely on, but no registry entry checks filesystem access or canary data in
  outbound requests.
- **Concrete fix:** add a tagged local-I/O/privacy test with sentinel files and
  an intercepted network. Assert only named inputs are read, only configured or
  demo-temporary paths are written, and canary contents never leave the host.

#### F-2-7 — Image-pull behavior is an unlisted claim

- **Location/quote:** `/demo/`: “Docker pulls the sample image if it is
  missing.”
- **Why this matters:** the demo claim test makes `docker image inspect` succeed,
  so it never observes the promised missing-image branch.
- **Concrete fix:** add a claim entry whose harness fails the first inspect,
  records `docker pull postgres:16-alpine`, then permits the second inspect.

#### F-2-8 — Full-page navigation does not move focus or announce the route

- **Location:** home → Demo and browser Back.
- **Evidence:** after activating the sample link, the URL and title change but
  `document.activeElement` is `<body>` and `#route-announcement` is empty. Back
  returns home with the same empty announcement. Hash navigation to `#how` does
  focus and announce correctly.
- **Why this matters:** keyboard and screen-reader users receive no explicit
  route-change context.
- **Concrete fix:** focus the destination `<h1>` after page load/navigation and
  announce its text. Add forward/back tests in addition to the existing hash
  test.

#### F-2-9 — Demo terminology overstates what the browser page does

- **Location/quotes:** landing uses “sample drill”; README calls the same URL a
  “browser walkthrough”; the demo heading says “Run a sample Postgres restore.”
- **Why this matters:** “run”, “drill”, and “walkthrough” describe different
  levels of interaction. The unqualified adjective “safe” is also unsupported
  by the web page itself.
- **Concrete fix:** use “web walkthrough” consistently until the page replays a
  verified run. Reserve “demo command” for `restore-drill demo` and remove
  “safe” unless its exact boundary is tested.

#### F-2-10 — The recurring-drill job has no ready scheduling path

- **Location:** brief success measure (“A weekly drill succeeds unattended for
  four weeks”); README and repository.
- **Why this matters:** the CLI can emit JSON, but the product supplies no
  cron/systemd timer or CI workflow that runs the command weekly and preserves
  its report. Recurrence is the obvious next action after the first drill.
- **Concrete fix:** ship one tested, provider-neutral cron or systemd example
  and one GitHub Actions example. Each must preserve the signed report, expose a
  failing exit code, and avoid embedding credentials. AI is not appropriate for
  this deterministic recovery check.

## Demo and sandbox checks

- Landing → demo takes one click and returns HTTP 200 at
  `/demo/?demo=1`.
- The first demo screen has three realistic fictional orders, an internal
  network configuration summary, and sample console output.
- The banner is present. Reset reloads `/demo/?demo=1`; Start for real returns
  to `/`.
- Fresh browser contexts had no cookies, localStorage, sessionStorage, or
  IndexedDB databases. The site did register its first-party service worker and
  populate Cache Storage, as detailed in F-2-2.
- All observed browser requests were same-origin. After the service worker took
  control, the demo reloaded successfully with the browser offline.
- The built CLI was run from a fresh temporary directory. It created its own
  `/tmp/restore-drill-demo-*` directory, then exited `2` because this worker has
  no Docker executable or socket. No file appeared in the caller directory.
- The browser demo has no editable data and therefore cannot touch user data,
  but it also has no meaningful state for Reset to clear.

## Claims audit

All commands below ran from clean clone
`/tmp/restore-drill-review2-clean-oGI5ma/repo` at the candidate commit.

| Claim | Command | Result | Evidence |
| --- | --- | --- | --- |
| `demo-isolated` | `cargo test --workspace demo_uses_shipped_sample_in_a_fresh_temporary_directory` | PASS | 1 test passed; uses a shell-script Docker substitute. F-1-34 remains. |
| `production-boundary` | `cargo test --workspace rejects_production_http_host` | PASS | 1 test passed; only an HTTPS public host is tried. F-1-24 remains. |
| `mit-license` | `node --test tests/claims.test.mjs` | PASS | LICENSE text and Cargo metadata matched MIT. |
| `site-no-tracking` | `npm run test:site -- --grep @claim:site-no-tracking` | PASS | 2 browser projects passed, but the declared service-worker/IndexedDB sandbox is not asserted. F-2-2 remains. |

No listed command failed. Unlisted or under-tested public claims are recorded in
F-1-22, F-1-24, F-1-27, F-1-34, and F-2-5 through F-2-7. The broad landing and
README promise that Restore Drill proves a restore is not accepted as tested by
a fake process that never starts Postgres.

## Structure, links, visual identity, and accessibility

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown path returns
  the designed Restore Drill 404 with HTTP 404, one h1, main, header, footer,
  and return links.
- Every HTML route has `lang=en`, one h1, a main landmark, a description,
  canonical URL, favicon, touch icon, and local OG image. Metadata gaps are in
  F-1-36.
- Every same-origin link was crawled. Documents and in-page targets resolve;
  the two dead GitHub fragments are in F-2-3.
- The live CSP, Referrer-Policy, X-Content-Type-Options, frame protection, and
  permissions policy are present. No console or request error occurred on the
  200 routes. The expected 404 document request appears as a browser console
  resource error because its HTTP status is 404.
- The homepage has no horizontal overflow or sub-44 px visible controls at
  desktop or 390 px. Axe found no issues on home, Privacy, Terms, or 404. The
  mobile demo failure is F-2-1.
- The landing page JavaScript is 1.85 kB raw (0.92 kB gzip), below the budget.
- The halftone field-report identity, asymmetric chamber illustration, paper/
  ink/vermilion palette, local system type, and restrained motion are distinct
  from a generic SaaS template and match `.factory/design.md`. Reduced-motion
  rules are present.
- `/opt/fleet/lib/verify-url.sh` passed for the live home page: title, `lang`,
  h1, main, alt text, button labels, and console checks all passed.

## Copy audit

Counts use visible alphanumeric words. Hyphenated terms, URLs, paths, and code
tokens count as one. Commands in fenced code blocks are not sentences. Labels,
headings, buttons, list items, and alt text are included so the audit does not
hide fragments. No unit exceeds 22 words and none contains a word from the
plain-words banned list.

### Landing page

| Words | Exact copy |
| ---: | --- |
| 2 | Restore Drill |
| 1 | Demo |
| 3 | How it works |
| 1 | Privacy |
| 3 | Postgres restore rehearsal |
| 5 | Prove your Postgres backup restores. |
| 11 | For small self-hosted teams that need recovery proof before an outage. |
| 5 | Try it with sample data |
| 9 | Opens a safe sample drill and its signed report. |
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
| 1 / 2 | Sample / 3 orders |
| 1 / 1 | Network / internal |
| 1 / 3 | Output / signed JSON report |
| 1 / 2 | Command / restore-drill demo |
| 2 / 6 | Restore Drill / Recovery proof for self-hosted Postgres teams. |
| 1 / 1 / 1 / 4 | Demo / Privacy / Terms / Built by Param Factory |

Copy flags: “safe” is an unproved adjective; “signed JSON report” is specialist
language without a visible report; and “demo”, “drill”, “rehearsal”, and
“walkthrough” are inconsistent for the browser experience. These map to
F-1-2 and F-2-9. Every action control names a result or required demo action;
all headings make sense out of context.

### README

| Words | Exact copy |
| ---: | --- |
| 2 | Restore Drill |
| 14 | Restore Drill is for small self-hosted teams that need recovery proof before an outage. |
| 11 | It rehearses a Postgres backup restore in an isolated Docker network. |
| 10 | It records a signed JSON report after the checks finish. |
| 4 | Try the shipped sample |
| 11 | Docker Engine must be installed and reachable by the current user. |
| 14 | `demo` copies the shipped SQL sample and configuration to a fresh system temporary directory. |
| 8 | It prints that directory when the drill completes. |
| 12 | The sample contains three fictional orders and checks that all three restore. |
| 7 | The browser walkthrough is available at `https://restore-drill.sociobot.in/demo/?demo=1`. |
| 4 | Run your own drill |
| 8 | Create a starter configuration and local credential file: |
| 11 | Use a copied backup and review the configuration before running it. |
| 11 | The command rejects HTTP checks that target public or undeclared hosts. |
| 11 | Keep the report and public signing key outside temporary CI storage. |
| 8 | `--json` writes the final result to standard output. |
| 5 | Progress goes to standard error. |
| 7 | Exit code `0` means the drill passed. |
| 6 | `1` means it ran and failed. |
| 8 | `2` means the configuration or prerequisites were invalid. |
| 1 | Commands |
| 7 | Run `restore-drill --help` for each command's options. |
| 4 | Develop, test, and deploy |
| 15 | `npm run build` writes the Linux binary to `dist/bin/` and the static site to `dist/site/`. |
| 16 | The factory deploys `dist/site/` as a static site; workers do not change DNS, infrastructure, or billing. |
| 5 | The claim registry is `.factory/claims.json`. |
| 10 | Run every listed command from a clean clone before release. |
| 3 | License and policies |
| 4 | Restore Drill is MIT-licensed. |
| 5 | See LICENSE, Privacy, and Terms. |

Copy flags: “CI” is unexplained jargon; rewrite it as “continuous-integration
(CI) storage” on first use. “Signed JSON report” is clearer as “tamper-evident
report file (JSON)” for a first read. The substantive README claim gaps are
F-1-22, F-1-24, F-1-27, and F-1-34. No README sentence exceeds 22 words. The
headings work out of context; the code blocks are commands rather than copy.

## Earlier finding verification

Each review-1 finding was checked against the live site and current code.

| Finding | Result in this review |
| --- | --- |
| F-1-1 | Fixed: both cold first screens identify job, audience, and one action. |
| F-1-2 | **Half-fixed; BLOCKING again:** route/banner/CLI command exist, but the web demo is a fixed transcript with no report. |
| F-1-3 | Fixed: registry exists and all four commands run. Coverage defects are separately identified. |
| F-1-4 | Fixed for the old copy; the broad real-restore promise is covered by F-1-34. |
| F-1-5 | Fixed for the narrowed production-host wording; test breadth is reissued as F-1-24. |
| F-1-6 | Old README claim removed; new privacy statements are F-2-6. |
| F-1-7 | Fixed: old image-order sentence removed. |
| F-1-8 | Fixed: old existing-network sentence removed. |
| F-1-9 | Fixed: old multi-format landing/README claim removed. |
| F-1-10 | Fixed: old read-only/secrets marketing sentence removed. |
| F-1-11 | Fixed: old SQL/HTTP scope sentence removed. |
| F-1-12 | Fixed for sample scope; registered sample test checks signature and cleanup commands. |
| F-1-13 | Fixed: old browser no-data preview claim removed. |
| F-1-14 | Fixed: checkpoint-equivalence sentence removed. |
| F-1-15 | Fixed: release-binary sentence removed; the new Terms claim is F-2-4. |
| F-1-16 | Fixed: broad later-verification claim removed. |
| F-1-17 | Fixed: paid offer removed. |
| F-1-18 | Fixed: free-tier promise removed. |
| F-1-19 | Fixed: merchant/refund promise removed. |
| F-1-20 | Fixed: license/offline-entitlement promise removed. |
| F-1-21 | Fixed: image-order prerequisite promise removed. |
| F-1-22 | **Half-fixed; BLOCKING again:** copy is shorter, but claims remain unregistered. |
| F-1-23 | Fixed: archive-compatibility marketing detail removed. |
| F-1-24 | **Half-fixed; BLOCKING again:** host claim remains and its test covers only HTTPS rejection. |
| F-1-25 | Fixed: detailed report-schema/public-key promise removed. |
| F-1-26 | Fixed: truncated-dump promise removed. |
| F-1-27 | **Half-fixed; BLOCKING again:** build passes, but the README claim is still unregistered. |
| F-1-28 | Fixed: overview scope reduced to implemented commands/sample. |
| F-1-29 | Fixed only for the registered sample scope; real-daemon proof remains F-1-34. |
| F-1-30 | Fixed: no behavioral promise remains; option is listed as syntax only. |
| F-1-31 | Fixed: MIT claim and packaging test pass. |
| F-1-32 | Fixed: dead paid path removed. |
| F-1-33 | Fixed: `/demo/` works and unknown paths use the designed 404. |
| F-1-34 | **Unfixed; BLOCKING again:** no real Docker/Postgres run exists. |
| F-1-35 | Fixed: shared header/footer/legal links/build label appear on all HTML routes. |
| F-1-36 | **Half-fixed; BLOCKING again:** assets exist, but route social metadata is incomplete. |
| F-1-37 | Fixed for hash navigation; full-route focus is a new F-2-8. |
| F-1-38 | Fixed: visible controls meet 44 × 44 px; the demo has a different keyboard defect in F-2-1. |
| F-1-39 | Fixed: home title uses “prove”. |
| F-1-40 | Fixed: old 29-word sentence was split. |
| F-1-41 | Fixed: old 24-word exit sentence was split. |
| F-1-42 | Fixed: job-based h1 is live. |
| F-1-43 | Fixed: old abstract preview heading removed. |
| F-1-44 | Fixed: concrete four-check heading is live. |
| F-1-45 | Fixed: internal-network heading is live. |
| F-1-46 | Fixed: report/resource heading is explicit. |
| F-1-47 | Fixed: install heading names the action and host. |
| F-1-48 | Fixed: safety section uses “What this does not do”. |
| F-1-49 | Fixed: paid habit section removed. |
| F-1-50 | Fixed: ambiguous Copy button removed. |
| F-1-51 | Fixed: license button removed. |
| F-1-52 | Fixed: primary action says “Try it with sample data”. |
| F-1-53 | Fixed for backup input terminology; browser-demo terminology is F-2-9. |
| F-1-54 | Fixed in overview copy; remaining technical terms fit the CLI audience except unexplained “CI”. |
| F-1-55 | Fixed: paid bullet removed. |
| F-1-56 | Fixed: brief summary and catalog description are present and verb-first. |

## Quality-gate evidence

From the clean clone:

- `npm test`: PASS — 4 Rust unit tests, 4 Rust CLI tests, license and response
  policy tests, then 13 Playwright passes and 1 intentional mobile Axe skip.
- `npm run build`: PASS — executable at
  `dist/bin/restore-drill-linux-x86_64` and deployable `dist/site/`.
- `npm run check:types`, `npm run check:lint`, and
  `cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty`:
  PASS.
- Live home verification script: PASS.
- Live cross-route Axe: one serious failure on mobile Demo (F-2-1); all other
  checked route/viewport combinations had zero violations.
- Live link crawl: internal documents and anchors resolve; F-2-3 records the
  two dead external fragments.

## What would make this perfect

Nothing should remain implicit or simulated. Provide a real-Docker terminal
recording and inspectable signed sample report, publish a complete install path,
run the healthy and corrupt samples on Docker/Postgres, register and adequately
test every retained claim, fix mobile transcript focus and full-route focus,
complete metadata on every route, and ship a tested weekly scheduling example.
Then repeat this entire review from fresh browser and clean-clone contexts; PASS
requires zero findings.
