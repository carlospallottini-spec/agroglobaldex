/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — KYC GATE
 * Verifica on-chain si una wallet tiene ComplianceRecord
 * con kyc_verified=true y jurisdicción no bloqueada.
 * Usado por tokenize.html y por el flujo de buy.
 * ═══════════════════════════════════════════════════════════ */

import { fetchComplianceRecord } from './agroglobaldex-client.js';
import { bytesToJurisdiction } from './network-config.js';

const BLOCKED_JURISDICTIONS = ['KP', 'IR', 'SY', 'CU'];

/** Resultado:
 *   { ok: true, record }                    -> wallet apta
 *   { ok: false, reason: 'no-record' }      -> nunca pasó KYC
 *   { ok: false, reason: 'not-verified' }   -> existe record pero kyc_verified=false
 *   { ok: false, reason: 'blocked' }        -> jurisdicción en blocklist
 */
export async function checkKyc(walletAddress) {
  if (!walletAddress) return { ok: false, reason: 'no-wallet' };
  const record = await fetchComplianceRecord(walletAddress);
  if (!record) return { ok: false, reason: 'no-record' };
  if (!record.kycVerified && !record.kyc_verified) {
    return { ok: false, reason: 'not-verified', record };
  }
  const iso2 = bytesToJurisdiction(record.jurisdiction);
  if (BLOCKED_JURISDICTIONS.includes(iso2)) {
    return { ok: false, reason: 'blocked', record, jurisdiction: iso2 };
  }
  return { ok: true, record, jurisdiction: iso2 };
}

export function reasonToMessage(reason, jurisdiction) {
  switch (reason) {
    case 'no-wallet':
      return 'Conectá tu wallet Phantom para continuar.';
    case 'no-record':
      return 'Tu wallet aún no completó KYC. Solicitá la verificación al operador del marketplace antes de tokenizar.';
    case 'not-verified':
      return 'El operador todavía no aprobó tu KYC. Recibirás un mail cuando esté listo.';
    case 'blocked':
      return `La jurisdicción (${jurisdiction}) está bloqueada por la política regulatoria de AgroGlobalDex. Contactanos si creés que es un error.`;
    default:
      return 'No se pudo verificar el KYC. Probá de nuevo o contactá soporte.';
  }
}
