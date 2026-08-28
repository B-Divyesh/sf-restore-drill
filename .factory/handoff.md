# Restore Drill review 1 handoff

## Outcome

Adversarial first-read review 1 is complete. Verdict: **FAIL**.

The full report is `.factory/review-1.md`. Product code was not modified.

## What was done

- Audited the live first screen in fresh Chromium contexts at 390 × 844 and
  1440 × 900.
- Audited every landing-page and README sentence, heading, and control against
  the supplied plain-words rules, with word counts and concrete rewrites.
- Exercised the healthy/broken preview, storage isolation, same-origin network
  behavior, service-worker offline reload, `/demo`, and the CLI demo entry from
  a temporary directory.
- Cross-checked all public claims. `.factory/claims.json` and `@claim` tests are
  absent, so all retained claim groups are recorded as unlisted findings.
- Read the earlier handoff and both verification reports; confirmed the response
  policy repair and repeated the still-open billing and real-Docker gaps.
- Crawled all landing links and checked titles, descriptions, canonicals,
  landmarks, route chrome, 404 behavior, social metadata, keyboard focus, touch
  targets, outbound requests, and visual identity.
- Ran the complete quality gates from a separate clean clone.

## Verification evidence

Clean clone at `4f84762554e3b693a064578a29d7178a0b93c211`:

```sh
npm ci
npm test
npm run check:types
npm run check:lint
npm run build
```

All commands passed. The test result was 7 Rust tests, 3 policy tests, and 9
Playwright passes with 1 intentional duplicate-mobile axe skip. The build
produced `dist/bin/restore-drill-linux-x86_64` and `dist/site/`.

Additional live checks:

- `/opt/fleet/lib/verify-url.sh https://restore-drill.sociobot.in <temp-dir>`:
  pass.
- Playwright axe at mobile and desktop: zero WCAG 2 A/AA and 2.1 AA violations.
- Online load followed by offline reload: pass, same-origin requests only.
- On-page preview storage before/after: no local/session storage or cookies.
- `restore-drill demo` in a temporary directory: exit 2, unrecognized command.
- `/demo`: HTTP 404, generic Azure page.
- Team Kit checkout: HTTP 404 JSON response.
- Live production assets match the clean build hashes.

## Work left

Resolve every finding in `.factory/review-1.md`. The release blockers are the
unclear first screen, missing real demo/sandbox, absent claims registry and
claim-tagged tests, dead checkout, generic/broken demo and 404 routing, and lack
of a real Docker/Postgres end-to-end run. The report also records copy,
metadata, route consistency/focus, and mobile target-size findings.
