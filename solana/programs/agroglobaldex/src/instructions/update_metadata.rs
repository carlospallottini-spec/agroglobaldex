use anchor_lang::prelude::*;
use anchor_spl::token_2022_extensions::{token_metadata_update_field, TokenMetadataUpdateField};
use anchor_spl::token_interface::{spl_token_metadata_interface::state::Field, Mint, TokenInterface};

use crate::errors::AgroError;
use crate::state::*;

/// Update mutable asset metadata (product name, metadata_uri, white_paper_uri)
/// BEFORE the first mint. After the first `mint_token`, `frozen_metadata` is
/// set to true and this becomes a no-op (reverts with `MetadataFrozen`).
///
/// We propagate `product_name` and `metadata_uri` to the Token-2022
/// `TokenMetadata` extension via CPI so wallets (Phantom/Solflare/Backpack)
/// pick up the change without needing the off-chain JSON to be re-pinned.
#[derive(Accounts)]
pub struct UpdateAssetMetadata<'info> {
    pub issuer: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
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
        constraint = !asset_registry.frozen_metadata @ AgroError::MetadataFrozen,
    )]
    pub asset_registry: Account<'info, AssetRegistry>,

    /// CHECK: address-checked against asset_registry.mint and used as the
    /// TokenMetadata target.
    #[account(
        mut,
        address = asset_registry.mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<UpdateAssetMetadata>,
    product_name: String,
    metadata_uri: String,
    white_paper_uri: String,
) -> Result<()> {
    require!(!product_name.is_empty(), AgroError::InvalidAssetMetadata);
    require!(!white_paper_uri.is_empty(), AgroError::MissingWhitePaper);
    require!(
        product_name.len() <= MAX_PRODUCT_NAME_LEN
            && metadata_uri.len() <= MAX_URI_LEN
            && white_paper_uri.len() <= MAX_URI_LEN,
        AgroError::StringTooLong
    );

    // ---- Persist registry fields -----------------------------------------
    let marketplace_key = ctx.accounts.marketplace.key();
    let index_bytes = ctx.accounts.asset_registry.index.to_le_bytes();
    let registry_bump = ctx.accounts.asset_registry.bump;
    let registry = &mut ctx.accounts.asset_registry;
    registry.product_name = product_name.clone();
    registry.metadata_uri = metadata_uri.clone();
    registry.white_paper_uri = white_paper_uri.clone();

    // ---- Push to Token-2022 TokenMetadata extension -----------------------
    let registry_signer_seeds: &[&[u8]] = &[
        ASSET_REGISTRY_SEED,
        marketplace_key.as_ref(),
        &index_bytes,
        std::slice::from_ref(&registry_bump),
    ];

    token_metadata_update_field(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataUpdateField {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
                update_authority: registry.to_account_info(),
            },
            &[registry_signer_seeds],
        ),
        Field::Name,
        product_name.clone(),
    )?;
    token_metadata_update_field(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataUpdateField {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
                update_authority: registry.to_account_info(),
            },
            &[registry_signer_seeds],
        ),
        Field::Uri,
        if metadata_uri.is_empty() {
            white_paper_uri.clone()
        } else {
            metadata_uri.clone()
        },
    )?;

    emit!(AssetMetadataUpdated {
        asset_registry: registry.key(),
        product_name,
        metadata_uri,
        white_paper_uri,
    });

    Ok(())
}
