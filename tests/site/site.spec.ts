import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('home is semantic, private on first load, and interactive', async ({ page }) => {
  const external: string[] = [];
  page.on('request', request => { if (!request.url().startsWith('http://127.0.0.1:4173')) external.push(request.url()); });
  await page.goto('/');
  await expect(page).toHaveTitle(/Restore Drill/);
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.locator('main')).toHaveCount(1);
  await expect(page.getByRole('img')).toHaveAttribute('alt', /backup entering an isolated test chamber/i);
  expect(external).toEqual([]);

  await page.getByRole('button', { name: 'Run a 12-second preview' }).click();
  await expect(page.locator('#demo-state')).toHaveText('Passed', { timeout: 5_000 });
  await expect(page.locator('#demo-live')).toContainText('five checkpoints', { ignoreCase: true });

  await page.getByRole('button', { name: /Try a broken backup/ }).click();
  await expect(page.locator('#demo-state')).toHaveText('Failed', { timeout: 3_000 });
  await expect(page.locator('#demo-live')).toContainText('Broken backup detected');
});

test('has no serious accessibility findings', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name === 'mobile', 'One axe pass covers the same DOM at desktop');
  await page.goto('/');
  const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
  expect(results.violations.filter(item => ['serious', 'critical'].includes(item.impact || ''))).toEqual([]);
});

test('mobile layout does not overflow and keyboard focus is visible', async ({ page }) => {
  await page.goto('/');
  const overflows = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
  expect(overflows).toBe(false);
  await page.keyboard.press('Tab');
  await expect(page.locator('.skip-link')).toBeFocused();
  const outline = await page.locator('.skip-link').evaluate(element => getComputedStyle(element).outlineStyle);
  expect(outline).not.toBe('none');
});

test('license return is stored, stripped, and unlocks verified templates', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/restore-drill/verify**', route => route.fulfill({
    status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null })
  }));
  await page.goto('/?license=test-license-token');
  await expect(page).toHaveURL('http://127.0.0.1:4173/');
  await expect(page.locator('#kit-downloads')).toBeVisible();
  await expect(page.locator('#license-status')).toContainText('verified');
  expect(await page.evaluate(() => localStorage.getItem('sb_license:restore-drill'))).toBe('test-license-token');
});

test('privacy and terms are complete standalone pages', async ({ page }) => {
  for (const path of ['/privacy/', '/terms/']) {
    await page.goto(path);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.getByRole('link', { name: /Restore Drill/ })).toBeVisible();
  }
});
