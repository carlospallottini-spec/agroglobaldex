use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [MARKETPLACE_SEED, authority.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: PDA — never read or written here, only its address is stored on
    /// the marketplace so other instructions can require it as `Signer` via
    /// `seeds`/`bump` when stamping `ComplianceRecord`s.
    #[account(
        seeds = [COMPLIANCE_AUTHORITY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub compliance_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 1_000, AgroError::FeeTooHigh);

    let marketplace = &mut ctx.accounts.marketplace;
    marketplace.authority = ctx.accounts.authority.key();
    marketplace.compliance_authority = ctx.accounts.compliance_authority.key();
    marketplace.bump = ctx.bumps.marketplace;
    marketplace.compliance_bump = ctx.bumps.compliance_authority;
    marketplace.fee_bps = fee_bps;
    marketplace.asset_count = 0;

    msg!(
        "AgroGlobalDex marketplace initialized. authority={} fee_bps={}",
        marketplace.authority,
        fee_bps
    );
    Ok(())
}
