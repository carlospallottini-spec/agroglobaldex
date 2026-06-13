/* ════════════════════════════════════════════════════════════════════
   AgroGlobalDex — immersive.js
   Progressive-enhancement "living" layer: animated aurora, cursor-reactive
   spotlight, and scroll-reveal. 100% additive and defensive — if anything
   throws, the page still works and nothing stays hidden (the CSS only hides
   reveal elements once this script has flagged <html> AND wired the observer).
   Fully respects prefers-reduced-motion. Safe in the Capacitor webview.
   ════════════════════════════════════════════════════════════════════ */
(() => {
  'use strict';
  const reduce = window.matchMedia &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  // Flag <html> so immersive.css activates (no flag ⇒ classic render).
  try { document.documentElement.classList.add('ag-immersive'); } catch (_) {}

  function boot() {
    // ── Aurora background ────────────────────────────────────────────────
    try {
      if (!document.querySelector('.ag-aurora')) {
        const aurora = document.createElement('div');
        aurora.className = 'ag-aurora';
        aurora.setAttribute('aria-hidden', 'true');
        document.body.insertBefore(aurora, document.body.firstChild);
      }
    } catch (_) {}

    // ── Cursor-reactive spotlight (desktop, motion-on only) ──────────────
    if (!reduce && window.matchMedia('(pointer:fine)').matches) {
      let raf = 0, mx = 50, my = 30;
      window.addEventListener('pointermove', (e) => {
        mx = (e.clientX / window.innerWidth) * 100;
        my = (e.clientY / window.innerHeight) * 100;
        if (!raf) raf = requestAnimationFrame(() => {
          try {
            document.body.style.setProperty('--ag-mx', mx + '%');
            document.body.style.setProperty('--ag-my', my + '%');
          } catch (_) {}
          raf = 0;
        });
      }, { passive: true });
    }

    // ── Scroll reveal ────────────────────────────────────────────────────
    try {
      // Conservative, decorative targets only (never hides critical content;
      // a failsafe reveals anything left over).
      const els = Array.from(document.querySelectorAll('.card, .pstat, .step'));
      if (!els.length) return;
      if (reduce || !('IntersectionObserver' in window)) return; // leave visible

      const io = new IntersectionObserver((entries) => {
        entries.forEach((en) => {
          if (en.isIntersecting) {
            en.target.classList.add('ag-shown');
            io.unobserve(en.target);
          }
        });
      }, { threshold: 0.08, rootMargin: '0px 0px -6% 0px' });

      els.forEach((el, i) => {
        el.setAttribute('data-reveal', String((i % 3) + 1));
        io.observe(el);
      });

      // Failsafe: never leave anything hidden.
      setTimeout(() => {
        document.querySelectorAll('[data-reveal]:not(.ag-shown)')
          .forEach((el) => el.classList.add('ag-shown'));
      }, 1800);
    } catch (_) {
      // On any failure, force everything visible.
      try {
        document.querySelectorAll('[data-reveal]')
          .forEach((el) => el.classList.add('ag-shown'));
      } catch (__) {}
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }
})();
