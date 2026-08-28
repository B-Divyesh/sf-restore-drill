# Restore Drill polish 1 handoff

## Delivered

Commit `ebe1c9fdfe5ab23ae6b168607616af28c354874e` repairs every copy, route,
demo, metadata, mobile target, legal-chrome, and claims-registry issue recorded
in review 1.

- `restore-drill demo` now runs the bundled three-order SQL sample through the
  normal drill pipeline from a fresh system temporary directory.
- `/demo/?demo=1` is a direct sample walkthrough with a persistent isolation
  banner, reset control, and start-real link.
- The landing page now states the audience, job, and one evaluation action in
  the first screen. The unregistered paid offer and untestable marketing claims
  were removed.
- Real `/demo/`, `/privacy/`, `/terms/`, and product-owned 404 documents ship
  with titles, canonical/social metadata, favicon, touch icon, shared chrome,
  focus behavior, and static-host 404 override.
- Claim evidence is in `.factory/claims.json`; the complete finding-by-finding
  mapping is in `.factory/polish-1.md`.

## Verification

From a separate clean clone at `/tmp/restore-drill-clean`:

```sh
npm ci
npm test                         # 13 passed, 1 intentional duplicate axe skip
npm run check:types
npm run check:lint
npm run build                    # dist/bin + dist/site
cargo package --manifest-path crates/restore-drill/Cargo.toml --allow-dirty
```

All four commands in `.factory/claims.json` passed from that clean clone. The
package verification passed with the bundled demo data included in the crate.
The static build is 1.85 KB gzip JavaScript and 3.57 KB gzip CSS. Screenshots:

- `.factory/evidence/home-390.png`
- `.factory/evidence/demo-1440.png`
- `.factory/evidence/404-390.png`

## Run and deploy

```sh
npm ci
npm test
npm run build
/opt/fleet/lib/deploy-static.sh restore-drill dist/site
```

Deployment completed through `/opt/fleet/lib/deploy-static.sh restore-drill
dist/site` (Azure deployment `b74ee18e-1c5a-44fc-8c39-0258dc95644a`). Cold
checks on 2026-08-28 returned 200 for `/`, `/demo/?demo=1`, `/privacy/`, and
`/terms/`; `/does-not-exist` returned the product 404 with HTTP 404. Live
Playwright found one h1/main, expected titles, no page errors, no third-party
requests, and zero serious/critical axe findings. Use `restore-drill demo` on a
Docker host for the real sample drill.

## Known environment limitation

This worker image has no `docker` executable or daemon/socket. The demo and
existing integration test exercise the normal command flow with a
Docker-compatible harness, including report signing and cleanup. A real Docker
daemon run remains required operational evidence; it cannot be truthfully
executed in this container. No product behavior is stubbed for that limitation.
