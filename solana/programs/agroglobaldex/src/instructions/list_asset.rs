use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, Token2022, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(price_lamports: u64, amount: u64)]
pub struct ListAsset<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        has_one = mint,
        constraint = asset_registry.marketplace == marketplace.key()
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        space = 8 + MarketplaceListing::INIT_SPACE,
        seeds = [
            LISTING_SEED,
            asset_registry.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump
    )]
    pub listing: Account<'info, MarketplaceListing>,

    /// Escrow token account owned by the listing PDA. It holds the tokens
    /// for sale until the listing is bought or cancelled.
    #[account(
        init,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub escrow: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ListAsset>, price_lamports: u64, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    require!(price_lamports > 0, AgroError::InvalidAmount);

    // Move tokens from seller -> escrow ATA owned by listing PDA.
    let decimals = ctx.accounts.mint.decimals;
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.seller_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.escrow.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer_checked(cpi_ctx, amount, decimals)?;

    let listing = &mut ctx.accounts.listing;
    listing.marketplace = ctx.accounts.marketplace.key();
    listing.asset_registry = ctx.accounts.asset_registry.key();
    listing.seller = ctx.accounts.seller.key();
    listing.escrow = ctx.accounts.escrow.key();
    listing.price_lamports = price_lamports;
    listing.remaining = amount;
    listing.created_at = Clock::get()?.unix_timestamp;
    listing.bump = ctx.bumps.listing;
    listing.active = true;

    emit!(AssetListed {
        listing: listing.key(),
        seller: listing.seller,
        asset_registry: listing.asset_registry,
        price_lamports,
        amount,
    });

    Ok(())
}
