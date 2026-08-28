import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('home gives small self-hosted teams one clear sample entry', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Restore Drill — prove your Postgres backup restores');
  await expect(page.locator('h1')).toHaveText('Prove your Postgres backup restores.');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toHaveAttribute('href', /\/demo\/\?demo=1/);
  await expect(page.getByText('For small self-hosted teams that need recovery proof before an outage.')).toBeVisible();
  await expect(page.locator('.plain-facts li')).toHaveCount(3);
});

test('@claim:site-no-tracking documentation pages make no tracking requests or browser-data writes', async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const thirdParty: string[] = [];
  page.on('request', request => {
    if (!request.url().startsWith('http://127.0.0.1:4173')) thirdParty.push(request.url());
  });
  await page.goto('/privacy/');
  expect(thirdParty).toEqual([]);
  expect(await context.cookies()).toEqual([]);
  expect(await page.evaluate(() => ({
    local: localStorage.length,
    session: sessionStorage.length,
    indexedDb: indexedDB.databases ? 'available' : 'unavailable'
  }))).toEqual({ local: 0, session: 0, indexedDb: 'available' });
  await context.close();
});

test('demo has the isolation banner, reset, and real start link', async ({ page }) => {
  await page.goto('/demo/?demo=1');
  await expect(page).toHaveTitle('Demo — Restore Drill');
  await expect(page.locator('h1')).toHaveText('Run a sample Postgres restore.');
  await expect(page.getByLabel('Demo status')).toContainText('sample data');
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Start for real' })).toHaveAttribute('href', '/');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page).toHaveURL(/\/demo\/\?demo=1/);
});

test('legal and 404 documents use product chrome and route metadata', async ({ page }) => {
  for (const [path, title] of [['/privacy/', 'Privacy — Restore Drill'], ['/terms/', 'Terms — Restore Drill'], ['/404.html', 'Page not found — Restore Drill']]) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('header')).toHaveCount(1);
    await expect(page.locator('footer')).toHaveCount(1);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
  }
});

test('hash navigation moves focus and announces the section', async ({ page }) => {
  await page.goto('/#how');
  await expect(page.locator('#method-title')).toBeFocused();
  await expect(page.locator('#route-announcement')).toContainText('Run four checks before an outage');
});

test('mobile layout has no overflow and controls meet touch size', async ({ page }) => {
  await page.goto('/');
  expect(await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth)).toBe(false);
  const controls = await page.locator('a, button').evaluateAll(items => items.map(item => {
    const rect = item.getBoundingClientRect(); return { text: item.textContent?.trim(), width: rect.width, height: rect.height };
  }));
  expect(controls.filter(item => item.width > 0 && item.height > 0 && (item.width < 44 || item.height < 44))).toEqual([]);
  await page.keyboard.press('Tab');
  await expect(page.locator('.skip-link')).toBeFocused();
});

test('has no serious accessibility findings', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name === 'mobile', 'Desktop axe covers the identical document');
  await page.goto('/demo/?demo=1');
  const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
  expect(results.violations.filter(item => ['serious', 'critical'].includes(item.impact || ''))).toEqual([]);
});
