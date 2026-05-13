use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::token_2022::{
    initialize_mint2,
    spl_token_2022::{
        extension::{transfer_hook, ExtensionType},
        state::Mint as MintState,
    },
    InitializeMint2, Token2022,
};

use crate::errors::AgroError;
use crate::state::*;

/// Registers a new tokenized asset and creates its SPL Token-2022 mint
/// with the `TransferHook` extension pointing at the compliance-hook program.
///
/// We do the mint init manually (rather than via Anchor's `init` constraint)
/// because Token-2022 extensions must be initialized in a precise order:
///   1. `create_account` with enough space for the extension(s).
///   2. `transfer_hook::initialize` to wire the hook program id.
///   3. `initialize_mint2` to write the mint state itself.
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
            &marketplace.asset_count.to_le_bytes(),
        ],
        bump
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    /// Token-2022 mint PDA. Created manually below with the TransferHook
    /// extension wired to the compliance-hook program id.
    ///
    /// CHECK: validated by seeds + initialized via CPI inside the handler.
    #[account(
        mut,
        seeds = [ASSET_MINT_SEED, asset_registry.key().as_ref()],
        bump,
    )]
    pub mint: UncheckedAccount<'info>,

    /// The compliance-hook program id. Stored inside the TransferHook
    /// extension so every transfer of the mint CPIs into it.
    /// CHECK: passed in explicitly so deployments can rotate hooks if needed.
    pub compliance_hook_program: UncheckedAccount<'info>,

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
    let index = marketplace.asset_count;

    let token_program_id = ctx.accounts.token_program.key();
    let registry_key = ctx.accounts.asset_registry.key();
    let mint_key = ctx.accounts.mint.key();
    let hook_program_id = ctx.accounts.compliance_hook_program.key();
    let registry_authority = registry_key;

    // ---- Create mint account with space for the TransferHook extension ---
    let extensions = [ExtensionType::TransferHook];
    let mint_space = ExtensionType::try_calculate_account_len::<MintState>(&extensions)
        .map_err(|_| AgroError::InvalidAssetMetadata)?;
    let rent_lamports = Rent::get()?.minimum_balance(mint_space);

    let mint_bump = ctx.bumps.mint;
    let mint_seeds: &[&[u8]] = &[
        ASSET_MINT_SEED,
        registry_key.as_ref(),
        std::slice::from_ref(&mint_bump),
    ];

    create_account(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            CreateAccount {
                from: ctx.accounts.issuer.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
            &[mint_seeds],
        ),
        rent_lamports,
        mint_space as u64,
        &token_program_id,
    )?;

    // ---- Initialize TransferHook extension (BEFORE initialize_mint2) -------
    // Authority over the hook config = the asset_registry PDA, so only this
    // program (via signer seeds) can rotate the hook program id later.
    let ix = transfer_hook::instruction::initialize(
        &token_program_id,
        &mint_key,
        Some(registry_authority),
        Some(hook_program_id),
    )
    .map_err(|_| AgroError::InvalidAssetMetadata)?;
    invoke(
        &ix,
        &[
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    // ---- Initialize the mint itself --------------------------------------
    initialize_mint2(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint2 {
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        6, // decimals
        &registry_authority,
        Some(&registry_authority),
    )?;

    // ---- Persist the AssetRegistry ---------------------------------------
    let registry = &mut ctx.accounts.asset_registry;
    registry.marketplace = marketplace.key();
    registry.issuer = ctx.accounts.issuer.key();
    registry.mint = mint_key;
    registry.asset_class = asset_class;
    registry.total_supply = total_supply;
    registry.minted_supply = 0;
    registry.oracle_attestation = oracle_attestation;
    registry.white_paper_uri = white_paper_uri;
    registry.metadata_uri = metadata_uri;
    registry.redeemable = !matches!(asset_class, AssetClass::HarvestFraction { .. });
    registry.frozen_metadata = false;
    registry.index = index;
    registry.bump = ctx.bumps.asset_registry;

    marketplace.asset_count = marketplace
        .asset_count
        .checked_add(1)
        .ok_or(AgroError::PriceOverflow)?;

    emit!(AssetRegistered {
        marketplace: marketplace.key(),
        asset_registry: registry.key(),
        issuer: registry.issuer,
        mint: mint_key,
        index,
    });

    Ok(())
}
