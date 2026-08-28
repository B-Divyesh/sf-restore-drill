import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import test from 'node:test';

const root = resolve(import.meta.dirname, '../..');
const policyPath = resolve(root, 'site/public/staticwebapp.config.json');
const legacyPolicyPath = resolve(root, 'site/public/_headers');

async function policy() {
  return JSON.parse(await readFile(policyPath, 'utf8'));
}

test('Azure Static Web Apps response policy supplies the release security headers', async () => {
  const config = await policy();
  const headers = config.globalHeaders;
  const csp = headers['Content-Security-Policy'];

  assert.match(csp, /default-src 'self'/);
  assert.match(csp, /connect-src 'self' https:\/\/api\.sociobot\.in/);
  assert.match(csp, /frame-ancestors 'none'/);
  assert.equal(headers['Permissions-Policy'], 'camera=(), microphone=(), geolocation=()');
  assert.equal(headers['Referrer-Policy'], 'strict-origin-when-cross-origin');
  assert.equal(headers['X-Content-Type-Options'], 'nosniff');
  assert.equal(headers['X-Frame-Options'], 'DENY');
});

test('Azure Static Web Apps response policy caches only fingerprinted assets and the versioned hero immutably', async () => {
  const config = await policy();
  const immutable = 'public, max-age=31536000, immutable';
  const cacheByRoute = new Map(config.routes.map(route => [route.route, route.headers['Cache-Control']]));

  assert.equal(cacheByRoute.get('/assets/*'), immutable);
  assert.equal(cacheByRoute.get('/restore-chamber.webp'), immutable);
  assert.equal(cacheByRoute.has('/*'), false, 'HTML must stay revalidatable for releases');
});

test('the Azure policy and _headers fallback cannot silently diverge', async () => {
  const [config, legacy] = await Promise.all([policy(), readFile(legacyPolicyPath, 'utf8')]);
  const headers = config.globalHeaders;
  const escape = value => value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

  assert.match(legacy, new RegExp(`Content-Security-Policy: ${escape(headers['Content-Security-Policy'])}`));
  assert.match(legacy, new RegExp(`Permissions-Policy: ${escape(headers['Permissions-Policy'])}`));
  assert.match(legacy, /\/assets\/\*[\s\S]*?Cache-Control: public, max-age=31536000, immutable/);
  assert.match(legacy, /\/restore-chamber\.webp[\s\S]*?Cache-Control: public, max-age=31536000, immutable/);
});

test('the host rewrites a missing route to the product 404 document', async () => {
  const config = await policy();
  assert.equal(config.responseOverrides['404'].rewrite, '/404.html');
});
