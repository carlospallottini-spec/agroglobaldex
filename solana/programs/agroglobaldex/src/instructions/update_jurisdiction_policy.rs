use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Replace both jurisdiction lists. Authority-only.
#[derive(Accounts)]
pub struct UpdateJurisdictionPolicy<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [JURISDICTION_POLICY_SEED, marketplace.key().as_ref()],
        bump = policy.bump,
        constraint = policy.marketplace == marketplace.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub policy: Account<'info, JurisdictionPolicy>,
}

pub fn handler(
    ctx: Context<UpdateJurisdictionPolicy>,
    blocked: Vec<[u8; 2]>,
    requires_accredited: Vec<[u8; 2]>,
) -> Result<()> {
    require!(
        blocked.len() <= MAX_JURISDICTIONS && requires_accredited.len() <= MAX_JURISDICTIONS,
        AgroError::TooManyJurisdictions
    );
    let policy = &mut ctx.accounts.policy;
    policy.blocked = blocked.clone();
    policy.requires_accredited = requires_accredited.clone();

    emit!(JurisdictionPolicyUpdated {
        marketplace: policy.marketplace,
        blocked,
        requires_accredited,
    });
    Ok(())
}
