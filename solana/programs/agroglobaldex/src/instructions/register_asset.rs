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
use anchor_spl::token_2022_extensions::{
    metadata_pointer_initialize, token_metadata_initialize, MetadataPointerInitialize,
    TokenMetadataInitialize,
};

use crate::errors::AgroError;
use crate::state::*;

/// Registers a new tokenized asset and creates its SPL Token-2022 mint
/// with three extensions wired in order:
///   1. TransferHook → compliance-hook program (enforces KYC on transfer)
///   2. MetadataPointer → self (the mint stores its own TokenMetadata)
///   3. TokenMetadata → name/symbol/uri so wallets (Phantom/Solflare/Backpack)
///      render the token with a human label and image.
#[derive(Accounts)]
#[instruction(asset_class: AssetClass, total_supply: u64)]
pub struct RegisterAsset<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
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

    /// CHECK: validated by seeds + initialized via CPI inside the handler.
    #[account(
        mut,
        seeds = [ASSET_MINT_SEED, asset_registry.key().as_ref()],
        bump,
    )]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: passed in explicitly so deployments can rotate hooks.
    pub compliance_hook_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Derive a short alphanumeric symbol (max 10) from the asset class.
fn derive_symbol(asset_class: &AssetClass) -> String {
    match asset_class {
        AssetClass::Grain { kind, .. } => match kind {
            GrainKind::Soy => "AGRO-SOY".to_string(),
            GrainKind::Corn => "AGRO-CRN".to_string(),
            GrainKind::Wheat => "AGRO-WHT".to_string(),
            GrainKind::Other => "AGRO-GRN".to_string(),
        },
        AssetClass::CarbonCredit { .. } => "AGRO-CO2".to_string(),
        AssetClass::HarvestFraction { .. } => "AGRO-HRV".to_string(),
        AssetClass::InvestmentOffering { .. } => "AGRO-INV".to_string(),
    }
}

pub fn handler(
    ctx: Context<RegisterAsset>,
    asset_class: AssetClass,
    total_supply: u64,
    oracle_attestation: [u8; 32],
    white_paper_uri: String,
    metadata_uri: String,
    product_name: String,
) -> Result<()> {
    require!(total_supply > 0, AgroError::InvalidAmount);
    require!(!white_paper_uri.is_empty(), AgroError::MissingWhitePaper);
    require!(
        white_paper_uri.len() <= MAX_URI_LEN && metadata_uri.len() <= MAX_URI_LEN,
        AgroError::StringTooLong
    );
    require!(
        product_name.len() <= MAX_PRODUCT_NAME_LEN,
        AgroError::StringTooLong
    );
    require!(!product_name.is_empty(), AgroError::InvalidAssetMetadata);

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
        AssetClass::InvestmentOffering {
            duration_months,
            expected_yield_bps,
            maturity_unix_ts,
            ..
        } => {
            require!(
                *duration_months >= 1 && *duration_months <= 120,
                AgroError::InvalidDuration
            );
            require!(*expected_yield_bps <= 5000, AgroError::InvalidYield);
            let now = Clock::get()?.unix_timestamp;
            require!(*maturity_unix_ts > now, AgroError::InvalidMaturity);
        }
    }

    let marketplace = &mut ctx.accounts.marketplace;
    let index = marketplace.asset_count;

    let token_program_id = ctx.accounts.token_program.key();
    let registry_key = ctx.accounts.asset_registry.key();
    let mint_key = ctx.accounts.mint.key();
    let hook_program_id = ctx.accounts.compliance_hook_program.key();
    let registry_authority = registry_key;

    let symbol = derive_symbol(&asset_class);
    let name = product_name.clone();
    let token_uri = if !metadata_uri.is_empty() {
        metadata_uri.clone()
    } else {
        white_paper_uri.clone()
    };

    // ---- Compute mint space (base + TransferHook + MetadataPointer + room for TokenMetadata)
    let extensions = [ExtensionType::TransferHook, ExtensionType::MetadataPointer];
    let base_size = ExtensionType::try_calculate_account_len::<MintState>(&extensions)
        .map_err(|_| AgroError::InvalidAssetMetadata)?;
    // TokenMetadata extension overhead: TLV header (4) + struct overhead.
    // Body: update_authority(33) + mint(32) + name + symbol + uri + Vec<(...)>(4).
    let metadata_extension_size = 4 // TLV header
        + 33                                 // update_authority Option<Pubkey>
        + 32                                 // mint pubkey
        + (4 + name.len())                   // name
        + (4 + symbol.len())                 // symbol
        + (4 + token_uri.len())              // uri
        + 4; // additional_metadata empty Vec
    let mint_space = base_size + metadata_extension_size;
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

    // ---- 1. TransferHook extension (BEFORE initialize_mint2) ---------------
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

    // ---- 2. MetadataPointer extension --------------------------------------
    // Authority = asset_registry PDA. Metadata target = the mint itself (the
    // TokenMetadata extension lives in this same account).
    metadata_pointer_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MetadataPointerInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        Some(registry_authority),
        Some(mint_key),
    )?;

    // ---- 3. Initialize the mint itself ------------------------------------
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

    // ---- 4. TokenMetadata extension (after mint init) ---------------------
    // Mint authority and update authority are both the asset_registry PDA.
    let marketplace_key = marketplace.key();
    let index_bytes = index.to_le_bytes();
    let registry_bump = ctx.bumps.asset_registry;
    let registry_signer_seeds: &[&[u8]] = &[
        ASSET_REGISTRY_SEED,
        marketplace_key.as_ref(),
        &index_bytes,
        std::slice::from_ref(&registry_bump),
    ];
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.asset_registry.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.asset_registry.to_account_info(),
            },
            &[registry_signer_seeds],
        ),
        name.clone(),
        symbol,
        token_uri,
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
    registry.product_name = product_name.clone();
    registry.redeemable = !matches!(
        asset_class,
        AssetClass::HarvestFraction { .. } | AssetClass::InvestmentOffering { .. }
    );
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

    if let AssetClass::InvestmentOffering {
        product_kind,
        duration_months,
        expected_yield_bps,
        maturity_unix_ts,
    } = asset_class
    {
        emit!(InvestmentOfferingRegistered {
            asset_registry: registry.key(),
            mint: mint_key,
            product_kind,
            duration_months,
            expected_yield_bps,
            maturity_unix_ts,
            product_name,
        });
    }

    Ok(())
}
