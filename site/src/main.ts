import './style.css';

const live = document.querySelector<HTMLElement>('#route-announcement');

function announce(text: string) {
  if (!live) return;
  live.textContent = '';
  window.setTimeout(() => { live.textContent = text; }, 40);
}

function focusElement(target: HTMLElement, message: string) {
  target.setAttribute('tabindex', '-1');
  window.requestAnimationFrame(() => {
    target.focus({ preventScroll: true });
    announce(message);
  });
}

function focusHashTarget() {
  const id = decodeURIComponent(location.hash.slice(1));
  if (!id) return;
  const section = document.getElementById(id);
  if (!section) return;
  const target = section.matches('h1,h2,h3,h4,h5,h6') ? section : section.querySelector<HTMLElement>('h1,h2,h3,h4,h5,h6') || section;
  focusElement(target, `${target.textContent?.trim() || 'Section'} section`);
}

function focusRouteHeading() {
  const heading = document.querySelector<HTMLElement>('h1');
  if (heading) focusElement(heading, `${document.title}. ${heading.textContent?.trim() || ''}`);
}

addEventListener('hashchange', focusHashTarget);
addEventListener('pageshow', event => {
  if (event.persisted) focusRouteHeading();
});

document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]').forEach(link => {
  link.addEventListener('click', event => {
    event.preventDefault();
    history.pushState({}, '', link.hash);
    const target = document.getElementById(decodeURIComponent(link.hash.slice(1)));
    target?.scrollIntoView({ behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
    focusHashTarget();
  });
});

const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming | undefined;
const cameFromThisSite = document.referrer ? new URL(document.referrer).origin === location.origin : false;
if (location.hash) {
  window.setTimeout(focusHashTarget, 0);
} else if (cameFromThisSite || navigation?.type === 'back_forward') {
  window.setTimeout(focusRouteHeading, 0);
}

type Recording = { command: string; source: string; captured_at: string; frames: Array<{ text: string; delay_ms: number }> };
type SampleReport = { status: string; drill: string; recovery_time_ms: number; artifact: { sha256: string }; assertions: Array<{ name: string; observed: string; passed: boolean }>; signature: string };

const demoKey = 'demo:restore-drill:playback';
const recordingList = document.querySelector<HTMLOListElement>('[data-recording]');
const playbackButton = document.querySelector<HTMLButtonElement>('[data-playback-toggle]');
let playbackTimer = 0;
let paused = false;
let frameIndex = 0;
let recording: Recording | undefined;

function clearDemoState() {
  for (let index = sessionStorage.length - 1; index >= 0; index -= 1) {
    const key = sessionStorage.key(index);
    if (key?.startsWith('demo:restore-drill:')) sessionStorage.removeItem(key);
  }
}

function addFrame(text: string) {
  if (!recordingList) return;
  const item = document.createElement('li');
  item.textContent = text;
  recordingList.append(item);
  recordingList.scrollTop = recordingList.scrollHeight;
}

function scheduleFrame() {
  if (!recording || paused || frameIndex >= recording.frames.length) {
    if (playbackButton && recording && frameIndex >= recording.frames.length) {
      playbackButton.textContent = 'Replay recording';
      playbackButton.setAttribute('aria-pressed', 'false');
    }
    return;
  }
  const frame = recording.frames[frameIndex];
  const delay = matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : frame.delay_ms;
  playbackTimer = window.setTimeout(() => {
    addFrame(frame.text);
    frameIndex += 1;
    sessionStorage.setItem(demoKey, String(frameIndex));
    scheduleFrame();
  }, delay);
}

function restartPlayback() {
  window.clearTimeout(playbackTimer);
  paused = false;
  frameIndex = 0;
  sessionStorage.setItem(demoKey, '0');
  if (recordingList) recordingList.innerHTML = '<li><span aria-hidden="true">$</span> restore-drill demo</li>';
  if (playbackButton) {
    playbackButton.textContent = 'Pause replay';
    playbackButton.setAttribute('aria-pressed', 'false');
  }
  scheduleFrame();
}

async function loadDemo() {
  if (!recordingList) return;
  try {
    const [recordingResponse, reportResponse] = await Promise.all([
      fetch('/demo/demo-recording.json'),
      fetch('/demo/sample-report.json')
    ]);
    if (!recordingResponse.ok || !reportResponse.ok) throw new Error('recorded evidence is unavailable');
    recording = await recordingResponse.json() as Recording;
    const report = await reportResponse.json() as SampleReport;
    const summary = document.querySelector<HTMLElement>('[data-report-summary]');
    if (summary) {
      const assertion = report.assertions[0];
      summary.innerHTML = `<div><dt>Status</dt><dd class="verified">${report.status}</dd></div><div><dt>Rows restored</dt><dd>${assertion.observed}</dd></div><div><dt>Backup hash</dt><dd><code>${report.artifact.sha256.slice(0, 12)}…</code></dd></div><div><dt>Signature</dt><dd><code>${report.signature.slice(0, 12)}…</code></dd></div>`;
    }
    restartPlayback();
  } catch (error) {
    addFrame(`Recording unavailable: ${error instanceof Error ? error.message : 'reload this page'}`);
    if (playbackButton) playbackButton.disabled = true;
  }
}

playbackButton?.addEventListener('click', () => {
  if (!recording) return;
  if (frameIndex >= recording.frames.length) {
    restartPlayback();
    return;
  }
  paused = !paused;
  playbackButton.textContent = paused ? 'Resume replay' : 'Pause replay';
  playbackButton.setAttribute('aria-pressed', String(paused));
  window.clearTimeout(playbackTimer);
  if (!paused) scheduleFrame();
});

document.querySelector<HTMLButtonElement>('[data-reset-demo]')?.addEventListener('click', () => {
  clearDemoState();
  restartPlayback();
  announce('Demo reset. Recording restarted.');
});

document.querySelectorAll<HTMLAnchorElement>('[data-exit-demo], body:has(.demo-banner) a[href]:not([href^="/demo/"])').forEach(link => {
  link.addEventListener('click', clearDemoState);
});

void loadDemo();

if ('serviceWorker' in navigator && (location.protocol === 'https:' || ['localhost', '127.0.0.1'].includes(location.hostname))) {
  addEventListener('load', () => void navigator.serviceWorker.register('/sw.js'));
}
