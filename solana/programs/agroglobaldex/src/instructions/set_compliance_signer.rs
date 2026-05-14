use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Rotate the wallet allowed to stamp KYC records. Authority-only.
#[derive(Accounts)]
pub struct SetComplianceSigner<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: any pubkey — becomes the new compliance signer.
    pub new_signer: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetComplianceSigner>) -> Result<()> {
    let mp = &mut ctx.accounts.marketplace;
    let old = mp.compliance_signer;
    mp.compliance_signer = ctx.accounts.new_signer.key();

    emit!(ComplianceSignerRotated {
        marketplace: mp.key(),
        old_signer: old,
        new_signer: mp.compliance_signer,
    });
    Ok(())
}
