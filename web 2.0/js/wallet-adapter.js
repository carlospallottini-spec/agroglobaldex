/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — PHANTOM WALLET ADAPTER
 * Conecta a Phantom (window.solana / window.phantom?.solana),
 * persiste la conexión en localStorage y emite eventos custom:
 *   - wallet:connected   { detail: { publicKey: string } }
 *   - wallet:disconnected
 *
 * Wirea automáticamente todos los botones .nwb y .wallet-btn
 * del DOM (incluido el del nav y el del menú mobile).
 * ═══════════════════════════════════════════════════════════ */

import { truncateAddress } from './network-config.js';

const STORAGE_KEY = 'agroglobaldex_wallet_v2';

const state = {
  publicKey: null,
  connecting: false,
  provider: null,
};

function getProvider() {
  if (state.provider) return state.provider;
  // Phantom inyecta tanto window.solana como window.phantom.solana
  const p = (window.phantom && window.phantom.solana) || window.solana;
  if (p && p.isPhantom) {
    state.provider = p;
    return p;
  }
  return null;
}

function emit(name, detail = null) {
  window.dispatchEvent(new CustomEvent(name, { detail }));
}

function save() {
  if (state.publicKey) {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ pk: state.publicKey }));
  } else {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function load() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    return JSON.parse(raw).pk || null;
  } catch (e) { return null; }
}

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

/** Conecta a Phantom. Si ya está conectada, devuelve el publicKey actual. */
export async function connectWallet({ silent = false } = {}) {
  const provider = getProvider();
  if (!provider) {
    showWalletToast('Phantom no detectado. Instálalo desde phantom.app', 'error');
    setTimeout(() => window.open('https://phantom.app/download', '_blank', 'noopener'), 1200);
    return null;
  }
  if (state.connecting) return null;
  state.connecting = true;
  updateAllButtons();
  try {
    const resp = silent
      ? await provider.connect({ onlyIfTrusted: true }).catch(() => null)
      : await provider.connect();
    if (!resp || !resp.publicKey) {
      state.connecting = false;
      updateAllButtons();
      return null;
    }
    state.publicKey = resp.publicKey.toString();
    state.connecting = false;
    save();
    updateAllButtons();
    emit('wallet:connected', { publicKey: state.publicKey });
    if (!silent) showWalletToast('Wallet conectada: ' + truncateAddress(state.publicKey), 'success');
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
  const provider = getProvider();
  try { await provider?.disconnect(); } catch (e) {}
  state.publicKey = null;
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

/** Devuelve el objeto provider de Phantom (para crear AnchorProvider). */
export function getWalletProvider() {
  return getProvider();
}

/* ═══════ Toast helper (usa #toast-stack si existe, sino crea uno) ═══════ */
function showWalletToast(msg, kind = 'success') {
  // Reutiliza window.showToast si la página lo definió
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

/* ═══════ INIT (auto en cada página) ═══════ */
function bindButtons() {
  document.querySelectorAll('.nwb, .wallet-btn').forEach(btn => {
    if (btn.dataset.walletBound === '1') return;
    btn.dataset.walletBound = '1';
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      if (state.publicKey) {
        // Click en estado conectado: pequeño menú confirm para desconectar
        if (confirm('¿Desconectar la wallet ' + truncateAddress(state.publicKey) + '?')) {
          disconnectWallet();
        }
      } else {
        connectWallet();
      }
    });
  });
}

function init() {
  bindButtons();
  // Provider events
  const provider = getProvider();
  if (provider) {
    provider.on?.('disconnect', () => {
      state.publicKey = null;
      save();
      updateAllButtons();
      emit('wallet:disconnected');
    });
    provider.on?.('accountChanged', (pk) => {
      if (pk) {
        state.publicKey = pk.toString();
        save();
        updateAllButtons();
        emit('wallet:connected', { publicKey: state.publicKey });
      } else {
        disconnectWallet();
      }
    });
  }
  // Reconnect silently si había sesión guardada
  if (load() && provider) {
    connectWallet({ silent: true });
  }
  updateAllButtons();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

// Re-bind cuando se inyecte HTML dinámico
const _mo = new MutationObserver(() => bindButtons());
_mo.observe(document.documentElement, { childList: true, subtree: true });

export default { connectWallet, disconnectWallet, getPublicKey, isConnected, getWalletProvider };
