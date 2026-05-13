use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Stamp / update the `ComplianceRecord` for a wallet. Only the marketplace's
/// `compliance_authority` PDA may sign — in practice this means only an
/// instruction routed through `update_kyc` with the marketplace authority as
/// the human signer, since the PDA is derived from the marketplace.
#[derive(Accounts)]
#[instruction(kyc_verified: bool, jurisdiction: [u8; 2])]
pub struct UpdateKyc<'info> {
    /// The human / service wallet permitted to manage compliance. Currently
    /// equal to `marketplace.authority` — production deployments should
    /// separate this role.
    #[account(mut)]
    pub compliance_signer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == compliance_signer.key()
            @ AgroError::UnauthorizedComplianceAuthority
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: only used to derive PDA & assert the address stored on marketplace.
    #[account(
        seeds = [COMPLIANCE_AUTHORITY_SEED, marketplace.key().as_ref()],
        bump = marketplace.compliance_bump,
        constraint = compliance_authority.key() == marketplace.compliance_authority
            @ AgroError::UnauthorizedComplianceAuthority
    )]
    pub compliance_authority: UncheckedAccount<'info>,

    /// CHECK: any wallet — does not need to sign, the record is about them.
    pub wallet: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = compliance_signer,
        space = 8 + ComplianceRecord::INIT_SPACE,
        seeds = [
            COMPLIANCE_RECORD_SEED,
            marketplace.key().as_ref(),
            wallet.key().as_ref()
        ],
        bump
    )]
    pub compliance_record: Account<'info, ComplianceRecord>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateKyc>,
    kyc_verified: bool,
    jurisdiction: [u8; 2],
    accredited_investor: bool,
) -> Result<()> {
    let record = &mut ctx.accounts.compliance_record;
    record.wallet = ctx.accounts.wallet.key();
    record.marketplace = ctx.accounts.marketplace.key();
    record.kyc_verified = kyc_verified;
    record.jurisdiction = jurisdiction;
    record.accredited_investor = accredited_investor;
    record.updated_at = Clock::get()?.unix_timestamp;
    record.bump = ctx.bumps.compliance_record;

    emit!(ComplianceUpdated {
        wallet: record.wallet,
        kyc_verified,
        jurisdiction,
        accredited_investor,
    });
    Ok(())
}

// ---------------------------------------------------------------------------
// Shared compliance helpers (used by buy_asset / future transfer hook).
// ---------------------------------------------------------------------------

/// Verify a wallet may receive or hold a regulated asset.
///
/// Policy (PoC):
/// - `kyc_verified` MUST be true.
/// - Jurisdiction MUST NOT be on a hard-coded blocklist (sanctioned regions).
/// - `HarvestFraction` additionally requires `accredited_investor == true`
///   because it is a yield-bearing speculative instrument.
pub fn enforce_compliance(
    record: &ComplianceRecord,
    asset_class: &AssetClass,
) -> Result<()> {
    require!(record.kyc_verified, AgroError::KycNotVerified);

    // Hard-coded sanctioned jurisdictions for the PoC. Replace with an
    // on-chain mutable allow/block-list owned by `compliance_authority`.
    const BLOCKED: &[[u8; 2]] = &[*b"KP", *b"IR", *b"SY"];
    require!(
        !BLOCKED.contains(&record.jurisdiction),
        AgroError::JurisdictionNotAllowed
    );

    if matches!(asset_class, AssetClass::HarvestFraction { .. }) {
        require!(
            record.accredited_investor,
            AgroError::AccreditedInvestorRequired
        );
    }
    Ok(())
}
