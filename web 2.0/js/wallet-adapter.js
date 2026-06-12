/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — MULTI-WALLET ADAPTER
 *
 * Soporta Phantom, Solflare, Backpack y Glow detectando el
 * provider inyectado por la extensión/dApp browser. NO depende
 * de npm; toda la web es ESM por CDN, así que evitamos los
 * paquetes @solana/wallet-adapter-* (requieren Buffer/global).
 *
 * Eventos emitidos sobre window:
 *   - wallet:connected    { detail: { publicKey, provider } }
 *   - wallet:disconnected
 *
 * Auto-bindea botones .nwb y .wallet-btn y abre el modal picker
 * de wallets. Persiste sesión en localStorage para reconexión
 * silenciosa al recargar.
 * ═══════════════════════════════════════════════════════════ */

import { truncateAddress } from './network-config.js';
import { preferredMobilePath, openInWallet } from './mwa-helper.js';

const STORAGE_KEY = 'agroglobaldex_wallet_v3';

/** Catálogo de wallets soportadas. icon = SVG inline para no depender de CDN. */
const WALLETS = [
  {
    id: 'phantom',
    name: 'Phantom',
    chain: 'Solana · SPL Tokens',
    deepLink: 'https://phantom.app/download',
    icon: `<svg viewBox="0 0 40 40"><circle cx="20" cy="20" r="19" fill="#AB9FF2"/><path d="M29 16c0-4.4-3.6-8-9-8s-9 3.6-9 8c0 4.2 3.4 7.5 7.5 7.5.7 0 1.4-.1 2-.3 3.5-1 6.5-4.2 6.5-7.2z" fill="white"/><ellipse cx="15.5" cy="16.5" rx="1.5" ry="2" fill="#AB9FF2"/><ellipse cx="21.5" cy="16.5" rx="1.5" ry="2" fill="#AB9FF2"/></svg>`,
    detect() {
      const p = (window.phantom && window.phantom.solana) || window.solana;
      return p && p.isPhantom ? p : null;
    },
  },
  {
    id: 'solflare',
    name: 'Solflare',
    chain: 'Solana · Multi-device',
    deepLink: 'https://solflare.com/download',
    icon: `<svg viewBox="0 0 40 40"><circle cx="20" cy="20" r="19" fill="#FC8B0F"/><path d="M20 8c-1.3 6.5-4.5 9.7-11 11 6.5 1.3 9.7 4.5 11 11 1.3-6.5 4.5-9.7 11-11-6.5-1.3-9.7-4.5-11-11z" fill="#fff"/></svg>`,
    detect() {
      // Solflare inyecta window.solflare con isSolflare=true
      const s = window.solflare;
      if (s && s.isSolflare) return s;
      return null;
    },
  },
  {
    id: 'backpack',
    name: 'Backpack',
    chain: 'Solana · xNFT',
    deepLink: 'https://backpack.app/downloads',
    icon: `<svg viewBox="0 0 40 40"><rect x="6" y="10" width="28" height="24" rx="6" fill="#E33E3F"/><rect x="11" y="6" width="18" height="10" rx="5" fill="#E33E3F" stroke="#fff" stroke-width="1.5" fill-opacity=".5"/><rect x="6" y="20" width="28" height="3" fill="#fff" fill-opacity=".25"/></svg>`,
    detect() {
      // Backpack inyecta window.backpack y también window.xnft?.solana
      const b = window.backpack;
      if (b && (b.isBackpack || b.isXnft)) return b;
      const x = window.xnft?.solana;
      if (x && x.isBackpack) return x;
      return null;
    },
  },
  {
    id: 'glow',
    name: 'Glow',
    chain: 'Solana · iOS/Android',
    deepLink: 'https://glow.app/download',
    icon: `<svg viewBox="0 0 40 40"><defs><radialGradient id="gg" cx=".5" cy=".5" r=".5"><stop offset="0" stop-color="#A0FF66"/><stop offset="1" stop-color="#00FF6A"/></radialGradient></defs><circle cx="20" cy="20" r="19" fill="url(#gg)"/><circle cx="20" cy="20" r="8" fill="#fff" fill-opacity=".85"/></svg>`,
    detect() {
      const g = window.glowSolana || window.glow?.solana;
      return g || null;
    },
  },
];

