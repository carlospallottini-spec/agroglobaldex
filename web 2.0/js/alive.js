/* ════════════════════════════════════════════════════════════════════
   AgroGlobalDex — alive.js
   The "vivo" engine. 100% vanilla, dependency-free, defensive:
   every block is try/catch-isolated, so a failure anywhere degrades to
   the classic render (css/alive.css gates JS-dependent looks behind the
   `.ag-alive` flag this file sets on <html>).

   1. Organic particle field (grains/spores) on .hero / .page-hero —
      canvas + rAF, DPR-aware, pauses when tab hidden OR hero off-screen,
      fewer particles on mobile, disabled under prefers-reduced-motion.
   2. Count-up stats when they enter the viewport (exact final text is
      always restored — never breaks the audited copy).
   3. Pointer-tracked card glow (sets --ag-x/--ag-y/--ag-o vars).
   4. Reading-progress hairline (element injection only; CSS animates).
   5. Directional cross-document View Transitions (pageswap/pagereveal).
   6. Bottom app tab-bar on dApp pages for mobile/Capacitor.
   ════════════════════════════════════════════════════════════════════ */
(() => {
  'use strict';

  const reduce = !!(window.matchMedia &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  const small = !!(window.matchMedia &&
    window.matchMedia('(max-width: 760px)').matches);

  try { document.documentElement.classList.add('ag-alive'); } catch (_) {}

  /* ── 5 · Directional View Transitions ──────────────────────────────
     The outgoing page records itself on pagehide; the incoming page reads
     it SYNCHRONOUSLY here (deferred scripts run before first render, so
     the attribute exists before the cross-document transition animates).
     Order in PAGE_ORDER = nav order ⇒ slide left/right feels spatial. */
  const PAGE_ORDER = [
    'index', 'marketplace', 'tokenize', 'borrow', 'invest', 'receipts',
    'aggregate', 'investors', 'about', 'team', 'contact', 'admin', 'legal',
  ];
  const pageKey = (path) => {
    const m = /([^/]*)\.html$/.exec(path || '');
    return m ? m[1] : (/\/$/.test(path || '') ? 'index' : '');
  };
  try {
    const from = sessionStorage.getItem('agNavFrom');
    sessionStorage.removeItem('agNavFrom');
    const a = PAGE_ORDER.indexOf(from);
    const b = PAGE_ORDER.indexOf(pageKey(location.pathname));
    if (a !== -1 && b !== -1 && a !== b) {
      document.documentElement.setAttribute('data-ag-nav', b > a ? 'fwd' : 'back');
    }
  } catch (_) {}
  try {
    window.addEventListener('pagehide', () => {
      try {
        sessionStorage.setItem('agNavFrom', pageKey(location.pathname));
      } catch (_) {}
    });
  } catch (_) {}

  /* ── 1 · Particle field: drifting grains / spores ─────────────────── */
  function initField() {
    if (reduce) return;
    const hero = document.querySelector('.hero, .page-hero');
    if (!hero) return;
    const canvas = document.createElement('canvas');
    canvas.className = 'ag-field';
    canvas.setAttribute('aria-hidden', 'true');
    const ctx = canvas.getContext && canvas.getContext('2d');
    if (!ctx) return;
    hero.insertBefore(canvas, hero.firstChild);

    const DPR = Math.min(window.devicePixelRatio || 1, 2);
    let W = 0, H = 0, parts = [], raf = 0, visible = true, onScreen = true;

    function size() {
      const r = hero.getBoundingClientRect();
      W = Math.max(1, r.width); H = Math.max(1, r.height);
      canvas.width = Math.round(W * DPR);
      canvas.height = Math.round(H * DPR);
      canvas.style.width = W + 'px';
      canvas.style.height = H + 'px';
      ctx.setTransform(DPR, 0, 0, DPR, 0, 0);
      seed();
    }

    function seed() {
      const divisor = small ? 46000 : 20000;
      const cap = small ? 34 : 90;
      const n = Math.max(18, Math.min(cap, Math.round((W * H) / divisor)));
      parts = [];
      for (let i = 0; i < n; i++) {
        const gold = Math.random() < 0.28;           // harvest grains
        parts.push({
          x: Math.random() * W,
          y: Math.random() * H,
          r: 0.6 + Math.random() * (gold ? 1.5 : 1.1),
          vy: -(0.06 + Math.random() * 0.22),        // slow upward drift
          sway: 0.35 + Math.random() * 0.9,          // px of lateral sway
          ph: Math.random() * Math.PI * 2,           // sway phase
          tw: 0.4 + Math.random() * 0.6,             // twinkle depth
          ts: 0.4 + Math.random() * 1.1,             // twinkle speed
          gold,
        });
      }
    }

    let t = 0;
    function frame() {
      raf = 0;
      if (!visible || !onScreen) return;             // fully parked
      t += 0.016;
      ctx.clearRect(0, 0, W, H);
      for (let i = 0; i < parts.length; i++) {
        const p = parts[i];
        p.y += p.vy;
        if (p.y < -4) { p.y = H + 4; p.x = Math.random() * W; }
        const x = p.x + Math.sin(t * p.ts + p.ph) * p.sway * 8;
        const a = 0.12 + p.tw * 0.3 * (0.5 + 0.5 * Math.sin(t * p.ts * 1.7 + p.ph));
        ctx.beginPath();
        ctx.arc(x, p.y, p.r, 0, 6.2832);
        ctx.fillStyle = p.gold
          ? 'rgba(232,200,106,' + a.toFixed(3) + ')'
          : 'rgba(0,255,106,' + (a * 0.9).toFixed(3) + ')';
        ctx.fill();
      }
      raf = requestAnimationFrame(frame);
    }
    function play() { if (!raf && visible && onScreen) raf = requestAnimationFrame(frame); }

    document.addEventListener('visibilitychange', () => {
      visible = !document.hidden; play();
    });
    if ('IntersectionObserver' in window) {
      new IntersectionObserver((en) => {
        onScreen = !!(en[0] && en[0].isIntersecting); play();
      }, { threshold: 0 }).observe(hero);
    }
    let rsz = 0;
    window.addEventListener('resize', () => {
      clearTimeout(rsz); rsz = setTimeout(size, 180);
    }, { passive: true });

    size();
    canvas.classList.add('ag-on');
    play();
  }

  /* ── 2 · Count-up stats ────────────────────────────────────────────
     Conservative: only plain "<prefix><number><suffix>" values where the
     suffix has no further digits ("€280B", "63", "$24M", "6.8K"), read
     from a pure text node. The exact original string is restored at the
     end of the tween, so audited copy is never altered. */
  function initCountUp() {
    if (reduce || !('IntersectionObserver' in window)) return;
    const SEL = '.hstat-v, .h-stat-v, .sv, .mp-bs-n, .stat .v, .pstat .v';
    const RX = /^([^0-9]{0,4})(\d{1,3}(?:[.,]\d{1,3})?)([^0-9]{0,6})$/;
    const targets = [];
    document.querySelectorAll(SEL).forEach((el) => {
      const node = el.firstChild;
      if (!node || node.nodeType !== 3) return;
      const txt = node.nodeValue.trim();
      const m = RX.exec(txt);
      if (!m) return;
      const num = parseFloat(m[2].replace(',', '.'));
      if (!isFinite(num) || num <= 0) return;
      targets.push({ el, node, txt, pre: m[1], num, post: m[3],
        dec: (m[2].split(/[.,]/)[1] || '').length,
        sep: m[2].indexOf(',') !== -1 ? ',' : '.' });
    });
    if (!targets.length) return;

    const ease = (x) => 1 - Math.pow(1 - x, 3);
    function run(tg) {
      const t0 = performance.now(), D = 1300;
      (function step(now) {
        const k = Math.min(1, (now - t0) / D);
        if (k >= 1) { tg.node.nodeValue = tg.txt; return; }
        let v = (tg.num * ease(k)).toFixed(tg.dec);
        if (tg.sep === ',') v = v.replace('.', ',');
        tg.node.nodeValue = tg.pre + v + tg.post;
        requestAnimationFrame(step);
      })(t0);
    }
    const io = new IntersectionObserver((entries) => {
      entries.forEach((en) => {
        if (!en.isIntersecting) return;
        io.unobserve(en.target);
        const tg = targets.find((x) => x.el === en.target);
        if (tg) run(tg);
      });
    }, { threshold: 0.4 });
    targets.forEach((tg) => io.observe(tg.el));
  }

  /* ── 3 · Pointer-tracked card glow ─────────────────────────────────── */
  function initCardGlow() {
    if (reduce) return;
    if (!(window.matchMedia && window.matchMedia('(hover:hover) and (pointer:fine)').matches)) return;
    const SEL = '.card, .pstat, .mp-bs, .hfc';
    let raf = 0, ev = null;
    document.addEventListener('pointermove', (e) => {
      ev = e;
      if (raf) return;
      raf = requestAnimationFrame(() => {
        raf = 0;
        const el = ev.target && ev.target.closest ? ev.target.closest(SEL) : null;
        if (!el) return;
        const r = el.getBoundingClientRect();
        el.style.setProperty('--ag-x', (((ev.clientX - r.left) / r.width) * 100).toFixed(2) + '%');
        el.style.setProperty('--ag-y', (((ev.clientY - r.top) / r.height) * 100).toFixed(2) + '%');
      });
    }, { passive: true });
  }

  /* ── 4 · Reading-progress hairline (CSS scroll-timeline drives it) ── */
  function initProgress() {
    if (reduce) return;
    if (!(window.CSS && CSS.supports && CSS.supports('animation-timeline: scroll()'))) return;
    if (document.querySelector('.ag-progress')) return;
    const bar = document.createElement('div');
    bar.className = 'ag-progress';
    bar.setAttribute('aria-hidden', 'true');
    document.body.appendChild(bar);
  }

  /* ── 6 · Bottom app tab-bar (mobile / Capacitor, dApp pages only) ─── */
  function initTabbar() {
    const key = pageKey(location.pathname);
    const APP = ['marketplace', 'tokenize', 'borrow', 'invest', 'receipts'];
    if (APP.indexOf(key) === -1) return;
    if (document.querySelector('.ag-tabbar')) return;

    const ICONS = {
      marketplace: 'M3 9l9-6 9 6v11a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1zM9 21V12h6v9',
      tokenize: 'M12 3v18M5.5 7.5L12 3l6.5 4.5M5.5 16.5L12 21l6.5-4.5M3 12h18',
      borrow: 'M12 1v22M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6',
      invest: 'M3 17l6-6 4 4 8-8M15 7h6v6',
      receipts: 'M4 3h16v18l-2.5-1.5L15 21l-3-1.5L9 21l-2.5-1.5L4 21zM8 8h8M8 12h8M8 16h5',
    };
    const LABELS = {
      marketplace: 'Mercado', tokenize: 'Tokenizar', borrow: 'Crédito',
      invest: 'Invertir', receipts: 'Recibos',
    };
    const SVG_NS = 'http://www.w3.org/2000/svg';
    const bar = document.createElement('nav');
    bar.className = 'ag-tabbar';
    bar.setAttribute('aria-label', 'Navegación de la app');
    const inner = document.createElement('div');
    inner.className = 'ag-tabbar-inner';
    APP.forEach((k) => {
      const a = document.createElement('a');
      a.className = 'ag-tab';
      a.href = k + '.html';
      if (k === key) a.setAttribute('aria-current', 'page');
      const svg = document.createElementNS(SVG_NS, 'svg');
      svg.setAttribute('viewBox', '0 0 24 24');
      svg.setAttribute('aria-hidden', 'true');
      const path = document.createElementNS(SVG_NS, 'path');
      path.setAttribute('d', ICONS[k]);
      svg.appendChild(path);
      const label = document.createElement('span');
      label.textContent = LABELS[k];
      a.appendChild(svg);
      a.appendChild(label);
      inner.appendChild(a);
    });
    bar.appendChild(inner);
    document.body.appendChild(bar);
    document.body.classList.add('ag-has-tabbar');
  }

  function boot() {
    try { initField(); } catch (_) {}
    try { initCountUp(); } catch (_) {}
    try { initCardGlow(); } catch (_) {}
    try { initProgress(); } catch (_) {}
    try { initTabbar(); } catch (_) {}
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }
})();
