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
  NETWORK, RPC_ENDPOINTS, PROGRAM_ID, COMPLIANCE_HOOK_PROGRAM_ID, IDL_URL, USDC_MINT,
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

function hookProgramIdPk() {
  return new PublicKey(COMPLIANCE_HOOK_PROGRAM_ID);
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
// ── Lending PDAs ──────────────────────────────────────────────────────
export function findLendingMarketPda(marketplace) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('lending_market'), new PublicKey(marketplace).toBuffer()],
    programIdPk(),
  )[0];
}
export function findLendingVaultPda(lendingMarket) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('lending_vault'), new PublicKey(lendingMarket).toBuffer()],
    programIdPk(),
  )[0];
}
export function findCollateralConfigPda(lendingMarket, assetRegistry) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('collateral_config'), new PublicKey(lendingMarket).toBuffer(), new PublicKey(assetRegistry).toBuffer()],
    programIdPk(),
  )[0];
}
export function findLoanPda(lendingMarket, borrower, assetRegistry) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('loan'), new PublicKey(lendingMarket).toBuffer(), new PublicKey(borrower).toBuffer(), new PublicKey(assetRegistry).toBuffer()],
    programIdPk(),
  )[0];
}
export function findLiquidityProviderPda(lendingMarket, provider) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('liquidity_provider'), new PublicKey(lendingMarket).toBuffer(), new PublicKey(provider).toBuffer()],
    programIdPk(),
  )[0];
}

// PDAs on the compliance_hook program (NOT on agroglobaldex):
// hook_config seed = [b"hook_config", mint]
export function findHookConfigPda(mint) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('hook_config'), new PublicKey(mint).toBuffer()],
    hookProgramIdPk(),
  )[0];
}
// extra_account_meta_list seed = [b"extra-account-metas", mint]
export function findExtraAccountMetaListPda(mint) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('extra-account-metas'), new PublicKey(mint).toBuffer()],
    hookProgramIdPk(),
  )[0];
}
// jurisdiction_policy seed = [b"jurisdiction_policy", marketplace]
export function findJurisdictionPolicyPda(marketplace) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('jurisdiction_policy'), new PublicKey(marketplace).toBuffer()],
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

/**
 * Fetch the global trade ledger (TradeReceipt PDAs). Optionally filter by
 * buyer or seller wallet. Returns newest-first.
 */
export async function fetchAllTradeReceipts({ buyer, seller, network = NETWORK } = {}) {
  try {
    const program = await getReadProgram(network);
    const a = ns(program, ['tradeReceipt']);
    if (!a) return [];
    const filters = [];
    // TradeReceipt layout after 8-byte discriminator:
    //   marketplace(32) listing(32) asset_mint(32) buyer(32) seller(32) ...
    if (buyer) filters.push({ memcmp: { offset: 8 + 32 * 3, bytes: buyer } });
    else if (seller) filters.push({ memcmp: { offset: 8 + 32 * 4, bytes: seller } });
    const list = await a.all(filters);
    return list
      .map(({ publicKey, account }) => ({ publicKey: publicKey.toString(), ...account }))
      .sort((x, y) => Number(y.tradeIndex) - Number(x.tradeIndex));
  } catch (e) {
    console.warn('[agroglobaldex] fetchAllTradeReceipts failed:', e.message);
    return [];
  }
}

