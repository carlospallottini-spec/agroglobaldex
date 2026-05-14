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
// Shared compliance helpers (used by buy_asset / future transfer hook).
// ---------------------------------------------------------------------------

/// Verify a wallet may receive or hold a regulated asset.
///
/// Policy (PoC, defense-in-depth alongside the on-chain `JurisdictionPolicy`
/// enforced by the compliance-hook):
/// - `kyc_verified` MUST be true.
/// - Jurisdiction MUST NOT be on the hard-coded blocklist.
/// - `HarvestFraction` and `InvestmentOffering` additionally require
///   `accredited_investor == true` (both are speculative yield instruments
///   and security tokens under MiCA/MiFID II).
pub fn enforce_compliance(
    record: &ComplianceRecord,
    asset_class: &AssetClass,
) -> Result<()> {
    require!(record.kyc_verified, AgroError::KycNotVerified);

    const BLOCKED: &[[u8; 2]] = &[*b"KP", *b"IR", *b"SY", *b"CU"];
    require!(
        !BLOCKED.contains(&record.jurisdiction),
        AgroError::JurisdictionNotAllowed
    );

    match asset_class {
        AssetClass::HarvestFraction { .. } | AssetClass::InvestmentOffering { .. } => {
            require!(
                record.accredited_investor,
                AgroError::AccreditedInvestorRequired
            );
        }
        _ => {}
    }
    Ok(())
}
