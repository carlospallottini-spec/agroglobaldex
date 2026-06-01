use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Record an off-chain yield distribution for an `InvestmentOffering`.
///
/// The actual USDC / fiat settlement happens OFF-CHAIN (bank wire, custodian
/// payout). This instruction is purely the on-chain RECEIPT so:
///   1. Holders can audit which epochs have been paid.
///   2. MLRO / regulators have an immutable log per offering.
///   3. The marketplace UI can show "Last distribution: ...".
///
/// Validations:
///   - Caller must be the asset issuer.
///   - The registry must be `AssetClass::InvestmentOffering`.
///   - `yield_paid_usdc > 0`.
///   - `epoch` is monotonically tracked off-chain via event indexing — there
///     is no on-chain epoch counter to avoid bloating `AssetRegistry`.
#[derive(Accounts)]
pub struct SettleInvestmentOffering<'info> {
    pub issuer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
        constraint = asset_registry.issuer == issuer.key()
            @ AgroError::UnauthorizedIssuer,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,
}

pub fn handler(
    ctx: Context<SettleInvestmentOffering>,
    epoch: u32,
    yield_paid_usdc: u64,
    attestation: [u8; 32],
) -> Result<()> {
    require!(yield_paid_usdc > 0, AgroError::InvalidAmount);
    require!(
        matches!(
            ctx.accounts.asset_registry.asset_class,
            AssetClass::InvestmentOffering { .. }
        ),
        AgroError::NotInvestmentOffering
    );

    let now = Clock::get()?.unix_timestamp;
    emit!(InvestmentSettlementRecorded {
        asset_registry: ctx.accounts.asset_registry.key(),
        mint: ctx.accounts.asset_registry.mint,
        issuer: ctx.accounts.issuer.key(),
        epoch,
        yield_paid_usdc,
        attestation,
        recorded_at: now,
    });
    Ok(())
}
