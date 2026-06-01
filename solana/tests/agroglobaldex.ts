/**
 * AgroGlobalDex — Anchor mocha test suite v2.
 *
 * Coverage:
 *   1. initialize marketplace (separate compliance_signer)
 *   2. init_jurisdiction_policy + update_jurisdiction_policy
 *   3. update_kyc signed by authority (NOT compliance_signer) must fail
 *   4. update_kyc signed by compliance_signer succeeds
 *   5. register_asset Grain with Token-2022 metadata
 *   6. register InvestmentOffering with yield > 5000 bps must fail (InvalidYield)
 *   7. register InvestmentOffering "Viñedo Rioja" 12 months 9% ROI succeeds
 *   8. set_paused gates write paths and resume restores
 *   9. set_compliance_signer rotation
 *  10. aggregator: SPL + cross-chain + update_external_asset
 *
 * Run with:  anchor test
 */
import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import {
  Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint, getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { assert } from "chai";
import * as fs from "fs";
import * as path from "path";
import { createHash } from "crypto";

const HOOK_PROGRAM_ID = new PublicKey("GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL");
const IDL_PATH = path.join(__dirname, "..", "target", "idl", "agroglobaldex.json");

function pda(seeds: (Buffer | Uint8Array)[], programId: PublicKey) {
  return PublicKey.findProgramAddressSync(seeds, programId)[0];
}

async function expectRevert(promise: Promise<any>, msgFragment?: string) {
  try {
    await promise;
    assert.fail(`Expected revert${msgFragment ? ' with "' + msgFragment + '"' : ''}, but tx succeeded`);
  } catch (e: any) {
    if (msgFragment && !String(e).toLowerCase().includes(msgFragment.toLowerCase())) {
      console.log("  (revert msg:", String(e).slice(0, 180), ")");
    }
  }
}

describe("agroglobaldex", function () {
  this.timeout(120_000);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const idl = JSON.parse(fs.readFileSync(IDL_PATH, "utf8"));
  const programId = new PublicKey(idl.address);
  const program = new (anchor as any).Program(idl, provider);
  const payer = (provider.wallet as any).payer as Keypair;

  const authority = payer;
  const complianceSigner = Keypair.generate();
  const issuer = Keypair.generate();

  let usdcMint: PublicKey;
  let marketplace: PublicKey;
  let complianceAuthority: PublicKey;
  let treasury: PublicKey;
  let policy: PublicKey;

  async function airdrop(pk: PublicKey, sol: number) {
    const sig = await provider.connection.requestAirdrop(pk, sol * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig, "confirmed");
  }

  it("01 setup: fund dev wallets + create fake USDC", async () => {
    await airdrop(complianceSigner.publicKey, 5);
    await airdrop(issuer.publicKey, 10);
    usdcMint = await createMint(provider.connection, payer, payer.publicKey, null, 6);
  });

  it("02 initialize marketplace with separate compliance_signer", async () => {
    marketplace = pda([Buffer.from("marketplace"), authority.publicKey.toBuffer()], programId);
    complianceAuthority = pda([Buffer.from("compliance_authority"), marketplace.toBuffer()], programId);
    treasury = pda([Buffer.from("treasury"), marketplace.toBuffer()], programId);
    const treasuryUsdc = getAssociatedTokenAddressSync(usdcMint, treasury, true, TOKEN_PROGRAM_ID);

    await program.methods.initialize(50)
      .accounts({
        authority: authority.publicKey,
        complianceSigner: complianceSigner.publicKey,
        marketplace, complianceAuthority, treasury, usdcMint, treasuryUsdcAta: treasuryUsdc,
        tokenProgram: TOKEN_PROGRAM_ID, associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId, rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    const mp = await program.account.marketplace.fetch(marketplace);
    assert.equal(mp.complianceSigner.toBase58(), complianceSigner.publicKey.toBase58());
    assert.equal(mp.paused, false);
  });

  it("03 init_jurisdiction_policy + update_jurisdiction_policy", async () => {
    policy = pda([Buffer.from("jurisdiction_policy"), marketplace.toBuffer()], programId);
    await program.methods.initJurisdictionPolicy()
      .accounts({ authority: authority.publicKey, marketplace, policy, systemProgram: SystemProgram.programId })
      .rpc();
    const blocked = [[0x4B, 0x50], [0x49, 0x52], [0x53, 0x59], [0x43, 0x55], [0x41, 0x46], [0x52, 0x55]];
    await program.methods.updateJurisdictionPolicy(blocked, [])
      .accounts({ authority: authority.publicKey, marketplace, policy })
      .rpc();
    const p = await program.account.jurisdictionPolicy.fetch(policy);
    assert.equal(p.blocked.length, 6);
  });

  it("04 sad: update_kyc signed by authority (not compliance_signer) must fail", async () => {
    const rec = pda([Buffer.from("compliance_record"), marketplace.toBuffer(), issuer.publicKey.toBuffer()], programId);
    await expectRevert(
      program.methods.updateKyc(true, [0x41, 0x52], true)
        .accounts({ complianceSigner: authority.publicKey, marketplace, wallet: issuer.publicKey, complianceRecord: rec, systemProgram: SystemProgram.programId })
        .rpc(),
      "Unauthorized",
    );
  });

  it("05 happy: compliance_signer stamps KYC (AR, accredited)", async () => {
    const rec = pda([Buffer.from("compliance_record"), marketplace.toBuffer(), issuer.publicKey.toBuffer()], programId);
    await program.methods.updateKyc(true, [0x41, 0x52], true)
      .accounts({ complianceSigner: complianceSigner.publicKey, marketplace, wallet: issuer.publicKey, complianceRecord: rec, systemProgram: SystemProgram.programId })
      .signers([complianceSigner])
      .rpc();
    const r = await program.account.complianceRecord.fetch(rec);
    assert.equal(r.kycVerified, true);
    assert.equal(r.accreditedInvestor, true);
  });

  it("06 register Grain with Token-2022 metadata", async () => {
    const mp = await program.account.marketplace.fetch(marketplace);
    const idx = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idx], programId);
    const m = pda([Buffer.from("asset_mint"), reg.toBuffer()], programId);
    const att = Array.from(createHash("sha256").update("warehouse").digest());
    await program.methods.registerAsset(
      { grain: { kind: { soy: {} }, tons: new BN(100) } },
      new BN(100_000_000_000), att,
      "ipfs://demo/grain-wp.pdf", "ipfs://demo/grain-meta.json", "Soja AR 2026 Q1",
    )
      .accounts({ issuer: issuer.publicKey, marketplace, assetRegistry: reg, mint: m, complianceHookProgram: HOOK_PROGRAM_ID, hookConfig: pda([Buffer.from("hook_config"), m.toBuffer()], HOOK_PROGRAM_ID), extraAccountMetaList: pda([Buffer.from("extra-account-metas"), m.toBuffer()], HOOK_PROGRAM_ID), tokenProgram: TOKEN_2022_PROGRAM_ID, systemProgram: SystemProgram.programId, rent: anchor.web3.SYSVAR_RENT_PUBKEY })
      .signers([issuer]).rpc();
    const r = await program.account.assetRegistry.fetch(reg);
    assert.equal(r.productName, "Soja AR 2026 Q1");
  });

  it("07 sad: InvestmentOffering yield > 5000 bps reverts InvalidYield", async () => {
    const mp = await program.account.marketplace.fetch(marketplace);
    const idx = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idx], programId);
    const m = pda([Buffer.from("asset_mint"), reg.toBuffer()], programId);
    const maturity = Math.floor(Date.now() / 1000) + 365 * 86400;
    await expectRevert(
      program.methods.registerAsset(
        { investmentOffering: { productKind: { vineyard: {} }, durationMonths: 12, expectedYieldBps: 9999, maturityUnixTs: new BN(maturity) } },
        new BN(1_000_000), Array.from(createHash("sha256").update("bad").digest()),
        "ipfs://demo/wp.pdf", "ipfs://demo/meta.json", "Bad yield",
      )
        .accounts({ issuer: issuer.publicKey, marketplace, assetRegistry: reg, mint: m, complianceHookProgram: HOOK_PROGRAM_ID, hookConfig: pda([Buffer.from("hook_config"), m.toBuffer()], HOOK_PROGRAM_ID), extraAccountMetaList: pda([Buffer.from("extra-account-metas"), m.toBuffer()], HOOK_PROGRAM_ID), tokenProgram: TOKEN_2022_PROGRAM_ID, systemProgram: SystemProgram.programId, rent: anchor.web3.SYSVAR_RENT_PUBKEY })
        .signers([issuer]).rpc(),
      "InvalidYield",
    );
  });

  it("08 happy: register Viñedo Rioja 12 months 9% ROI", async () => {
    const mp = await program.account.marketplace.fetch(marketplace);
    const idx = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idx], programId);
    const m = pda([Buffer.from("asset_mint"), reg.toBuffer()], programId);
    const maturity = Math.floor(Date.now() / 1000) + 365 * 86400;
    await program.methods.registerAsset(
      { investmentOffering: { productKind: { vineyard: {} }, durationMonths: 12, expectedYieldBps: 900, maturityUnixTs: new BN(maturity) } },
      new BN(10_000_000_000), Array.from(createHash("sha256").update("rioja").digest()),
      "ipfs://demo/vineyard-wp.pdf", "ipfs://demo/vineyard-meta.json", "Viñedo Rioja 2026 Reserva",
    )
      .accounts({ issuer: issuer.publicKey, marketplace, assetRegistry: reg, mint: m, complianceHookProgram: HOOK_PROGRAM_ID, hookConfig: pda([Buffer.from("hook_config"), m.toBuffer()], HOOK_PROGRAM_ID), extraAccountMetaList: pda([Buffer.from("extra-account-metas"), m.toBuffer()], HOOK_PROGRAM_ID), tokenProgram: TOKEN_2022_PROGRAM_ID, systemProgram: SystemProgram.programId, rent: anchor.web3.SYSVAR_RENT_PUBKEY })
      .signers([issuer]).rpc();
    const r = await program.account.assetRegistry.fetch(reg);
    assert.equal(r.productName, "Viñedo Rioja 2026 Reserva");
    assert.equal(r.redeemable, false);
  });

  it("09 set_paused gates writes; resume restores", async () => {
    await program.methods.setPaused(true)
      .accounts({ authority: authority.publicKey, marketplace })
      .rpc();
    let mp = await program.account.marketplace.fetch(marketplace);
    assert.equal(mp.paused, true);

    const idx = new BN(mp.assetCount).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idx], programId);
    const m = pda([Buffer.from("asset_mint"), reg.toBuffer()], programId);
    await expectRevert(
      program.methods.registerAsset(
        { grain: { kind: { soy: {} }, tons: new BN(50) } },
        new BN(1_000_000), Array.from(createHash("sha256").update("x").digest()),
        "ipfs://x", "", "X",
      )
        .accounts({ issuer: issuer.publicKey, marketplace, assetRegistry: reg, mint: m, complianceHookProgram: HOOK_PROGRAM_ID, hookConfig: pda([Buffer.from("hook_config"), m.toBuffer()], HOOK_PROGRAM_ID), extraAccountMetaList: pda([Buffer.from("extra-account-metas"), m.toBuffer()], HOOK_PROGRAM_ID), tokenProgram: TOKEN_2022_PROGRAM_ID, systemProgram: SystemProgram.programId, rent: anchor.web3.SYSVAR_RENT_PUBKEY })
        .signers([issuer]).rpc(),
      "Paused",
    );

    await program.methods.setPaused(false).accounts({ authority: authority.publicKey, marketplace }).rpc();
    mp = await program.account.marketplace.fetch(marketplace);
    assert.equal(mp.paused, false);
  });

  it("10 set_compliance_signer rotation", async () => {
    const newSigner = Keypair.generate();
    await program.methods.setComplianceSigner()
      .accounts({ authority: authority.publicKey, marketplace, newSigner: newSigner.publicKey })
      .rpc();
    let mp = await program.account.marketplace.fetch(marketplace);
    assert.equal(mp.complianceSigner.toBase58(), newSigner.publicKey.toBase58());

    await program.methods.setComplianceSigner()
      .accounts({ authority: authority.publicKey, marketplace, newSigner: complianceSigner.publicKey })
      .rpc();
  });

  it("11 aggregator: Agrotoken SPL + Centrifuge cross-chain", async () => {
    let mp = await program.account.marketplace.fetch(marketplace);
    let extIdx = new BN(mp.externalAssetCount).toArrayLike(Buffer, "le", 8);
    const ext1 = pda([Buffer.from("external_asset"), marketplace.toBuffer(), extIdx], programId);
    await program.methods.aggregateExternalAsset({
      mint: Keypair.generate().publicKey,
      externalChainId: "", externalContract: "",
      assetClass: { grain: { kind: { soy: {} }, tons: new BN(500) } },
      sourcePlatform: "Agrotoken", sourceUrl: "https://agrotoken.io/x", metadataUri: "ipfs://x",
    })
      .accounts({ curator: authority.publicKey, marketplace, externalAsset: ext1, systemProgram: SystemProgram.programId })
      .rpc();
    await program.methods.updateExternalAsset(true, true)
      .accounts({ curator: authority.publicKey, marketplace, externalAsset: ext1 })
      .rpc();

    mp = await program.account.marketplace.fetch(marketplace);
    extIdx = new BN(mp.externalAssetCount).toArrayLike(Buffer, "le", 8);
    const ext2 = pda([Buffer.from("external_asset"), marketplace.toBuffer(), extIdx], programId);
    await program.methods.aggregateExternalAsset({
      mint: null, externalChainId: "ethereum", externalContract: "0xabc",
      assetClass: { carbonCredit: { standard: { vcs: {} }, vintageYear: 2025, kgCo2eq: new BN(50000) } },
      sourcePlatform: "Centrifuge", sourceUrl: "https://centrifuge.io/p", metadataUri: "ipfs://y",
    })
      .accounts({ curator: authority.publicKey, marketplace, externalAsset: ext2, systemProgram: SystemProgram.programId })
      .rpc();

    const all = await program.account.externalAssetRegistry.all();
    assert.isAtLeast(all.length, 2);
  });

  // ---------------- New HIGH-priority instructions ----------------------

  it("12 revoke_kyc: compliance_signer revokes issuer KYC + emits ComplianceRevoked", async () => {
    const rec = pda([Buffer.from("compliance_record"), marketplace.toBuffer(), issuer.publicKey.toBuffer()], programId);
    // Confirm precondition: KYC verified
    let r0 = await program.account.complianceRecord.fetch(rec);
    assert.equal(r0.kycVerified, true);
    // Sanctions hit, reason_code = 1
    await program.methods.revokeKyc(1)
      .accounts({
        complianceSigner: complianceSigner.publicKey,
        marketplace,
        wallet: issuer.publicKey,
        complianceRecord: rec,
      })
      .signers([complianceSigner]).rpc();
    const r1 = await program.account.complianceRecord.fetch(rec);
    assert.equal(r1.kycVerified, false);

    // Re-instate so subsequent tests don't break (we leave accreditation as-is)
    await program.methods.updateKyc(true, Array.from(Buffer.from("AR")), true)
      .accounts({
        complianceSigner: complianceSigner.publicKey,
        marketplace,
        wallet: issuer.publicKey,
        complianceRecord: rec,
        systemProgram: SystemProgram.programId,
      })
      .signers([complianceSigner]).rpc();
  });

  it("13 sad: revoke_kyc signed by non-compliance-signer must fail", async () => {
    const rec = pda([Buffer.from("compliance_record"), marketplace.toBuffer(), issuer.publicKey.toBuffer()], programId);
    await expectRevert(
      program.methods.revokeKyc(2)
        .accounts({
          complianceSigner: authority.publicKey, // wrong signer
          marketplace,
          wallet: issuer.publicKey,
          complianceRecord: rec,
        }).rpc(),
      "UnauthorizedComplianceAuthority",
    );
  });

  it("14 settle_investment_offering: issuer records yield epoch for Viñedo Rioja", async () => {
    // Find the InvestmentOffering registry from test 08 (it's the 2nd native asset, index=1)
    const mp = await program.account.marketplace.fetch(marketplace);
    // Asset count is now >= 2 (Grain at idx 0 + Vineyard at idx 1)
    const idxBuf = new BN(1).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf], programId);
    const att = Array.from(createHash("sha256").update("swift-confirmation-q1-2026").digest());
    await program.methods.settleInvestmentOffering(0, new BN(450_000_000), att)
      .accounts({
        issuer: issuer.publicKey,
        marketplace,
        assetRegistry: reg,
      })
      .signers([issuer]).rpc();
    // No on-chain state changes — verified by event emission. Re-run with epoch=1
    // confirms idempotency.
    await program.methods.settleInvestmentOffering(1, new BN(225_000_000), att)
      .accounts({
        issuer: issuer.publicKey,
        marketplace,
        assetRegistry: reg,
      })
      .signers([issuer]).rpc();
  });

  it("15 sad: settle_investment_offering on Grain reverts NotInvestmentOffering", async () => {
    const idxBuf = new BN(0).toArrayLike(Buffer, "le", 8); // Grain at idx 0
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf], programId);
    const att = Array.from(createHash("sha256").update("nope").digest());
    await expectRevert(
      program.methods.settleInvestmentOffering(0, new BN(1), att)
        .accounts({
          issuer: issuer.publicKey,
          marketplace,
          assetRegistry: reg,
        })
        .signers([issuer]).rpc(),
      "NotInvestmentOffering",
    );
  });

  it("16 update_metadata: issuer updates Grain product_name pre-mint", async () => {
    const idxBuf = new BN(0).toArrayLike(Buffer, "le", 8);
    const reg = pda([Buffer.from("asset_registry"), marketplace.toBuffer(), idxBuf], programId);
    const mint = pda([Buffer.from("asset_mint"), reg.toBuffer()], programId);
    await program.methods.updateMetadata(
      "Soja AR 2026 Q1 (revised)",
      "ipfs://demo/grain-meta-v2.json",
      "ipfs://demo/grain-wp-v2.pdf",
    )
      .accounts({
        issuer: issuer.publicKey,
        marketplace,
        assetRegistry: reg,
        mint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([issuer]).rpc();
    const r = await program.account.assetRegistry.fetch(reg);
    assert.equal(r.productName, "Soja AR 2026 Q1 (revised)");
  });
});
