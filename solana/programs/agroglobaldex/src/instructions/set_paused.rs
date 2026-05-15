use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Set the marketplace `paused` flag. Authority-only. While paused, all
/// write paths (register_asset, mint_token, list_asset, buy_asset,
/// buy_external_asset, aggregate_external_asset) revert with `Paused`.
/// Read-only access (fetching listings, redeem of already-minted tokens by
/// holder) remains unaffected.
#[derive(Accounts)]
pub struct SetPaused<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,
}

pub fn handler(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
    let mp = &mut ctx.accounts.marketplace;
    let was = mp.paused;
    mp.paused = paused;

    emit!(PauseChanged {
        marketplace: mp.key(),
        was,
        now: paused,
    });
    Ok(())
}
