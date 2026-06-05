/* ════════════════════════════════════════════════════════════════════
   AgroGlobalDex — ux.js
   Tiny, dependency-free progressive-enhancement layer for high-quality
   interactions: tactile button ripples + scroll-reveal. Safe to load on
   every page; degrades to nothing if features are unavailable, and fully
   respects prefers-reduced-motion. Works in the Capacitor mobile webview.
   ════════════════════════════════════════════════════════════════════ */
(() => {
  'use strict';
  const reduce = window.matchMedia &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  /* ── Tactile ripple on buttons ─────────────────────────────────────── */
  const RIPPLE_SEL = '.btn-p, .btn-s, .nbtn-p, .nbtn-g, .nwb';
  function spawnRipple(e) {
    const el = e.target.closest(RIPPLE_SEL);
    if (!el || el.disabled) return;
    el.classList.add('ag-ripple-host');
    const r = el.getBoundingClientRect();
    const size = Math.max(r.width, r.height);
    const span = document.createElement('span');
    span.className = 'ag-ripple';
    span.style.width = span.style.height = size + 'px';
    const x = (e.clientX ?? r.left + r.width / 2) - r.left - size / 2;
    const y = (e.clientY ?? r.top + r.height / 2) - r.top - size / 2;
    span.style.left = x + 'px';
    span.style.top = y + 'px';
    el.appendChild(span);
    span.addEventListener('animationend', () => span.remove());
  }
  if (!reduce) {
    document.addEventListener('pointerdown', spawnRipple, { passive: true });
  }

  /* ── Scroll reveal for opted-in elements (.ag-reveal) ──────────────── */
  function initReveal() {
    const items = document.querySelectorAll('.ag-reveal');
    if (!items.length) return;
    if (reduce || !('IntersectionObserver' in window)) {
      items.forEach((el) => el.classList.add('ag-in'));
      return;
    }
    const io = new IntersectionObserver((entries) => {
      entries.forEach((en) => {
        if (en.isIntersecting) {
          en.target.classList.add('ag-in');
          io.unobserve(en.target);
        }
      });
    }, { threshold: 0.12, rootMargin: '0px 0px -8% 0px' });
    items.forEach((el) => io.observe(el));
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initReveal);
  } else {
    initReveal();
  }
})();
