use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::{transfer as usdc_transfer, Transfer as UsdcTransfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::AgroError;
use crate::instructions::update_kyc::enforce_compliance;
use crate::state::*;

/// Buy `amount` tokens from a native listing, paying in USDC.
///
/// Settlement flow:
///   1. KYC + jurisdiction gate on the buyer.
///   2. Buyer → seller: (gross * (10000 - fee_bps) / 10000) USDC.
///   3. Buyer → treasury: (gross * fee_bps / 10000) USDC.
///   4. Listing escrow → buyer ATA: `amount` of the asset token.
#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [
            ASSET_REGISTRY_SEED,
            marketplace.key().as_ref(),
            &asset_registry.index.to_le_bytes(),
        ],
        bump = asset_registry.bump,
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        seeds = [
            LISTING_SEED,
            asset_registry.key().as_ref(),
            listing.seller.as_ref(),
        ],
        bump = listing.bump,
        constraint = listing.active @ AgroError::ListingUnavailable,
        constraint = listing.source == ListingSource::Native
            @ AgroError::ListingMismatch,
        constraint = listing.source_registry == asset_registry.key()
            @ AgroError::ListingMismatch,
        constraint = listing.mint == asset_registry.mint
            @ AgroError::ListingMismatch,
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(
        mut,
        address = listing.escrow,
    )]
    pub escrow: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: identity verified by listing.seller; only needs to receive USDC.
    #[account(address = listing.seller)]
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        address = asset_registry.mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            COMPLIANCE_RECORD_SEED,
            marketplace.key().as_ref(),
            buyer.key().as_ref()
        ],
        bump = buyer_compliance.bump,
        constraint = buyer_compliance.wallet == buyer.key()
            @ AgroError::KycNotVerified,
    )]
    pub buyer_compliance: Account<'info, ComplianceRecord>,

    #[account(
        seeds = [JURISDICTION_POLICY_SEED, marketplace.key().as_ref()],
        bump = jurisdiction_policy.bump,
        constraint = jurisdiction_policy.marketplace == marketplace.key()
            @ AgroError::JurisdictionPolicyMismatch,
    )]
    pub jurisdiction_policy: Account<'info, JurisdictionPolicy>,

    // ---- USDC side ---------------------------------------------------------
    #[account(
        address = marketplace.usdc_mint @ AgroError::InvalidUsdcMint,
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = buyer,
        associated_token::token_program = usdc_token_program,
    )]
    pub buyer_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = seller,
        associated_token::token_program = usdc_token_program,
    )]
    pub seller_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury,
        associated_token::token_program = usdc_token_program,
    )]
    pub treasury_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA validated by seeds + matches marketplace.treasury.
    #[account(
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
        constraint = treasury.key() == marketplace.treasury
            @ AgroError::ListingMismatch,
    )]
    pub treasury: UncheckedAccount<'info>,

    /// Immutable proof-of-trade. Derived from the global `trade_count` so it
    /// is unique and sequential. Created here (init) and never mutated again.
    #[account(
        init,
        payer = buyer,
        space = 8 + TradeReceipt::INIT_SPACE,
        seeds = [TRADE_RECEIPT_SEED, marketplace.key().as_ref(), &marketplace.trade_count.to_le_bytes()],
        bump
    )]
    pub trade_receipt: Account<'info, TradeReceipt>,

    // Two token programs because USDC is classic SPL Token and asset tokens
    // are SPL Token-2022.
    pub token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, BuyAsset<'info>>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    // Defense in depth: initialize already caps fee_bps at 1000 (10%) but we
    // re-check here so this handler is correct on its own — protects against
    // any future ix that mutates `marketplace.fee_bps` without re-validating.
    require!(
        ctx.accounts.marketplace.fee_bps <= 10_000,
        AgroError::FeeTooHigh
    );
    require!(
        ctx.accounts.listing.remaining >= amount,
        AgroError::ListingUnavailable
    );

    enforce_compliance(
        &ctx.accounts.buyer_compliance,
        &ctx.accounts.asset_registry.asset_class,
        &ctx.accounts.jurisdiction_policy,
    )?;

    // ---- Settlement math ---------------------------------------------------
    let gross = (amount as u128)
        .checked_mul(ctx.accounts.listing.price_usdc as u128)
        .ok_or(AgroError::PriceOverflow)?;
    let fee = gross
        .checked_mul(ctx.accounts.marketplace.fee_bps as u128)
        .ok_or(AgroError::PriceOverflow)?
        / 10_000u128;
    let seller_amount = gross.checked_sub(fee).ok_or(AgroError::PriceOverflow)?;

    let gross_u64: u64 = gross.try_into().map_err(|_| AgroError::PriceOverflow)?;
    let fee_u64: u64 = fee.try_into().map_err(|_| AgroError::PriceOverflow)?;
    let seller_u64: u64 = seller_amount
        .try_into()
        .map_err(|_| AgroError::PriceOverflow)?;

    require!(
        ctx.accounts.buyer_usdc_ata.amount >= gross_u64,
        AgroError::InsufficientFunds
    );

    // ---- USDC: buyer -> seller --------------------------------------------
    usdc_transfer(
        CpiContext::new(
            ctx.accounts.usdc_token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.buyer_usdc_ata.to_account_info(),
                to: ctx.accounts.seller_usdc_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        seller_u64,
    )?;

    // ---- USDC: buyer -> treasury (fee) ------------------------------------
    if fee_u64 > 0 {
        usdc_transfer(
            CpiContext::new(
                ctx.accounts.usdc_token_program.to_account_info(),
                UsdcTransfer {
                    from: ctx.accounts.buyer_usdc_ata.to_account_info(),
                    to: ctx.accounts.treasury_usdc_ata.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            fee_u64,
        )?;
    }

    // ---- Asset token: listing escrow -> buyer (Token-2022 hook fires) ------
    let asset_registry_key = ctx.accounts.asset_registry.key();
    let seller_key = ctx.accounts.listing.seller;
    let listing_bump = ctx.accounts.listing.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LISTING_SEED,
        asset_registry_key.as_ref(),
        seller_key.as_ref(),
        std::slice::from_ref(&listing_bump),
    ]];

    let decimals = ctx.accounts.mint.decimals;
    spl_token_2022::onchain::invoke_transfer_checked(
        &ctx.accounts.token_program.key(),
        ctx.accounts.escrow.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.buyer_token_account.to_account_info(),
        ctx.accounts.listing.to_account_info(),
        ctx.remaining_accounts,
        amount,
        decimals,
        signer_seeds,
    )?;

    let listing = &mut ctx.accounts.listing;
    listing.remaining = listing.remaining.saturating_sub(amount);
    if listing.remaining == 0 {
        listing.active = false;
    }
    let listing_key = listing.key();
    let listing_seller = listing.seller;
    let unit_price = listing.price_usdc;

    // ---- Proof-of-trade: mint the immutable TradeReceipt -------------------
    let buyer_jur = ctx.accounts.buyer_compliance.jurisdiction;
    let now = Clock::get()?.unix_timestamp;
    let trade_index = ctx.accounts.marketplace.trade_count;

    let receipt = &mut ctx.accounts.trade_receipt;
    receipt.marketplace = ctx.accounts.marketplace.key();
    receipt.listing = listing_key;
    receipt.asset_mint = ctx.accounts.mint.key();
    receipt.buyer = ctx.accounts.buyer.key();
    receipt.seller = listing_seller;
    receipt.source = ListingSource::Native;
    receipt.amount = amount;
    receipt.unit_price_usdc = unit_price;
    receipt.gross_usdc = gross_u64;
    receipt.fee_usdc = fee_u64;
    receipt.buyer_jurisdiction = buyer_jur;
    receipt.trade_index = trade_index;
    receipt.settled_at = now;
    receipt.bump = ctx.bumps.trade_receipt;

    let mp = &mut ctx.accounts.marketplace;
    mp.trade_count = mp
        .trade_count
        .checked_add(1)
        .ok_or(AgroError::PriceOverflow)?;

    emit!(AssetPurchased {
        listing: listing_key,
        buyer: ctx.accounts.buyer.key(),
        seller: listing_seller,
        amount,
        total_usdc: gross_u64,
        fee_usdc: fee_u64,
    });
    emit!(TradeReceiptCreated {
        trade_receipt: receipt.key(),
        marketplace: receipt.marketplace,
        buyer: receipt.buyer,
        seller: receipt.seller,
        asset_mint: receipt.asset_mint,
        source: ListingSource::Native,
        amount,
        gross_usdc: gross_u64,
        trade_index,
        settled_at: now,
    });

    Ok(())
}
