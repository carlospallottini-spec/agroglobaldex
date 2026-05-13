use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::Mint;

use crate::errors::AgroError;
use crate::state::*;

/// Registers a new tokenized asset and creates its SPL Token-2022 mint.
///
/// The mint's authority is the `AssetRegistry` PDA itself, so only this
/// program can mint or freeze. We rely on **Token-2022 transfer hooks** to
/// gate transfers against the on-chain `ComplianceRecord` — see
/// `// TODO: transfer hook` below.
#[derive(Accounts)]
#[instruction(asset_class: AssetClass, total_supply: u64)]
pub struct RegisterAsset<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = issuer,
        space = 8 + AssetRegistry::INIT_SPACE,
        seeds = [
            ASSET_REGISTRY_SEED,
            marketplace.key().as_ref(),
            issuer.key().as_ref(),
            &marketplace.asset_count.to_le_bytes(),
        ],
        bump
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    /// Token-2022 mint, PDA so the program controls authorities.
    ///
    /// NOTE: For a *real* deployment you should initialize the mint with the
    /// `TransferHook` extension pointing at a sibling program that calls back
    /// into AgroGlobalDex to verify the receiver's `ComplianceRecord`. That
    /// adds substantial complexity, so this scaffold initializes a vanilla
    /// Token-2022 mint and enforces compliance only at the `buy_asset`
    /// instruction boundary. See the README "Roadmap to production".
    #[account(
        init,
        payer = issuer,
        seeds = [ASSET_MINT_SEED, asset_registry.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = asset_registry,
        mint::freeze_authority = asset_registry,
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterAsset>,
    asset_class: AssetClass,
    total_supply: u64,
    oracle_attestation: [u8; 32],
    white_paper_uri: String,
    metadata_uri: String,
) -> Result<()> {
    require!(total_supply > 0, AgroError::InvalidAmount);
    require!(!white_paper_uri.is_empty(), AgroError::MissingWhitePaper);
    require!(
        white_paper_uri.len() <= MAX_URI_LEN && metadata_uri.len() <= MAX_URI_LEN,
        AgroError::StringTooLong
    );

    // Sanity-check class-specific metadata.
    match &asset_class {
        AssetClass::Grain { tons, .. } => {
            require!(*tons > 0, AgroError::InvalidAssetMetadata);
        }
        AssetClass::CarbonCredit {
            kg_co2eq,
            vintage_year,
            ..
        } => {
            require!(
                *kg_co2eq > 0 && *vintage_year >= 2000 && *vintage_year <= 2100,
                AgroError::InvalidAssetMetadata
            );
        }
        AssetClass::HarvestFraction {
            hectares,
            harvest_year,
            ..
        } => {
            require!(
                *hectares > 0 && *harvest_year >= 2024 && *harvest_year <= 2100,
                AgroError::InvalidAssetMetadata
            );
        }
    }

    let marketplace = &mut ctx.accounts.marketplace;
    let registry = &mut ctx.accounts.asset_registry;

    registry.marketplace = marketplace.key();
    registry.issuer = ctx.accounts.issuer.key();
    registry.mint = ctx.accounts.mint.key();
    registry.asset_class = asset_class;
    registry.total_supply = total_supply;
    registry.minted_supply = 0;
    registry.oracle_attestation = oracle_attestation;
    registry.white_paper_uri = white_paper_uri;
    registry.metadata_uri = metadata_uri;
    // Carbon credits & grain warehouse tokens are redeemable; harvest fractions
    // settle to cash at harvest time, so they are NOT redeemable in-protocol.
    registry.redeemable = !matches!(asset_class, AssetClass::HarvestFraction { .. });
    registry.frozen_metadata = false;
    registry.bump = ctx.bumps.asset_registry;

    marketplace.asset_count = marketplace
        .asset_count
        .checked_add(1)
        .ok_or(AgroError::PriceOverflow)?;

    // Silence unused-import warning until we actually CPI into token_2022.
    let _ = token_2022::ID;

    emit!(AssetRegistered {
        marketplace: marketplace.key(),
        asset_registry: registry.key(),
        issuer: registry.issuer,
        mint: registry.mint,
    });

    Ok(())
}
