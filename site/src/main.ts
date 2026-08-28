import './style.css';

const live = document.querySelector<HTMLElement>('#route-announcement');

function announce(text: string) {
  if (!live) return;
  live.textContent = '';
  window.setTimeout(() => { live.textContent = text; }, 40);
}

function focusHashTarget() {
  const id = decodeURIComponent(location.hash.slice(1));
  if (!id) return;
  const section = document.getElementById(id);
  if (!section) return;
  const target = section.matches('h1,h2,h3,h4,h5,h6') ? section : section.querySelector<HTMLElement>('h1,h2,h3,h4,h5,h6') || section;
  target.setAttribute('tabindex', '-1');
  window.requestAnimationFrame(() => {
    target.focus({ preventScroll: true });
    announce(`${target.textContent?.trim() || 'Section'} section`);
  });
}

addEventListener('hashchange', focusHashTarget);
document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]').forEach(link => {
  link.addEventListener('click', event => {
    event.preventDefault();
    history.pushState({}, '', link.hash);
    const target = document.getElementById(decodeURIComponent(link.hash.slice(1)));
    target?.scrollIntoView({ behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
    focusHashTarget();
  });
});

document.querySelector<HTMLButtonElement>('[data-reset-demo]')?.addEventListener('click', () => {
  history.replaceState({}, '', '/demo/?demo=1');
  location.reload();
});

if ('serviceWorker' in navigator && location.hostname !== 'localhost') {
  addEventListener('load', () => void navigator.serviceWorker.register('/sw.js'));
}

if (location.hash) window.setTimeout(focusHashTarget, 0);
