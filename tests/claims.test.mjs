import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { access } from 'node:fs/promises';
import { constants } from 'node:fs';
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

// @claim:distribution-build
test('the distribution build contains an executable CLI and every site route', async () => {
  await access(resolve(root, 'dist/bin/restore-drill-linux-x86_64'), constants.X_OK);
  for (const file of ['index.html', 'demo/index.html', 'privacy/index.html', 'terms/index.html', '404.html']) {
    await access(resolve(root, 'dist/site', file), constants.R_OK);
  }
});

// @claim:weekly-scheduling
test('the scheduling examples preserve reports, failures, and credential boundaries', async () => {
  const [runner, cron, workflow] = await Promise.all([
    readFile(resolve(root, 'examples/schedule/restore-drill-weekly.sh'), 'utf8'),
    readFile(resolve(root, 'examples/schedule/restore-drill.crontab'), 'utf8'),
    readFile(resolve(root, 'examples/schedule/github-actions.yml'), 'utf8')
  ]);
  assert.match(runner, /^set -eu$/m);
  assert.match(runner, /exec restore-drill run/);
  assert.match(runner, /RESTORE_DRILL_LOG_DIR/);
  assert.match(cron, /^17 3 \* \* 1 /m);
  assert.match(workflow, /schedule:/);
  assert.match(workflow, /secrets\.RESTORE_DRILL_POSTGRES_PASSWORD/);
  assert.match(workflow, /if: always\(\)/);
  assert.match(workflow, /actions\/upload-artifact@v4/);
  assert.equal(
    workflow.match(/^\s+POSTGRES_PASSWORD: (.+)$/m)?.[1],
    '${{ secrets.RESTORE_DRILL_POSTGRES_PASSWORD }}'
  );
  assert.doesNotMatch(workflow, /replace-with|demo-only|test-only/);
  assert.doesNotMatch(workflow, /continue-on-error/);
});
