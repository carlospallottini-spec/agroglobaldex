/* ═══════════════════════════════════════════════════════════
 * AGROGLOBALDEX — ANCHOR CLIENT (browser ES module)
 *
 * Wrapper sobre @coral-xyz/anchor 0.31.1.
 * Alineado con el programa final (lib.rs):
 *   initialize, register_asset, mint_token, update_kyc,
 *   list_asset, buy_asset, redeem,
 *   aggregate_external_asset, update_external_asset
 *
 * Las seeds y el orden de args coinciden con el programa
 * en solana/programs/agroglobaldex/src/.
 * ═══════════════════════════════════════════════════════════ */

import {
  AnchorProvider, Program, web3, BN,
} from 'https://esm.sh/@coral-xyz/anchor@0.31.1';

import {
  NETWORK, RPC_ENDPOINTS, PROGRAM_ID, IDL_URL, USDC_MINT,
} from './network-config.js';

import { getWalletProvider, getPublicKey } from './wallet-adapter.js';

const { PublicKey, Connection, SystemProgram } = web3;

const TOKEN_2022_PROGRAM_ID = new PublicKey('TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb');
const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
const SYSVAR_RENT_PUBKEY = new PublicKey('SysvarRent111111111111111111111111111111111');

let _idl = null;
let _program = null;
let _connection = null;

async function loadIdl() {
  if (_idl) return _idl;
  const res = await fetch(IDL_URL).catch(() => null);
  if (!res || !res.ok) {
    throw new Error(`No se pudo cargar el IDL desde ${IDL_URL}. Tras 'anchor build' copiar target/idl/agroglobaldex.json a /web 2.0/js/idl/`);
  }
  _idl = await res.json();
  return _idl;
}

function programIdPk() {
  return new PublicKey(PROGRAM_ID);
}

export async function getReadConnection(network = NETWORK) {
  if (_connection) return _connection;
  _connection = new Connection(RPC_ENDPOINTS[network] || RPC_ENDPOINTS.devnet, 'confirmed');
  return _connection;
}

export async function getReadProgram(network = NETWORK) {
  const idl = await loadIdl();
  const connection = await getReadConnection(network);
  const dummyWallet = {
    publicKey: PublicKey.default,
    signTransaction: async (tx) => tx,
    signAllTransactions: async (txs) => txs,
  };
  const provider = new AnchorProvider(connection, dummyWallet, { commitment: 'confirmed' });
  return new Program(idl, provider);
}

export async function getProgram(network = NETWORK) {
  if (_program) return _program;
  const idl = await loadIdl();
  const endpoint = RPC_ENDPOINTS[network] || RPC_ENDPOINTS.devnet;
  _connection = new Connection(endpoint, 'confirmed');

  const phantom = getWalletProvider();
  if (!phantom || !getPublicKey()) {
    throw new Error('Wallet no conectada. Conectá Phantom antes de invocar el programa.');
  }
  const wallet = {
    publicKey: new PublicKey(getPublicKey()),
    signTransaction: (tx) => phantom.signTransaction(tx),
    signAllTransactions: (txs) => phantom.signAllTransactions(txs),
  };
  const provider = new AnchorProvider(_connection, wallet, { commitment: 'confirmed' });
  _program = new Program(idl, provider);
  return _program;
}

