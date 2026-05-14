use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: any pubkey — the wallet permitted to stamp ComplianceRecords.
    /// Distinct from `authority` so compliance ops can be delegated to a
    /// service account without touching treasury funds. Can equal authority.
    pub compliance_signer: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [MARKETPLACE_SEED, authority.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: PDA — informational only (legacy).
    #[account(
        seeds = [COMPLIANCE_AUTHORITY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub compliance_authority: UncheckedAccount<'info>,

    /// CHECK: PDA that owns the USDC treasury ATA.
    #[account(
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub treasury: UncheckedAccount<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury,
    )]
    pub treasury_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 1_000, AgroError::FeeTooHigh);

    let marketplace = &mut ctx.accounts.marketplace;
    marketplace.authority = ctx.accounts.authority.key();
    marketplace.compliance_signer = ctx.accounts.compliance_signer.key();
    marketplace.compliance_authority = ctx.accounts.compliance_authority.key();
    marketplace.usdc_mint = ctx.accounts.usdc_mint.key();
    marketplace.treasury = ctx.accounts.treasury.key();
    marketplace.bump = ctx.bumps.marketplace;
    marketplace.compliance_bump = ctx.bumps.compliance_authority;
    marketplace.treasury_bump = ctx.bumps.treasury;
    marketplace.fee_bps = fee_bps;
    marketplace.asset_count = 0;
    marketplace.external_asset_count = 0;
    marketplace.paused = false;

    msg!(
        "AgroGlobalDex marketplace initialized. authority={} compliance_signer={} usdc_mint={} fee_bps={}",
        marketplace.authority,
        marketplace.compliance_signer,
        marketplace.usdc_mint,
        fee_bps
    );
    Ok(())
}
