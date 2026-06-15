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

    /// The marketplace circuit breaker must be off to mint new supply.
    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    /// Deterministically reconstructed using the explicit `index` stored
    /// inside the registry itself. No reliance on client-supplied seeds.
    #[account(
        mut,
        seeds = [
            ASSET_REGISTRY_SEED,
            asset_registry.marketplace.as_ref(),
            &asset_registry.index.to_le_bytes(),
        ],
        bump = asset_registry.bump,
        has_one = issuer @ AgroError::UnauthorizedIssuer,
        has_one = mint,
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
    )]
    pub asset_registry: Box<Account<'info, AssetRegistry>>,

    /// The issuer must still hold a valid KYC at mint time; a revoked KYC
    /// blocks further minting.
    #[account(
        seeds = [
            COMPLIANCE_RECORD_SEED,
            marketplace.key().as_ref(),
            issuer.key().as_ref(),
        ],
        bump = issuer_compliance.bump,
        constraint = issuer_compliance.kyc_verified @ AgroError::KycNotVerified,
    )]
    pub issuer_compliance: Box<Account<'info, ComplianceRecord>>,

    #[account(
        mut,
        seeds = [ASSET_MINT_SEED, asset_registry.key().as_ref()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = issuer,
        associated_token::mint = mint,
        associated_token::authority = issuer,
        associated_token::token_program = token_program,
    )]
    pub issuer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

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
    require!(
        new_minted <= registry.total_supply,
        AgroError::SupplyExceeded
    );

    let registry_key = registry.key();
    let marketplace_key = registry.marketplace;
    let index_bytes = registry.index.to_le_bytes();
    let bump = registry.bump;

    let signer_seeds: &[&[&[u8]]] = &[&[
        ASSET_REGISTRY_SEED,
        marketplace_key.as_ref(),
        index_bytes.as_ref(),
        std::slice::from_ref(&bump),
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
