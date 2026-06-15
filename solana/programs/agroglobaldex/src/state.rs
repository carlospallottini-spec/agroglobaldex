use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const MAX_URI_LEN: usize = 200;
pub const MAX_PLATFORM_LEN: usize = 32;
pub const MAX_CONTRACT_LEN: usize = 96;
pub const MAX_CHAIN_LEN: usize = 24;
pub const MAX_JURISDICTIONS: usize = 32;
pub const MAX_PRODUCT_NAME_LEN: usize = 64;

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
pub const JURISDICTION_POLICY_SEED: &[u8] = b"jurisdiction_policy";
pub const TRADE_RECEIPT_SEED: &[u8] = b"trade_receipt";
pub const LENDING_MARKET_SEED: &[u8] = b"lending_market";
pub const LENDING_VAULT_SEED: &[u8] = b"lending_vault";
pub const COLLATERAL_CONFIG_SEED: &[u8] = b"collateral_config";
pub const LOAN_SEED: &[u8] = b"loan";
pub const LIQUIDITY_PROVIDER_SEED: &[u8] = b"liquidity_provider";

/// Seconds in a (365-day) year, for linear interest accrual.
pub const SECONDS_PER_YEAR: i64 = 365 * 24 * 60 * 60;

/// Permanently-locked shares minted to nobody on the first deposit, so the
/// share price can't be inflated by a first-depositor donation attack
/// (Uniswap-V2-style minimum liquidity).
pub const MINIMUM_LIQUIDITY_SHARES: u64 = 1_000;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Top-level asset classes supported by the marketplace. Each carries
/// class-specific metadata that the off-chain certificate (referenced through
/// `oracle_attestation`) must back.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum AssetClass {
    /// Physical grain commodity (soy, corn, wheat).
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

    /// **InvestmentOffering** — yield-bearing tokenized investment product.
    ///
    /// Examples:
    /// - "Viñedo Rioja 2026 Reserva — 12 months @ 9% expected ROI"
    /// - "Olive grove olive-oil yield share — 18 months @ 7% ROI"
    ///
    /// REGULATORY FOOTNOTE: this is unambiguously a **security** under
    /// MiCA / MiFID II — any wallet receiving these tokens MUST be an
    /// accredited / professional investor (enforced in `enforce_compliance`
    /// and in `register_asset`). The principal + yield redemption is
    /// settled **off-chain** by a legal contract referenced from
    /// `metadata_uri` and the `white_paper_uri`. On-chain `redeemable` is
    /// therefore set to `false` for this class.
    InvestmentOffering {
        /// What is being tokenized (vineyard, olive grove, livestock…).
        product_kind: ProductKind,
        /// Investment lock-up in months. Bounded 1..=120.
        duration_months: u16,
        /// Expected yield in basis points (e.g. 900 = 9% target ROI).
        /// Capped at 5000 bps (50%) — anything higher is almost certainly
        /// a misconfigured input and would invite obvious abuse.
        expected_yield_bps: u16,
        /// Unix timestamp (seconds) at which the off-chain redemption is
        /// expected to settle. MUST be > `Clock::now` at registration.
        maturity_unix_ts: i64,
    },

    /// **Commodity** — physical-backed tokenization of any agricultural unit
    /// (kg of meat, liters of wine, kg of olive oil, liters of milk, kg of
    /// fruit, etc.). One token = a fixed amount of the physical commodity
    /// declared by `grams_per_token`. This is the most general bucket for
    /// "tokenize kg de X" use cases beyond Grain.
    ///
    /// Examples:
    /// - 1 token = 1 kg de carne vacuna AR · vintage 2026
    /// - 1 token = 1 liter de vino Malbec Mendoza · vintage 2024
    /// - 1 token = 1 kg de aceite de oliva extra virgen ES · vintage 2025
    Commodity {
        /// Top-level sector classification (Meat, Wine, Oil, Dairy, ...).
        sector: Sector,
        /// Opaque sub-classification within the sector. Interpreted by the
        /// frontend. E.g. for `Meat`: 0=Beef, 1=Pork, 2=Poultry, 3=Lamb,
        /// 4=Fish, 255=Other. For `Wine`: 0=Red, 1=White, 2=Rose, 3=Sparkling.
        sub_kind: u8,
        /// ISO-3166 alpha-2 country code of origin (e.g. "AR", "ES", "BR").
        origin_country: [u8; 2],
        /// Year of production / vintage.
        vintage_year: u16,
        /// How many grams of the physical commodity each token base unit
        /// represents. e.g. `1000` = each token (1 base unit at 6 decimals,
        /// so 1.000000) is 1 kg. For liquids (wine/oil/milk) use grams equiv
        /// or convert in the certificate.
        grams_per_token: u64,
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

/// Top-level sectors for `AssetClass::Commodity`. Cubre los casos de uso 1
/// del modelo de negocio: tokenizar kg/litros de cualquier producto agro
/// físico.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum Sector {
    /// Granos especiales no cubiertos por AssetClass::Grain (rice, sorghum,
    /// sunflower, barley, etc.). Permite expandir sin tocar GrainKind.
    GrainSpecial,
    /// Carnes: vacuno, porcino, aviar, ovino, pescado.
    Meat,
    /// Vinos y mostos de uva.
    Wine,
    /// Aceites: oliva, girasol, soja, palma.
    Oil,
    /// Lácteos: leche, queso, manteca.
    Dairy,
    /// Frutas frescas o procesadas.
    Fruit,
    /// Vegetales / hortalizas.
    Vegetable,
    /// Fibras: lana, algodón, lino, cáñamo.
    Fiber,
    /// Otros (catch-all extensible).
    Other,
}

