# Restore Drill — review 4 handoff

## Outcome

**FAIL.** The full adversarial review is in `.factory/review-4.md`.

No product code was changed. Review 4 found one blocking issue and two minor
issues:

- `F-4-1` reopens `F-3-1` / `F-1-34`: the registered real-Docker claim fails
  in the required clean sandbox because no Docker CLI or daemon is available.
- `F-4-2`: the 390 px header hides its product name and every navigation link
  without a replacement menu.
- `F-4-3`: “Run four checks before an outage” labels a list with only one
  actual check; the list describes four workflow steps.

## What was reviewed

- Cold live visits at 390 × 844 and 1440 × 900.
- One-click walkthrough, report, Reset, demo-only storage, non-demo sentinels,
  same-origin traffic, exit behavior, and offline reload.
- Every `.factory/claims.json` command from a clean clone.
- The complete landing and README copy, with sentence counts.
- Every earlier review, polish report, verification report, and handoff.
- Titles, h1/main structure, metadata, 404 behavior, links, deep links, Back,
  route focus/announcements, touch sizes, overflow, Axe, CSP, and visual identity.
- Missed leverage and embedded AI/provider-key checks.

## Verification

Clean clone: `/tmp/restore-drill-review4-clean-owHdAG/repo` at
`06246d0f5da6b5019541faa97bb7afcd2ffbb321`.

- Ten of eleven registered claim commands passed.
- `real-docker-restore` failed with `Docker CLI must be installed ... No such
  file or directory`.
- `npm test`, `npm run check:types`, `npm run check:lint`, and `npm run build`
  passed. The default test command ignores the separate real-Docker test.
- `npm run test:live -- https://restore-drill.sociobot.in/
  /tmp/restore-drill-review4-live` passed.
- `/opt/fleet/lib/verify-url.sh https://restore-drill.sociobot.in/
  /tmp/restore-drill-review4-verify` passed.

## Next steps

1. Run the exact `real-docker-restore` claim on a clean review worker with a
   usable Docker daemon and retain its healthy, corrupt, signature, and cleanup
   evidence.
2. Provide an accessible mobile header menu or keep the essential links visible.
3. Change the landing heading to “Rehearse a restore in four steps.”
4. Repeat the entire review from fresh browser and clean-clone contexts.
