use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const MAX_URI_LEN: usize = 200;
pub const MAX_JURISDICTIONS: usize = 32;

// PDA seeds (keep in one place so client + program agree).
pub const MARKETPLACE_SEED: &[u8] = b"marketplace";
pub const COMPLIANCE_AUTHORITY_SEED: &[u8] = b"compliance_authority";
pub const ASSET_REGISTRY_SEED: &[u8] = b"asset_registry";
pub const ASSET_MINT_SEED: &[u8] = b"asset_mint";
pub const COMPLIANCE_RECORD_SEED: &[u8] = b"compliance_record";
pub const LISTING_SEED: &[u8] = b"listing";
pub const LISTING_ESCROW_SEED: &[u8] = b"listing_escrow";

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Three top-level asset classes supported by the marketplace.
/// Each carries class-specific metadata that the off-chain certificate
/// (referenced through `oracle_attestation`) must back.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrainKind {
    Soy,
    Corn,
    Wheat,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CarbonStandard {
    Vcs,
    GoldStandard,
    EuEts,
    Other,
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
    /// Bump for the marketplace PDA.
    pub bump: u8,
    /// Bump for the compliance authority PDA.
    pub compliance_bump: u8,
    /// Fee charged on each buy, in basis points (e.g. 50 = 0.5%).
    pub fee_bps: u16,
    /// Monotonic counter of registered assets (for analytics / off-chain index).
    pub asset_count: u64,
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

/// A fixed-price listing of `remaining` tokens of `asset_registry.mint`.
#[account]
#[derive(InitSpace)]
pub struct MarketplaceListing {
    pub marketplace: Pubkey,
    pub asset_registry: Pubkey,
    pub seller: Pubkey,
    /// Escrow ATA (owned by the listing PDA) that holds the tokens for sale.
    pub escrow: Pubkey,
    /// Price in lamports per single base-unit of the token.
    pub price_lamports: u64,
    /// Remaining quantity available for sale.
    pub remaining: u64,
    /// Listing creation timestamp.
    pub created_at: i64,
    /// Bump for this PDA.
    pub bump: u8,
    /// Whether the listing is still active.
    pub active: bool,
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
    pub asset_registry: Pubkey,
    pub price_lamports: u64,
    pub amount: u64,
}

#[event]
pub struct AssetPurchased {
    pub listing: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub total_lamports: u64,
    pub fee_lamports: u64,
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
