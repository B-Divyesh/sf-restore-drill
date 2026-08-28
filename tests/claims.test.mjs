import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import test from 'node:test';

const root = resolve(import.meta.dirname, '..');

// @claim:mit-license
test('the distributed repository includes the MIT license', async () => {
  const license = await readFile(resolve(root, 'LICENSE'), 'utf8');
  const manifest = await readFile(resolve(root, 'crates/restore-drill/Cargo.toml'), 'utf8');
  assert.match(license, /Permission is hereby granted, free of charge/);
  assert.match(manifest, /^license = "MIT"$/m);
});