const state = {
  publicKey: null,
  connecting: false,
  providerObj: null,   // wallet provider object (the actual injected wallet API)
  providerId: null,    // 'phantom' | 'solflare' | 'backpack' | 'glow'
};

function emit(name, detail = null) {
  window.dispatchEvent(new CustomEvent(name, { detail }));
}

function save() {
  if (state.publicKey && state.providerId) {
    localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({ provider: state.providerId, publicKey: state.publicKey }),
    );
  } else {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function load() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    const o = JSON.parse(raw);
    if (!o.provider || !o.publicKey) return null;
    return o;
  } catch (e) { return null; }
}

function findWallet(id) {
  return WALLETS.find(w => w.id === id);
}

function getProviderById(id) {
  const w = findWallet(id);
  if (!w) return null;
  return w.detect();
}

/** Devuelve el catálogo de wallets con estado de instalación. */
export function getInstalledWallets() {
  return WALLETS.map(w => ({
    id: w.id,
    name: w.name,
    icon: w.icon,
    chain: w.chain,
    installed: !!w.detect(),
    deepLink: w.deepLink,
  }));
}

/* ═══════ BUTTON UI HELPERS ═══════ */
function setButtonState(btn, label, connected) {
  if (!btn) return;
  const span = btn.querySelector('span');
  if (span) span.textContent = label;
  if (connected) btn.classList.add('connected');
  else btn.classList.remove('connected');
}

function updateAllButtons() {
  const buttons = document.querySelectorAll('.nwb, .wallet-btn');
  if (state.connecting) {
    buttons.forEach(b => setButtonState(b, 'Conectando…', false));
    return;
  }
  if (state.publicKey) {
    const label = truncateAddress(state.publicKey, 4, 4);
    buttons.forEach(b => setButtonState(b, label, true));
  } else {
    buttons.forEach(b => setButtonState(b, 'Conectar Wallet', false));
  }
}

/* ═══════ CONNECT / DISCONNECT ═══════ */
/**
 * Conecta una wallet. Si no se especifica `provider`, abre el modal picker.
 * `silent`: solo intenta `onlyIfTrusted`, no abre popup.
 */
export async function connectWallet({ provider, silent = false } = {}) {
  if (state.connecting) return null;

  if (!provider) {
    if (silent) {
      // Reconexión silenciosa: usar el provider guardado si existe.
      const saved = load();
      if (saved) provider = saved.provider;
      else return null;
    } else {
      // Abrir picker, el modal llamará de nuevo con { provider }.
      openWalletPicker();
      return null;
    }
  }

  const providerObj = getProviderById(provider);
  if (!providerObj) {
    // Mobile-web fallback: if the device is mobile and the wallet has a
    // universal-link deep link, send the user into the wallet's in-app
    // browser. Once there, window.solana etc. is injected and reconnect
    // happens silently on next load.
    if (!silent && preferredMobilePath() === 'deeplink') {
      const opened = openInWallet(provider);
      if (opened) return null;
    }
    const w = findWallet(provider);
    if (!silent && w) {
      showWalletToast(`${w.name} no detectada. Abriendo descarga…`, 'error');
      setTimeout(() => window.open(w.deepLink, '_blank', 'noopener'), 1200);
    }
    return null;
  }

  state.connecting = true;
  updateAllButtons();
  try {
    const resp = silent
      ? await providerObj.connect({ onlyIfTrusted: true }).catch(() => null)
      : await providerObj.connect();
    if (!resp || !resp.publicKey) {
      state.connecting = false;
      updateAllButtons();
      return null;
    }
    state.publicKey = resp.publicKey.toString();
    state.providerObj = providerObj;
    state.providerId = provider;
    state.connecting = false;
    save();
    bindProviderEvents(providerObj);
    updateAllButtons();
    closeWalletPicker();
    emit('wallet:connected', { publicKey: state.publicKey, provider });
    if (!silent) {
      const w = findWallet(provider);
      showWalletToast(`${w?.name || 'Wallet'} conectada: ` + truncateAddress(state.publicKey), 'success');
    }
    return state.publicKey;
  } catch (err) {
    state.connecting = false;
    updateAllButtons();
    if (err?.code === 4001 || err?.message?.includes('User rejected')) {
      if (!silent) showWalletToast('Conexión rechazada', 'error');
    } else if (!silent) {
      showWalletToast('Error al conectar: ' + (err?.message || 'desconocido'), 'error');
    }
    return null;
  }
}

