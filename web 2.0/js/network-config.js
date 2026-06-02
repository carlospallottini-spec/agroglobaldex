/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — NETWORK CONFIG
 * Constantes de red Solana + helpers de formato.
 * Las claves marcadas con __PLACEHOLDER__ deben sustituirse
 * tras el primer `anchor deploy` (ver web 2.0/README.md).
 * ═══════════════════════════════════════════════════════════ */

export const NETWORK = 'devnet';

export const RPC_ENDPOINTS = {
  devnet: 'https://api.devnet.solana.com',
  mainnet: 'https://api.mainnet-beta.solana.com',
  localnet: 'http://127.0.0.1:8899',
};

/** Program ID — placeholder hasta el primer `anchor deploy`. */
export const PROGRAM_ID = 'G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a';

/** Compliance Hook program ID — second Anchor program in the workspace. */
export const COMPLIANCE_HOOK_PROGRAM_ID = 'GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL';

/** Ruta al IDL generado por Anchor. El integrador puede copiarlo a /web 2.0/js/idl/ */
export const IDL_URL = '../js/idl/agroglobaldex.json';

/** USDC mint en devnet (Circle faucet). Mainnet: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v */
export const USDC_MINT = {
  devnet:  '4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU',
  mainnet: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
};

/** Explorer URLs (Solscan) — switch by network query param. */
export const EXPLORER_BASE = {
  devnet:  'https://solscan.io/tx/{sig}?cluster=devnet',
  mainnet: 'https://solscan.io/tx/{sig}',
  localnet:'https://solscan.io/tx/{sig}?cluster=custom&customUrl=http%3A%2F%2F127.0.0.1%3A8899',
};

export const ACCOUNT_EXPLORER_BASE = {
  devnet:  'https://solscan.io/account/{addr}?cluster=devnet',
  mainnet: 'https://solscan.io/account/{addr}',
  localnet:'https://solscan.io/account/{addr}?cluster=custom&customUrl=http%3A%2F%2F127.0.0.1%3A8899',
};

export function txUrl(sig, net = NETWORK) {
  return EXPLORER_BASE[net].replace('{sig}', sig);
}

export function addrUrl(addr, net = NETWORK) {
  return ACCOUNT_EXPLORER_BASE[net].replace('{addr}', addr);
}

/* ═══════ FORMATTERS ═══════ */
export const LAMPORTS_PER_SOL = 1_000_000_000;

export function formatLamports(lamports, decimals = 4) {
  if (lamports == null) return '—';
  const n = typeof lamports === 'bigint' ? Number(lamports) : Number(lamports);
  return (n / LAMPORTS_PER_SOL).toFixed(decimals) + ' SOL';
}

/** USDC tiene 6 decimales. */
export function formatUsdc(rawAmount, decimals = 2) {
  if (rawAmount == null) return '—';
  const n = typeof rawAmount === 'bigint' ? Number(rawAmount) : Number(rawAmount);
  return (n / 1_000_000).toLocaleString('es-ES', { maximumFractionDigits: decimals }) + ' USDC';
}

export function truncateAddress(addr, head = 4, tail = 4) {
  if (!addr) return '';
  const s = typeof addr === 'string' ? addr : addr.toString();
  if (s.length <= head + tail + 2) return s;
  return `${s.slice(0, head)}…${s.slice(-tail)}`;
}

/** Convierte un array Uint8Array (32 bytes) a hex con prefijo. */
export function bytesToHex(bytes) {
  return '0x' + Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
}

/** ISO-3166 alpha-2 → Uint8Array(2). */
export function jurisdictionToBytes(iso2) {
  if (!iso2 || iso2.length !== 2) return new Uint8Array([0, 0]);
  return new Uint8Array([iso2.charCodeAt(0), iso2.charCodeAt(1)]);
}

export function bytesToJurisdiction(bytes) {
  if (!bytes || bytes.length < 2) return '—';
  return String.fromCharCode(bytes[0]) + String.fromCharCode(bytes[1]);
}
