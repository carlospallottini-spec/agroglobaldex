/**
 * AgroGlobalDex — initialize-devnet.ts
 *
 * Idempotent bootstrap del marketplace en devnet pública.
 * Despues de `anchor build` + `solana program deploy --url devnet`, corré:
 *
 *   export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"
 *   # o con Helius para evitar el rate-limit del RPC publico:
 *   export ANCHOR_PROVIDER_URL="https://devnet.helius-rpc.com/?api-key=$HELIUS_API_KEY"
 *   export ANCHOR_WALLET="$HOME/.config/solana/id.json"
 *   npx ts-node --project tsconfig.seed.json scripts/initialize-devnet.ts
 *
 * Idempotencia: cada paso lee primero la cuenta; si ya existe, salta.
 * Asi podes volver a correrlo despues de un upgrade sin romper nada.
 *
 * Acciones:
 *  1. initialize marketplace (fee_bps=50 = 0.5%, compliance_signer separado)
 *  2. init_jurisdiction_policy (blocked: KP/IR/SY/CU, requires_accredited: vacio)
 *  3. update_kyc(authority, ES, accredited) — para que el founder ya pueda
 *     register_asset / list_asset sin trabarse
 *  4. init_lending_market (APR 12%, max LTV 50%, liq threshold 80%, liq bonus 5%)
 *  5. (opcional) deposit_liquidity de USDC si el founder ya tiene devnet USDC
 *
 * Outputs:
 *  - Tabla con todas las public keys clave (marketplace, treasury, policy,
 *    lending_market, compliance_signer)
 *  - Solscan devnet links para cada una
 *  - Lo que falta hacer manual antes de que un productor pueda tokenizar
 */
import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import {
  Connection, Keypair, PublicKey, SystemProgram,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";

const IDL_PATH = path.join(__dirname, "..", "target", "idl", "agroglobaldex.json");
const KEYPAIR_PATH = process.env.ANCHOR_WALLET || path.join(os.homedir(), ".config", "solana", "id.json");
const RPC = process.env.ANCHOR_PROVIDER_URL || "https://api.devnet.solana.com";

// Devnet USDC faucet mint de Circle. Si querés probar el flow real, mintea desde:
//   https://faucet.circle.com/
// Si no, el script crea un fake USDC propio para el marketplace.
const DEVNET_USDC_FAUCET = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

// Compliance signer determinístico: derivado del path keypair + un salt fijo
// para que sea reproducible (no perdés el signer si corrés el script de nuevo).
// Producción usa HSM del servicio KYC.
const COMPLIANCE_SIGNER_PATH = path.join(os.homedir(), ".config", "solana", "agroglobaldex-compliance-signer.json");

function loadOrCreateKeypair(p: string): Keypair {
  if (fs.existsSync(p)) {
    return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(p, "utf8"))));
  }
  const kp = Keypair.generate();
  fs.mkdirSync(path.dirname(p), { recursive: true });
  fs.writeFileSync(p, JSON.stringify(Array.from(kp.secretKey)));
  fs.chmodSync(p, 0o600);
  return kp;
}
function loadKeypair(p: string): Keypair {
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(p, "utf8"))));
}
function findPda(seeds: (Buffer | Uint8Array)[], programId: PublicKey) {
  return PublicKey.findProgramAddressSync(seeds, programId)[0];
}
function solscan(addr: PublicKey | string): string {
  return `https://solscan.io/account/${addr.toString()}?cluster=devnet`;
}

async function accountExists(connection: Connection, pubkey: PublicKey): Promise<boolean> {
  const info = await connection.getAccountInfo(pubkey);
  return info !== null;
}

