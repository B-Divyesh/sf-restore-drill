import { spawnSync } from 'node:child_process';
import { mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';

const root = resolve(import.meta.dirname, '..');
const captureRoot = mkdtempSync(join(tmpdir(), 'restore-drill-capture-'));
const binary = resolve(root, 'target/release/restore-drill');
const run = spawnSync(binary, ['demo'], {
  cwd: root,
  env: { ...process.env, TMPDIR: captureRoot },
  encoding: 'utf8'
});
if (run.status !== 0) {
  process.stderr.write(run.stderr);
  process.exit(run.status ?? 1);
}
const demoDir = join(captureRoot, readdirSync(captureRoot)[0]);
const reportsDir = join(demoDir, 'reports');
const reportName = readdirSync(reportsDir).find(name => name.endsWith('.json'));
if (!reportName) throw new Error('real demo did not create a report');
const report = JSON.parse(readFileSync(join(reportsDir, reportName), 'utf8'));
if (report.status !== 'passed' || report.assertions?.[0]?.observed !== '3') throw new Error('real demo report did not prove three rows');

const escaped = demoDir.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
const lines = run.stderr.trim().split('\n').map(line => line
  .replace(new RegExp(escaped, 'g'), '/tmp/restore-drill-demo-…')
  .replace(/in \d+\.\d+s/, 'in <measured time>')
  .replace(/sample-orders-[^ /]+\.json/, 'sample-orders-….json'));
const recording = {
  command: 'restore-drill demo',
  source: process.env.GITHUB_SHA || 'local-real-docker-run',
  captured_at: new Date().toISOString(),
  frames: lines.map((text, index) => ({ text, delay_ms: index === 0 ? 250 : 420 }))
};
writeFileSync(resolve(root, 'site/public/demo/demo-recording.json'), `${JSON.stringify(recording, null, 2)}\n`);
writeFileSync(resolve(root, 'site/public/demo/sample-report.json'), `${JSON.stringify(report, null, 2)}\n`);
rmSync(captureRoot, { recursive: true, force: true });
