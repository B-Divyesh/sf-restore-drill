import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

const routes = [
  ['/', 'Restore Drill — prove your Postgres backup restores', 'https://restore-drill.sociobot.in/'],
  ['/demo/?demo=1', 'Demo — Restore Drill', 'https://restore-drill.sociobot.in/demo/'],
  ['/privacy/', 'Privacy — Restore Drill', 'https://restore-drill.sociobot.in/privacy/'],
  ['/terms/', 'Terms — Restore Drill', 'https://restore-drill.sociobot.in/terms/'],
  ['/404.html', 'Page not found — Restore Drill', 'https://restore-drill.sociobot.in/404']
] as const;

test('home gives small self-hosted teams one clear sample entry', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Restore Drill — prove your Postgres backup restores');
  await expect(page.locator('h1')).toHaveText('Prove your Postgres backup restores.');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toHaveAttribute('href', '/demo/?demo=1');
  await expect(page.getByText('For small self-hosted teams that need recovery proof before an outage.')).toBeVisible();
  await expect(page.getByText('Replays a recorded sample restore and opens its signed report.')).toBeVisible();
  await expect(page.getByText('signed report you can check later')).toBeVisible();
  await expect(page.locator('.plain-facts li')).toHaveCount(3);
});

test('@claim:site-no-tracking uses only a public first-party cache', async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests: string[] = [];
  page.on('request', request => requests.push(request.url()));
  await page.goto('/privacy/');
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await expect.poll(() => page.evaluate(async () => (await caches.keys()).length)).toBe(1);
  const state = await page.evaluate(async () => ({
    local: localStorage.length,
    session: sessionStorage.length,
    indexedDb: indexedDB.databases ? (await indexedDB.databases()).length : 0,
    registrations: (await navigator.serviceWorker.getRegistrations()).map(item => new URL(item.scope).origin),
    caches: await Promise.all((await caches.keys()).map(async name => ({
      name,
      urls: (await (await caches.open(name)).keys()).map(request => request.url)
    })))
  }));
  expect(requests.every(url => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  expect(await context.cookies()).toEqual([]);
  expect(state.local).toBe(0);
  expect(state.session).toBe(0);
  expect(state.indexedDb).toBe(0);
  expect(state.registrations).toEqual(['http://127.0.0.1:4173']);
  expect(state.caches).toHaveLength(1);
  expect(state.caches[0].name).toBe('restore-drill-shell-v3');
  expect(state.caches[0].urls.length).toBeGreaterThanOrEqual(9);
  expect(state.caches[0].urls.every(url => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  await context.close();
});

test('@claim:demo-sandbox isolates playback state and Reset restarts it', async ({ page }) => {
  await page.goto('/demo/?demo=1');
  await expect(page).toHaveTitle('Demo — Restore Drill');
  await expect(page.locator('h1')).toHaveText('Replay a sample Postgres restore.');
  await expect(page.getByLabel('Demo status')).toContainText('sample data');
  await expect(page.getByRole('link', { name: 'View installation steps' })).toHaveAttribute('href', '/#install-from-source');
  await expect.poll(() => page.locator('[data-recording] li').count()).toBeGreaterThan(1);
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual(['demo:restore-drill:playback']);
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual([]);
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.locator('[data-recording] li')).toHaveCount(1);
  expect(await page.evaluate(() => sessionStorage.getItem('demo:restore-drill:playback'))).toBe('0');
  await page.getByRole('link', { name: 'View installation steps' }).click();
  await expect(page).toHaveURL(/\/#install-from-source$/);
  await expect(page.locator('#install-title')).toBeFocused();
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual([]);
});

test('demo exposes a real-run recording and inspectable signed report', async ({ page }) => {
  await page.goto('/demo/?demo=1');
  const report = await (await page.request.get('/demo/sample-report.json')).json();
  const recording = await (await page.request.get('/demo/demo-recording.json')).json();
  expect(report.status).toBe('passed');
  expect(report.assertions[0]).toMatchObject({ observed: '3', passed: true });
  expect(report.signature.length).toBeGreaterThan(40);
  expect(recording.source).not.toContain('pending');
  expect(recording.frames.map((frame: { text: string }) => frame.text).join('\n')).toContain('three sample orders restore: 3');
  await expect(page.getByRole('link', { name: 'Download signed report' })).toHaveAttribute('href', '/demo/sample-report.json');
});

test('every route has complete route-specific metadata and product chrome', async ({ page }) => {
  for (const [path, title, canonical] of routes) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('header')).toHaveCount(1);
    await expect(page.locator('footer')).toHaveCount(1);
    await expect(page.locator('footer')).toContainText('build polish4');
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', canonical);
    await expect(page.locator('meta[property="og:type"]')).toHaveAttribute('content', 'website');
    await expect(page.locator('meta[property="og:url"]')).toHaveAttribute('content', canonical);
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /restore-drill-og\.jpg$/);
    await expect(page.locator('meta[name="twitter:title"]')).toHaveAttribute('content', title);
    await expect(page.locator('meta[name="twitter:description"]')).toHaveCount(1);
    await expect(page.locator('meta[name="twitter:image"]')).toHaveAttribute('content', /restore-drill-og\.jpg$/);
  }
});

