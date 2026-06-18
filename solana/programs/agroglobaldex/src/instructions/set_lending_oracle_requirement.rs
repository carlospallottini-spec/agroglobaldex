use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Toggle whether this lending market REQUIRES oracle-priced collateral.
/// Authority-only. When `required == true`, `open_loan` and `liquidate` reject
/// any collateral whose price is set manually (`oracle_enabled == false`),
/// closing the oracle-manipulation vector of audit H-1. Defaults to `false` at
/// market init so manual-priced devnet flows keep working unchanged.
#[derive(Accounts)]
pub struct SetLendingOracleRequirement<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, marketplace.key().as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,
}

pub fn handler(ctx: Context<SetLendingOracleRequirement>, required: bool) -> Result<()> {
    let lm = &mut ctx.accounts.lending_market;
    let was = lm.require_oracle_for_loans;
    lm.require_oracle_for_loans = required;

    emit!(LendingOracleRequirementChanged {
        lending_market: lm.key(),
        was,
        now: required,
    });
    Ok(())
}
