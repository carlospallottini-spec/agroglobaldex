use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Transfer the `issuer` role of an `AssetRegistry` to a different wallet.
///
/// Use cases:
///   - The producer entity rotates its operational wallet (e.g. hardware
///     wallet replacement).
///   - The producer sells the underlying SPV to another operator who takes
///     over future minting + settlement responsibilities.
///   - Disaster recovery: the original wallet is lost; the marketplace
///     authority cannot do this — only the current issuer can sign.
///
/// The new issuer MUST have a valid `ComplianceRecord` so we surface a
/// regulatory trail. (Stamping the new issuer must happen BEFORE this ix.)
#[derive(Accounts)]
pub struct TransferIssuer<'info> {
    pub current_issuer: Signer<'info>,

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
        constraint = asset_registry.issuer == current_issuer.key()
            @ AgroError::UnauthorizedIssuer,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    /// CHECK: the wallet receiving the issuer role. Must already have a
    /// valid ComplianceRecord (verified by the constraint on `new_issuer_compliance`).
    pub new_issuer: UncheckedAccount<'info>,

    /// New issuer's ComplianceRecord. Must be KYC-verified.
    #[account(
        seeds = [
            COMPLIANCE_RECORD_SEED,
            marketplace.key().as_ref(),
            new_issuer.key().as_ref(),
        ],
        bump = new_issuer_compliance.bump,
        constraint = new_issuer_compliance.kyc_verified @ AgroError::KycNotVerified,
    )]
    pub new_issuer_compliance: Account<'info, ComplianceRecord>,
}

pub fn handler(ctx: Context<TransferIssuer>) -> Result<()> {
    let new = ctx.accounts.new_issuer.key();
    let registry = &mut ctx.accounts.asset_registry;
    let old = registry.issuer;
    require!(new != old, AgroError::InvalidIssuer);
    require!(new != Pubkey::default(), AgroError::InvalidIssuer);

    registry.issuer = new;

    emit!(IssuerTransferred {
        asset_registry: registry.key(),
        old_issuer: old,
        new_issuer: new,
    });
    Ok(())
}