test('every same-origin link and in-page target resolves', async ({ page, request, baseURL }) => {
  const origin = new URL(baseURL || 'http://127.0.0.1:4173').origin;
  const links = new Set<string>();
  for (const [path] of routes) {
    await page.goto(path);
    for (const href of await page.locator('a[href]').evaluateAll(items => items.map(item => item.getAttribute('href') || ''))) {
      const url = new URL(href, origin);
      if (url.origin === origin) links.add(url.href);
    }
  }
  for (const href of links) {
    const url = new URL(href);
    const response = await request.get(`${url.pathname}${url.search}`);
    expect(response.status(), href).toBeLessThan(400);
    if (url.hash) {
      await page.goto(href);
      await expect(page.locator(url.hash)).toHaveCount(1);
    }
  }
});

test('hash and document navigation move focus and announce context', async ({ page }) => {
  await page.goto('/#how');
  await expect(page.locator('#method-title')).toBeFocused();
  await expect(page.locator('#route-announcement')).toContainText('Rehearse a restore in four steps');
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page.locator('h1')).toBeFocused();
  await expect(page.locator('#route-announcement')).toContainText('Demo — Restore Drill');
  await page.goBack();
  await expect(page.locator('h1')).toBeFocused();
  await expect(page.locator('#route-announcement')).toContainText('prove your Postgres backup restores');
});

test('all routes fit the viewport and visible controls meet touch size', async ({ page }) => {
  for (const [path] of routes) {
    await page.goto(path);
    expect(await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth), path).toBe(false);
    const controls = await page.locator('a, button').evaluateAll(items => items.map(item => {
      const rect = item.getBoundingClientRect();
      return { text: item.textContent?.trim(), width: rect.width, height: rect.height };
    }));
    expect(controls.filter(item => item.width > 0 && item.height > 0 && (item.width < 44 || item.height < 44)), path).toEqual([]);
  }
});

test('mobile headers keep the product name and primary navigation visible', async ({ page }) => {
  for (const [path] of routes) {
    await page.goto(path);
    await expect(page.locator('.brand').getByText('Restore Drill', { exact: true })).toBeVisible();
    const navigation = page.getByRole('navigation', { name: 'Primary navigation' });
    await expect(navigation.getByRole('link', { name: 'Demo', exact: true })).toBeVisible();
    await expect(navigation.getByRole('link', { name: 'How it works', exact: true })).toBeVisible();
    await expect(navigation.getByRole('link', { name: 'Privacy', exact: true })).toBeVisible();
  }
});

test('all routes have no serious accessibility findings', async ({ page }) => {
  for (const [path] of routes) {
    await page.goto(path);
    const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
    expect(results.violations.filter(item => ['serious', 'critical'].includes(item.impact || '')), path).toEqual([]);
  }
});

test('@claim:offline-web-walkthrough demo reloads offline after its first visit', async ({ page, context }) => {
  await page.goto('/demo/?demo=1');
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator('h1')).toHaveText('Replay a sample Postgres restore.');
  await expect(page.locator('[data-report-summary]')).toContainText('passed');
});