/* ═══════ PDA HELPERS (alineado con programa) ═══════ */
// marketplace seed = [b"marketplace", authority]
export function findMarketplacePda(authority) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('marketplace'), new PublicKey(authority).toBuffer()],
    programIdPk(),
  )[0];
}
export function findComplianceAuthorityPda(marketplace) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('compliance_authority'), new PublicKey(marketplace).toBuffer()],
    programIdPk(),
  )[0];
}
export function findTreasuryPda(marketplace) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('treasury'), new PublicKey(marketplace).toBuffer()],
    programIdPk(),
  )[0];
}
// asset_registry seed = [b"asset_registry", marketplace, asset_count_le_bytes]
export function findAssetRegistryPda(marketplace, index) {
  const idx = new BN(index).toArrayLike(Buffer, 'le', 8);
  return PublicKey.findProgramAddressSync(
    [Buffer.from('asset_registry'), new PublicKey(marketplace).toBuffer(), idx],
    programIdPk(),
  )[0];
}
export function findAssetMintPda(assetRegistry) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('asset_mint'), new PublicKey(assetRegistry).toBuffer()],
    programIdPk(),
  )[0];
}
export function findComplianceRecordPda(marketplace, wallet) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('compliance_record'),
     new PublicKey(marketplace).toBuffer(),
     new PublicKey(wallet).toBuffer()],
    programIdPk(),
  )[0];
}
export function findListingPda(assetRegistry, seller) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('listing'),
     new PublicKey(assetRegistry).toBuffer(),
     new PublicKey(seller).toBuffer()],
    programIdPk(),
  )[0];
}
export function findExternalAssetPda(marketplace, index) {
  const idx = new BN(index).toArrayLike(Buffer, 'le', 8);
  return PublicKey.findProgramAddressSync(
    [Buffer.from('external_asset'), new PublicKey(marketplace).toBuffer(), idx],
    programIdPk(),
  )[0];
}

/* ═══════ FETCHERS (lectura on-chain) ═══════ */
function ns(program, candidates) {
  for (const n of candidates) {
    if (program.account?.[n]) return program.account[n];
  }
  return null;
}

export async function fetchMarketplace(authority, network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const pda = findMarketplacePda(authority);
    const a = ns(program, ['marketplace']);
    if (!a) return null;
    return await a.fetchNullable(pda);
  } catch (e) {
    console.warn('[agroglobaldex] fetchMarketplace failed:', e.message);
    return null;
  }
}

export async function fetchAllListings(network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const a = ns(program, ['marketplaceListing']);
    if (!a) return [];
    const list = await a.all();
    return list.map(({ publicKey, account }) => ({
      publicKey: publicKey.toString(),
      ...account,
    }));
  } catch (e) {
    console.warn('[agroglobaldex] fetchAllListings failed:', e.message);
    return [];
  }
}

export async function fetchAllAssets(network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const a = ns(program, ['assetRegistry']);
    if (!a) return [];
    const list = await a.all();
    return list.map(({ publicKey, account }) => ({
      publicKey: publicKey.toString(),
      ...account,
    }));
  } catch (e) {
    console.warn('[agroglobaldex] fetchAllAssets failed:', e.message);
    return [];
  }
}

export async function fetchAllExternalAssets(network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const a = ns(program, ['externalAssetRegistry']);
    if (!a) return [];
    const list = await a.all();
    return list.map(({ publicKey, account }) => ({
      publicKey: publicKey.toString(),
      ...account,
    }));
  } catch (e) {
    console.warn('[agroglobaldex] fetchAllExternalAssets failed:', e.message);
    return [];
  }
}

export async function fetchComplianceRecord(walletAddress, marketplaceAuthority, network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
    if (!auth) return null;
    const marketplace = findMarketplacePda(auth);
    const pda = findComplianceRecordPda(marketplace, walletAddress);
    const a = ns(program, ['complianceRecord']);
    if (!a) return null;
    return await a.fetchNullable(pda);
  } catch (e) {
    if (e.message?.includes('Account does not exist')) return null;
    console.warn('[agroglobaldex] fetchComplianceRecord failed:', e.message);
    return null;
  }
}

async function firstMarketplaceAuthority(program) {
  const a = ns(program, ['marketplace']);
  if (!a) return null;
  const all = await a.all();
  return all[0]?.account?.authority?.toString() || null;
}

/* ═══════ HASH HELPERS ═══════ */
export async function sha256OfFile(file) {
  const buf = await file.arrayBuffer();
  return new Uint8Array(await crypto.subtle.digest('SHA-256', buf));
}
export async function sha256OfBytes(bytes) {
  return new Uint8Array(await crypto.subtle.digest('SHA-256', bytes));
}

