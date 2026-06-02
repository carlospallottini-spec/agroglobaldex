/**
 * AgroGlobalDex — seed-localnet.ts
 *
 * Carga datos demo end-to-end en un solana-test-validator local con los DOS
 * programas (agroglobaldex + compliance_hook) deployados.
 *
 * Demuestra:
 *  - initialize con compliance_signer separado de authority
 *  - init_jurisdiction_policy (mutable on-chain)
 *  - update_kyc del issuer (AR, accredited)
 *  - register_asset Grain (Soja, 100 ton)
 *  - mint_token
 *  - register_asset InvestmentOffering (Viñedo Rioja 12 meses 9% ROI)
 *  - aggregate_external_asset SPL (Agrotoken)
 *  - aggregate_external_asset cross-chain (Centrifuge)
 *  - update_jurisdiction_policy
 *  - set_compliance_signer (rotation)
 *
 * Ejecutar:
 *   npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts
 */
import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import {
  Connection, Keypair, PublicKey, SystemProgram,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint, getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";
import { createHash } from "crypto";

const IDL_PATH = path.join(__dirname, "..", "target", "idl", "agroglobaldex.json");
const KEYPAIR_PATH = path.join(os.homedir(), ".config", "solana", "id.json");
const RPC = "http://127.0.0.1:8899";

function loadKeypair(p: string): Keypair {
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(p, "utf8"))));
}
function findPda(seeds: (Buffer | Uint8Array)[], programId: PublicKey) {
  return PublicKey.findProgramAddressSync(seeds, programId)[0];
}