/** Derive a TradeReceipt PDA from a marketplace + trade index. */
export function findTradeReceiptPda(marketplace, tradeIndex) {
  const idxBuf = new BN(tradeIndex).toArrayLike(Buffer, 'le', 8);
  return PublicKey.findProgramAddressSync(
    [Buffer.from('trade_receipt'), new PublicKey(marketplace).toBuffer(), idxBuf],
    programIdPk(),
  )[0];
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
    case 'investment':
      return {
        investmentOffering: {
          productKind: { [form.productKind || 'other']: {} },
          durationMonths: Number(form.durationMonths) || 12,
          expectedYieldBps: Number(form.expectedYieldBps) || 0,
          maturityUnixTs: new BN(form.maturityUnixTs || 0),
        },
      };
    case 'commodity': {
      const cc = (form.originCountry || 'AR').toUpperCase();
      return {
        commodity: {
          sector: { [form.sector || 'other']: {} },
          subKind: Number(form.subKind) || 0,
          originCountry: [cc.charCodeAt(0), cc.charCodeAt(1)],
          vintageYear: Number(form.vintageYear) || new Date().getFullYear(),
          gramsPerToken: new BN(form.gramsPerToken || 1000),
        },
      };
    }
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
  // El programa valida `address = compliance_hook::ID`, asi que el caller
  // NO puede rotar el hook program ID arbitrariamente — siempre usamos la
  // constante on-chain.
  const hookProgram = hookProgramIdPk();
  const hookConfig = findHookConfigPda(mint);
  const extraAccountMetaList = findExtraAccountMetaListPda(mint);

  const assetClass = buildAssetClass(form);
  const attestation = Array.from(form.oracleAttestation);

  const tx = await program.methods
    .registerAsset(
      assetClass,
      new BN(form.totalSupply),
      attestation,
      form.whitePaperUri || '',
      form.metadataUri || '',
      form.productName || '',
    )
    .accounts({
      issuer,
      marketplace,
      assetRegistry,
      mint,
      complianceHookProgram: hookProgram,
      hookConfig,
      extraAccountMetaList,
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
export const registerInvestmentOffering = (form) => registerAsset({ ...form, kind: 'investment' });
export const registerCommodity = (form) => registerAsset({ ...form, kind: 'commodity' });

/** Catalog of commodity sub-kinds by sector. Frontend interpreta el sub_kind u8. */
export const COMMODITY_SUB_KINDS = {
  meat:        { 0: 'Beef',   1: 'Pork',     2: 'Poultry', 3: 'Lamb',    4: 'Fish',     255: 'Other' },
  wine:        { 0: 'Red',    1: 'White',    2: 'Rose',    3: 'Sparkling',                255: 'Other' },
  oil:         { 0: 'Olive',  1: 'Sunflower',2: 'Soy',     3: 'Palm',    4: 'Avocado',  255: 'Other' },
  dairy:       { 0: 'Milk',   1: 'Cheese',   2: 'Butter',  3: 'Yogurt',                   255: 'Other' },
  fruit:       { 0: 'Apple',  1: 'Citrus',   2: 'Banana',  3: 'Berry',   4: 'StoneFruit', 255: 'Other' },
  vegetable:   { 0: 'Leafy',  1: 'Root',     2: 'Tomato',  3: 'Pepper',                   255: 'Other' },
  fiber:       { 0: 'Cotton', 1: 'Wool',     2: 'Linen',   3: 'Hemp',                     255: 'Other' },
  grainSpecial:{ 0: 'Rice',   1: 'Sorghum',  2: 'Sunflower',3:'Barley',  4: 'Oats',     255: 'Other' },
  other:       { 0: 'Generic', 255: 'Other' },
};

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
  const jurisdictionPolicy = findJurisdictionPolicyPda(marketplace);
  const treasury = findTreasuryPda(marketplace);
  const tradeReceipt = findTradeReceiptPda(marketplace, mpAcc.tradeCount);
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
      jurisdictionPolicy,
      usdcMint,
      buyerUsdcAta: buyerUsdc,
      sellerUsdcAta: sellerUsdc,
      treasuryUsdcAta: treasuryUsdc,
      treasury,
      tradeReceipt,
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

/* ═══════ LISTING MANAGEMENT (seller-only) ═══════ */
export async function cancelListing(listingPubkey) {
  const program = await getProgram();
  const seller = program.provider.wallet.publicKey;
  const listing = new PublicKey(listingPubkey);
  const listingAcc = await program.account.marketplaceListing.fetch(listing);
  const tx = await program.methods
    .cancelListing()
    .accounts({
      seller,
      marketplace: listingAcc.marketplace,
      assetRegistry: listingAcc.sourceRegistry,
      listing,
      escrow: listingAcc.escrow,
      mint: listingAcc.mint,
      buyerTokenAccount: undefined, // not needed by anchor inferred
      sellerTokenAccount: getAssociatedTokenAddress(listingAcc.mint, seller, TOKEN_2022_PROGRAM_ID),
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx };
}

export async function updateListingPrice(listingPubkey, newPriceUsdc) {
  const program = await getProgram();
  const seller = program.provider.wallet.publicKey;
  const listing = new PublicKey(listingPubkey);
  const listingAcc = await program.account.marketplaceListing.fetch(listing);
  const tx = await program.methods
    .updateListingPrice(new BN(newPriceUsdc))
    .accounts({
      seller,
      assetRegistry: listingAcc.sourceRegistry,
      listing,
    })
    .rpc();
  return { tx };
}

/* ═══════ TREASURY (authority-only) ═══════ */
export async function treasuryWithdraw(destinationWallet, amount) {
  const program = await getProgram();
  const authority = program.provider.wallet.publicKey;
  const auth = await firstMarketplaceAuthority(program);
  const marketplace = findMarketplacePda(auth);
  const mp = await fetchMarketplace(auth);
  const treasury = findTreasuryPda(marketplace);
  const usdcMint = mp.usdcMint;
  const dest = new PublicKey(destinationWallet);
  const tx = await program.methods
    .treasuryWithdraw(new BN(amount))
    .accounts({
      authority,
      marketplace,
      treasury,
      usdcMint,
      treasuryUsdcAta: getAssociatedTokenAddress(usdcMint, treasury, TOKEN_PROGRAM_ID),
      destination: dest,
      destinationUsdcAta: getAssociatedTokenAddress(usdcMint, dest, TOKEN_PROGRAM_ID),
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx };
}

/* ═══════ BUY EXTERNAL (curated SPL) ═══════ */
export async function buyExternalAsset({ listingPubkey, amount }) {
  const program = await getProgram();
  const buyer = program.provider.wallet.publicKey;
  const listing = new PublicKey(listingPubkey);
  const listingAcc = await program.account.marketplaceListing.fetch(listing);
  const externalAsset = listingAcc.sourceRegistry;
  const externalAcc = await program.account.externalAssetRegistry.fetch(externalAsset);
  const marketplace = listingAcc.marketplace;
  const mpAcc = await program.account.marketplace.fetch(marketplace);
  const usdcMint = mpAcc.usdcMint;
  const treasury = findTreasuryPda(marketplace);
  const buyerCompliance = findComplianceRecordPda(marketplace, buyer);
  const jurisdictionPolicy = findJurisdictionPolicyPda(marketplace);
  const tradeReceipt = findTradeReceiptPda(marketplace, mpAcc.tradeCount);

  const tx = await program.methods
    .buyExternalAsset(new BN(amount))
    .accounts({
      buyer,
      marketplace,
      externalAsset,
      listing,
      escrow: listingAcc.escrow,
      seller: listingAcc.seller,
      mint: listingAcc.mint,
      buyerTokenAccount: getAssociatedTokenAddress(listingAcc.mint, buyer, TOKEN_PROGRAM_ID),
      buyerCompliance,
      jurisdictionPolicy,
      usdcMint,
      buyerUsdcAta: getAssociatedTokenAddress(usdcMint, buyer, TOKEN_PROGRAM_ID),
      sellerUsdcAta: getAssociatedTokenAddress(usdcMint, listingAcc.seller, TOKEN_PROGRAM_ID),
      treasuryUsdcAta: getAssociatedTokenAddress(usdcMint, treasury, TOKEN_PROGRAM_ID),
      treasury,
      tradeReceipt,
      tokenProgram: TOKEN_PROGRAM_ID,
      usdcTokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx };
}

/* ═══════ REVOKE KYC (compliance signer revoca KYC ante sanctions / fraud) ═══════ */
/**
 * Llamada por el compliance_signer wallet. reasonCode:
 *   0 = manual, 1 = sanctions, 2 = fraud, 3 = regulatory, 4 = self-request.
 */
export async function revokeKyc({ walletAddress, marketplaceAuthority, reasonCode = 0 }) {
  const program = await getProgram();
  const complianceSigner = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  if (!auth) throw new Error('Marketplace no inicializado');
  const marketplace = findMarketplacePda(auth);
  const wallet = new PublicKey(walletAddress);
  const complianceRecord = findComplianceRecordPda(marketplace, wallet);

  const tx = await program.methods
    .revokeKyc(reasonCode)
    .accounts({
      complianceSigner,
      marketplace,
      wallet,
      complianceRecord,
    })
    .rpc();
  return { tx };
}

/* ═══════ SETTLE INVESTMENT OFFERING (issuer registra yield off-chain receipt) ═══════ */
/**
 * Por epoch (0,1,2...). `yieldPaidUsdc` en base units (6 decimales). `attestation`
 * es Uint8Array(32) — usar sha256OfFile(swiftConfirmationPdf) o similar.
 */
export async function settleInvestmentOffering({ assetRegistryPubkey, epoch, yieldPaidUsdc, attestation }) {
  const program = await getProgram();
  const issuer = program.provider.wallet.publicKey;
  const assetRegistry = new PublicKey(assetRegistryPubkey);
  const regAcc = await program.account.assetRegistry.fetch(assetRegistry);
  const marketplace = regAcc.marketplace;

  const tx = await program.methods
    .settleInvestmentOffering(epoch, new BN(yieldPaidUsdc), Array.from(attestation))
    .accounts({
      issuer,
      marketplace,
      assetRegistry,
    })
    .rpc();
  return { tx };
}

/* ═══════ UPDATE METADATA (pre-first-mint) ═══════ */
/**
 * Issuer-only. Revierte con MetadataFrozen una vez que ya se hizo mint_token.
 */
export async function updateMetadata({ assetRegistryPubkey, productName, metadataUri, whitePaperUri }) {
  const program = await getProgram();
  const issuer = program.provider.wallet.publicKey;
  const assetRegistry = new PublicKey(assetRegistryPubkey);
  const regAcc = await program.account.assetRegistry.fetch(assetRegistry);
  const marketplace = regAcc.marketplace;
  const mint = regAcc.mint;

  const tx = await program.methods
    .updateMetadata(productName, metadataUri, whitePaperUri)
    .accounts({
      issuer,
      marketplace,
      assetRegistry,
      mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
  return { tx };
}

/* ═══════ TRANSFER ISSUER (ceder rol issuer a wallet KYC-verified) ═══════ */
export async function transferIssuer({ assetRegistryPubkey, newIssuerAddress }) {
  const program = await getProgram();
  const currentIssuer = program.provider.wallet.publicKey;
  const assetRegistry = new PublicKey(assetRegistryPubkey);
  const regAcc = await program.account.assetRegistry.fetch(assetRegistry);
  const marketplace = regAcc.marketplace;
  const newIssuer = new PublicKey(newIssuerAddress);
  const newIssuerCompliance = findComplianceRecordPda(marketplace, newIssuer);

  const tx = await program.methods
    .transferIssuer()
    .accounts({
      currentIssuer,
      marketplace,
      assetRegistry,
      newIssuer,
      newIssuerCompliance,
    })
    .rpc();
  return { tx };
}

/* ═══════ LENDING ═══════ */

/** Read the lending market state (apr, ltv, liquidity, etc.). */
export async function fetchLendingMarket(marketplaceAuthority, network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
    if (!auth) return null;
    const marketplace = findMarketplacePda(auth);
    const lm = findLendingMarketPda(marketplace);
    const a = ns(program, ['lendingMarket']);
    if (!a) return null;
    const acc = await a.fetchNullable(lm);
    return acc ? { publicKey: lm.toString(), ...acc } : null;
  } catch (e) {
    console.warn('[agroglobaldex] fetchLendingMarket failed:', e.message);
    return null;
  }
}

/** Read all loan positions, optionally filter by borrower. */
export async function fetchAllLoans({ borrower, network = NETWORK } = {}) {
  try {
    const program = await getReadProgram(network);
    const a = ns(program, ['loanPosition']);
    if (!a) return [];
    // LoanPosition after 8-byte discriminator: lending_market(32) borrower(32)
    const filters = borrower ? [{ memcmp: { offset: 8 + 32, bytes: borrower } }] : [];
    const list = await a.all(filters);
    return list.map(({ publicKey, account }) => ({ publicKey: publicKey.toString(), ...account }));
  } catch (e) {
    console.warn('[agroglobaldex] fetchAllLoans failed:', e.message);
    return [];
  }
}

/** Read the collateral config (price + enabled) for an asset. */
export async function fetchCollateralConfig(lendingMarket, assetRegistry, network = NETWORK) {
  try {
    const program = await getReadProgram(network);
    const cfg = findCollateralConfigPda(lendingMarket, assetRegistry);
    const a = ns(program, ['collateralConfig']);
    if (!a) return null;
    const acc = await a.fetchNullable(cfg);
    return acc ? { publicKey: cfg.toString(), ...acc } : null;
  } catch (e) {
    return null;
  }
}

/** Parse a Pyth feed-id hex string ("0x..." or bare hex) into a 32-byte array. */
export function pythFeedIdToBytes(hex) {
  const clean = String(hex).trim().replace(/^0x/i, '');
  if (clean.length !== 64) throw new Error('Pyth feed id must be 32 bytes (64 hex chars)');
  const out = new Array(32);
  for (let i = 0; i < 32; i++) out[i] = parseInt(clean.slice(i * 2, i * 2 + 2), 16);
  return out;
}

/** Bind a Pyth price feed to a collateral. Authority-only. */
export async function setCollateralOracle({ assetRegistryPubkey, feedIdHex, maxStalenessSecs = 60, maxConfidenceBps = 200, enabled = true, marketplaceAuthority }) {
  const program = await getProgram();
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const lendingMarket = findLendingMarketPda(marketplace);
  const collateralConfig = findCollateralConfigPda(lendingMarket, new PublicKey(assetRegistryPubkey));
  const tx = await program.methods
    .setCollateralOracle(pythFeedIdToBytes(feedIdHex), new BN(maxStalenessSecs), maxConfidenceBps, enabled)
    .accounts({
      authority: program.provider.wallet.publicKey,
      marketplace,
      lendingMarket,
      collateralConfig,
    })
    .rpc();
  return { tx };
}

/** Permissionless crank: refresh a collateral price from its Pyth feed. */
export async function refreshCollateralPrice({ assetRegistryPubkey, priceUpdateAccount, marketplaceAuthority }) {
  const program = await getProgram();
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const lendingMarket = findLendingMarketPda(marketplace);
  const collateralConfig = findCollateralConfigPda(lendingMarket, new PublicKey(assetRegistryPubkey));
  const tx = await program.methods
    .refreshCollateralPrice()
    .accounts({
      cranker: program.provider.wallet.publicKey,
      collateralConfig,
      priceUpdate: new PublicKey(priceUpdateAccount),
    })
    .rpc();
  return { tx };
}

/** Open a loan: lock collateral, receive USDC. */
export async function openLoan({ assetRegistryPubkey, collateralAmount, borrowAmount, marketplaceAuthority }) {
  const program = await getProgram();
  const borrower = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const mpAcc = await fetchMarketplace(auth);
  const assetRegistry = new PublicKey(assetRegistryPubkey);
  const regAcc = await program.account.assetRegistry.fetch(assetRegistry);
  const collateralMint = regAcc.mint;

  const lendingMarket = findLendingMarketPda(marketplace);
  const lmAcc = await program.account.lendingMarket.fetch(lendingMarket);
  const vaultAuthority = findLendingVaultPda(lendingMarket);
  const collateralConfig = findCollateralConfigPda(lendingMarket, assetRegistry);
  const borrowerCompliance = findComplianceRecordPda(marketplace, borrower);
  const loan = findLoanPda(lendingMarket, borrower, assetRegistry);
  const usdcMint = mpAcc.usdcMint;

  const tx = await program.methods
    .openLoan(new BN(collateralAmount), new BN(borrowAmount))
    .accounts({
      borrower,
      marketplace,
      lendingMarket,
      collateralConfig,
      assetRegistry,
      borrowerCompliance,
      collateralMint,
      borrowerCollateralAta: getAssociatedTokenAddress(collateralMint, borrower, TOKEN_2022_PROGRAM_ID),
      vaultAuthority,
      collateralVault: getAssociatedTokenAddress(collateralMint, vaultAuthority, TOKEN_2022_PROGRAM_ID),
      usdcMint,
      usdcPool: lmAcc.usdcPool,
      borrowerUsdcAta: getAssociatedTokenAddress(usdcMint, borrower, TOKEN_PROGRAM_ID),
      loan,
      collateralTokenProgram: TOKEN_2022_PROGRAM_ID,
      usdcTokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx, loan: loan.toString() };
}

/** Repay a loan in full (principal + interest), unlock collateral. */
export async function repayLoan({ assetRegistryPubkey, marketplaceAuthority }) {
  const program = await getProgram();
  const borrower = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const mpAcc = await fetchMarketplace(auth);
  const assetRegistry = new PublicKey(assetRegistryPubkey);
  const regAcc = await program.account.assetRegistry.fetch(assetRegistry);
  const collateralMint = regAcc.mint;

  const lendingMarket = findLendingMarketPda(marketplace);
  const lmAcc = await program.account.lendingMarket.fetch(lendingMarket);
  const vaultAuthority = findLendingVaultPda(lendingMarket);
  const loan = findLoanPda(lendingMarket, borrower, assetRegistry);
  const usdcMint = mpAcc.usdcMint;

  const tx = await program.methods
    .repayLoan()
    .accounts({
      borrower,
      lendingMarket,
      loan,
      collateralMint,
      borrowerCollateralAta: getAssociatedTokenAddress(collateralMint, borrower, TOKEN_2022_PROGRAM_ID),
      vaultAuthority,
      collateralVault: getAssociatedTokenAddress(collateralMint, vaultAuthority, TOKEN_2022_PROGRAM_ID),
      usdcMint,
      usdcPool: lmAcc.usdcPool,
      borrowerUsdcAta: getAssociatedTokenAddress(usdcMint, borrower, TOKEN_PROGRAM_ID),
      collateralTokenProgram: TOKEN_2022_PROGRAM_ID,
      usdcTokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
  return { tx };
}

/** Deposit USDC liquidity into the lending pool. */
export async function depositLiquidity({ amount, marketplaceAuthority }) {
  const program = await getProgram();
  const provider = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const mpAcc = await fetchMarketplace(auth);
  const lendingMarket = findLendingMarketPda(marketplace);
  const lmAcc = await program.account.lendingMarket.fetch(lendingMarket);
  const usdcMint = mpAcc.usdcMint;

  const tx = await program.methods
    .depositLiquidity(new BN(amount))
    .accounts({
      provider,
      lendingMarket,
      usdcMint,
      usdcPool: lmAcc.usdcPool,
      providerUsdcAta: getAssociatedTokenAddress(usdcMint, provider, TOKEN_PROGRAM_ID),
      liquidityProvider: findLiquidityProviderPda(lendingMarket, provider),
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  return { tx };
}

/** Withdraw USDC liquidity previously deposited. Bounded by idle pool funds. */
export async function withdrawLiquidity({ amount, marketplaceAuthority }) {
  const program = await getProgram();
  const provider = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const mpAcc = await fetchMarketplace(auth);
  const lendingMarket = findLendingMarketPda(marketplace);
  const lmAcc = await program.account.lendingMarket.fetch(lendingMarket);
  const usdcMint = mpAcc.usdcMint;

  const tx = await program.methods
    .withdrawLiquidity(new BN(amount))
    .accounts({
      provider,
      lendingMarket,
      liquidityProvider: findLiquidityProviderPda(lendingMarket, provider),
      usdcMint,
      usdcPool: lmAcc.usdcPool,
      vaultAuthority: findLendingVaultPda(lendingMarket),
      providerUsdcAta: getAssociatedTokenAddress(usdcMint, provider, TOKEN_PROGRAM_ID),
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
  return { tx };
}

/** Read the connected wallet's liquidity-provider record (net deposited USDC).
 *  Returns null if the wallet has never deposited. */
export async function fetchMyLiquidityPosition({ marketplaceAuthority } = {}) {
  const program = await getProgram();
  const provider = program.provider.wallet.publicKey;
  const auth = marketplaceAuthority || (await firstMarketplaceAuthority(program));
  const marketplace = findMarketplacePda(auth);
  const lendingMarket = findLendingMarketPda(marketplace);
  const lpPda = findLiquidityProviderPda(lendingMarket, provider);
  try {
    return await program.account.liquidityProvider.fetch(lpPda);
  } catch (_) {
    return null;
  }
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
  registerAsset, registerGrainAsset, registerCarbonAsset, registerHarvestFraction, registerInvestmentOffering, registerCommodity,
  COMMODITY_SUB_KINDS,
  buyAsset, buyExternalAsset,
  cancelListing, updateListingPrice, treasuryWithdraw,
  aggregateExternalAsset, updateExternalAsset,
  revokeKyc, settleInvestmentOffering, updateMetadata, transferIssuer,
  fetchAllTradeReceipts,
  fetchLendingMarket, fetchAllLoans, fetchCollateralConfig,
  setCollateralOracle, refreshCollateralPrice, pythFeedIdToBytes,
  openLoan, repayLoan, depositLiquidity, withdrawLiquidity, fetchMyLiquidityPosition,
  findMarketplacePda, findAssetRegistryPda, findListingPda, findExternalAssetPda,
  findComplianceRecordPda, findJurisdictionPolicyPda,
  findHookConfigPda, findExtraAccountMetaListPda, findTradeReceiptPda,
  findLendingMarketPda, findLendingVaultPda, findCollateralConfigPda, findLoanPda,
  findLiquidityProviderPda,
};