/// Kinds of `InvestmentOffering` products the marketplace tokenizes.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum ProductKind {
    Vineyard,
    Olive,
    Dairy,
    Livestock,
    Crops,
    Other,
}

/// A listing in the marketplace can either reference a native `AssetRegistry`
/// (tokens minted by this program) or an aggregated `ExternalAssetRegistry`
/// (tokens minted by another platform but listed here for trading).
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
    /// Wallet that controls marketplace parameters: fee_bps, treasury
    /// withdrawals, aggregator entries. Can be a human wallet OR a Squads
    /// (or any other) multisig PDA — the program is agnostic.
    pub authority: Pubkey,
    /// Wallet permitted to stamp `ComplianceRecord`s. Distinct from
    /// `authority` so a compliance bot / service account can operate
    /// without ever touching treasury funds. Rotated via
    /// `set_compliance_signer` (authority-only).
    pub compliance_signer: Pubkey,
    /// PDA derived from [COMPLIANCE_AUTHORITY_SEED, marketplace] — kept for
    /// backwards compatibility / future use as a CPI signer over hook
    /// programs. Currently informational.
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
    /// Circuit breaker. When `true`, all write paths abort with `Paused`.
    pub paused: bool,
    /// Monotonic counter of settled trades. Used to derive the
    /// `TradeReceipt` PDA for every buy. Provides a global, queryable,
    /// immutable proof-of-trade ledger — the structured-data answer to
    /// AgriDex's per-trade receipt NFTs.
    pub trade_count: u64,
}

/// One per tokenized real-world asset lot. The mint of the SPL Token-2022 is
/// derived from this registry as a PDA so the program is its mint authority.
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
    /// Cumulative amount burned through `redeem` so far. Lets off-chain
    /// reconcile physical deliveries and lets the issuer detect when supply
    /// is fully drained (`minted_supply == redeemed_supply`).
    pub redeemed_supply: u64,
    /// SHA-256 of the off-chain attestation (warehouse receipt, VCS issuance, etc).
    pub oracle_attestation: [u8; 32],
    /// MiCA Art. 6 white paper URI (IPFS / Arweave preferred).
    #[max_len(MAX_URI_LEN)]
    pub white_paper_uri: String,
    /// Off-chain JSON metadata URI (image, full description).
    #[max_len(MAX_URI_LEN)]
    pub metadata_uri: String,
    /// Human-friendly product label, e.g. "Viñedo Rioja 2026 Reserva".
    #[max_len(MAX_PRODUCT_NAME_LEN)]
    pub product_name: String,
    /// Whether burning tokens redeems the underlying off-chain commodity.
    /// Carbon credits, for example, are typically redeemable (retirement).
    /// For `InvestmentOffering` this is `false` (settlement is off-chain).
    pub redeemable: bool,
    /// True once the issuer has minted at least once and no more changes to
    /// immutable fields are allowed.
    pub frozen_metadata: bool,
    /// For `InvestmentOffering`: highest epoch settled so far (+1 means N
    /// distributions recorded). 0 with `total_yield_paid_usdc == 0` means no
    /// distribution yet. Lets the UI show settlement history without an
    /// off-chain indexer.
    pub last_settled_epoch: u32,
    /// For `InvestmentOffering`: cumulative USDC yield recorded across all
    /// epochs (base units, 6 decimals).
    pub total_yield_paid_usdc: u64,
    /// Unix timestamp of the most recent settlement. 0 if none.
    pub last_settled_at: i64,
    /// Sequential index assigned by the marketplace at registration time.
    /// Used as a seed component so PDA seeds are deterministically
    /// reconstructible without trusting the client.
    pub index: u64,
    /// Bump for this PDA.
    pub bump: u8,
}

