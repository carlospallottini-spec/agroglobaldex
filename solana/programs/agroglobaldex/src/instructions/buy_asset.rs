use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer as sol_transfer, Transfer as SolTransfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, Token2022, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::instructions::update_kyc::enforce_compliance;
use crate::state::*;

#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: lamports recipient for the fee — must match marketplace.authority.
    #[account(
        mut,
        constraint = fee_recipient.key() == marketplace.authority
            @ AgroError::UnauthorizedMarketplaceAuthority
    )]
    pub fee_recipient: UncheckedAccount<'info>,

    #[account(constraint = asset_registry.marketplace == marketplace.key())]
    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        has_one = asset_registry,
        constraint = listing.active @ AgroError::ListingUnavailable,
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(
        mut,
        address = listing.escrow,
    )]
    pub escrow: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: paid in SOL — checked by address only.
    #[account(mut, address = listing.seller)]
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

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    let listing = &mut ctx.accounts.listing;
    require!(listing.remaining >= amount, AgroError::ListingUnavailable);

    // ---- Compliance gate ----------------------------------------------------
    enforce_compliance(
        &ctx.accounts.buyer_compliance,
        &ctx.accounts.asset_registry.asset_class,
    )?;

    // ---- Compute settlement -------------------------------------------------
    let gross = (amount as u128)
        .checked_mul(listing.price_lamports as u128)
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
        ctx.accounts.buyer.lamports() >= gross_u64,
        AgroError::InsufficientFunds
    );

    // ---- Pay seller + fee in SOL -------------------------------------------
    sol_transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            SolTransfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        seller_u64,
    )?;
    if fee_u64 > 0 {
        sol_transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SolTransfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.fee_recipient.to_account_info(),
                },
            ),
            fee_u64,
        )?;
    }

    // ---- Transfer tokens out of escrow to buyer -----------------------------
    let asset_registry_key = ctx.accounts.asset_registry.key();
    let seller_key = listing.seller;
    let listing_bump = listing.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LISTING_SEED,
        asset_registry_key.as_ref(),
        seller_key.as_ref(),
        &[listing_bump],
    ]];

    let decimals = ctx.accounts.mint.decimals;
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.escrow.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: listing.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, amount, decimals)?;

    listing.remaining = listing.remaining.saturating_sub(amount);
    if listing.remaining == 0 {
        listing.active = false;
    }

    emit!(AssetPurchased {
        listing: listing.key(),
        buyer: ctx.accounts.buyer.key(),
        seller: listing.seller,
        amount,
        total_lamports: gross_u64,
        fee_lamports: fee_u64,
    });

    Ok(())
}
