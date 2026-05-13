use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const MAX_URI_LEN: usize = 200;
pub const MAX_PLATFORM_LEN: usize = 32;
pub const MAX_CONTRACT_LEN: usize = 96;
pub const MAX_CHAIN_LEN: usize = 24;
pub const MAX_JURISDICTIONS: usize = 32;

// PDA seeds (keep in one place so client + program agree).
pub const MARKETPLACE_SEED: &[u8] = b"marketplace";
pub const COMPLIANCE_AUTHORITY_SEED: &[u8] = b"compliance_authority";
pub const ASSET_REGISTRY_SEED: &[u8] = b"asset_registry";
pub const ASSET_MINT_SEED: &[u8] = b"asset_mint";
pub const COMPLIANCE_RECORD_SEED: &[u8] = b"compliance_record";
pub const LISTING_SEED: &[u8] = b"listing";
pub const LISTING_ESCROW_SEED: &[u8] = b"listing_escrow";
pub const LISTING_USDC_ESCROW_SEED: &[u8] = b"listing_usdc_escrow";
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const EXTERNAL_ASSET_SEED: &[u8] = b"external_asset";

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Three top-level asset classes supported by the marketplace.
/// Each carries class-specific metadata that the off-chain certificate
/// (referenced through `oracle_attestation`) must back.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum AssetClass {
    /// Physical grain commodity (soy, corn, wheat).
    /// `metadata.amount_units` = tons.
    Grain { kind: GrainKind, tons: u64 },

    /// Voluntary or compliance carbon credit, agro-sourced.
    /// `metadata.amount_units` = kg CO2eq.
    CarbonCredit {
        standard: CarbonStandard,
        vintage_year: u16,
        kg_co2eq: u64,
    },

    /// Future harvest fraction (yield-share NFT-like).
    HarvestFraction {
        crop: GrainKind,
        hectares: u32,
        harvest_year: u16,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum GrainKind {
    Soy,
    Corn,
    Wheat,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum CarbonStandard {
    Vcs,
    GoldStandard,
    EuEts,
    Other,
}

/// A listing in the marketplace can either reference a native `AssetRegistry`
/// (tokens minted by this program) or an aggregated `ExternalAssetRegistry`
/// (tokens minted by another platform but listed here for trading).
///
/// In both cases `source` points at the registry account so the buy_asset
/// instruction can dispatch correctly. Cross-chain externals are display-only
/// and cannot be listed (the `aggregate_external_asset` instruction enforces
/// that listing requires a Solana SPL mint).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum ListingSource {
    Native,
    External,
}

// ---------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------

/// Global marketplace configuration. One per deployment.
#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    /// Wallet that can update marketplace parameters.
    pub authority: Pubkey,
    /// PDA derived from [COMPLIANCE_AUTHORITY_SEED, marketplace] — the only
    /// signer authorized to write `ComplianceRecord`s.
    pub compliance_authority: Pubkey,
    /// USDC SPL mint used to settle trades. Stored on the marketplace so we
    /// can swap mainnet/devnet without changing program code.
    pub usdc_mint: Pubkey,
    /// PDA that owns the USDC treasury ATA where fees accumulate.
    pub treasury: Pubkey,
    /// Bump for the marketplace PDA.
    pub bump: u8,
    /// Bump for the compliance authority PDA.
    pub compliance_bump: u8,
    /// Bump for the treasury PDA.
    pub treasury_bump: u8,
    /// Fee charged on each buy, in basis points (e.g. 50 = 0.5%).
    pub fee_bps: u16,
    /// Monotonic counter of native registered assets.
    pub asset_count: u64,
    /// Monotonic counter of external (aggregated) registered assets.
    pub external_asset_count: u64,
}

/// One per tokenized real-world asset lot. The mint of the SPL Token-2022
/// is derived from this registry as a PDA so the program is its mint authority.
#[account]
#[derive(InitSpace)]
pub struct AssetRegistry {
    /// The marketplace this asset belongs to.
    pub marketplace: Pubkey,
    /// Issuer (producer / cooperative / project) wallet. Only they can mint.
    pub issuer: Pubkey,
    /// The SPL Token-2022 mint that represents this asset.
    pub mint: Pubkey,
    /// Class + class-specific metadata.
    pub asset_class: AssetClass,
    /// Maximum tokens that may ever be minted against this lot.
    pub total_supply: u64,
    /// Amount minted so far.
    pub minted_supply: u64,
    /// SHA-256 of the off-chain attestation (warehouse receipt, VCS issuance, etc).
    pub oracle_attestation: [u8; 32],
    /// MiCA Art. 6 white paper URI (IPFS / Arweave preferred).
    #[max_len(MAX_URI_LEN)]
    pub white_paper_uri: String,
    /// Off-chain JSON metadata URI (image, full description).
    #[max_len(MAX_URI_LEN)]
    pub metadata_uri: String,
    /// Whether burning tokens redeems the underlying off-chain commodity.
    /// Carbon credits, for example, are typically redeemable (retirement).
    pub redeemable: bool,
    /// True once the issuer has minted at least once and no more changes
    /// to immutable fields are allowed.
    pub frozen_metadata: bool,
    /// Sequential index assigned by the marketplace at registration time.
    /// Used as a seed component so PDA seeds are deterministically
    /// reconstructible without trusting the client.
    pub index: u64,
    /// Bump for this PDA.
    pub bump: u8,
}

