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
        mut,
        seeds = [
            ASSET_REGISTRY_SEED,
            marketplace.key().as_ref(),
            &asset_registry.index.to_le_bytes(),
        ],
        bump = asset_registry.bump,
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
    // Epochs must be strictly monotonic so the on-chain `last_settled_epoch`
    // is a reliable cursor. First call: epoch=0 with last_settled_epoch=0 and
    // total_yield_paid_usdc=0 is acceptable; subsequent calls must increment.
    let prior_epoch = ctx.accounts.asset_registry.last_settled_epoch;
    let already_settled_anything = ctx.accounts.asset_registry.total_yield_paid_usdc > 0
        || ctx.accounts.asset_registry.last_settled_at > 0;
    if already_settled_anything {
        require!(epoch > prior_epoch, AgroError::EpochNotMonotonic);
    }

    let now = Clock::get()?.unix_timestamp;
    let asset_key = ctx.accounts.asset_registry.key();
    let asset_mint = ctx.accounts.asset_registry.mint;
    let issuer_key = ctx.accounts.issuer.key();

    let registry = &mut ctx.accounts.asset_registry;
    registry.last_settled_epoch = epoch;
    registry.last_settled_at = now;
    registry.total_yield_paid_usdc = registry
        .total_yield_paid_usdc
        .checked_add(yield_paid_usdc)
        .ok_or(AgroError::PriceOverflow)?;

    emit!(InvestmentSettlementRecorded {
        asset_registry: asset_key,
        mint: asset_mint,
        issuer: issuer_key,
        epoch,
        yield_paid_usdc,
        attestation,
        recorded_at: now,
    });
    Ok(())
}
