use anchor_lang::prelude::*;
use anchor_spl::token_2022::{burn, Burn, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::state::*;

/// Burn tokens to redeem the underlying physical commodity (grain delivery)
/// or retire a carbon credit. The off-chain workflow watches the
/// `AssetRedeemed` event and physically settles to the holder.
///
/// Gated by `marketplace.paused` so a circuit-breaker scenario halts
/// off-chain delivery commitments too.
#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [
            ASSET_REGISTRY_SEED,
            marketplace.key().as_ref(),
            &asset_registry.index.to_le_bytes(),
        ],
        bump = asset_registry.bump,
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
        constraint = asset_registry.redeemable @ AgroError::AssetNotRedeemable,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        address = asset_registry.mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = holder,
        associated_token::token_program = token_program,
    )]
    pub holder_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<Redeem>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.holder_token_account.to_account_info(),
            authority: ctx.accounts.holder.to_account_info(),
        },
    );
    burn(cpi_ctx, amount)?;

    let registry = &mut ctx.accounts.asset_registry;
    registry.redeemed_supply = registry
        .redeemed_supply
        .checked_add(amount)
        .ok_or(AgroError::PriceOverflow)?;

    emit!(AssetRedeemed {
        asset_registry: registry.key(),
        holder: ctx.accounts.holder.key(),
        amount,
        redeemed_at: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