/// One per wallet. Stamped by the marketplace `compliance_signer`.
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

/// On-chain mutable jurisdiction policy. One per marketplace. Authority-only.
///
/// The compliance-hook program and `buy_asset` both read this account to
/// determine which jurisdictions are blocked outright and which additionally
/// require accredited-investor status (irrespective of asset class).
#[account]
#[derive(InitSpace)]
pub struct JurisdictionPolicy {
    pub marketplace: Pubkey,
    /// Hard-blocked ISO-3166-alpha-2 codes (e.g. "KP", "IR", "SY", "CU").
    /// Wallets in these jurisdictions cannot transact at all.
    #[max_len(MAX_JURISDICTIONS)]
    pub blocked: Vec<[u8; 2]>,
    /// Jurisdictions that require `accredited_investor == true` regardless
    /// of asset class.
    #[max_len(MAX_JURISDICTIONS)]
    pub requires_accredited: Vec<[u8; 2]>,
    pub bump: u8,
}

/// A fixed-price USDC listing of `remaining` tokens of the underlying mint.
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
/// authority.
#[account]
#[derive(InitSpace)]
pub struct ExternalAssetRegistry {
    /// Marketplace that aggregates this asset.
    pub marketplace: Pubkey,
    /// Wallet that submitted the curation (normally `marketplace.authority`).
    pub curator: Pubkey,
    /// Solana SPL mint, if the token is native to Solana.
    pub mint: Option<Pubkey>,
    /// Free-form chain identifier for cross-chain assets.
    #[max_len(MAX_CHAIN_LEN)]
    pub external_chain_id: String,
    /// Contract address on the foreign chain. Empty if `mint` is set.
    #[max_len(MAX_CONTRACT_LEN)]
    pub external_contract: String,
    /// Reuse the same asset taxonomy as native assets.
    pub asset_class: AssetClass,
    /// Short label for the upstream platform.
    #[max_len(MAX_PLATFORM_LEN)]
    pub source_platform: String,
    /// URL pointing at the upstream platform listing / explorer.
    #[max_len(MAX_URI_LEN)]
    pub source_url: String,
    /// Off-chain JSON metadata URI.
    #[max_len(MAX_URI_LEN)]
    pub metadata_uri: String,
    /// Curator-verified flag (independent of on-chain validation).
    pub verified: bool,
    /// Whether this aggregated asset is currently visible/tradable.
    pub active: bool,
    /// Creation timestamp.
    pub created_at: i64,
    /// Sequential index assigned at aggregation time.
    pub index: u64,
    /// Bump for this PDA.
    pub bump: u8,
}

/// Immutable proof-of-trade. One per successful `buy_asset` /
/// `buy_external_asset` (including partial fills). Derived from the global
/// `marketplace.trade_count` so it is unique, sequential and queryable. This
/// is AgriDex's "trade receipt NFT" reimagined as structured on-chain data:
/// auditors, regulators and the UI can index the full trade ledger without
/// parsing NFT metadata.
#[account]
#[derive(InitSpace)]
pub struct TradeReceipt {
    /// Marketplace this trade happened under.
    pub marketplace: Pubkey,
    /// The listing that was bought from.
    pub listing: Pubkey,
    /// The asset mint that changed hands.
    pub asset_mint: Pubkey,
    /// Buyer wallet.
    pub buyer: Pubkey,
    /// Seller wallet.
    pub seller: Pubkey,
    /// Native (program-minted) or External (aggregated SPL).
    pub source: ListingSource,
    /// Tokens transferred (base units).
    pub amount: u64,
    /// Per-token USDC price at trade time (base units, 6 decimals).
    pub unit_price_usdc: u64,
    /// Gross USDC paid (amount * unit_price).
    pub gross_usdc: u64,
    /// Protocol fee charged (USDC base units).
    pub fee_usdc: u64,
    /// Buyer jurisdiction at trade time (ISO-3166 alpha-2). Snapshotted so a
    /// later KYC update doesn't rewrite history.
    pub buyer_jurisdiction: [u8; 2],
    /// Global sequential index (== marketplace.trade_count at mint time).
    pub trade_index: u64,
    /// Settlement timestamp.
    pub settled_at: i64,
    /// Bump for this PDA.
    pub bump: u8,
}

