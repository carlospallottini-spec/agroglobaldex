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

    /// Pause/resume the marketplace circuit breaker. Authority-only.
    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        instructions::set_paused::handler(ctx, paused)
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

    pub fn update_listing_price(
        ctx: Context<UpdateListingPrice>,
        new_price_usdc: u64,
    ) -> Result<()> {
        instructions::update_listing_price::handler(ctx, new_price_usdc)
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        instructions::cancel_listing::handler(ctx)
    }

    pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        instructions::buy_asset::handler(ctx, amount)
    }

    /// Buy from a listing whose underlying mint is an EXTERNAL SPL token
    /// curated through `aggregate_external_asset`. Compliance-checked.
    pub fn buy_external_asset(ctx: Context<BuyExternalAsset>, amount: u64) -> Result<()> {
        instructions::buy_external_asset::handler(ctx, amount)
    }

    /// Authority withdraws accumulated protocol fees (USDC) from the treasury.
    pub fn treasury_withdraw(ctx: Context<TreasuryWithdraw>, amount: u64) -> Result<()> {
        instructions::treasury_withdraw::handler(ctx, amount)
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

    /// Revoke a wallet's KYC. Compliance-signer-only. Used for sanctions hits,
    /// fraud, or regulatory directives. Emits `ComplianceRevoked`.
    pub fn revoke_kyc(ctx: Context<RevokeKyc>, reason_code: u8) -> Result<()> {
        instructions::revoke_kyc::handler(ctx, reason_code)
    }

    /// Record an off-chain yield distribution for an `InvestmentOffering`.
    /// Issuer-only. Settlement of USDC is OFF-CHAIN; this is the on-chain
    /// receipt. Emits `InvestmentSettlementRecorded`.
    pub fn settle_investment_offering(
        ctx: Context<SettleInvestmentOffering>,
        epoch: u32,
        yield_paid_usdc: u64,
        attestation: [u8; 32],
    ) -> Result<()> {
        instructions::settle_investment_offering::handler(
            ctx,
            epoch,
            yield_paid_usdc,
            attestation,
        )
    }

    /// Update mutable asset metadata BEFORE the first mint. Issuer-only.
    /// After `mint_token`, `frozen_metadata=true` and this reverts.
    pub fn update_metadata(
        ctx: Context<UpdateAssetMetadata>,
        product_name: String,
        metadata_uri: String,
        white_paper_uri: String,
    ) -> Result<()> {
        instructions::update_metadata::handler(
            ctx,
            product_name,
            metadata_uri,
            white_paper_uri,
        )
    }

    /// Transfer the `issuer` role of an AssetRegistry to a new wallet.
    /// Current issuer signs. New issuer must already have valid KYC.
    pub fn transfer_issuer(ctx: Context<TransferIssuer>) -> Result<()> {
        instructions::transfer_issuer::handler(ctx)
    }

    // ──────────────────────────────────────────────────────────────────
    // Lending — collateralized USDC loans against tokenized agro-RWA.
    // The killer ag-finance primitive: instant credit against a tokenized
    // harvest / warehouse receipt without a bank.
    // ──────────────────────────────────────────────────────────────────

    /// Create the lending market for this marketplace. Authority-only.
    pub fn init_lending_market(
        ctx: Context<InitLendingMarket>,
        apr_bps: u16,
        max_ltv_bps: u16,
        liquidation_threshold_bps: u16,
        liquidation_bonus_bps: u16,
    ) -> Result<()> {
        instructions::lending::init_lending_market_handler(
            ctx,
            apr_bps,
            max_ltv_bps,
            liquidation_threshold_bps,
            liquidation_bonus_bps,
        )
    }

    /// Deposit USDC liquidity into the lending pool. Anyone can fund.
    pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
        instructions::lending::deposit_liquidity_handler(ctx, amount)
    }

    /// Withdraw previously-deposited USDC liquidity, bounded by the provider's
    /// tracked net principal and the pool's currently-available liquidity.
    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
        instructions::lending::withdraw_liquidity_handler(ctx, amount)
    }

    /// Set the collateral price (USDC per token) + enable flag for an asset.
    /// Authority acts as oracle relay (manual mode). For a real price feed use
    /// `set_collateral_oracle` + `refresh_collateral_price`.
    pub fn set_collateral_config(
        ctx: Context<SetCollateralConfig>,
        price_usdc_per_token: u64,
        enabled: bool,
    ) -> Result<()> {
        instructions::lending::set_collateral_config_handler(ctx, price_usdc_per_token, enabled)
    }

    /// Bind a Pyth price feed to a collateral. Authority-only. Once bound the
    /// price is driven by `refresh_collateral_price` and `open_loan` enforces
    /// staleness against `max_staleness_secs`.
    pub fn set_collateral_oracle(
        ctx: Context<SetCollateralOracle>,
        oracle_feed_id: [u8; 32],
        max_staleness_secs: i64,
        max_confidence_bps: u16,
        enabled: bool,
    ) -> Result<()> {
        instructions::oracle::set_collateral_oracle_handler(
            ctx,
            oracle_feed_id,
            max_staleness_secs,
            max_confidence_bps,
            enabled,
        )
    }

    /// Permissionless crank: read the Pyth `PriceUpdateV2` account, validate it
    /// (owner, discriminator, feed id, staleness, confidence) and cache the
    /// price into the collateral config. Anyone can call this.
    pub fn refresh_collateral_price(ctx: Context<RefreshCollateralPrice>) -> Result<()> {
        instructions::oracle::refresh_collateral_price_handler(ctx)
    }

    /// Open a loan: lock collateral Token-2022, receive USDC up to max LTV.
    pub fn open_loan(
        ctx: Context<OpenLoan>,
        collateral_amount: u64,
        borrow_amount: u64,
    ) -> Result<()> {
        instructions::lending::open_loan_handler(ctx, collateral_amount, borrow_amount)
    }

    /// Repay principal + accrued interest, unlock collateral.
    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        instructions::lending::repay_loan_handler(ctx)
    }

    /// Liquidate an unhealthy loan: repay the debt, seize the collateral.
    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        instructions::lending::liquidate_handler(ctx)
    }
}
