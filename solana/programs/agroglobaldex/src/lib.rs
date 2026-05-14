//! # AgroGlobalDex Program
//!
//! Solana/Anchor program powering the AgroGlobalDex marketplace: a MiCA-aligned
//! venue for tokenized agricultural Real-World Assets (RWAs) **with an
//! aggregator** for third-party tokenized agro assets.
//!
//! Native asset classes:
//! - `Grain`               — physically-backed commodity tokens (soy, corn, wheat)
//! - `CarbonCredit`        — agro carbon credits (kg CO2eq)
//! - `HarvestFraction`     — fractional future harvests (hectares + year)
//! - `InvestmentOffering`  — yield-bearing investment products (e.g. "Viñedo
//!                            Rioja 2026, 12 months @ 9% expected ROI")
//!
//! ## Compliance model
//! Every wallet must have a `ComplianceRecord` PDA stamped by the marketplace
//! `compliance_signer` (a wallet separate from `authority`, rotatable via
//! `set_compliance_signer`). Token mints are created via SPL Token-2022 with
//! a `TransferHook` extension pointing at the **compliance-hook** program
//! (deployed as a second program in this workspace) which enforces KYC +
//! jurisdiction policy on every transfer.
//!
//! The blocked jurisdiction list lives on-chain in a mutable
//! `JurisdictionPolicy` PDA so the authority can update it without redeploying.

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::AssetClass;

declare_id!("G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a");

#[program]
pub mod agroglobaldex {
    use super::*;

    /// Initialize the marketplace global config.
    pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        instructions::initialize::handler(ctx, fee_bps)
    }

    /// Rotate the wallet allowed to stamp KYC. Authority-only.
    pub fn set_compliance_signer(ctx: Context<SetComplianceSigner>) -> Result<()> {
        instructions::set_compliance_signer::handler(ctx)
    }

    /// Create the on-chain mutable JurisdictionPolicy with conservative defaults.
    pub fn init_jurisdiction_policy(ctx: Context<InitJurisdictionPolicy>) -> Result<()> {
        instructions::init_jurisdiction_policy::handler(ctx)
    }

    /// Replace both jurisdiction lists in JurisdictionPolicy. Authority-only.
    pub fn update_jurisdiction_policy(
        ctx: Context<UpdateJurisdictionPolicy>,
        blocked: Vec<[u8; 2]>,
        requires_accredited: Vec<[u8; 2]>,
    ) -> Result<()> {
        instructions::update_jurisdiction_policy::handler(ctx, blocked, requires_accredited)
    }

    /// Register a new tokenizable asset. `product_name` is the human label
    /// e.g. "Viñedo Rioja 2026 Reserva".
    pub fn register_asset(
        ctx: Context<RegisterAsset>,
        asset_class: AssetClass,
        total_supply: u64,
        oracle_attestation: [u8; 32],
        white_paper_uri: String,
        metadata_uri: String,
        product_name: String,
    ) -> Result<()> {
        instructions::register_asset::handler(
            ctx,
            asset_class,
            total_supply,
            oracle_attestation,
            white_paper_uri,
            metadata_uri,
            product_name,
        )
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::mint_token::handler(ctx, amount)
    }

    pub fn update_kyc(
        ctx: Context<UpdateKyc>,
        kyc_verified: bool,
        jurisdiction: [u8; 2],
        accredited_investor: bool,
    ) -> Result<()> {
        instructions::update_kyc::handler(ctx, kyc_verified, jurisdiction, accredited_investor)
    }

    pub fn list_asset(ctx: Context<ListAsset>, price_usdc: u64, amount: u64) -> Result<()> {
        instructions::list_asset::handler(ctx, price_usdc, amount)
    }

    pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        instructions::buy_asset::handler(ctx, amount)
    }

    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        instructions::redeem::handler(ctx, amount)
    }

    pub fn aggregate_external_asset(
        ctx: Context<AggregateExternalAsset>,
        payload: AggregatePayload,
    ) -> Result<()> {
        instructions::aggregate::aggregate_handler(ctx, payload)
    }

    pub fn update_external_asset(
        ctx: Context<UpdateExternalAsset>,
        verified: bool,
        active: bool,
    ) -> Result<()> {
        instructions::aggregate::update_handler(ctx, verified, active)
    }
}
