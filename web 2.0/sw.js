/* AgroGlobalDex — Service Worker (PWA shell + network-first fallback)
 * Estrategia:
 *   - install: precachea el shell mínimo (HTML + JS + manifest)
 *   - fetch:   network-first para HTML/JS/JSON (siempre data fresca on-chain),
 *              cache-first para assets estáticos (fuentes, iconos, imágenes)
 */
const CACHE = 'agroglobaldex-v3';
const SHELL = [
  '/',
  '/index.html',
  '/marketplace.html',
  '/tokenize.html',
  '/invest.html',
  '/aggregate.html',
  '/about.html',
  '/contact.html',
  '/team.html',
  '/manifest.webmanifest',
  '/js/wallet-adapter.js',
  '/js/agroglobaldex-client.js',
  '/js/network-config.js',
  '/js/kyc-gate.js',
  '/js/idl/agroglobaldex.json',
];

self.addEventListener('install', (e) => {
  self.skipWaiting();
  e.waitUntil(
    caches.open(CACHE).then((c) => c.addAll(SHELL).catch(() => {}))
  );
});

self.addEventListener('activate', (e) => {
  e.waitUntil(
    (async () => {
      const keys = await caches.keys();
      await Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)));
      await self.clients.claim();
    })()
  );
});

function isStaticAsset(url) {
  return /\.(png|jpe?g|svg|ico|woff2?|ttf|otf|webp|gif)$/i.test(url.pathname);
}

self.addEventListener('fetch', (e) => {
  const req = e.request;
  if (req.method !== 'GET') return;
  const url = new URL(req.url);

  // Bypass RPC + esm.sh + cross-origin live data
  if (url.origin !== location.origin) return;

  if (isStaticAsset(url)) {
    // cache-first
    e.respondWith(
      caches.match(req).then((cached) => cached || fetch(req).then((res) => {
        const clone = res.clone();
        caches.open(CACHE).then((c) => c.put(req, clone)).catch(() => {});
        return res;
      }).catch(() => caches.match('/icons/icon-192.png')))
    );
  } else {
    // network-first, fallback al shell cacheado
    e.respondWith(
      fetch(req).then((res) => {
        const clone = res.clone();
        caches.open(CACHE).then((c) => c.put(req, clone)).catch(() => {});
        return res;
      }).catch(() => caches.match(req).then((c) => c || caches.match('/index.html')))
    );
  }
});
