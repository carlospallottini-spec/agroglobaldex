//! # AgroGlobalDex Program
//!
//! Solana/Anchor program powering the AgroGlobalDex marketplace: a MiCA-aligned
//! venue for tokenized agricultural Real-World Assets (RWAs).
//!
//! Supported asset classes:
//! - `Grain`            -> physically-backed commodity tokens (soy, corn, wheat)
//! - `CarbonCredit`     -> agro carbon credits (kg CO2eq)
//! - `HarvestFraction`  -> fractional future harvests (hectares + year)
//!
//! ## Compliance model
//! Every wallet that touches a regulated asset must have a `ComplianceRecord`
//! PDA stamped by the marketplace `compliance_authority`. The token mints are
//! created via **SPL Token-2022** so we can attach a transfer-hook to enforce
//! KYC/jurisdiction checks on-chain at every transfer.
//!
//! NOTE: this is a Proof-of-Concept scaffold. It is NOT audited and MUST NOT
//! be used in mainnet with real value. See `README.md` for the roadmap.

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::AssetClass;

// Placeholder program id. Replace after first `anchor build` (run
// `anchor keys sync` to update both Anchor.toml and declare_id!).
declare_id!("AGRoG1obA1Dex11111111111111111111111111111");

#[program]
pub mod agroglobaldex {
    use super::*;

    /// Initialize the marketplace global config.
    /// Stores the protocol authority and the compliance authority PDA seed.
    pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        instructions::initialize::handler(ctx, fee_bps)
    }

    /// Register a new tokenizable asset (one `AssetRegistry` per real-world lot).
    ///
    /// `oracle_attestation` is the SHA-256 hash of an off-chain certificate
    /// (warehouse receipt for grain, VCS/Gold-Standard issuance for carbon,
    /// notarized land + crop plan for harvest fractions).
    ///
    /// `white_paper_uri` is required by MiCA Art. 6 for asset-referenced tokens.
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

    /// Mint tokens against a registered asset. Only the issuer may mint and only
    /// up to `total_supply` declared in the `AssetRegistry`.
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::mint_token::handler(ctx, amount)
    }

    /// Update KYC / jurisdiction flags for a wallet. Only callable by the
    /// `compliance_authority` of the marketplace.
    pub fn update_kyc(
        ctx: Context<UpdateKyc>,
        kyc_verified: bool,
        jurisdiction: [u8; 2], // ISO-3166-alpha-2
        accredited_investor: bool,
    ) -> Result<()> {
        instructions::update_kyc::handler(ctx, kyc_verified, jurisdiction, accredited_investor)
    }

    /// List a tokenized asset on the marketplace at a fixed price (lamports per token).
    pub fn list_asset(ctx: Context<ListAsset>, price_lamports: u64, amount: u64) -> Result<()> {
        instructions::list_asset::handler(ctx, price_lamports, amount)
    }

    /// Buy a listed asset. Performs compliance check on the buyer before settling.
    pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        instructions::buy_asset::handler(ctx, amount)
    }

    /// Burn tokens to redeem the underlying off-chain commodity. Emits an event
    /// the off-chain workflow listens to in order to release the physical good.
    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        instructions::redeem::handler(ctx, amount)
    }
}
