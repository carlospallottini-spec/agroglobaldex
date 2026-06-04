/* AgroGlobalDex — Service Worker (PWA shell + kill-switch para Electron)
 * v6 — Agrega borrow.html, receipts.html, admin.html al precache shell.
 *      Detecta protocolo file:// (Electron / Capacitor desktop) y se
 *      auto-destruye limpiando caches. En navegador web real (https://) se
 *      comporta como PWA shell normal.
 */
const CACHE = 'agroglobaldex-v6';
const IS_FILE = location.protocol === 'file:';
const SHELL = [
  '/',
  '/index.html',
  '/marketplace.html',
  '/tokenize.html',
  '/borrow.html',
  '/invest.html',
  '/receipts.html',
  '/investors.html',
  '/aggregate.html',
  '/admin.html',
  '/about.html',
  '/contact.html',
  '/team.html',
  '/manifest.webmanifest',
  '/js/wallet-adapter.js',
  '/js/agroglobaldex-client.js',
  '/js/network-config.js',
  '/js/kyc-gate.js',
  '/js/mwa-helper.js',
  '/js/pwa-install.js',
  '/js/idl/agroglobaldex.json',
  '/js/idl/compliance_hook.json',
];

self.addEventListener('install', (e) => {
  self.skipWaiting();
  if (IS_FILE) return; // no precachear en Electron
  e.waitUntil(
    caches.open(CACHE).then((c) => c.addAll(SHELL).catch(() => {}))
  );
});

self.addEventListener('activate', (e) => {
  e.waitUntil((async () => {
    // Kill switch en Electron/Capacitor desktop (file://)
    if (IS_FILE) {
      const keys = await caches.keys();
      await Promise.all(keys.map((k) => caches.delete(k)));
      await self.registration.unregister().catch(() => {});
      // Refrescar todos los clients para que vean la versión sin SW
      const clients = await self.clients.matchAll({ type: 'window' });
      clients.forEach((c) => { try { c.navigate(c.url); } catch (_) {} });
      return;
    }
    // Navegador: borrar caches viejos pero quedarse con el actual
    const keys = await caches.keys();
    await Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)));
    await self.clients.claim();
  })());
});

function isStaticAsset(url) {
  return /\.(png|jpe?g|svg|ico|woff2?|ttf|otf|webp|gif)$/i.test(url.pathname);
}

self.addEventListener('fetch', (e) => {
  // En Electron: NO interceptar absolutamente nada. Dejar que el browser
  // engine cargue file:// directamente. Sin esto, navegación = black screen.
  if (IS_FILE) return;

  const req = e.request;
  if (req.method !== 'GET') return;
  const url = new URL(req.url);
  if (url.origin !== location.origin) return;

  if (isStaticAsset(url)) {
    e.respondWith(
      caches.match(req).then((cached) => cached || fetch(req).then((res) => {
        const clone = res.clone();
        caches.open(CACHE).then((c) => c.put(req, clone)).catch(() => {});
        return res;
      }).catch(() => caches.match('/icons/icon-192.png')))
    );
  } else {
    e.respondWith(
      fetch(req).then((res) => {
        const clone = res.clone();
        caches.open(CACHE).then((c) => c.put(req, clone)).catch(() => {});
        return res;
      }).catch(() => caches.match(req).then((c) => c || caches.match('/index.html')))
    );
  }
});