/// One per wallet. Stamped by the marketplace `compliance_authority`.
#[account]
#[derive(InitSpace)]
pub struct ComplianceRecord {
    /// The wallet this record refers to.
    pub wallet: Pubkey,
    /// Marketplace the record is scoped to (multi-tenant friendly).
    pub marketplace: Pubkey,
    /// True if the wallet has passed KYC.
    pub kyc_verified: bool,
    /// ISO-3166-alpha-2 country code (e.g. "AR", "DE").
    pub jurisdiction: [u8; 2],
    /// Whether the wallet qualifies as an accredited / professional investor
    /// under MiFID II / MiCA categorization.
    pub accredited_investor: bool,
    /// Unix timestamp when the record was last updated.
    pub updated_at: i64,
    /// Bump for this PDA.
    pub bump: u8,
}

/// A fixed-price USDC listing of `remaining` tokens of the underlying mint.
///
/// `source` discriminates whether `source_registry` points at an
/// `AssetRegistry` (native, minted by us) or an `ExternalAssetRegistry`
/// (aggregated SPL token from another platform, e.g. Agrotoken).
#[account]
#[derive(InitSpace)]
pub struct MarketplaceListing {
    pub marketplace: Pubkey,
    /// Discriminator for `source_registry`.
    pub source: ListingSource,
    /// Either an `AssetRegistry` PDA or an `ExternalAssetRegistry` PDA.
    pub source_registry: Pubkey,
    /// The actual SPL mint being traded (same as either
    /// `AssetRegistry.mint` or `ExternalAssetRegistry.mint`).
    pub mint: Pubkey,
    pub seller: Pubkey,
    /// Escrow ATA (owned by the listing PDA) that holds the tokens for sale.
    pub escrow: Pubkey,
    /// Price in USDC base units (6 decimals) per single base-unit of the token.
    pub price_usdc: u64,
    /// Remaining quantity available for sale.
    pub remaining: u64,
    /// Listing creation timestamp.
    pub created_at: i64,
    /// Bump for this PDA.
    pub bump: u8,
    /// Whether the listing is still active.
    pub active: bool,
}

/// One per third-party (aggregated) token. Curated by the marketplace
/// authority. `mint` is `Some` if the token is a Solana SPL token tradable
/// on-chain; if it is on another chain we only store metadata and link out.
#[account]
#[derive(InitSpace)]
pub struct ExternalAssetRegistry {
    /// Marketplace that aggregates this asset.
    pub marketplace: Pubkey,
    /// Wallet that submitted the curation (normally `marketplace.authority`).
    pub curator: Pubkey,
    /// Solana SPL mint, if the token is native to Solana. When `None` the
    /// asset is cross-chain and only display metadata is stored.
    pub mint: Option<Pubkey>,
    /// Free-form chain identifier for cross-chain assets ("ethereum",
    /// "polygon", "bsc", "rsk", ...). Empty if `mint` is set.
    #[max_len(MAX_CHAIN_LEN)]
    pub external_chain_id: String,
    /// Contract address on the foreign chain. Empty if `mint` is set.
    #[max_len(MAX_CONTRACT_LEN)]
    pub external_contract: String,
    /// Reuse the same asset taxonomy as native assets.
    pub asset_class: AssetClass,
    /// Short label for the upstream platform ("Agrotoken", "Topaz", "RIPE",
    /// "AgroToken", "Centrifuge").
    #[max_len(MAX_PLATFORM_LEN)]
    pub source_platform: String,
    /// URL pointing at the upstream platform listing / explorer.
    #[max_len(MAX_URI_LEN)]
    pub source_url: String,
    /// Off-chain JSON metadata URI (image, full description).
    #[max_len(MAX_URI_LEN)]
    pub metadata_uri: String,
    /// Curator-verified flag (independent of on-chain validation).
    pub verified: bool,
    /// Whether this aggregated asset is currently visible/tradable.
    pub active: bool,
    /// Creation timestamp.
    pub created_at: i64,
    /// Sequential index assigned at aggregation time (for PDA seeds).
    pub index: u64,
    /// Bump for this PDA.
    pub bump: u8,
}

// ---------------------------------------------------------------------------
// Events (consumed by the off-chain indexer / web frontend)
// ---------------------------------------------------------------------------

#[event]
pub struct AssetRegistered {
    pub marketplace: Pubkey,
    pub asset_registry: Pubkey,
    pub issuer: Pubkey,
    pub mint: Pubkey,
    pub index: u64,
}

#[event]
pub struct TokensMinted {
    pub asset_registry: Pubkey,
    pub amount: u64,
    pub new_minted_supply: u64,
}

#[event]
pub struct ComplianceUpdated {
    pub wallet: Pubkey,
    pub kyc_verified: bool,
    pub jurisdiction: [u8; 2],
    pub accredited_investor: bool,
}

#[event]
pub struct AssetListed {
    pub listing: Pubkey,
    pub seller: Pubkey,
    pub source: ListingSource,
    pub source_registry: Pubkey,
    pub mint: Pubkey,
    pub price_usdc: u64,
    pub amount: u64,
}

#[event]
pub struct AssetPurchased {
    pub listing: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub total_usdc: u64,
    pub fee_usdc: u64,
}

#[event]
pub struct AssetRedeemed {
    pub asset_registry: Pubkey,
    pub holder: Pubkey,
    pub amount: u64,
    /// The off-chain settlement workflow MUST listen to this event and release
    /// the physical commodity (or retire the credit) to `holder`.
    pub redeemed_at: i64,
}

#[event]
pub struct ExternalAssetAggregated {
    pub marketplace: Pubkey,
    pub external_asset: Pubkey,
    pub curator: Pubkey,
    pub mint: Option<Pubkey>,
    pub source_platform: String,
    pub index: u64,
}

#[event]
pub struct ExternalAssetUpdated {
    pub external_asset: Pubkey,
    pub verified: bool,
    pub active: bool,
}
