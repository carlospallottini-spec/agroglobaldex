use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::{transfer as usdc_transfer, Transfer as UsdcTransfer};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::state::*;

/// Withdraw accumulated protocol fees (USDC) from the treasury PDA to a
/// destination wallet of the authority's choice. Authority-only.
#[derive(Accounts)]
pub struct TreasuryWithdraw<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    /// CHECK: PDA validated by seeds + matches marketplace.treasury.
    #[account(
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
        constraint = treasury.key() == marketplace.treasury
            @ AgroError::ListingMismatch,
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(
        address = marketplace.usdc_mint @ AgroError::InvalidUsdcMint,
    )]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury,
    )]
    pub treasury_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: any wallet — recipient of the withdrawn USDC.
    pub destination: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = destination,
    )]
    pub destination_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TreasuryWithdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    require!(
        ctx.accounts.treasury_usdc_ata.amount >= amount,
        AgroError::InsufficientFunds
    );

    let marketplace_key = ctx.accounts.marketplace.key();
    let treasury_bump = ctx.accounts.marketplace.treasury_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        TREASURY_SEED,
        marketplace_key.as_ref(),
        std::slice::from_ref(&treasury_bump),
    ]];

    usdc_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.treasury_usdc_ata.to_account_info(),
                to: ctx.accounts.destination_usdc_ata.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    emit!(TreasuryWithdrawn {
        marketplace: marketplace_key,
        destination: ctx.accounts.destination.key(),
        amount,
    });
    Ok(())
}
