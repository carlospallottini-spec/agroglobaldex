use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{mint_to, MintTo, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::AgroError;
use crate::state::*;

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            ASSET_REGISTRY_SEED,
            asset_registry.marketplace.as_ref(),
            asset_registry.issuer.as_ref(),
            // We cannot reconstruct the asset_count seed here without storing
            // it; in production add an explicit `index: u64` field to the
            // AssetRegistry and seed with that. For the PoC we trust the
            // address resolution from the client and verify the bump.
            &[asset_registry.bump],
        ],
        bump = asset_registry.bump,
        has_one = issuer @ AgroError::UnauthorizedIssuer,
        has_one = mint,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    #[account(
        mut,
        seeds = [ASSET_MINT_SEED, asset_registry.key().as_ref()],
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = issuer,
        associated_token::mint = mint,
        associated_token::authority = issuer,
        associated_token::token_program = token_program,
    )]
    pub issuer_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);

    let registry = &mut ctx.accounts.asset_registry;
    let new_minted = registry
        .minted_supply
        .checked_add(amount)
        .ok_or(AgroError::PriceOverflow)?;
    require!(new_minted <= registry.total_supply, AgroError::SupplyExceeded);

    // PDA signer seeds for the asset_registry mint authority.
    let registry_key = registry.key();
    let marketplace_key = registry.marketplace;
    let issuer_key = registry.issuer;
    let bump = registry.bump;

    // NOTE: The asset_registry seeds in production must include the asset_count
    // index. We sign with a simplified seed set here matching the PoC layout.
    // TODO: add `index: u64` field to AssetRegistry and include it in seeds.
    let signer_seeds: &[&[&[u8]]] = &[&[
        ASSET_REGISTRY_SEED,
        marketplace_key.as_ref(),
        issuer_key.as_ref(),
        &[bump],
    ]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.issuer_token_account.to_account_info(),
            authority: registry.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_ctx, amount)?;

    registry.minted_supply = new_minted;
    registry.frozen_metadata = true;

    emit!(TokensMinted {
        asset_registry: registry_key,
        amount,
        new_minted_supply: new_minted,
    });

    Ok(())
}