export async function disconnectWallet() {
  try { await state.providerObj?.disconnect?.(); } catch (e) {}
  state.publicKey = null;
  state.providerObj = null;
  state.providerId = null;
  save();
  updateAllButtons();
  emit('wallet:disconnected');
}

export function getPublicKey() {
  return state.publicKey;
}

export function isConnected() {
  return !!state.publicKey;
}

/** Devuelve el provider activo (para crear AnchorProvider). */
export function getWalletProvider() {
  return state.providerObj;
}

/** Devuelve el id del provider activo ('phantom', 'solflare', etc.). */
export function getProviderId() {
  return state.providerId;
}

/* ═══════ PROVIDER EVENTS ═══════ */
function bindProviderEvents(provider) {
  if (!provider || provider.__aggBound) return;
  provider.__aggBound = true;
  provider.on?.('disconnect', () => {
    state.publicKey = null;
    state.providerObj = null;
    state.providerId = null;
    save();
    updateAllButtons();
    emit('wallet:disconnected');
  });
  provider.on?.('accountChanged', (pk) => {
    if (pk) {
      state.publicKey = pk.toString();
      save();
      updateAllButtons();
      emit('wallet:connected', { publicKey: state.publicKey, provider: state.providerId });
    } else {
      disconnectWallet();
    }
  });
}

/* ═══════ TOAST HELPER ═══════ */
function showWalletToast(msg, kind = 'success') {
  if (typeof window.showToast === 'function') return window.showToast(msg, kind);
  let stack = document.getElementById('toast-stack');
  if (!stack) {
    stack = document.createElement('div');
    stack.id = 'toast-stack';
    stack.style.cssText = 'position:fixed;bottom:24px;right:24px;z-index:600;display:flex;flex-direction:column;gap:8px';
    document.body.appendChild(stack);
  }
  const t = document.createElement('div');
  t.className = 'toast show ' + kind;
  t.innerHTML = `<div class="toast-ico">${kind === 'error' ? '!' : '✓'}</div><div class="toast-txt">${msg}</div>`;
  stack.appendChild(t);
  setTimeout(() => { t.style.opacity = '0'; setTimeout(() => t.remove(), 350); }, 3800);
}

/* ═══════ MODAL PICKER (idempotente, single-instance) ═══════ */
const MODAL_ID = 'agg-wallet-picker';