/// On-chain lending market: lets producers borrow USDC against their
/// tokenized agro-RWA collateral. One per marketplace. The killer ag-finance
/// primitive — instant credit against a warehouse receipt / tokenized harvest
/// without a bank. USDC liquidity sits in a pool ATA owned by the
/// `lending_vault` authority PDA.
#[account]
#[derive(InitSpace)]
pub struct LendingMarket {
    /// Marketplace this lending market belongs to.
    pub marketplace: Pubkey,
    /// USDC mint used for both liquidity and loans (== marketplace.usdc_mint).
    pub usdc_mint: Pubkey,
    /// USDC pool ATA owned by the vault authority PDA.
    pub usdc_pool: Pubkey,
    /// Fixed annualized interest rate in basis points (e.g. 1200 = 12% APR).
    pub apr_bps: u16,
    /// Maximum loan-to-value in basis points (e.g. 5000 = borrow up to 50%
    /// of collateral value). Conservative defaults protect the pool.
    pub max_ltv_bps: u16,
    /// Health-factor threshold for liquidation in basis points. When the
    /// current LTV exceeds this (e.g. 8000 = 80%), the loan can be liquidated.
    pub liquidation_threshold_bps: u16,
    /// Bonus paid to the liquidator in basis points of seized collateral
    /// (e.g. 500 = 5% discount). Incentivizes timely liquidation.
    pub liquidation_bonus_bps: u16,
    /// Total USDC currently available in the pool.
    pub total_liquidity: u64,
    /// Total USDC currently lent out (principal only).
    pub total_borrowed: u64,
    /// Monotonic counter of loans ever opened.
    pub loan_count: u64,
    /// Total LP shares outstanding. Pool value (= total_liquidity +
    /// total_borrowed) is split pro-rata across shares, so accrued interest
    /// (which grows total_liquidity on repay/liquidate) appreciates every LP's
    /// shares instead of being stranded.
    pub total_shares: u64,
    /// Bump for this PDA.
    pub bump: u8,
    /// Bump for the vault authority PDA (owns usdc_pool + collateral ATAs).
    pub vault_authority_bump: u8,
    /// When true, `open_loan` and `liquidate` REQUIRE the collateral to be
    /// oracle-driven (`CollateralConfig::oracle_enabled == true`), forbidding
    /// the manual authority-relayed price path. Defaults to `false` at init so
    /// manual-priced devnet flows keep working; a mainnet deployment flips this
    /// on via `set_lending_oracle_requirement` to close the oracle-manipulation
    /// vector (audit H-1).
    pub require_oracle_for_loans: bool,
}

/// Per-asset collateral configuration. Sets the price (USDC per token) used
/// to value collateral, plus an enable flag. Price is set by the marketplace
/// authority acting as an oracle relay; production should wire a signed
/// price feed (see audit #12).
#[account]
#[derive(InitSpace)]
pub struct CollateralConfig {
    pub lending_market: Pubkey,
    pub asset_registry: Pubkey,
    pub mint: Pubkey,
    /// USDC base units (6 decimals) per ONE collateral token base unit.
    pub price_usdc_per_token: u64,
    /// Whether this asset may be used as collateral.
    pub enabled: bool,
    /// Last time the price was refreshed.
    pub updated_at: i64,
    pub bump: u8,
    // ── Oracle (Pyth) wiring ────────────────────────────────────────────
    /// When true, `price_usdc_per_token` is driven by the Pyth feed via
    /// `refresh_collateral_price`, and `open_loan` enforces price staleness.
    /// When false (default) the authority sets the price manually (relay mode).
    pub oracle_enabled: bool,
    /// Pyth price-feed id (32 bytes) this collateral is priced against.
    /// All-zero when `oracle_enabled == false`.
    pub oracle_feed_id: [u8; 32],
    /// Max age (seconds) of a Pyth price before it is considered stale.
    pub max_staleness_secs: i64,
    /// Max Pyth confidence-to-price ratio tolerated, in bps (e.g. 200 = 2%).
    /// Rejects wide/uncertain prints that could be manipulated.
    pub max_confidence_bps: u16,
}

