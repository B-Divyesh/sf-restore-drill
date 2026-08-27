import './style.css';

const BILLING = 'https://api.sociobot.in/api/v1/products/restore-drill';
const LICENSE_KEY = 'sb_license:restore-drill';
const VERDICT_KEY = 'sb_license_verdict:restore-drill';
const DAY = 86_400_000;

type Verdict = { valid: boolean; checkedAt: number; reason?: string };

const $ = <T extends Element>(selector: string) => document.querySelector<T>(selector);
const $$ = <T extends Element>(selector: string) => [...document.querySelectorAll<T>(selector)];

function setupDemo() {
  const log = $('#demo-log');
  const state = $('#demo-state');
  const live = $('#demo-live');
  if (!log || !state || !live) return;
  const rows = [...log.children] as HTMLElement[];
  let generation = 0;
  const reset = () => rows.forEach(row => {
    row.className = '';
    row.querySelector('.log-mark')!.textContent = '○';
    row.querySelector('code')!.textContent = 'waiting';
  });
  const delay = (ms: number) => new Promise(resolve => window.setTimeout(resolve, ms));
  const run = async (broken: boolean) => {
    const current = ++generation;
    reset(); state.className = 'state running'; state.textContent = 'Running'; live.textContent = 'Restore drill preview started.';
    $('#demo-rto')!.textContent = 'measuring'; $('#demo-hash')!.textContent = 'calculating';
    const values = ['8d71…c204', 'ready · 8.4 GB', '4 / 4 passed', '200 · “ok”', 'Ed25519 valid'];
    for (let i = 0; i < rows.length; i++) {
      await delay(window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 20 : 380);
      if (current !== generation) return;
      const fails = broken && i === 1;
      rows[i].className = fails ? 'failed' : 'passed';
      rows[i].querySelector('.log-mark')!.textContent = fails ? '×' : '✓';
      rows[i].querySelector('code')!.textContent = fails ? 'pg_restore: unexpected EOF' : values[i];
      if (fails) {
        rows.slice(i + 1).forEach(row => row.querySelector('code')!.textContent = 'not run');
        state.className = 'state failed'; state.textContent = 'Failed';
        $('#demo-rto')!.textContent = '00:04.821'; $('#demo-hash')!.textContent = 'f039…91ad';
        live.textContent = 'Broken backup detected during isolated restore. Later probes were not run.';
        return;
      }
    }
    state.className = 'state passed'; state.textContent = 'Passed';
    $('#demo-rto')!.textContent = '00:12.044'; $('#demo-hash')!.textContent = '8d71…c204';
    live.textContent = 'Restore drill passed. Five checkpoints complete in 12.044 seconds.';
  };
  $$<HTMLButtonElement>('[data-demo]').forEach(button => button.addEventListener('click', () => {
    document.querySelector('.demo-band')?.scrollIntoView({ behavior: window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
    void run(button.dataset.demo === 'broken');
  }));
}

function setupCopy() {
  $('#copy-command')?.addEventListener('click', async event => {
    const button = event.currentTarget as HTMLButtonElement;
    try {
      await navigator.clipboard.writeText($('#install-command')?.textContent || '');
      button.textContent = 'Copied';
    } catch {
      button.textContent = 'Select command';
      const selection = getSelection(); const range = document.createRange(); range.selectNodeContents($('#install-command')!);
      selection?.removeAllRanges(); selection?.addRange(range);
    }
    window.setTimeout(() => button.textContent = 'Copy', 1800);
  });
}

function storedVerdict(): Verdict | null {
  try { return JSON.parse(localStorage.getItem(VERDICT_KEY) || 'null') as Verdict | null; } catch { return null; }
}

function showUnlocked(valid: boolean) {
  const downloads = $('#kit-downloads') as HTMLElement | null;
  if (downloads) downloads.hidden = !valid;
}

async function verifyLicense(token: string, force = false) {
  const status = $('#license-status');
  const cached = storedVerdict();
  if (!force && cached && Date.now() - cached.checkedAt < DAY) {
    showUnlocked(cached.valid);
    if (status) status.textContent = cached.valid ? 'License verified on this device.' : 'License no longer active. You can purchase a new Team Kit license.';
    return;
  }
  if (status) status.textContent = 'Checking license…';
  try {
    const response = await fetch(`${BILLING}/verify?license=${encodeURIComponent(token)}`, { headers: { accept: 'application/json' } });
    if (!response.ok) throw new Error('verification service unavailable');
    const result = await response.json() as { valid: boolean; reason?: string };
    const verdict = { valid: result.valid, reason: result.reason, checkedAt: Date.now() };
    localStorage.setItem(VERDICT_KEY, JSON.stringify(verdict));
    showUnlocked(result.valid);
    if (status) status.textContent = result.valid ? 'License verified. Team Kit unlocked on this device.' : 'License no longer active. Check the token or purchase a new license.';
  } catch {
    if (cached?.valid) {
      showUnlocked(true);
      if (status) status.textContent = 'Offline—using the last verified license. We will check again later.';
    } else {
      showUnlocked(false);
      if (status) status.textContent = 'Could not reach license verification. Check your connection and try again.';
    }
  }
}

function setupLicense() {
  const params = new URLSearchParams(location.search);
  const returned = params.get('license');
  if (returned) {
    localStorage.setItem(LICENSE_KEY, returned);
    params.delete('license');
    history.replaceState({}, '', `${location.pathname}${params.size ? `?${params}` : ''}${location.hash}`);
  }
  const token = returned || localStorage.getItem(LICENSE_KEY);
  const cached = storedVerdict();
  if (cached?.valid) showUnlocked(true);
  if (token) void verifyLicense(token);

  const toggle = $('#show-license') as HTMLButtonElement | null;
  const form = $('#license-form') as HTMLFormElement | null;
  toggle?.addEventListener('click', () => {
    if (!form) return;
    form.hidden = !form.hidden; toggle.setAttribute('aria-expanded', String(!form.hidden));
    if (!form.hidden) (form.elements.namedItem('license') as HTMLInputElement).focus();
  });
  form?.addEventListener('submit', event => {
    event.preventDefault();
    const input = form.elements.namedItem('license') as HTMLInputElement;
    const value = input.value.trim();
    if (!value) { $('#license-status')!.textContent = 'Paste the full license token, then verify again.'; input.focus(); return; }
    localStorage.setItem(LICENSE_KEY, value); localStorage.removeItem(VERDICT_KEY); void verifyLicense(value, true);
  });
}

const templates: Record<string, { name: string; type: string; body: string }> = {
  workflow: { name: 'restore-drill.yml', type: 'text/yaml', body: `name: Weekly restore drill\non:\n  schedule:\n    - cron: '0 3 * * 0'\n  workflow_dispatch:\njobs:\n  drill:\n    runs-on: ubuntu-latest\n    timeout-minutes: 20\n    steps:\n      - uses: actions/checkout@v4\n      - name: Fetch backup from your storage\n        run: ./ops/fetch-backup.sh backups/latest.dump\n      - name: Run restore proof\n        run: restore-drill run --config restore-drill.toml --json\n` },
  scorecard: { name: 'restore-scorecard.csv', type: 'text/csv', body: 'week,finished_at,status,recovery_time_ms,artifact_sha256,owner,follow_up\n1,,,,,,\n2,,,,,,\n3,,,,,,\n4,,,,,,\n' },
  checklist: { name: 'restore-drill-checklist.md', type: 'text/markdown', body: '# Restore drill checklist\n\n- [ ] Backup source is read-only\n- [ ] Credential files are excluded from version control\n- [ ] Docker network name starts with `restore-drill-`\n- [ ] At least one data-integrity SQL assertion exists\n- [ ] App health is probed from the internal network\n- [ ] A deliberately broken artifact fails the drill\n- [ ] Signed reports are retained outside the runner\n- [ ] Four consecutive weekly drills are reviewed\n' }
};

function setupDownloads() {
  $$<HTMLButtonElement>('[data-download]').forEach(button => button.addEventListener('click', () => {
    const template = templates[button.dataset.download || '']; if (!template) return;
    const url = URL.createObjectURL(new Blob([template.body], { type: template.type }));
    const anchor = document.createElement('a'); anchor.href = url; anchor.download = template.name; anchor.click(); URL.revokeObjectURL(url);
  }));
}

function setupOffline() {
  const note = $('#offline-note') as HTMLElement | null;
  const update = () => { if (note) note.hidden = navigator.onLine; };
  addEventListener('online', update); addEventListener('offline', update); update();
}

setupDemo(); setupCopy(); setupLicense(); setupDownloads(); setupOffline();
if ('serviceWorker' in navigator && location.hostname !== 'localhost') addEventListener('load', () => void navigator.serviceWorker.register('/sw.js'));
