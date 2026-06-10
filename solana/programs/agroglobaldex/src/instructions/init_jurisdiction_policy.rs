use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Initialize the on-chain mutable JurisdictionPolicy. Authority-only.
/// Defaults to a conservative blocklist (KP/IR/SY/CU) and empty
/// requires_accredited list.
#[derive(Accounts)]
pub struct InitJurisdictionPolicy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        init,
        payer = authority,
        space = 8 + JurisdictionPolicy::INIT_SPACE,
        seeds = [JURISDICTION_POLICY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub policy: Box<Account<'info, JurisdictionPolicy>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitJurisdictionPolicy>) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.marketplace = ctx.accounts.marketplace.key();
    policy.blocked = vec![*b"KP", *b"IR", *b"SY", *b"CU"];
    policy.requires_accredited = vec![];
    policy.bump = ctx.bumps.policy;

    emit!(JurisdictionPolicyUpdated {
        marketplace: policy.marketplace,
        blocked: policy.blocked.clone(),
        requires_accredited: policy.requires_accredited.clone(),
    });
    Ok(())
}
