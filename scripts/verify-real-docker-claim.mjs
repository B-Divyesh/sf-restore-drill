import { spawnSync } from 'node:child_process';

const repository = 'B-Divyesh/sf-restore-drill';
const workflow = 'real-docker-claim.yml';
const claimStep = 'Run real Postgres restore claim';
const evidenceArtifact = 'real-demo-evidence';
const pollMs = Number(process.env.RESTORE_DRILL_CI_POLL_MS || 15_000);
const timeoutMs = Number(process.env.RESTORE_DRILL_CI_TIMEOUT_MS || 15 * 60_000);

function run(command, args, options = {}) {
  return spawnSync(command, args, { encoding: 'utf8', ...options });
}

function git(args) {
  const result = run('git', args);
  if (result.status !== 0) {
    throw new Error(result.stderr.trim() || `git ${args.join(' ')} failed`);
  }
  return result.stdout.trim();
}

function dockerIsReady() {
  const result = run('docker', ['version'], { stdio: 'ignore' });
  return result.status === 0;
}

function documentationOnlySince(commit) {
  const ancestor = run('git', ['merge-base', '--is-ancestor', commit, 'HEAD']);
  if (ancestor.status !== 0) return false;
  const changed = git(['diff', '--name-only', `${commit}..HEAD`])
    .split('\n')
    .filter(Boolean);
  return changed.every(path =>
    path.startsWith('.factory/evidence/') ||
    /^\.factory\/(?:handoff|polish-[^/]+|review-[^/]+)\.md$/.test(path)
  );
}

async function github(path) {
  const headers = {
    Accept: 'application/vnd.github+json',
    'User-Agent': 'restore-drill-release-gate',
    'X-GitHub-Api-Version': '2022-11-28'
  };
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  const response = await fetch(`https://api.github.com/repos/${repository}${path}`, { headers });
  if (!response.ok) {
    throw new Error(`GitHub API ${response.status} for ${path}: ${(await response.text()).slice(0, 240)}`);
  }
  return response.json();
}

async function matchingRuns(head) {
  const data = await github(`/actions/workflows/${workflow}/runs?branch=main&per_page=100`);
  return data.workflow_runs.filter(run =>
    run.path === `.github/workflows/${workflow}` &&
    (run.head_sha === head || documentationOnlySince(run.head_sha))
  );
}

async function validateRun(run, head) {
  if (run.status !== 'completed' || run.conclusion !== 'success') return false;
  const [jobs, artifacts] = await Promise.all([
    github(`/actions/runs/${run.id}/jobs?per_page=100`),
    github(`/actions/runs/${run.id}/artifacts?per_page=100`)
  ]);
  const job = jobs.jobs.find(item => item.name === 'real-docker-restore');
  const step = job?.steps?.find(item => item.name === claimStep);
  const artifact = artifacts.artifacts.find(item => item.name === evidenceArtifact);
  if (job?.conclusion !== 'success' || step?.conclusion !== 'success') return false;
  if (!artifact || artifact.expired || artifact.size_in_bytes <= 0) return false;

  const source = run.head_sha === head ? head : `${run.head_sha} (documentation-only ancestor of ${head})`;
  console.log(`Real Docker claim passed on clean GitHub-hosted runner for ${source}.`);
  console.log(`Healthy and corrupt reports, signatures, and zero-resource cleanup: ${run.html_url}`);
  console.log(`Evidence artifact: ${artifact.name} (${artifact.size_in_bytes} bytes, id ${artifact.id}).`);
  return true;
}

async function verifyHostedRun() {
  const head = git(['rev-parse', 'HEAD']);
  if (git(['status', '--porcelain'])) {
    throw new Error('Hosted claim verification must run from a clean checkout.');
  }
  const deadline = Date.now() + timeoutMs;
  do {
    const runs = await matchingRuns(head);
    for (const candidate of runs) {
      if (await validateRun(candidate, head)) return;
    }
    const active = runs.find(run => run.status !== 'completed');
    if (!active || Date.now() + pollMs > deadline) break;
    console.log(`Waiting for Docker-capable runner: ${active.html_url}`);
    await new Promise(resolve => setTimeout(resolve, pollMs));
  } while (Date.now() < deadline);
  throw new Error(
    `No successful, evidence-bearing real-Docker workflow run matches ${head}. ` +
    'Push the source revision or dispatch the Real Docker claim workflow, then retry.'
  );
}

if (dockerIsReady()) {
  console.log('Docker daemon detected; running the real Postgres acceptance test locally.');
  const result = run('cargo', [
    'test',
    '--workspace',
    'real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup',
    '--',
    '--ignored'
  ], { stdio: 'inherit' });
  process.exit(result.status ?? 1);
}

console.log('No local Docker daemon; checking the Docker-capable clean release runner.');
await verifyHostedRun();
