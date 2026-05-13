/**
 * AgroGlobalDex — seed-localnet.ts
 *
 * Carga datos demo en un solana-test-validator local. Asume:
 *  - validator corriendo en http://127.0.0.1:8899
 *  - programa deployado bajo target/deploy/agroglobaldex-keypair.json
 *  - wallet del usuario en ~/.config/solana/id.json con SOL suficiente
 *
 * Ejecutar:
 *   npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts
 */
import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import {
  Connection, Keypair, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint, mintTo, getOrCreateAssociatedTokenAccount, getAssociatedTokenAddressSync,
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
  const connection = new Connection(RPC, "confirmed");
  const wallet = new anchor.Wallet(payer);
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);
  const program = new (anchor as any).Program(idl, provider);

  console.log("Program ID:", programId.toBase58());
  console.log("Authority :", payer.publicKey.toBase58());

  // 1) Fake USDC mint (Token clásico, 6 decimales)
  console.log("\n[1] Creando fake USDC mint…");
  const usdcMint = await createMint(connection, payer, payer.publicKey, null, 6);
  console.log("    USDC mint:", usdcMint.toBase58());

  // 2) PDAs
  const marketplace = findPda([Buffer.from("marketplace"), payer.publicKey.toBuffer()], programId);
  const complianceAuthority = findPda([Buffer.from("compliance_authority"), marketplace.toBuffer()], programId);
  const treasury = findPda([Buffer.from("treasury"), marketplace.toBuffer()], programId);
  const treasuryUsdcAta = getAssociatedTokenAddressSync(usdcMint, treasury, true, TOKEN_PROGRAM_ID);

  // 3) initialize
  console.log("\n[2] Inicializando marketplace…");
  let tx = await program.methods
    .initialize(50) // 0.5% fee
    .accounts({
      authority: payer.publicKey,
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

  // 4) KYC para el issuer (AR)
  console.log("\n[3] update_kyc del authority (AR) …");
  const issuerCompliance = findPda(
    [Buffer.from("compliance_record"), marketplace.toBuffer(), payer.publicKey.toBuffer()],
    programId,
  );
  tx = await program.methods
    .updateKyc(true, [0x41, 0x52], true)
    .accounts({
      complianceSigner: payer.publicKey,
      marketplace,
      complianceAuthority,
      wallet: payer.publicKey,
      complianceRecord: issuerCompliance,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 5) register_asset (Grain · Soja · 100 toneladas)
  console.log("\n[4] register_asset Grain…");
  const mp = await program.account.marketplace.fetch(marketplace);
  const idxBuf = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
  const assetRegistry = findPda(
    [Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf],
    programId,
  );
  const assetMint = findPda([Buffer.from("asset_mint"), assetRegistry.toBuffer()], programId);

  const attestation = Array.from(createHash("sha256").update("warehouse-receipt-AR-2026-demo").digest());

  tx = await program.methods
    .registerAsset(
      { grain: { kind: { soy: {} }, tons: new BN(100) } },
      new BN(100_000_000_000), // 100 ton × 1000 kg × 1e6 (6 decimales del mint)
      attestation,
      "ipfs://demo/whitepaper.pdf",
      "ipfs://demo/metadata.json",
    )
    .accounts({
      issuer: payer.publicKey,
      marketplace,
      assetRegistry,
      mint: assetMint,
      complianceHookProgram: programId, // placeholder: el propio programa
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();
  console.log("    tx:", tx, "\n    assetRegistry:", assetRegistry.toBase58(), "\n    mint:", assetMint.toBase58());

  // 6) mint_token: mint 50 toneladas (50M base units)
  console.log("\n[5] mint_token 50 ton…");
  const issuerTokenAta = getAssociatedTokenAddressSync(assetMint, payer.publicKey, true, TOKEN_2022_PROGRAM_ID);
  tx = await program.methods
    .mintToken(new BN(50_000_000_000))
    .accounts({
      issuer: payer.publicKey,
      assetRegistry,
      mint: assetMint,
      issuerTokenAccount: issuerTokenAta,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx);

  // 7) aggregate_external_asset (Agrotoken-like)
  console.log("\n[6] aggregate_external_asset (Agrotoken-like SPL)…");
  const dummyExtMint = Keypair.generate().publicKey;
  const mp2 = await program.account.marketplace.fetch(marketplace);
  const extIdx = new BN(mp2.externalAssetCount).toArrayLike(Buffer, "le", 8);
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
      metadataUri: "ipfs://agrotoken-demo/meta.json",
    })
    .accounts({
      curator: payer.publicKey,
      marketplace,
      externalAsset,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx, "\n    externalAsset:", externalAsset.toBase58());

  // 8) Cross-chain reference (Centrifuge / Ethereum)
  console.log("\n[7] aggregate_external_asset (Centrifuge cross-chain)…");
  const mp3 = await program.account.marketplace.fetch(marketplace);
  const extIdx2 = new BN(mp3.externalAssetCount).toArrayLike(Buffer, "le", 8);
  const externalAsset2 = findPda(
    [Buffer.from("external_asset"), marketplace.toBuffer(), extIdx2],
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
      metadataUri: "ipfs://centrifuge-demo/meta.json",
    })
    .accounts({
      curator: payer.publicKey,
      marketplace,
      externalAsset: externalAsset2,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("    tx:", tx, "\n    externalAsset:", externalAsset2.toBase58());

  // 9) Verificación final: leer todas las cuentas
  console.log("\n[VERIFY] Estado on-chain:");
  const marketplaceAcc = await program.account.marketplace.fetch(marketplace);
  console.log("  Marketplace assetCount:", marketplaceAcc.assetCount.toString(),
              " externalAssetCount:", marketplaceAcc.externalAssetCount.toString(),
              " feeBps:", marketplaceAcc.feeBps);
  const allAssets = await program.account.assetRegistry.all();
  console.log("  AssetRegistry rows:", allAssets.length, "→", allAssets.map((a: any) => a.publicKey.toBase58()));
  const allExt = await program.account.externalAssetRegistry.all();
  console.log("  ExternalAssetRegistry rows:", allExt.length, "→", allExt.map((a: any) => `${a.account.sourcePlatform} (${a.publicKey.toBase58()})`));
  const allCompl = await program.account.complianceRecord.all();
  console.log("  ComplianceRecord rows:", allCompl.length);

  console.log("\n✓ Seed completo. Marketplace listo en", marketplace.toBase58());
  console.log("  USDC mint (fake localnet):", usdcMint.toBase58());
}

main().catch((e) => { console.error(e); process.exit(1); });
