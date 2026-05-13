//! # AgroGlobalDex Program
//!
//! Solana/Anchor program powering the AgroGlobalDex marketplace: a MiCA-aligned
//! venue for tokenized agricultural Real-World Assets (RWAs) **with an
//! aggregator** for third-party tokenized agro assets.
//!
//! Native asset classes:
//! - `Grain`            — physically-backed commodity tokens (soy, corn, wheat)
//! - `CarbonCredit`     — agro carbon credits (kg CO2eq)
//! - `HarvestFraction`  — fractional future harvests (hectares + year)
//!
//! Aggregated assets are curated by the marketplace authority and surfaced
//! alongside natives in the UI (Agrotoken, Topaz, RIPE, Centrifuge…). See
//! `instructions/aggregate.rs`.
//!
//! ## Compliance model
//! Every wallet that touches a regulated asset must have a `ComplianceRecord`
//! PDA stamped by the marketplace `compliance_authority`. The token mints are
//! created via **SPL Token-2022** with a `TransferHook` extension pointing at
//! a compliance-hook program (deployed separately; see roadmap in README).
//!
//! Trades are settled in **USDC** (configurable per-marketplace) with the
//! protocol fee going to a treasury USDC ATA owned by a PDA.
//!
//! NOTE: this is a Proof-of-Concept scaffold. It is NOT audited and MUST NOT
//! be used in mainnet with real value. See `README.md`.

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::AssetClass;

// Placeholder program id. After the first `anchor build` run `anchor keys
// sync` to overwrite both `Anchor.toml` and `declare_id!` with the keypair
// generated under `target/deploy/agroglobaldex-keypair.json`.
declare_id!("G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a");

#[program]
pub mod agroglobaldex {
    use super::*;

    /// Initialize the marketplace global config.
    pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        instructions::initialize::handler(ctx, fee_bps)
    }

    /// Register a new tokenizable asset (one `AssetRegistry` per real-world lot).
    /// `oracle_attestation` is the SHA-256 hash of the off-chain certificate.
    /// `white_paper_uri` is required by MiCA Art. 6.
    pub fn register_asset(
        ctx: Context<RegisterAsset>,
        asset_class: AssetClass,
        total_supply: u64,
        oracle_attestation: [u8; 32],
        white_paper_uri: String,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::register_asset::handler(
            ctx,
            asset_class,
            total_supply,
            oracle_attestation,
            white_paper_uri,
            metadata_uri,
        )
    }

    /// Mint tokens against a registered asset. Issuer-only, capped at
    /// `total_supply`.
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::mint_token::handler(ctx, amount)
    }

    /// Stamp / update KYC + jurisdiction flags for a wallet. Authority-only.
    pub fn update_kyc(
        ctx: Context<UpdateKyc>,
        kyc_verified: bool,
        jurisdiction: [u8; 2],
        accredited_investor: bool,
    ) -> Result<()> {
        instructions::update_kyc::handler(ctx, kyc_verified, jurisdiction, accredited_investor)
    }

    /// List a native asset at a USDC price (6 decimals per token base unit).
    pub fn list_asset(ctx: Context<ListAsset>, price_usdc: u64, amount: u64) -> Result<()> {
        instructions::list_asset::handler(ctx, price_usdc, amount)
    }

    /// Buy a native listing in USDC. Compliance-checked.
    pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        instructions::buy_asset::handler(ctx, amount)
    }

    /// Burn tokens to redeem the underlying off-chain commodity.
    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        instructions::redeem::handler(ctx, amount)
    }

    /// Aggregator: register an externally-minted RWA-agro asset for display.
    /// Authority-only. Either `mint` is `Some` (Solana SPL) or
    /// `external_chain_id`/`external_contract` must be non-empty (cross-chain).
    pub fn aggregate_external_asset(
        ctx: Context<AggregateExternalAsset>,
        payload: AggregatePayload,
    ) -> Result<()> {
        instructions::aggregate::aggregate_handler(ctx, payload)
    }

    /// Aggregator: toggle `verified` / `active` flags on an aggregated asset.
    pub fn update_external_asset(
        ctx: Context<UpdateExternalAsset>,
        verified: bool,
        active: bool,
    ) -> Result<()> {
        instructions::aggregate::update_handler(ctx, verified, active)
    }
}
