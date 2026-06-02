use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Revoke a wallet's KYC. Only the compliance signer may invoke. Used when a
/// post-onboarding event (sanctions hit, fraud detection, regulatory request)
/// requires immediately blocking a wallet from buying or holding regulated
/// assets.
///
/// This sets `kyc_verified = false` on the `ComplianceRecord` and emits a
/// `ComplianceRevoked` event with the `reason_code` so the off-chain
/// surveillance pipeline can prioritize the case.
///
/// Note: revoking KYC does NOT seize tokens already held. For clawback the
/// authority must operate at the SPL Token-2022 layer (PermanentDelegate
/// extension, future work).
#[derive(Accounts)]
pub struct RevokeKyc<'info> {
    #[account(mut)]
    pub compliance_signer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.compliance_signer == compliance_signer.key()
            @ AgroError::UnauthorizedComplianceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: identified by the existing ComplianceRecord PDA seeds.
    pub wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            COMPLIANCE_RECORD_SEED,
            marketplace.key().as_ref(),
            wallet.key().as_ref()
        ],
        bump = compliance_record.bump,
        constraint = compliance_record.wallet == wallet.key()
            @ AgroError::KycNotVerified,
    )]
    pub compliance_record: Account<'info, ComplianceRecord>,
}

pub fn handler(ctx: Context<RevokeKyc>, reason_code: u8) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let record = &mut ctx.accounts.compliance_record;
    record.kyc_verified = false;
    record.updated_at = now;

    emit!(ComplianceRevoked {
        marketplace: ctx.accounts.marketplace.key(),
        wallet: record.wallet,
        reason_code,
        revoked_at: now,
    });
    Ok(())
}