/* ═══════ AssetClass builder (variant Anchor) ═══════ */
export function buildAssetClass(form) {
  switch (form.kind) {
    case 'grain':
      return {
        grain: {
          kind: { [form.grainKind || 'soy']: {} },
          tons: new BN(form.tons || 0),
        },
      };
    case 'carbon':
      return {
        carbonCredit: {
          standard: { [form.standard || 'vcs']: {} },
          vintageYear: Number(form.vintageYear) || 0,
          kgCo2eq: new BN(form.kgCo2eq || 0),
        },
      };
    case 'harvest':
      return {
        harvestFraction: {
          crop: { [form.crop || 'soy']: {} },
          hectares: Number(form.hectares) || 0,
          harvestYear: Number(form.harvestYear) || 0,
        },
      };
    default:
      throw new Error('Tipo de activo desconocido: ' + form.kind);
  }
}

/* ═══════ TOKENIZATION ═══════ */
/**
 * register_asset (compliance hook program se pasa como UncheckedAccount).
 * `form`: { kind, totalSupply, oracleAttestation(Uint8Array 32),
 *           whitePaperUri, metadataUri, marketplaceAuthority,
 *           complianceHookProgram, ...campos por clase }
 * Devuelve: { tx, assetRegistry, mint, index }
 */
