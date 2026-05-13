//! Aggregator module. Lets the marketplace authority curate third-party
//! tokenized RWA-agro assets (Agrotoken, Topaz, RIPE, Centrifuge…) so they
//! show up alongside natively-minted assets.
//!
//! - If the token is a Solana SPL mint, store its mint pubkey. (Trading
//!   on-chain via `buy_asset` is intentionally NOT enabled in this PoC for
//!   external SPLs — they are display-with-link only. The reason: external
//!   mints have their own transfer-hook / compliance regime that we cannot
//!   bypass without an adapter program. See README for the roadmap.)
//! - If the token lives on another chain, store an opaque
//!   `external_chain_id` + `external_contract` string and a `source_url` the
//!   frontend can deep-link to.

use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

// ---------------------------------------------------------------------------
// aggregate_external_asset
// ---------------------------------------------------------------------------

#[derive(Accounts)]
#[instruction(payload: AggregatePayload)]
pub struct AggregateExternalAsset<'info> {
    #[account(mut)]
    pub curator: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == curator.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = curator,
        space = 8 + ExternalAssetRegistry::INIT_SPACE,
        seeds = [
            EXTERNAL_ASSET_SEED,
            marketplace.key().as_ref(),
            &marketplace.external_asset_count.to_le_bytes(),
        ],
        bump
    )]
    pub external_asset: Account<'info, ExternalAssetRegistry>,

    pub system_program: Program<'info, System>,
}

/// Off-chain payload for aggregator entries. Bundled into a single struct so
/// it can be serialized cleanly across the IX boundary and the IDL stays tidy.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AggregatePayload {
    pub mint: Option<Pubkey>,
    pub external_chain_id: String,
    pub external_contract: String,
    pub asset_class: AssetClass,
    pub source_platform: String,
    pub source_url: String,
    pub metadata_uri: String,
}

pub fn aggregate_handler(
    ctx: Context<AggregateExternalAsset>,
    payload: AggregatePayload,
) -> Result<()> {
    // ---- Validate payload --------------------------------------------------
    require!(
        payload.source_platform.len() <= MAX_PLATFORM_LEN
            && payload.external_chain_id.len() <= MAX_CHAIN_LEN
            && payload.external_contract.len() <= MAX_CONTRACT_LEN
            && payload.source_url.len() <= MAX_URI_LEN
            && payload.metadata_uri.len() <= MAX_URI_LEN,
        AgroError::StringTooLong
    );
    require!(
        !payload.source_platform.is_empty(),
        AgroError::InvalidAggregatorPayload
    );

    // Either an SPL mint OR a cross-chain reference — never both, never none.
    match &payload.mint {
        Some(_) => {
            require!(
                payload.external_chain_id.is_empty() && payload.external_contract.is_empty(),
                AgroError::InvalidAggregatorPayload
            );
        }
        None => {
            require!(
                !payload.external_chain_id.is_empty() && !payload.external_contract.is_empty(),
                AgroError::InvalidAggregatorPayload
            );
        }
    }

    let marketplace = &mut ctx.accounts.marketplace;
    let index = marketplace.external_asset_count;

    let external = &mut ctx.accounts.external_asset;
    external.marketplace = marketplace.key();
    external.curator = ctx.accounts.curator.key();
    external.mint = payload.mint;
    external.external_chain_id = payload.external_chain_id;
    external.external_contract = payload.external_contract;
    external.asset_class = payload.asset_class;
    external.source_platform = payload.source_platform.clone();
    external.source_url = payload.source_url;
    external.metadata_uri = payload.metadata_uri;
    external.verified = false; // curator must explicitly verify
    external.active = true;
    external.created_at = Clock::get()?.unix_timestamp;
    external.index = index;
    external.bump = ctx.bumps.external_asset;

    marketplace.external_asset_count = marketplace
        .external_asset_count
        .checked_add(1)
        .ok_or(AgroError::PriceOverflow)?;

    emit!(ExternalAssetAggregated {
        marketplace: marketplace.key(),
        external_asset: external.key(),
        curator: external.curator,
        mint: external.mint,
        source_platform: payload.source_platform,
        index,
    });

    Ok(())
}

// ---------------------------------------------------------------------------
// update_external_asset (toggle verified / active)
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct UpdateExternalAsset<'info> {
    pub curator: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == curator.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [
            EXTERNAL_ASSET_SEED,
            marketplace.key().as_ref(),
            &external_asset.index.to_le_bytes(),
        ],
        bump = external_asset.bump,
        constraint = external_asset.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
    )]
    pub external_asset: Account<'info, ExternalAssetRegistry>,
}

pub fn update_handler(
    ctx: Context<UpdateExternalAsset>,
    verified: bool,
    active: bool,
) -> Result<()> {
    let ext = &mut ctx.accounts.external_asset;
    ext.verified = verified;
    ext.active = active;

    emit!(ExternalAssetUpdated {
        external_asset: ext.key(),
        verified,
        active,
    });
    Ok(())
}
