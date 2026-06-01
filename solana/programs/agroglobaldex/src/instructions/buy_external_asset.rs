use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::{transfer as usdc_transfer, Transfer as UsdcTransfer};
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::AgroError;
use crate::instructions::update_kyc::enforce_compliance_basic;
use crate::state::*;

/// Buy from a listing whose underlying mint is an EXTERNAL SPL token
/// (Solana SPL or Token-2022) curated through `aggregate_external_asset`.
///
/// External SPLs do NOT necessarily have a TransferHook nor live under our
/// compliance regime, so the gate here is the marketplace-level compliance
/// check on the buyer (KYC + jurisdiction). Asset-class accredited gating is
/// reused via the curated `ExternalAssetRegistry.asset_class`.
#[derive(Accounts)]
pub struct BuyExternalAsset<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
    constraint = !marketplace.paused @ AgroError::Paused,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        constraint = external_asset.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
        constraint = external_asset.active @ AgroError::ExternalAssetInactive,
        constraint = external_asset.mint.is_some() @ AgroError::CrossChainNotTradable,
    )]
    pub external_asset: Account<'info, ExternalAssetRegistry>,

    #[account(
        mut,
        seeds = [
            LISTING_SEED,
            external_asset.key().as_ref(),
            listing.seller.as_ref(),
        ],
        bump = listing.bump,
        constraint = listing.active @ AgroError::ListingUnavailable,
        constraint = listing.source == ListingSource::External
            @ AgroError::ListingMismatch,
        constraint = listing.source_registry == external_asset.key()
            @ AgroError::ListingMismatch,
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(mut, address = listing.escrow)]
    pub escrow: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: identity verified by listing.seller.
    #[account(address = listing.seller)]
    pub seller: UncheckedAccount<'info>,

    #[account(mut, address = listing.mint)]
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

    #[account(address = marketplace.usdc_mint @ AgroError::InvalidUsdcMint)]
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

    /// CHECK: PDA validated by seeds.
    #[account(
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
        constraint = treasury.key() == marketplace.treasury
            @ AgroError::ListingMismatch,
    )]
    pub treasury: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyExternalAsset>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    let listing = &mut ctx.accounts.listing;
    require!(listing.remaining >= amount, AgroError::ListingUnavailable);

    enforce_compliance_basic(
        &ctx.accounts.buyer_compliance,
        &ctx.accounts.external_asset.asset_class,
        &ctx.accounts.jurisdiction_policy,
    )?;

    let gross = (amount as u128)
        .checked_mul(listing.price_usdc as u128)
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

    // USDC: buyer -> seller
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

    // External token: listing escrow -> buyer
    let external_key = ctx.accounts.external_asset.key();
    let seller_key = listing.seller;
    let listing_bump = listing.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LISTING_SEED,
        external_key.as_ref(),
        seller_key.as_ref(),
        std::slice::from_ref(&listing_bump),
    ]];

    let decimals = ctx.accounts.mint.decimals;
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: listing.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        decimals,
    )?;

    listing.remaining = listing.remaining.saturating_sub(amount);
    if listing.remaining == 0 {
        listing.active = false;
    }

    emit!(AssetPurchased {
        listing: listing.key(),
        buyer: ctx.accounts.buyer.key(),
        seller: listing.seller,
        amount,
        total_usdc: gross_u64,
        fee_usdc: fee_u64,
    });

    Ok(())
}