async function main() {
  const idl = JSON.parse(fs.readFileSync(IDL_PATH, "utf8"));
  const programId = new PublicKey(idl.address);
  const payer = loadKeypair(KEYPAIR_PATH);
  const complianceSignerKp = Keypair.generate();
  const connection = new Connection(RPC, "confirmed");
  const wallet = new anchor.Wallet(payer);
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);
  const program = new (anchor as any).Program(idl, provider);

  // Fund the compliance signer (it needs rent to write compliance records)
  await connection.confirmTransaction(
    await connection.requestAirdrop(complianceSignerKp.publicKey, 5 * 1e9),
    "confirmed",
  );

  // Compliance-hook program id (devnet/localnet uses the same id)
  const HOOK_PROGRAM_ID = new PublicKey("GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL");

  console.log("Program ID         :", programId.toBase58());
  console.log("Hook program ID    :", HOOK_PROGRAM_ID.toBase58());
  console.log("Authority          :", payer.publicKey.toBase58());
  console.log("Compliance signer  :", complianceSignerKp.publicKey.toBase58());

  // 1) Fake USDC mint
  console.log("\n[1] Creando fake USDC mint…");
  const usdcMint = await createMint(connection, payer, payer.publicKey, null, 6);
  console.log("    USDC mint:", usdcMint.toBase58());

  // PDAs
  const marketplace = findPda([Buffer.from("marketplace"), payer.publicKey.toBuffer()], programId);
  const complianceAuthority = findPda([Buffer.from("compliance_authority"), marketplace.toBuffer()], programId);
  const treasury = findPda([Buffer.from("treasury"), marketplace.toBuffer()], programId);
  const treasuryUsdcAta = getAssociatedTokenAddressSync(usdcMint, treasury, true, TOKEN_PROGRAM_ID);
  const policy = findPda([Buffer.from("jurisdiction_policy"), marketplace.toBuffer()], programId);

  // 2) initialize
  console.log("\n[2] Inicializando marketplace (authority y compliance_signer separados)…");
  let tx = await program.methods
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

  // 3) init_jurisdiction_policy
  console.log("\n[3] init_jurisdiction_policy (defaults KP/IR/SY/CU)…");
  tx = await program.methods
    .initJurisdictionPolicy()
    .accounts({
      authority: payer.publicKey,
      marketplace,
      policy,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 4) update_kyc del issuer (AR, accredited=true)
  console.log("\n[4] update_kyc del issuer (AR, accredited) firmado por compliance_signer…");
  const issuerCompliance = findPda(
    [Buffer.from("compliance_record"), marketplace.toBuffer(), payer.publicKey.toBuffer()],
    programId,
  );
  tx = await program.methods
    .updateKyc(true, [0x41, 0x52], true)
    .accounts({
      complianceSigner: complianceSignerKp.publicKey,
      marketplace,
      wallet: payer.publicKey,
      complianceRecord: issuerCompliance,
      systemProgram: SystemProgram.programId,
    })
    .signers([complianceSignerKp])
    .rpc();
  console.log("    tx:", tx);

  // 5) register_asset Grain (Soja · 100 ton)
  console.log("\n[5] register_asset Grain…");
  let mp = await program.account.marketplace.fetch(marketplace);
  let idxBuf = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
  let assetRegistry = findPda(
    [Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf],
    programId,
  );
  let assetMint = findPda([Buffer.from("asset_mint"), assetRegistry.toBuffer()], programId);
  let hookConfig = findPda([Buffer.from("hook_config"), assetMint.toBuffer()], HOOK_PROGRAM_ID);
  let extraMetas = findPda([Buffer.from("extra-account-metas"), assetMint.toBuffer()], HOOK_PROGRAM_ID);
  const att1 = Array.from(createHash("sha256").update("warehouse-receipt-AR-2026-demo").digest());

  tx = await program.methods
    .registerAsset(
      { grain: { kind: { soy: {} }, tons: new BN(100) } },
      new BN(100_000_000_000),
      att1,
      "ipfs://demo/grain-wp.pdf",
      "ipfs://demo/grain-meta.json",
      "Soja AR 2026 Q1",
    )
    .accounts({
      issuer: payer.publicKey,
      marketplace,
      assetRegistry,
      mint: assetMint,
      complianceHookProgram: HOOK_PROGRAM_ID,
      hookConfig,
      extraAccountMetaList: extraMetas,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();
  console.log("    tx:", tx, "\n    grain mint:", assetMint.toBase58());

  // 6) mint_token Grain: 50 ton
  console.log("\n[6] mint_token 50 ton…");
  const issuerGrainAta = getAssociatedTokenAddressSync(assetMint, payer.publicKey, true, TOKEN_2022_PROGRAM_ID);
  tx = await program.methods
    .mintToken(new BN(50_000_000_000))
    .accounts({
      issuer: payer.publicKey,
      assetRegistry,
      mint: assetMint,
      issuerTokenAccount: issuerGrainAta,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 7) register_asset InvestmentOffering · "Viñedo Rioja 2026 Reserva" · 12m · 9% ROI
  console.log("\n[7] register_asset InvestmentOffering (Viñedo Rioja 2026, 12 meses, 9% ROI)…");
  mp = await program.account.marketplace.fetch(marketplace);
  idxBuf = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
  const investRegistry = findPda(
    [Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf],
    programId,
  );
  const investMint = findPda([Buffer.from("asset_mint"), investRegistry.toBuffer()], programId);
  const investHookConfig = findPda([Buffer.from("hook_config"), investMint.toBuffer()], HOOK_PROGRAM_ID);
  const investExtraMetas = findPda([Buffer.from("extra-account-metas"), investMint.toBuffer()], HOOK_PROGRAM_ID);
  const att2 = Array.from(createHash("sha256").update("vineyard-rioja-2026-spv-contract").digest());
  const maturity = Math.floor(Date.now() / 1000) + 365 * 24 * 3600;

  tx = await program.methods
    .registerAsset(
      {
        investmentOffering: {
          productKind: { vineyard: {} },
          durationMonths: 12,
          expectedYieldBps: 900, // 9.00%
          maturityUnixTs: new BN(maturity),
        },
      },
      new BN(10_000_000_000), // 10k tokens × 1e6 = 10B base units
      att2,
      "ipfs://demo/vineyard-wp.pdf",
      "ipfs://demo/vineyard-meta.json",
      "Viñedo Rioja 2026 Reserva",
    )
    .accounts({
      issuer: payer.publicKey,
      marketplace,
      assetRegistry: investRegistry,
      mint: investMint,
      complianceHookProgram: HOOK_PROGRAM_ID,
      hookConfig: investHookConfig,
      extraAccountMetaList: investExtraMetas,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();
  console.log("    tx:", tx, "\n    invest mint:", investMint.toBase58());

  // 8) aggregate_external_asset SPL (Agrotoken)
  console.log("\n[8] aggregate_external_asset (Agrotoken-like SPL)…");
  const dummyExtMint = Keypair.generate().publicKey;
  let mp3 = await program.account.marketplace.fetch(marketplace);
  let extIdx = new BN(mp3.externalAssetCount).toArrayLike(Buffer, "le", 8);
  const externalAsset = findPda(
    [Buffer.from("external_asset"), marketplace.toBuffer(), extIdx],
    programId,
  );
  tx = await program.methods
    .aggregateExternalAsset({
      mint: dummyExtMint,
      externalChainId: "",
      externalContract: "",
      assetClass: { grain: { kind: { soy: {} }, tons: new BN(500) } },
      sourcePlatform: "Agrotoken",
      sourceUrl: "https://agrotoken.io/asset/SOYA-AR-2026",
      metadataUri: "ipfs://agrotoken/meta.json",
    })
    .accounts({
      curator: payer.publicKey,
      marketplace,
      externalAsset,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 9) aggregate_external_asset cross-chain (Centrifuge)
  console.log("\n[9] aggregate_external_asset (Centrifuge cross-chain)…");
  mp3 = await program.account.marketplace.fetch(marketplace);
  extIdx = new BN(mp3.externalAssetCount).toArrayLike(Buffer, "le", 8);
  const externalAsset2 = findPda(
    [Buffer.from("external_asset"), marketplace.toBuffer(), extIdx],
    programId,
  );
  tx = await program.methods
    .aggregateExternalAsset({
      mint: null,
      externalChainId: "ethereum",
      externalContract: "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
      assetClass: { carbonCredit: { standard: { vcs: {} }, vintageYear: 2025, kgCo2eq: new BN(50_000) } },
      sourcePlatform: "Centrifuge",
      sourceUrl: "https://centrifuge.io/pool/123",
      metadataUri: "ipfs://centrifuge/meta.json",
    })
    .accounts({
      curator: payer.publicKey,
      marketplace,
      externalAsset: externalAsset2,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 10) update_jurisdiction_policy — agregar AF a blocked
  console.log("\n[10] update_jurisdiction_policy (agregar AF)…");
  tx = await program.methods
    .updateJurisdictionPolicy(
      [Buffer.from("KP"), Buffer.from("IR"), Buffer.from("SY"), Buffer.from("CU"), Buffer.from("AF")].map(b => [b[0], b[1]]),
      [],
    )
    .accounts({
      authority: payer.publicKey,
      marketplace,
      policy,
    })
    .rpc();
  console.log("    tx:", tx);

  // 11) set_compliance_signer — rotate
  console.log("\n[11] set_compliance_signer (rotate)…");
  const newSigner = Keypair.generate();
  tx = await program.methods
    .setComplianceSigner()
    .accounts({
      authority: payer.publicKey,
      marketplace,
      newSigner: newSigner.publicKey,
    })
    .rpc();
  console.log("    tx:", tx, "new signer:", newSigner.publicKey.toBase58());

  // Restore old compliance signer so the rest of the seed can stamp KYC again.
  tx = await program.methods
    .setComplianceSigner()
    .accounts({
      authority: payer.publicKey,
      marketplace,
      newSigner: complianceSignerKp.publicKey,
    })
    .rpc();
  console.log("    restored old signer, tx:", tx);

  // ────────────────────────────────────────────────────────────────────
  // 12) update_metadata: Viñedo Rioja todavía NO fue minteado, su metadata
  //     sigue mutable. Probamos cambiar product_name + URIs.
  // ────────────────────────────────────────────────────────────────────
  console.log("\n[12] update_metadata Viñedo Rioja (pre-mint)…");
  tx = await program.methods
    .updateMetadata(
      "Viñedo Rioja 2026 Reserva (revised)",
      "ipfs://demo/vineyard-meta-v2.json",
      "ipfs://demo/vineyard-wp-v2.pdf",
    )
    .accounts({
      issuer: payer.publicKey,
      marketplace,
      assetRegistry: investRegistry,
      mint: investMint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
  console.log("    tx:", tx);

  // ────────────────────────────────────────────────────────────────────
  // 13) settle_investment_offering: epoch 0 con yield mock USD 450
  //     (450_000_000 base units USDC).
  // ────────────────────────────────────────────────────────────────────
  console.log("\n[13] settle_investment_offering Viñedo Rioja epoch=0 yield=450 USDC…");
  const settleAtt = Array.from(createHash("sha256").update("swift-confirmation-q1-2026").digest());
  tx = await program.methods
    .settleInvestmentOffering(0, new BN(450_000_000), settleAtt)
    .accounts({
      issuer: payer.publicKey,
      marketplace,
      assetRegistry: investRegistry,
    })
    .rpc();
  console.log("    tx:", tx);

  // ────────────────────────────────────────────────────────────────────
  // 14) revoke_kyc + re-stamp: ciclo completo de revocación.
  // ────────────────────────────────────────────────────────────────────
  console.log("\n[14] revoke_kyc + re-stamp KYC del issuer (reason=1 sanctions)…");
  const issuerRec = findPda(
    [Buffer.from("compliance_record"), marketplace.toBuffer(), payer.publicKey.toBuffer()],
    programId,
  );
  tx = await program.methods
    .revokeKyc(1)
    .accounts({
      complianceSigner: complianceSignerKp.publicKey,
      marketplace,
      wallet: payer.publicKey,
      complianceRecord: issuerRec,
    })
    .signers([complianceSignerKp])
    .rpc();
  console.log("    revoked, tx:", tx);
  tx = await program.methods
    .updateKyc(true, [0x41, 0x52], true)
    .accounts({
      complianceSigner: complianceSignerKp.publicKey,
      marketplace,
      wallet: payer.publicKey,
      complianceRecord: issuerRec,
      systemProgram: SystemProgram.programId,
    })
    .signers([complianceSignerKp])
    .rpc();
  console.log("    re-stamped, tx:", tx);

  // ────────────────────────────────────────────────────────────────────
  // 15) transfer_issuer: crear nueva wallet ES + KYC + ceder Grain.
  // ────────────────────────────────────────────────────────────────────
  console.log("\n[15] transfer_issuer Grain → nueva wallet ES…");
  const newIssuer = Keypair.generate();
  await connection.confirmTransaction(
    await connection.requestAirdrop(newIssuer.publicKey, 1 * 1e9),
    "confirmed",
  );
  const newIssuerRec = findPda(
    [Buffer.from("compliance_record"), marketplace.toBuffer(), newIssuer.publicKey.toBuffer()],
    programId,
  );
  // Stamp KYC para el nuevo issuer (ES, accredited)
  tx = await program.methods
    .updateKyc(true, [0x45, 0x53], true)
    .accounts({
      complianceSigner: complianceSignerKp.publicKey,
      marketplace,
      wallet: newIssuer.publicKey,
      complianceRecord: newIssuerRec,
      systemProgram: SystemProgram.programId,
    })
    .signers([complianceSignerKp])
    .rpc();
  console.log("    stamped new issuer KYC, tx:", tx);
  tx = await program.methods
    .transferIssuer()
    .accounts({
      currentIssuer: payer.publicKey,
      marketplace,
      assetRegistry,
      newIssuer: newIssuer.publicKey,
      newIssuerCompliance: newIssuerRec,
    })
    .rpc();
  console.log("    transferred, tx:", tx, "new issuer:", newIssuer.publicKey.toBase58());

  // Verification
  console.log("\n[VERIFY] Estado on-chain:");
  const mpAcc = await program.account.marketplace.fetch(marketplace);
  console.log("  Marketplace authority           :", mpAcc.authority.toBase58());
  console.log("  Marketplace compliance_signer   :", mpAcc.complianceSigner.toBase58());
  console.log("  Marketplace assetCount          :", mpAcc.assetCount.toString());
  console.log("  Marketplace externalAssetCount  :", mpAcc.externalAssetCount.toString());
  console.log("  Marketplace feeBps              :", mpAcc.feeBps);

  const policyAcc = await program.account.jurisdictionPolicy.fetch(policy);
  const blockedStrs = policyAcc.blocked.map((b: any) => String.fromCharCode(b[0]) + String.fromCharCode(b[1]));
  console.log("  JurisdictionPolicy blocked      :", blockedStrs.join(", "));

  const allAssets = await program.account.assetRegistry.all();
  console.log("  AssetRegistry rows              :", allAssets.length);
  for (const a of allAssets) {
    const cls = Object.keys((a.account as any).assetClass)[0];
    console.log(`    - "${(a.account as any).productName}" (${cls})  pk=${a.publicKey.toBase58()}`);
  }
  const allExt = await program.account.externalAssetRegistry.all();
  console.log("  ExternalAssetRegistry rows      :", allExt.length);
  for (const a of allExt) {
    console.log(`    - ${(a.account as any).sourcePlatform}  pk=${a.publicKey.toBase58()}`);
  }
  const allCompl = await program.account.complianceRecord.all();
  console.log("  ComplianceRecord rows           :", allCompl.length);

  console.log("\n✓ Seed completo. Marketplace listo en", marketplace.toBase58());
  console.log("  USDC mint (fake localnet):", usdcMint.toBase58());
}

main().catch((e) => { console.error(e); process.exit(1); });
