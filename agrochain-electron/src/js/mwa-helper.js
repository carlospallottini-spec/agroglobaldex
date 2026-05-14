/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — MOBILE WALLET ADAPTER (MWA) HELPER
 *
 * Solana Mobile Wallet Adapter integration for:
 *   - Capacitor Android build (uses MWA via Intent / Wallet Standard)
 *   - Mobile web PWA (uses universal-link deep-links to Phantom/Solflare/Backpack)
 *
 * Strategy:
 *   - If running in Capacitor: load @solana-mobile/mobile-wallet-adapter-protocol-web3js
 *     via CDN and use MWA's transact() API. Falls back to deep-links.
 *   - If running in mobile web (no Capacitor): use the wallets' universal-link
 *     deep-links so they open in-app (their in-wallet browser handles signing).
 *
 * Hooks into wallet-adapter.js: when isMobile() returns true and the user
 * picks a wallet from the modal, we route through this helper instead of
 * the desktop window.solana detection.
 * ═══════════════════════════════════════════════════════════ */

const UA = typeof navigator !== 'undefined' ? navigator.userAgent : '';
const IS_ANDROID = /Android/i.test(UA);
const IS_IOS = /iPhone|iPad|iPod/i.test(UA);
const IS_MOBILE_WEB = (IS_ANDROID || IS_IOS) && !window.Capacitor;
const IS_CAPACITOR = !!window.Capacitor;

export const ENV = {
  isAndroid: IS_ANDROID,
  isIOS: IS_IOS,
  isMobileWeb: IS_MOBILE_WEB,
  isCapacitor: IS_CAPACITOR,
  isDesktop: !IS_ANDROID && !IS_IOS && !IS_CAPACITOR,
};

/** Universal-link deep-links to open the dApp in a wallet's in-app browser. */
function dappUrl() {
  return window.location.origin + window.location.pathname;
}

export const DEEP_LINKS = {
  phantom: () => `https://phantom.app/ul/browse/${encodeURIComponent(dappUrl())}?ref=${encodeURIComponent(window.location.origin)}`,
  solflare: () => `https://solflare.com/ul/v1/browse/${encodeURIComponent(dappUrl())}?ref=${encodeURIComponent(window.location.origin)}`,
  backpack: () => `https://backpack.app/ul/v1/browse/${encodeURIComponent(dappUrl())}`,
  glow: () => `https://glow.app/ul/browse/${encodeURIComponent(dappUrl())}`,
};

/**
 * For a mobile-web user (NOT Capacitor): open the dApp in the wallet's
 * in-app browser. Once inside, the wallet auto-injects `window.solana` etc.
 * and the regular wallet-adapter.js detection kicks in.
 */
export function openInWallet(providerId) {
  const fn = DEEP_LINKS[providerId];
  if (!fn) return false;
  window.location.href = fn();
  return true;
}

/**
 * Capacitor / Saga / Seed Vault path: load MWA on demand from CDN and
 * delegate. Returns a thin facade compatible with wallet-adapter.js's
 * expected provider interface (connect, signTransaction, signAllTransactions,
 * disconnect, publicKey, on/off).
 */
let _mwaPromise = null;
async function loadMwa() {
  if (_mwaPromise) return _mwaPromise;
  _mwaPromise = import('https://esm.sh/@solana-mobile/mobile-wallet-adapter-protocol-web3js@2.2.2');
  return _mwaPromise;
}

export async function connectMwa() {
  const mod = await loadMwa();
  const { transact } = mod;

  return new Promise((resolve, reject) => {
    transact(async (wallet) => {
      try {
        const authResult = await wallet.authorize({
          cluster: 'devnet',
          identity: {
            name: 'AgroGlobalDex',
            uri: window.location.origin,
            icon: '/icons/icon-512.png',
          },
        });
        const pkBase58 = authResult.accounts[0].address; // base58 of the public key
        resolve({ publicKey: pkBase58, authToken: authResult.auth_token, wallet });
      } catch (e) { reject(e); }
    }).catch(reject);
  });
}

/**
 * Helper exposed for wallet-adapter.js to decide whether to use MWA or the
 * desktop window.solana branch.
 */
export function preferredMobilePath() {
  if (IS_CAPACITOR && IS_ANDROID) return 'mwa';
  if (IS_MOBILE_WEB) return 'deeplink';
  return 'desktop';
}

export default {
  ENV, DEEP_LINKS, openInWallet, connectMwa, preferredMobilePath,
};