async function main() {
  console.log("════════════════════════════════════════════════════════════");
  console.log("  AgroGlobalDex — initialize-devnet  (idempotent bootstrap)  ");
  console.log("════════════════════════════════════════════════════════════\n");

  if (!fs.existsSync(IDL_PATH)) {
    console.error(`✗ IDL no encontrado en ${IDL_PATH}.`);
    console.error("  Corré 'anchor build' antes de este script.");
    process.exit(1);
  }
  const idl = JSON.parse(fs.readFileSync(IDL_PATH, "utf8"));
  const programId = new PublicKey(idl.address);
  const HOOK_PROGRAM_ID = new PublicKey("GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL");

  const payer = loadKeypair(KEYPAIR_PATH);
  const complianceSignerKp = loadOrCreateKeypair(COMPLIANCE_SIGNER_PATH);

  const connection = new Connection(RPC, "confirmed");
  const wallet = new anchor.Wallet(payer);
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);
  const program = new (anchor as any).Program(idl, provider);

  console.log("RPC                :", RPC);
  console.log("Program ID         :", programId.toBase58());
  console.log("Hook program ID    :", HOOK_PROGRAM_ID.toBase58());
  console.log("Authority (payer)  :", payer.publicKey.toBase58());
  console.log("Compliance signer  :", complianceSignerKp.publicKey.toBase58());
  console.log("                     (saved at " + COMPLIANCE_SIGNER_PATH + ")\n");

  // Balance sanity check
  const balance = await connection.getBalance(payer.publicKey);
  if (balance < 0.5 * 1e9) {
    console.warn(`⚠ Authority tiene ${balance / 1e9} SOL — necesitás al menos 0.5 SOL para los init txs.`);
    console.warn("  Pedí airdrop:  solana airdrop 2 --url devnet");
  } else {
    console.log(`  Authority balance: ${(balance / 1e9).toFixed(3)} SOL ✓\n`);
  }

  // ── USDC mint ────────────────────────────────────────────────────────
  // Para devnet usamos el USDC del faucet de Circle por defecto. Si lo
  // querés cambiar, exportá USDC_MINT=<pubkey> antes de correr el script.
  const usdcMint = process.env.USDC_MINT
    ? new PublicKey(process.env.USDC_MINT)
    : new PublicKey(DEVNET_USDC_FAUCET);
  console.log("USDC mint          :", usdcMint.toBase58(), "(circle devnet faucet)");

  // ── PDAs ─────────────────────────────────────────────────────────────
  const marketplace = findPda([Buffer.from("marketplace"), payer.publicKey.toBuffer()], programId);
  const complianceAuthority = findPda([Buffer.from("compliance_authority"), marketplace.toBuffer()], programId);
  const treasury = findPda([Buffer.from("treasury"), marketplace.toBuffer()], programId);
  const treasuryUsdcAta = getAssociatedTokenAddressSync(usdcMint, treasury, true, TOKEN_PROGRAM_ID);
  const policy = findPda([Buffer.from("jurisdiction_policy"), marketplace.toBuffer()], programId);
  const lendingMarket = findPda([Buffer.from("lending_market"), marketplace.toBuffer()], programId);
  const lendingVaultAuthority = findPda([Buffer.from("lending_vault"), lendingMarket.toBuffer()], programId);
  const lendingUsdcPool = getAssociatedTokenAddressSync(usdcMint, lendingVaultAuthority, true, TOKEN_PROGRAM_ID);
  const authorityCompliance = findPda(
    [Buffer.from("compliance_record"), marketplace.toBuffer(), payer.publicKey.toBuffer()],
    programId,
  );

  // ── Step 1: initialize marketplace (idempotent) ──────────────────────
  if (await accountExists(connection, marketplace)) {
    console.log("\n[1] Marketplace ya inicializado, salto.");
  } else {
    console.log("\n[1] Inicializando marketplace (fee_bps=50)…");
    const tx = await program.methods
      .initialize(50)
      .accounts({
        authority: payer.publicKey,
        complianceSigner: complianceSignerKp.publicKey,
        marketplace,
        complianceAuthority,
        treasury,
        usdcMint,
        treasuryUsdcAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("    tx:", tx);
  }

  // ── Step 2: init_jurisdiction_policy ─────────────────────────────────
  if (await accountExists(connection, policy)) {
    console.log("\n[2] JurisdictionPolicy ya existe, salto.");
  } else {
    console.log("\n[2] init_jurisdiction_policy con defaults conservadores…");
    const tx = await program.methods
      .initJurisdictionPolicy()
      .accounts({
        authority: payer.publicKey,
        marketplace,
        policy,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("    tx:", tx);

    // Update con la lista real (init solo crea estructura vacia).
    console.log("    update_jurisdiction_policy con OFAC blocked + UE requires_accredited…");
    const blocked = ["KP", "IR", "SY", "CU"].map(c => [c.charCodeAt(0), c.charCodeAt(1)] as number[]);
    const reqAcc: number[][] = []; // vacio por ahora
    const tx2 = await program.methods
      .updateJurisdictionPolicy(blocked, reqAcc)
      .accounts({
        authority: payer.publicKey,
        marketplace,
        policy,
      })
      .rpc();
    console.log("    tx:", tx2);
  }

  // ── Step 3: stamp KYC del authority (asi puede registerAsset, listAsset, etc.)
  // El compliance_signer firma. Es necesario fundearlo con SOL para rent.
  const signerBalance = await connection.getBalance(complianceSignerKp.publicKey);
  if (signerBalance < 0.02 * 1e9) {
    console.log("\n[3a] Transfiriendo 0.05 SOL al compliance_signer para que pueda firmar updates…");
    const transferTx = new anchor.web3.Transaction().add(
      SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: complianceSignerKp.publicKey,
        lamports: 0.05 * 1e9,
      }),
    );
    const sig = await anchor.web3.sendAndConfirmTransaction(connection, transferTx, [payer]);
    console.log("     tx:", sig);
  } else {
    console.log("\n[3a] Compliance signer ya tiene balance suficiente.");
  }

  if (await accountExists(connection, authorityCompliance)) {
    console.log("[3b] ComplianceRecord del authority ya existe.");
  } else {
    console.log("[3b] Stamping KYC del authority (ES, accredited)…");
    const tx = await program.methods
      .updateKyc(true, [0x45, 0x53], true) // "ES"
      .accounts({
        complianceSigner: complianceSignerKp.publicKey,
        marketplace,
        wallet: payer.publicKey,
        complianceRecord: authorityCompliance,
        systemProgram: SystemProgram.programId,
      })
      .signers([complianceSignerKp])
      .rpc();
    console.log("    tx:", tx);
  }

  // ── Step 4: init_lending_market ──────────────────────────────────────
  if (await accountExists(connection, lendingMarket)) {
    console.log("\n[4] LendingMarket ya existe, salto.");
  } else {
    console.log("\n[4] init_lending_market (APR 12%, max LTV 50%, liq threshold 80%, liq bonus 5%)…");
    const tx = await program.methods
      .initLendingMarket(
        1200, // apr_bps = 12%
        5000, // max_ltv_bps = 50%
        8000, // liquidation_threshold_bps = 80%
        500,  // liquidation_bonus_bps = 5%
      )
      .accounts({
        authority: payer.publicKey,
        marketplace,
        lendingMarket,
        vaultAuthority: lendingVaultAuthority,
        usdcMint,
        usdcPool: lendingUsdcPool,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("    tx:", tx);
  }

  // ── VERIFY: print final state ────────────────────────────────────────
  console.log("\n════════════════════════════════════════════════════════════");
  console.log("  ✓ Bootstrap completo                                       ");
  console.log("════════════════════════════════════════════════════════════\n");

  const mpAcc = await program.account.marketplace.fetch(marketplace);
  console.log("Marketplace state:");
  console.log("  authority         :", mpAcc.authority.toBase58());
  console.log("  compliance_signer :", mpAcc.complianceSigner.toBase58());
  console.log("  fee_bps           :", mpAcc.feeBps, `(${mpAcc.feeBps / 100}%)`);
  console.log("  asset_count       :", mpAcc.assetCount.toString());
  console.log("  trade_count       :", mpAcc.tradeCount.toString());
  console.log("  paused            :", mpAcc.paused);

  console.log("\nAddresses (Solscan devnet):");
  const rows: [string, PublicKey][] = [
    ["Marketplace        ", marketplace],
    ["Treasury PDA       ", treasury],
    ["Treasury USDC ATA  ", treasuryUsdcAta],
    ["JurisdictionPolicy ", policy],
    ["LendingMarket      ", lendingMarket],
    ["Lending USDC pool  ", lendingUsdcPool],
    ["Compliance signer  ", complianceSignerKp.publicKey],
  ];
  for (const [label, pk] of rows) {
    console.log(`  ${label}: ${pk.toBase58()}`);
    console.log(`                       ${solscan(pk)}`);
  }

  console.log("\n──── PROXIMOS PASOS ────────────────────────────────────────");
  console.log("1. Mintear devnet USDC desde https://faucet.circle.com/ a esta wallet:");
  console.log(`     ${payer.publicKey.toBase58()}`);
  console.log("2. Depositar liquidez al lending pool:");
  console.log("     desde /borrow.html en la web, o via 'depositLiquidity' del cliente.");
  console.log("3. Onboardear productor: 'updateKyc' para su wallet → 'registerAsset' del");
  console.log("   token → 'mintToken' → 'listAsset' en marketplace.");
  console.log("4. Si vas a habilitar un asset como colateral del lending:");
  console.log("     authority firma 'setCollateralConfig(assetRegistry, price, true)'.");
  console.log("\nGuardá las direcciones de arriba en docs/devnet-deploy.md para tracking.");
}

main().catch((e) => { console.error("\n✗ FAIL:", e); process.exit(1); });
