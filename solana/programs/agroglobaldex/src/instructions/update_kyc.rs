use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Stamp / update the `ComplianceRecord` for a wallet.
///
/// Authorization model: only `marketplace.compliance_signer` may sign. This
/// is intentionally separate from `marketplace.authority` so compliance ops
/// can be delegated to a service account without touching treasury funds.
#[derive(Accounts)]
#[instruction(kyc_verified: bool, jurisdiction: [u8; 2])]
pub struct UpdateKyc<'info> {
    #[account(mut)]
    pub compliance_signer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.compliance_signer == compliance_signer.key()
            @ AgroError::UnauthorizedComplianceAuthority
    )]
    pub marketplace: Account<'info, Marketplace>,

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
// Shared compliance helpers (used by buy_asset / buy_external_asset).
// ---------------------------------------------------------------------------
//
// The jurisdiction blocklist and the "requires accredited" overlay both live
// on-chain in `JurisdictionPolicy`. Helpers below take the policy account by
// reference so callers MUST pass the live on-chain config; there is no
// hardcoded fallback.

/// Verify a wallet may receive or hold a regulated asset.
///
/// Performs (in order):
///   1. KYC: `record.kyc_verified == true`.
///   2. Jurisdiction blocklist: `record.jurisdiction ∉ policy.blocked`.
///   3. Asset-class accredited gate: `HarvestFraction` and `InvestmentOffering`
///      always require `record.accredited_investor == true`.
///   4. Policy accredited overlay: if the wallet's jurisdiction is in
///      `policy.requires_accredited`, the wallet must also be accredited.
pub fn enforce_compliance(
    record: &ComplianceRecord,
    asset_class: &AssetClass,
    policy: &JurisdictionPolicy,
) -> Result<()> {
    enforce_compliance_basic(record, asset_class, policy)
}

/// Same rules as `enforce_compliance`. Kept under a separate name so call
/// sites (e.g. `buy_external_asset`) can express intent: the rules are the
/// same for natives and curated externals.
pub fn enforce_compliance_basic(
    record: &ComplianceRecord,
    asset_class: &AssetClass,
    policy: &JurisdictionPolicy,
) -> Result<()> {
    require!(record.kyc_verified, AgroError::KycNotVerified);

    require!(
        !policy.blocked.iter().any(|j| j == &record.jurisdiction),
        AgroError::JurisdictionNotAllowed
    );

    let class_requires_accredited = matches!(
        asset_class,
        AssetClass::HarvestFraction { .. } | AssetClass::InvestmentOffering { .. }
    );
    let jurisdiction_requires_accredited = policy
        .requires_accredited
        .iter()
        .any(|j| j == &record.jurisdiction);

    if class_requires_accredited || jurisdiction_requires_accredited {
        require!(
            record.accredited_investor,
            AgroError::AccreditedInvestorRequired
        );
    }
    Ok(())
}
