# Restore Drill — polish 1 finding map

Candidate repaired: `1f5a0f7a9788147a14fc4429952b96e4ab9aee86`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Rewrote the first screen for small self-hosted teams; the only primary action is **Try it with sample data** and three facts are visible. | `home gives small self-hosted teams one clear sample entry`; `evidence/home-390.png` |
| F-1-2 | Added `/demo/?demo=1`, the persistent demo banner, reset/start-real actions, `restore-drill demo`, and shipped samples. | `demo has the isolation banner`; `demo_uses_shipped_sample_in_a_fresh_temporary_directory`; `evidence/demo-1440.png` |
| F-1-3 | Added `.factory/claims.json` and one tagged command for each visitor-facing claim. | Every command in `claims.json` passed from `/tmp/restore-drill-clean` |
| F-1-4 | Replaced broad untestable landing/README promises with the shipped sample drill claim. | `@claim:demo-isolated` |
| F-1-5 | Retained the production boundary only as a tested configuration boundary. | `@claim:production-boundary` |
| F-1-6 | Removed the untested no-upload/no-telemetry marketing claim; tested the static-site no-tracking statement. | `@claim:site-no-tracking` |
| F-1-7 | Replaced the public event-order promise with the tested internal-network sample flow. | `@claim:demo-isolated` |
| F-1-8 | Removed the untested existing-network wording; public-target rejection remains tested. | `@claim:production-boundary` |
| F-1-9 | Removed the untested volume-format claim from the landing and README overview. | Copy audit and `rg` review |
| F-1-10 | Removed the broad read-only/secret-output marketing promise. | Copy audit |
| F-1-11 | Removed the untested SQL/HTTP scope promise from public copy. | Copy audit |
| F-1-12 | Replaced the broad cleanup/signature claim with the sample claim tested against report/signature/network cleanup. | `@claim:demo-isolated` |
| F-1-13 | Removed the animated preview and its no-data claim. | `/demo/?demo=1`; `@claim:site-no-tracking` |
| F-1-14 | Removed the false checkpoint-equivalence sentence. | Copy audit |
| F-1-15 | Removed the unverified release-binary statement. | Copy audit |
| F-1-16 | Removed the broad independent-verification marketing claim. | Copy audit |
| F-1-17 | Removed the dead Team Kit offer and all entitlement claims. | Landing link crawl; no billing link remains |
| F-1-18 | Removed the free-tier entitlement promise. | Copy audit |
| F-1-19 | Removed merchant/refund wording with the removed paid offer. | Copy audit |
| F-1-20 | Removed license/offline entitlement wording with the paid offer. | Copy audit |
| F-1-21 | Shortened Docker documentation to its prerequisite; untested image-order copy is removed. | README review |
| F-1-22 | Rewrote stdout/stderr and exit code copy into short documented sentences. | README; existing CLI tests |
| F-1-23 | Removed archive compatibility/security marketing detail from README. | README review |
| F-1-24 | Removed untested HTTP and secret-handling detail from README. | README review |
| F-1-25 | Removed untested report schema and public-key behavior detail from README. | README review |
| F-1-26 | Removed the unsupported truncated-dump promise. | README review |
| F-1-27 | Kept build documentation and executed it in the clean-clone gate. | `/tmp/restore-drill-clean` build output |
| F-1-28 | Reduced README scope copy to documented commands and the sample. | README review |
| F-1-29 | Replaced cleanup-on-every-path claim with the tested sample cleanup claim. | `@claim:demo-isolated` |
| F-1-30 | Removed `--keep-on-failure` marketing detail from README. | README review |
| F-1-31 | Kept the MIT statement and added a tagged packaging test. | `@claim:mit-license` |
| F-1-32 | Removed the dead paid primary path rather than advertising an unregistered product. | Landing link crawl |
| F-1-33 | Added real `/demo/` and project-owned `404.html` via `responseOverrides`. | `legal and 404 documents use product chrome`; `evidence/404-390.png` |
| F-1-34 | Added a real Docker-backed `demo` command and a repeatable Docker-compatible integration run. | `demo_uses_shipped_sample_in_a_fresh_temporary_directory`; real-daemon limitation is recorded in handoff |
| F-1-35 | Added the same header, skip link, footer, legal links, factory credit, and build id to legal routes. | `legal and 404 documents use product chrome` |
| F-1-36 | Added served SVG favicon, 180px touch icon, original 1200×630 social image, and route metadata. | `legal and 404 documents use product chrome` |
| F-1-37 | Hash navigation now focuses the destination heading and announces it. | `hash navigation moves focus and announces the section` |
| F-1-38 | Raised interactive controls to 44×44 and tested desktop/mobile geometry. | `mobile layout has no overflow and controls meet touch size`; `evidence/home-390.png` |
| F-1-39 | Corrected the home title to the imperative plain-language form. | `home gives small self-hosted teams one clear sample entry` |
| F-1-40 | Split the long README workflow sentence. | `.factory/copy-audit.md` |
| F-1-41 | Split the README exit-code sentence. | README review |
| F-1-42 | Replaced the metaphor H1 with the recovery job. | `home gives small self-hosted teams one clear sample entry` |
| F-1-43 | Replaced the pseudo-preview heading with a real sample-demo route. | `/demo/?demo=1` |
| F-1-44 | Replaced vague schedule language with four concrete sample checks. | `.factory/copy-audit.md` |
| F-1-45 | Replaced “Seal the room” with “Create an isolated Docker network.” | `.factory/copy-audit.md` |
| F-1-46 | Replaced “Sign and clear” with explicit report/resource wording. | `.factory/copy-audit.md` |
| F-1-47 | Replaced the fragment with “Install one binary on your Docker host.” | `.factory/copy-audit.md` |
| F-1-48 | Replaced the slogan eyebrow with “What this does not do.” | `.factory/copy-audit.md` |
| F-1-49 | Removed the paid recovery-habit section. | Landing review |
| F-1-50 | Removed the ambiguous copy button with its unverified clipboard path. | Landing review |
| F-1-51 | Removed the paid-license button. | Landing review |
| F-1-52 | Replaced the preview button with **Try it with sample data**. | `home gives small self-hosted teams one clear sample entry` |
| F-1-53 | Standardized user input as “backup”; the sample format is “SQL backup.” | `.factory/copy-audit.md` |
| F-1-54 | Replaced overview jargon with plain “command”, “check”, and “Docker network” wording. | `.factory/copy-audit.md` |
| F-1-55 | Removed the unregistered paid bullet. | Landing review |
| F-1-56 | Added `brief.summary` and the verb-first catalog description. | `.factory/catalog-description.txt` |

## Verification set

- Clean clone: `/tmp/restore-drill-clean` at the repaired commit.
- `npm test`: 13 passed, 1 intentional duplicate axe skip.
- `npm run check:types`, `npm run check:lint`, `npm run build`, and `cargo package --allow-dirty`: passed.
- All four claim commands in `.factory/claims.json`: passed.
- Screenshot evidence: `.factory/evidence/home-390.png`,
  `.factory/evidence/demo-1440.png`, and `.factory/evidence/404-390.png`.