function ensurePickerStyles() {
  if (document.getElementById('agg-wallet-picker-style')) return;
  const s = document.createElement('style');
  s.id = 'agg-wallet-picker-style';
  s.textContent = `
    .agg-wm-overlay{position:fixed;inset:0;background:rgba(5,8,10,.85);backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px);z-index:9000;display:none;align-items:center;justify-content:center;padding:1rem;opacity:0;transition:opacity .2s}
    .agg-wm-overlay.open{display:flex;opacity:1}
    .agg-wm-modal{background:var(--bg2,#0A0F0C);border:1px solid var(--bd,#131D14);border-radius:18px;padding:28px;max-width:420px;width:100%;position:relative;transform:scale(.96);transition:transform .25s;font-family:var(--fb,'Outfit',sans-serif);color:var(--txt,#ECF0EC)}
    .agg-wm-overlay.open .agg-wm-modal{transform:scale(1)}
    .agg-wm-close{position:absolute;top:14px;right:14px;width:30px;height:30px;border-radius:8px;background:var(--bg3,#101610);border:1px solid var(--bd,#131D14);color:var(--t2,#7A9080);font-size:13px;cursor:pointer;display:flex;align-items:center;justify-content:center;font-family:inherit}
    .agg-wm-close:hover{border-color:var(--nm,#00FF6A28);color:var(--neon,#00FF6A)}
    .agg-wm-title{font-family:var(--fd,'DM Serif Display',serif);font-size:22px;margin-bottom:6px;letter-spacing:-.02em}
    .agg-wm-sub{font-size:13px;color:var(--t2,#7A9080);margin-bottom:20px;line-height:1.6}
    .agg-wm-option{display:flex;align-items:center;gap:14px;padding:13px 16px;border:1px solid var(--bd2,#1E2D20);background:var(--bg3,#101610);border-radius:12px;margin-bottom:8px;cursor:pointer;transition:all .18s;width:100%;text-align:left;font-family:inherit;color:inherit}
    .agg-wm-option:hover{border-color:var(--nm,#00FF6A28);background:var(--nd,#00FF6A0D);transform:translateX(3px)}
    .agg-wm-option svg{width:32px;height:32px;flex-shrink:0}
    .agg-wm-info{flex:1;min-width:0}
    .agg-wm-name{font-size:14px;font-weight:700;color:var(--txt,#ECF0EC);margin-bottom:2px;display:flex;align-items:center;gap:8px}
    .agg-wm-sub2{font-size:11px;color:var(--t3,#647E6A)}
    .agg-wm-arr{color:var(--t3,#647E6A);font-size:14px}
    .agg-wm-badge{font-size:9px;font-weight:700;background:var(--nd,#00FF6A0D);color:var(--neon,#00FF6A);border:1px solid var(--nm,#00FF6A28);border-radius:20px;padding:1px 7px;text-transform:uppercase;letter-spacing:.04em}
    .agg-wm-badge.off{background:#F0A02012;color:#F0A020;border-color:#F0A02055}
    .agg-wm-note{font-size:11px;color:var(--t3,#647E6A);text-align:center;margin-top:14px;line-height:1.6}
    .agg-wm-note a{color:var(--neon,#00FF6A);text-decoration:none}
    .agg-wm-disc{width:100%;padding:10px;margin-top:14px;background:transparent;border:1px solid var(--bd,#131D14);border-radius:8px;font-size:12px;color:var(--t3,#647E6A);cursor:pointer;font-family:inherit}
    .agg-wm-disc:hover{border-color:#FF3D4A55;color:#FF3D4A}
  `;
  document.head.appendChild(s);
}

function buildPickerDOM() {
  if (document.getElementById(MODAL_ID)) return;
  ensurePickerStyles();
  const overlay = document.createElement('div');
  overlay.id = MODAL_ID;
  overlay.className = 'agg-wm-overlay';
  overlay.innerHTML = `<div class="agg-wm-modal" role="dialog" aria-modal="true" aria-label="Conectar wallet" tabindex="-1">
    <button class="agg-wm-close" aria-label="Cerrar">✕</button>
    <div class="agg-wm-title">Conectar wallet</div>
    <div class="agg-wm-sub">Elegí tu wallet Solana. Si todavía no la tenés instalada, hacé click para descargarla.</div>
    <div id="agg-wm-list"></div>
    <div class="agg-wm-note">Nunca almacenamos tus claves privadas. Las firmas viven en tu wallet.</div>
    <button class="agg-wm-disc" id="agg-wm-disc" style="display:none">Desconectar wallet actual</button>
  </div>`;
  document.body.appendChild(overlay);
  overlay.addEventListener('click', (e) => { if (e.target === overlay) closeWalletPicker(); });
  overlay.querySelector('.agg-wm-close').addEventListener('click', closeWalletPicker);
  overlay.querySelector('#agg-wm-disc').addEventListener('click', async () => {
    await disconnectWallet();
    closeWalletPicker();
  });
}

function renderPickerList() {
  const list = document.getElementById('agg-wm-list');
  if (!list) return;
  const wallets = getInstalledWallets();
  list.innerHTML = wallets.map(w => `
    <button class="agg-wm-option" data-w="${w.id}">
      ${w.icon}
      <div class="agg-wm-info">
        <div class="agg-wm-name">${w.name}
          <span class="agg-wm-badge ${w.installed ? '' : 'off'}">${w.installed ? 'Detectada' : 'Instalar'}</span>
        </div>
        <div class="agg-wm-sub2">${w.chain}</div>
      </div>
      <div class="agg-wm-arr">→</div>
    </button>
  `).join('');
  list.querySelectorAll('button[data-w]').forEach(btn => {
    btn.addEventListener('click', async () => {
      const id = btn.dataset.w;
      const w = wallets.find(x => x.id === id);
      if (!w) return;
      if (!w.installed) {
        window.open(w.deepLink, '_blank', 'noopener');
        return;
      }
      await connectWallet({ provider: id });
    });
  });
  const disc = document.getElementById('agg-wm-disc');
  if (disc) disc.style.display = state.publicKey ? 'block' : 'none';
}

