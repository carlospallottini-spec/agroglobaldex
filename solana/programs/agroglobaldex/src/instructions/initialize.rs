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

    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [MARKETPLACE_SEED, authority.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: PDA — never read or written here, only its address is stored on
    /// the marketplace so other instructions can require it as `Signer` via
    /// `seeds`/`bump` when stamping `ComplianceRecord`s.
    #[account(
        seeds = [COMPLIANCE_AUTHORITY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub compliance_authority: UncheckedAccount<'info>,

    /// CHECK: PDA that owns the USDC treasury ATA. Validated by seeds.
    #[account(
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub treasury: UncheckedAccount<'info>,

    /// USDC mint (devnet or mainnet — caller passes the correct address).
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    /// Treasury USDC ATA, created here so fees can flow on the very first buy.
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
    marketplace.compliance_authority = ctx.accounts.compliance_authority.key();
    marketplace.usdc_mint = ctx.accounts.usdc_mint.key();
    marketplace.treasury = ctx.accounts.treasury.key();
    marketplace.bump = ctx.bumps.marketplace;
    marketplace.compliance_bump = ctx.bumps.compliance_authority;
    marketplace.treasury_bump = ctx.bumps.treasury;
    marketplace.fee_bps = fee_bps;
    marketplace.asset_count = 0;
    marketplace.external_asset_count = 0;

    msg!(
        "AgroGlobalDex marketplace initialized. authority={} usdc_mint={} fee_bps={}",
        marketplace.authority,
        marketplace.usdc_mint,
        fee_bps
    );
    Ok(())
}
