import assert from 'node:assert/strict';
import { mkdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { chromium } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

const base = new URL(process.argv[2] || 'https://restore-drill.sociobot.in/');
const evidence = resolve(process.argv[3] || '.factory/evidence/live-polish-2');
await mkdir(evidence, { recursive: true });

const expected = [
  ['/', 'Restore Drill — prove your Postgres backup restores'],
  ['/demo/?demo=1', 'Demo — Restore Drill'],
  ['/privacy/', 'Privacy — Restore Drill'],
  ['/terms/', 'Terms — Restore Drill']
];
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const page = await context.newPage();
const consoleErrors = [];
const externalRequests = [];
page.on('console', message => { if (message.type() === 'error') consoleErrors.push(message.text()); });
page.on('pageerror', error => consoleErrors.push(String(error)));
page.on('request', request => {
  if (new URL(request.url()).origin !== base.origin) externalRequests.push(request.url());
});

const routes = [];
for (const [path, title] of expected) {
  const response = await page.goto(new URL(path, base).href, { waitUntil: 'networkidle' });
  assert.equal(response?.status(), 200, path);
  assert.equal(await page.title(), title, path);
  const facts = await page.evaluate(() => ({
    lang: document.documentElement.lang,
    h1: document.querySelectorAll('h1').length,
    main: Boolean(document.querySelector('main')),
    ogUrl: document.querySelector('meta[property="og:url"]')?.getAttribute('content'),
    twitterTitle: document.querySelector('meta[name="twitter:title"]')?.getAttribute('content'),
    overflow: document.documentElement.scrollWidth > document.documentElement.clientWidth,
    brandNameVisible: (document.querySelector('.brand span:last-child')?.getBoundingClientRect().width || 0) > 0,
    primaryLinks: [...document.querySelectorAll('.site-header nav a')].map(link => {
      const rect = link.getBoundingClientRect();
      return { text: link.textContent?.trim(), width: rect.width, height: rect.height };
    })
  }));
  assert.deepEqual({ lang: facts.lang, h1: facts.h1, main: facts.main, overflow: facts.overflow }, { lang: 'en', h1: 1, main: true, overflow: false });
  assert.equal(facts.brandNameVisible, true, `${path} hides the product name at 390 px`);
  assert.deepEqual(facts.primaryLinks.map(link => link.text), ['Demo', 'How it works', 'Privacy'], `${path} mobile navigation is incomplete`);
  assert.ok(facts.primaryLinks.every(link => link.width >= 44 && link.height >= 44), `${path} mobile navigation target is below 44 px`);
  assert.ok(facts.ogUrl && facts.twitterTitle, `${path} metadata is incomplete`);
  if (path === '/') assert.equal(await page.locator('#method-title').textContent(), 'Rehearse a restore in four steps.');
  const axe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
  const serious = axe.violations.filter(item => ['serious', 'critical'].includes(item.impact || ''));
  assert.deepEqual(serious, [], `${path} has serious Axe findings`);
  routes.push({ path, title, status: response?.status(), seriousAxe: serious.length });
}

await context.clearCookies();
await page.goto(base.href, { waitUntil: 'networkidle' });
await page.screenshot({ path: resolve(evidence, 'home-cold-390.png'), fullPage: true });
await page.getByRole('link', { name: 'Try it with sample data' }).click();
assert.equal(await page.locator('h1').evaluate(element => element === document.activeElement), true);
await page.waitForTimeout(3200);
await page.screenshot({ path: resolve(evidence, 'demo-cold-390.png'), fullPage: true });
assert.equal(await page.locator('[data-recording] li').count(), 8);
assert.deepEqual(await page.evaluate(() => Object.keys(localStorage)), []);
assert.deepEqual(await page.evaluate(() => Object.keys(sessionStorage)), ['demo:restore-drill:playback']);
const report = await (await context.request.get(new URL('/demo/sample-report.json', base).href)).json();
assert.equal(report.status, 'passed');
assert.equal(report.assertions[0].observed, '3');
assert.ok(report.signature.length > 40);
await page.getByRole('button', { name: 'Replay recording' }).click();
assert.equal(await page.locator('[data-recording] li').count(), 1);

await page.evaluate(async () => { await navigator.serviceWorker.ready; });
await page.reload({ waitUntil: 'networkidle' });
const cache = await page.evaluate(async () => ({
  indexedDb: indexedDB.databases ? (await indexedDB.databases()).length : 0,
  registrations: (await navigator.serviceWorker.getRegistrations()).length,
  names: await caches.keys(),
  urls: (await (await caches.open('restore-drill-shell-v3')).keys()).map(request => request.url)
}));
assert.equal(cache.indexedDb, 0);
assert.equal(cache.registrations, 1);
assert.deepEqual(cache.names, ['restore-drill-shell-v3']);
assert.ok(cache.urls.every(url => new URL(url).origin === base.origin));
await context.setOffline(true);
await page.reload();
assert.equal(await page.locator('h1').textContent(), 'Replay a sample Postgres restore.');
assert.ok((await page.locator('[data-report-summary]').textContent()).includes('passed'));
await context.setOffline(false);

const missing = new URL('/not-a-real-route-polish-4', base);
const missingResponse = await context.request.get(missing.href);
assert.equal(missingResponse.status(), 404);
await page.goto(missing.href);
assert.equal(await page.title(), 'Page not found — Restore Drill');
await page.screenshot({ path: resolve(evidence, '404-cold-390.png'), fullPage: true });

assert.deepEqual(externalRequests, []);
assert.deepEqual(consoleErrors.filter(message => !message.includes('404')), []);
const result = { base: base.href, routes, report: { status: report.status, observed: report.assertions[0].observed, signatureLength: report.signature.length }, cache, externalRequests, consoleErrors };
await writeFile(resolve(evidence, 'cold-check.json'), `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify(result));
await browser.close();