/// A single borrower's loan position against one collateral asset.
#[account]
#[derive(InitSpace)]
pub struct LoanPosition {
    pub lending_market: Pubkey,
    pub borrower: Pubkey,
    pub asset_registry: Pubkey,
    pub collateral_mint: Pubkey,
    /// Collateral ATA owned by the vault authority PDA holding the tokens.
    pub collateral_vault: Pubkey,
    /// Collateral tokens locked (base units).
    pub collateral_amount: u64,
    /// Outstanding principal in USDC base units.
    pub principal_usdc: u64,
    /// Interest accrued and not yet paid (USDC base units).
    pub accrued_interest_usdc: u64,
    /// APR snapshot at open time (loan keeps its rate).
    pub apr_bps: u16,
    /// When the loan was opened.
    pub opened_at: i64,
    /// Last time interest was accrued into `accrued_interest_usdc`.
    pub last_accrued_at: i64,
    /// Global sequential index.
    pub loan_index: u64,
    /// Active until fully repaid or liquidated.
    pub active: bool,
    pub bump: u8,
}

/// Per-provider record of LP shares in a lending market. Shares (not a flat
/// USDC amount) are tracked so that pool growth from interest is shared
/// pro-rata and one LP can never withdraw another LP's principal.
#[account]
#[derive(InitSpace)]
pub struct LiquidityProvider {
    /// Lending market this record belongs to.
    pub lending_market: Pubkey,
    /// The liquidity provider wallet.
    pub provider: Pubkey,
    /// Pool shares owned by this provider.
    pub shares: u64,
    pub bump: u8,
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Emitted alongside `AssetPurchased` whenever a `TradeReceipt` PDA is minted.
/// Indexers build the global trade ledger from this stream.
#[event]
pub struct TradeReceiptCreated {
    pub trade_receipt: Pubkey,
    pub marketplace: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub asset_mint: Pubkey,
    pub source: ListingSource,
    pub amount: u64,
    pub gross_usdc: u64,
    pub trade_index: u64,
    pub settled_at: i64,
}

#[event]
pub struct AssetRegistered {
    pub marketplace: Pubkey,
    pub asset_registry: Pubkey,
    pub issuer: Pubkey,
    pub mint: Pubkey,
    pub index: u64,
}

#[event]
pub struct InvestmentOfferingRegistered {
    pub asset_registry: Pubkey,
    pub mint: Pubkey,
    pub product_kind: ProductKind,
    pub duration_months: u16,
    pub expected_yield_bps: u16,
    pub maturity_unix_ts: i64,
    pub product_name: String,
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
pub struct ComplianceSignerRotated {
    pub marketplace: Pubkey,
    pub old_signer: Pubkey,
    pub new_signer: Pubkey,
}

#[event]
pub struct JurisdictionPolicyUpdated {
    pub marketplace: Pubkey,
    pub blocked: Vec<[u8; 2]>,
    pub requires_accredited: Vec<[u8; 2]>,
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

#[event]
pub struct ListingPriceUpdated {
    pub listing: Pubkey,
    pub new_price_usdc: u64,
}

#[event]
pub struct TreasuryWithdrawn {
    pub marketplace: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}

#[event]
pub struct PauseChanged {
    pub marketplace: Pubkey,
    pub was: bool,
    pub now: bool,
}

/// Emitted when the authority flips `require_oracle_for_loans` on a
/// `LendingMarket` (audit H-1 mainnet hardening switch).
#[event]
pub struct LendingOracleRequirementChanged {
    pub lending_market: Pubkey,
    pub was: bool,
    pub now: bool,
}

/// Emitted when the compliance signer revokes a wallet's KYC, typically due
/// to a sanctions hit, fraud detection or regulatory request. Off-chain
/// surveillance MUST act on this event (freeze listings, surface to MLRO).
#[event]
pub struct ComplianceRevoked {
    pub marketplace: Pubkey,
    pub wallet: Pubkey,
    /// 0=manual, 1=sanctions, 2=fraud, 3=regulatory, 4=request
    pub reason_code: u8,
    pub revoked_at: i64,
}

/// Emitted when the issuer of an `InvestmentOffering` records an off-chain
/// yield distribution for an epoch. Used by the marketplace UI and auditors
/// to reconcile on-chain promise vs off-chain payout. Settlement of the
/// underlying yield (bank wire) is OFF-CHAIN; this event is the on-chain
/// receipt.
#[event]
pub struct InvestmentSettlementRecorded {
    pub asset_registry: Pubkey,
    pub mint: Pubkey,
    pub issuer: Pubkey,
    pub epoch: u32,
    pub yield_paid_usdc: u64,
    /// SHA-256 of the off-chain payout attestation (SWIFT confirmation,
    /// notary acta, etc.). Mirrors the discipline of `oracle_attestation`.
    pub attestation: [u8; 32],
    pub recorded_at: i64,
}

/// Emitted when the issuer updates metadata before the first mint. After the
/// first `mint_token`, `frozen_metadata` flips to true and this becomes a
/// no-op (will revert).
#[event]
pub struct AssetMetadataUpdated {
    pub asset_registry: Pubkey,
    pub product_name: String,
    pub metadata_uri: String,
    pub white_paper_uri: String,
}

/// Emitted when the issuer role of an AssetRegistry is transferred to a new
/// wallet. The old issuer signs the transfer; the new issuer must already be
/// KYC-verified.
#[event]
pub struct IssuerTransferred {
    pub asset_registry: Pubkey,
    pub old_issuer: Pubkey,
    pub new_issuer: Pubkey,
}

// ---------------------------------------------------------------------------
// Lending events
// ---------------------------------------------------------------------------

#[event]
pub struct LendingMarketInitialized {
    pub lending_market: Pubkey,
    pub marketplace: Pubkey,
    pub apr_bps: u16,
    pub max_ltv_bps: u16,
}

#[event]
pub struct LiquidityDeposited {
    pub lending_market: Pubkey,
    pub provider: Pubkey,
    pub amount: u64,
    pub shares_minted: u64,
    pub total_liquidity: u64,
}

#[event]
pub struct LiquidityWithdrawn {
    pub lending_market: Pubkey,
    pub provider: Pubkey,
    pub amount: u64,
    pub shares_burned: u64,
    pub total_liquidity: u64,
}

#[event]
pub struct CollateralConfigured {
    pub lending_market: Pubkey,
    pub asset_registry: Pubkey,
    pub price_usdc_per_token: u64,
    pub enabled: bool,
}

/// Emitted when a Pyth feed is bound to a collateral config.
#[event]
pub struct CollateralOracleSet {
    pub lending_market: Pubkey,
    pub asset_registry: Pubkey,
    pub oracle_feed_id: [u8; 32],
    pub max_staleness_secs: i64,
    pub max_confidence_bps: u16,
    pub enabled: bool,
}

/// Emitted each time the cached collateral price is cranked from Pyth.
#[event]
pub struct CollateralPriceRefreshed {
    pub lending_market: Pubkey,
    pub asset_registry: Pubkey,
    pub price_usdc_per_token: u64,
    pub pyth_price: i64,
    pub pyth_expo: i32,
    pub pyth_conf: u64,
    pub publish_time: i64,
}

#[event]
pub struct LoanOpened {
    pub loan: Pubkey,
    pub lending_market: Pubkey,
    pub borrower: Pubkey,
    pub asset_registry: Pubkey,
    pub collateral_amount: u64,
    pub principal_usdc: u64,
    pub loan_index: u64,
}

#[event]
pub struct LoanRepaid {
    pub loan: Pubkey,
    pub borrower: Pubkey,
    pub principal_usdc: u64,
    pub interest_usdc: u64,
    pub collateral_returned: u64,
}

#[event]
pub struct LoanLiquidated {
    pub loan: Pubkey,
    pub borrower: Pubkey,
    pub liquidator: Pubkey,
    pub debt_repaid_usdc: u64,
    pub collateral_seized: u64,
    /// Collateral returned to the borrower (kept their equity above debt+bonus).
    pub collateral_returned: u64,
}
