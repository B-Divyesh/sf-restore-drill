import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { resolve } from 'node:path';
import test from 'node:test';

const root = resolve(import.meta.dirname, '../..');

async function filesBelow(directory) {
  const found = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) found.push(...await filesBelow(path));
    else found.push(path);
  }
  return found;
}

test('every registered claim has exactly one tagged test', async () => {
  const claims = JSON.parse(await readFile(resolve(root, '.factory/claims.json'), 'utf8'));
  assert.equal(new Set(claims.map(claim => claim.id)).size, claims.length);
  const sources = [
    ...await filesBelow(resolve(root, 'crates/restore-drill/src')),
    ...await filesBelow(resolve(root, 'crates/restore-drill/tests')),
    resolve(root, 'tests/claims.test.mjs'),
    resolve(root, 'tests/site/site.spec.ts')
  ];
  const corpus = (await Promise.all(sources.map(path => readFile(path, 'utf8')))).join('\n');
  for (const claim of claims) {
    const tag = `@claim:${claim.id}`;
    assert.equal(corpus.split(tag).length - 1, 1, `${tag} must occur exactly once in test sources`);
  }
});

test('site setup links target the real README installation heading', async () => {
  const [readme, home, demo] = await Promise.all([
    readFile(resolve(root, 'README.md'), 'utf8'),
    readFile(resolve(root, 'site/index.html'), 'utf8'),
    readFile(resolve(root, 'site/demo/index.html'), 'utf8')
  ]);
  assert.match(readme, /^## Install from source$/m);
  assert.match(readme, /git clone https:\/\/github\.com\/B-Divyesh\/sf-restore-drill\.git/);
  for (const page of [home, demo]) {
    assert.match(page, /github\.com\/B-Divyesh\/sf-restore-drill#install-from-source/);
    assert.doesNotMatch(page, /#(?:usage|install)"/);
  }
});

test('catalog description is verb-first and no longer than 120 characters', async () => {
  const description = (await readFile(resolve(root, '.factory/catalog-description.txt'), 'utf8')).trim();
  assert.ok(description.startsWith('Prove '));
  assert.ok(description.length <= 120, `catalog description is ${description.length} characters`);
});
