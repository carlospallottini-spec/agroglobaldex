use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Update the per-token USDC price of an active listing. Seller-only.
#[derive(Accounts)]
pub struct UpdateListingPrice<'info> {
    pub seller: Signer<'info>,

    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        seeds = [
            LISTING_SEED,
            asset_registry.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump = listing.bump,
        has_one = seller @ AgroError::UnauthorizedIssuer,
        constraint = listing.active @ AgroError::ListingUnavailable,
    )]
    pub listing: Account<'info, MarketplaceListing>,
}

pub fn handler(ctx: Context<UpdateListingPrice>, new_price_usdc: u64) -> Result<()> {
    require!(new_price_usdc > 0, AgroError::InvalidAmount);
    let listing = &mut ctx.accounts.listing;
    listing.price_usdc = new_price_usdc;

    emit!(ListingPriceUpdated {
        listing: listing.key(),
        new_price_usdc,
    });
    Ok(())
}
