/* AgroGlobalDex — PWA install + service worker registration + mobile bottom nav.
 * Inyectado en cada página vía <script type="module" src="js/pwa-install.js">
 */

// ─── Service worker ──────────────────────────────────────────────────────
if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('sw.js').catch(() => {});
  });
}

// ─── Install prompt (Chrome/Edge/Android) ────────────────────────────────
let deferredPrompt = null;
window.addEventListener('beforeinstallprompt', (e) => {
  e.preventDefault();
  deferredPrompt = e;
  showInstallButton();
});
window.addEventListener('appinstalled', () => {
  deferredPrompt = null;
  hideInstallButton();
});

function ensureInstallStyles() {
  if (document.getElementById('agg-install-style')) return;
  const s = document.createElement('style');
  s.id = 'agg-install-style';
  s.textContent = `
    .agg-install-btn{position:fixed;bottom:78px;right:14px;z-index:200;background:var(--neon,#00FF6A);color:#000;border:none;border-radius:24px;padding:10px 16px;font-weight:700;font-size:13px;font-family:Outfit,sans-serif;cursor:pointer;box-shadow:0 10px 30px rgba(0,255,106,.25);display:flex;gap:7px;align-items:center}
    .agg-install-btn:hover{background:#00e55f}
    @media(min-width:780px){.agg-install-btn{bottom:24px;right:24px}}
    /* mobile bottom tab */
    .agg-mtab{display:none;position:fixed;left:0;right:0;bottom:0;z-index:140;background:rgba(5,8,10,.95);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border-top:1px solid var(--bd,#131D14);padding:6px 4px env(safe-area-inset-bottom,0)}
    .agg-mtab a{flex:1;display:flex;flex-direction:column;align-items:center;gap:3px;padding:8px 4px;color:var(--t2,#7A9080);font-size:10px;font-weight:600;text-decoration:none;font-family:Outfit,sans-serif}
    .agg-mtab a.active,.agg-mtab a:hover{color:var(--neon,#00FF6A)}
    .agg-mtab svg{width:22px;height:22px;stroke:currentColor;fill:none;stroke-width:1.8;stroke-linecap:round;stroke-linejoin:round}
    @media(max-width:780px){
      .agg-mtab{display:flex}
      body{padding-bottom:64px}
    }
  `;
  document.head.appendChild(s);
}

function showInstallButton() {
  ensureInstallStyles();
  if (document.getElementById('agg-install-btn')) return;
  const b = document.createElement('button');
  b.id = 'agg-install-btn';
  b.className = 'agg-install-btn';
  b.innerHTML = '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg> Instalar app';
  b.addEventListener('click', async () => {
    if (!deferredPrompt) return;
    deferredPrompt.prompt();
    await deferredPrompt.userChoice.catch(() => {});
    deferredPrompt = null;
    b.remove();
  });
  document.body.appendChild(b);
}

function hideInstallButton() {
  document.getElementById('agg-install-btn')?.remove();
}

// ─── Mobile bottom tab nav ───────────────────────────────────────────────
function buildMobileTabBar() {
  ensureInstallStyles();
  if (document.getElementById('agg-mtab')) return;
  const here = (location.pathname.split('/').pop() || 'index.html').toLowerCase();
  const tabs = [
    { href: 'index.html',       label: 'Inicio',     svg: '<path d="M3 9.5L12 3l9 6.5V20a1 1 0 0 1-1 1h-5v-7H9v7H4a1 1 0 0 1-1-1V9.5z"/>' },
    { href: 'marketplace.html', label: 'Mercado',    svg: '<rect x="3" y="6" width="18" height="14" rx="2"/><path d="M3 10h18"/><path d="M8 6V4a4 4 0 0 1 8 0v2"/>' },
    { href: 'tokenize.html',    label: 'Tokenizar',  svg: '<circle cx="12" cy="12" r="9"/><path d="M12 7v10M7 12h10"/>' },
    { href: 'invest.html',      label: 'Invertir',   svg: '<path d="M3 17l6-6 4 4 8-8"/><polyline points="14 7 21 7 21 14"/>' },
    { href: 'contact.html',     label: 'Contacto',   svg: '<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.13.96.37 1.91.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.9.34 1.85.57 2.81.7A2 2 0 0 1 22 16.92z"/>' },
  ];
  const bar = document.createElement('nav');
  bar.id = 'agg-mtab';
  bar.className = 'agg-mtab';
  bar.innerHTML = tabs.map(t => {
    const active = (t.href === here) ? 'active' : '';
    return `<a href="${t.href}" class="${active}"><svg viewBox="0 0 24 24">${t.svg}</svg><span>${t.label}</span></a>`;
  }).join('');
  document.body.appendChild(bar);
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', buildMobileTabBar);
} else {
  buildMobileTabBar();
}