export async function registerAsset(form) {
  const program = await getProgram();
  const issuer = program.provider.wallet.publicKey;
  const auth = form.marketplaceAuthority || (await firstMarketplaceAuthority(program));
  if (!auth) throw new Error('No se encontró el Marketplace inicializado. Pediile al operador que corra `initialize`.');

  const marketplace = findMarketplacePda(auth);
  const mp = await fetchMarketplace(auth);
  if (!mp) throw new Error('No se pudo leer el marketplace');
  const index = mp.assetCount;

  const assetRegistry = findAssetRegistryPda(marketplace, index);
  const mint = findAssetMintPda(assetRegistry);
  const hookProgram = new PublicKey(form.complianceHookProgram || PROGRAM_ID);

  const assetClass = buildAssetClass(form);
  const attestation = Array.from(form.oracleAttestation);

  const tx = await program.methods
    .registerAsset(
      assetClass,
      new BN(form.totalSupply),
      attestation,
      form.whitePaperUri || '',
      form.metadataUri || '',
    )
    .accounts({
      issuer,
      marketplace,
      assetRegistry,
      mint,
      complianceHookProgram: hookProgram,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();

  return {
    tx,
    assetRegistry: assetRegistry.toString(),
    mint: mint.toString(),
    index: index.toString ? index.toString() : Number(index),
  };
}

export const registerGrainAsset = (form) => registerAsset({ ...form, kind: 'grain' });
export const registerCarbonAsset = (form) => registerAsset({ ...form, kind: 'carbon' });
export const registerHarvestFraction = (form) => registerAsset({ ...form, kind: 'harvest' });

/* ═══════ BUY ═══════ */
/**
 * buy_asset (paga en USDC). `args = { listingPubkey, amount }`.
 * Carga el listing on-chain, deriva ATAs USDC y firma.
 */
export async function buyAsset({ listingPubkey, amount }) {
  const program = await getProgram();
  const buyer = program.provider.wallet.publicKey;
  const listing = new PublicKey(listingPubkey);
  const listingAcc = await program.account.marketplaceListing.fetch(listing);
  const assetReg = listingAcc.sourceRegistry;
  const regAcc = await program.account.assetRegistry.fetch(assetReg);
  const marketplace = listingAcc.marketplace;
  const mpAcc = await program.account.marketplace.fetch(marketplace);

  const buyerCompliance = findComplianceRecordPda(marketplace, buyer);
  const treasury = findTreasuryPda(marketplace);
  const usdcMint = mpAcc.usdcMint;

  // ATAs (clásicas para USDC)
  const buyerUsdc = getAssociatedTokenAddress(usdcMint, buyer, TOKEN_PROGRAM_ID);
  const sellerUsdc = getAssociatedTokenAddress(usdcMint, listingAcc.seller, TOKEN_PROGRAM_ID);
  const treasuryUsdc = getAssociatedTokenAddress(usdcMint, treasury, TOKEN_PROGRAM_ID);

  const tx = await program.methods
    .buyAsset(new BN(amount))
    .accounts({
      buyer,
      marketplace,
      assetRegistry: assetReg,
      listing,
      escrow: listingAcc.escrow,
      seller: listingAcc.seller,
      mint: regAcc.mint,
      buyerTokenAccount: getAssociatedTokenAddress(regAcc.mint, buyer, TOKEN_2022_PROGRAM_ID),
      buyerCompliance,
      usdcMint,
      buyerUsdcAta: buyerUsdc,
      sellerUsdcAta: sellerUsdc,
      treasuryUsdcAta: treasuryUsdc,
      treasury,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      usdcTokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx };
}

/* ═══════ AGGREGATE ═══════ */
/**
 * aggregate_external_asset (payload struct).
 * `form`:
 *   { kind: 'spl', mint, sourcePlatform, sourceUrl, metadataUri, assetClass: { kind, ... } }
 *   { kind: 'cross', chainId, contract, sourcePlatform, sourceUrl, metadataUri, assetClass }
 */
export async function aggregateExternalAsset(form) {
  const program = await getProgram();
  const curator = program.provider.wallet.publicKey;
  const auth = form.marketplaceAuthority || curator.toString();
  const marketplace = findMarketplacePda(auth);
  const mp = await fetchMarketplace(auth);
  if (!mp) throw new Error('Marketplace no inicializado');
  const index = mp.externalAssetCount;
  const externalAsset = findExternalAssetPda(marketplace, index);

  const payload = {
    mint: form.kind === 'spl' ? new PublicKey(form.mint) : null,
    externalChainId: form.kind === 'cross' ? form.chainId : '',
    externalContract: form.kind === 'cross' ? form.contract : '',
    assetClass: buildAssetClass(form.assetClass || form),
    sourcePlatform: form.sourcePlatform || '',
    sourceUrl: form.sourceUrl || '',
    metadataUri: form.metadataUri || '',
  };

  const tx = await program.methods
    .aggregateExternalAsset(payload)
    .accounts({
      curator,
      marketplace,
      externalAsset,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx, externalAsset: externalAsset.toString(), index: index.toString ? index.toString() : Number(index) };
}

export async function updateExternalAsset(externalAssetPubkey, verified, active) {
  const program = await getProgram();
  const curator = program.provider.wallet.publicKey;
  const ext = new PublicKey(externalAssetPubkey);
  const extAcc = await program.account.externalAssetRegistry.fetch(ext);
  const tx = await program.methods
    .updateExternalAsset(verified, active)
    .accounts({
      curator,
      marketplace: extAcc.marketplace,
      externalAsset: ext,
    })
    .rpc();
  return { tx };
}

/* ═══════ ATA helper (sync, sin spl-token dep) ═══════ */
function getAssociatedTokenAddress(mint, owner, tokenProgramId) {
  return PublicKey.findProgramAddressSync(
    [new PublicKey(owner).toBuffer(), new PublicKey(tokenProgramId).toBuffer(), new PublicKey(mint).toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  )[0];
}

export { PublicKey, BN };
export default {
  getProgram, getReadProgram,
  fetchMarketplace, fetchAllListings, fetchAllAssets, fetchAllExternalAssets,
  fetchComplianceRecord,
  sha256OfFile, sha256OfBytes,
  registerAsset, registerGrainAsset, registerCarbonAsset, registerHarvestFraction,
  buyAsset,
  aggregateExternalAsset, updateExternalAsset,
  findMarketplacePda, findAssetRegistryPda, findListingPda, findExternalAssetPda,
  findComplianceRecordPda,
};