/* ── Focus management for the picker (a11y): trap Tab within the dialog,
   close on Escape, and restore focus to the trigger on close. Dependency-free
   and defensive — never throws if the DOM is missing. ── */
let _pickerTrigger = null;   // element to refocus after close
let _pickerKeydown = null;   // bound keydown handler while open

function pickerFocusable(modal) {
  if (!modal) return [];
  const sel = 'a[href], button:not([disabled]), input:not([disabled]), [tabindex]:not([tabindex="-1"])';
  return Array.from(modal.querySelectorAll(sel))
    .filter((el) => el.offsetParent !== null || el === document.activeElement);
}

function onPickerKeydown(e) {
  const o = document.getElementById(MODAL_ID);
  if (!o || !o.classList.contains('open')) return;
  if (e.key === 'Escape' || e.key === 'Esc') {
    e.preventDefault();
    closeWalletPicker();
    return;
  }
  if (e.key !== 'Tab') return;
  const modal = o.querySelector('.agg-wm-modal');
  const items = pickerFocusable(modal);
  if (!items.length) return;
  const first = items[0];
  const last = items[items.length - 1];
  const active = document.activeElement;
  if (e.shiftKey) {
    if (active === first || !modal.contains(active)) {
      e.preventDefault();
      last.focus();
    }
  } else if (active === last || !modal.contains(active)) {
    e.preventDefault();
    first.focus();
  }
}

export function openWalletPicker() {
  buildPickerDOM();
  renderPickerList();
  const o = document.getElementById(MODAL_ID);
  if (o) {
    // Remember what had focus so we can restore it on close.
    _pickerTrigger = (document.activeElement instanceof HTMLElement) ? document.activeElement : null;
    o.classList.add('open');
    document.body.style.overflow = 'hidden';
    // Trap Tab + handle Escape while open.
    _pickerKeydown = onPickerKeydown;
    document.addEventListener('keydown', _pickerKeydown, true);
    // Move focus into the modal (first focusable, else the modal itself).
    const modal = o.querySelector('.agg-wm-modal');
    const items = pickerFocusable(modal);
    const target = items[0] || modal;
    try { target?.focus?.(); } catch (_) {}
  }
}

export function closeWalletPicker() {
  const o = document.getElementById(MODAL_ID);
  if (o) {
    o.classList.remove('open');
    document.body.style.overflow = '';
  }
  if (_pickerKeydown) {
    document.removeEventListener('keydown', _pickerKeydown, true);
    _pickerKeydown = null;
  }
  // Restore focus to whatever opened the picker.
  if (_pickerTrigger && typeof _pickerTrigger.focus === 'function') {
    try { _pickerTrigger.focus(); } catch (_) {}
  }
  _pickerTrigger = null;
}

/* ═══════ INIT + BUTTON BINDING ═══════ */
function bindButtons() {
  document.querySelectorAll('.nwb, .wallet-btn').forEach(btn => {
    if (btn.dataset.walletBound === '1') return;
    btn.dataset.walletBound = '1';
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      // Si ya está conectado, mostrar el picker con opción de desconectar.
      openWalletPicker();
    });
  });
}

function init() {
  bindButtons();
  // Reconnect silently si había sesión guardada
  const saved = load();
  if (saved) {
    connectWallet({ provider: saved.provider, silent: true });
  }
  updateAllButtons();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

// Re-bind cuando se inyecte HTML dinámico (botones en menús mobile, etc.)
const _mo = new MutationObserver(() => bindButtons());
_mo.observe(document.documentElement, { childList: true, subtree: true });

export default {
  connectWallet,
  disconnectWallet,
  getPublicKey,
  isConnected,
  getWalletProvider,
  getProviderId,
  getInstalledWallets,
  openWalletPicker,
  closeWalletPicker,
};
