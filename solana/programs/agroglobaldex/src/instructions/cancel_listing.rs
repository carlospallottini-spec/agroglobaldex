use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::AgroError;
use crate::state::*;

/// Cancel an active listing. Returns escrowed tokens to the seller and
/// closes the listing PDA, refunding rent to the seller.
#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        close = seller,
        seeds = [
            LISTING_SEED,
            asset_registry.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump = listing.bump,
        has_one = seller @ AgroError::UnauthorizedIssuer,
        constraint = listing.source_registry == asset_registry.key()
            @ AgroError::ListingMismatch,
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(
        mut,
        address = listing.escrow,
    )]
    pub escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, address = listing.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CancelListing>) -> Result<()> {
    let listing = &ctx.accounts.listing;
    let amount = listing.remaining;

    if amount > 0 {
        let asset_registry_key = ctx.accounts.asset_registry.key();
        let seller_key = listing.seller;
        let listing_bump = listing.bump;
        let signer_seeds: &[&[&[u8]]] = &[&[
            LISTING_SEED,
            asset_registry_key.as_ref(),
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
                    to: ctx.accounts.seller_token_account.to_account_info(),
                    authority: listing.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            decimals,
        )?;
    }

    msg!("Listing cancelled. amount_returned={}", amount);
    Ok(())
}
