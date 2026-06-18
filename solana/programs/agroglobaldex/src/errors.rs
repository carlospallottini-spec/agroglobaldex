use anchor_lang::prelude::*;

#[error_code]
pub enum AgroError {
    #[msg("Caller is not the marketplace authority")]
    UnauthorizedMarketplaceAuthority,

    #[msg("Caller is not the compliance signer")]
    UnauthorizedComplianceAuthority,

    #[msg("Caller is not the asset issuer")]
    UnauthorizedIssuer,

    #[msg("Caller is not the curator of this external asset")]
    UnauthorizedCurator,

    #[msg("Wallet has not passed KYC verification")]
    KycNotVerified,

    #[msg("Jurisdiction is not allowed by the marketplace policy")]
    JurisdictionNotAllowed,

    #[msg("Accredited investor status is required for this asset")]
    AccreditedInvestorRequired,

    #[msg("Asset is not redeemable")]
    AssetNotRedeemable,

    #[msg("Requested amount exceeds the declared total supply")]
    SupplyExceeded,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Listing is inactive or has insufficient remaining quantity")]
    ListingUnavailable,

    #[msg("Buyer balance is insufficient to settle the trade")]
    InsufficientFunds,

    #[msg("Price overflow when computing the trade settlement")]
    PriceOverflow,

    #[msg("White paper URI is required for MiCA-regulated assets")]
    MissingWhitePaper,

    #[msg("Provided string exceeds the maximum allowed length")]
    StringTooLong,

    #[msg("Fee basis points must be <= 1000 (10%)")]
    FeeTooHigh,

    #[msg("Asset class metadata is invalid or incomplete")]
    InvalidAssetMetadata,

    #[msg("Cross-chain external assets are display-only and cannot be listed")]
    CrossChainNotTradable,

    #[msg("External asset does not declare an SPL mint")]
    MissingExternalMint,

    #[msg("Listing source/mint mismatch")]
    ListingMismatch,

    #[msg("USDC mint does not match the marketplace USDC mint")]
    InvalidUsdcMint,

    #[msg("External asset is not currently active")]
    ExternalAssetInactive,

    #[msg("Aggregator payload incomplete or inconsistent")]
    InvalidAggregatorPayload,

    #[msg("Expected yield must be <= 5000 bps (50%)")]
    InvalidYield,

    #[msg("Maturity must be a future Unix timestamp")]
    InvalidMaturity,

    #[msg("Duration must be between 1 and 120 months")]
    InvalidDuration,

    #[msg("Too many jurisdictions in the policy list (max 32)")]
    TooManyJurisdictions,

    #[msg("Marketplace is paused — write operations are temporarily disabled")]
    Paused,

    #[msg("Provided compliance hook program does not match the configured one")]
    ComplianceHookMismatch,

    #[msg("JurisdictionPolicy account does not match the expected PDA for this marketplace")]
    JurisdictionPolicyMismatch,

    #[msg("Asset metadata is frozen — the first mint has already happened")]
    MetadataFrozen,

    #[msg("This instruction only applies to AssetClass::InvestmentOffering")]
    NotInvestmentOffering,

    #[msg(
        "Invalid compliance signer: must not be the default pubkey nor equal to the current signer"
    )]
    InvalidComplianceSigner,

    #[msg("Invalid issuer: must not be the default pubkey nor equal to the current issuer")]
    InvalidIssuer,

    #[msg("Settlement epochs must be strictly monotonic increasing")]
    EpochNotMonotonic,

    #[msg("Lending parameters out of bounds (LTV/threshold/APR)")]
    InvalidLendingParams,

    #[msg("This asset is not enabled as collateral")]
    CollateralNotEnabled,

    #[msg("Collateral price is zero or stale")]
    InvalidCollateralPrice,

    #[msg("Requested borrow exceeds the maximum loan-to-value")]
    ExceedsMaxLtv,

    #[msg("Lending pool has insufficient USDC liquidity")]
    InsufficientLiquidity,

    #[msg("Loan is not active")]
    LoanInactive,

    #[msg("Loan is healthy — cannot be liquidated yet")]
    LoanHealthy,

    #[msg("Repay amount does not cover the outstanding debt")]
    RepayTooLow,

    #[msg("Withdrawal exceeds provider's deposited liquidity")]
    ExceedsDeposit,

    #[msg("Oracle account is not a valid Pyth PriceUpdateV2 account")]
    InvalidOracleAccount,

    #[msg("Pyth feed id does not match the collateral's configured feed")]
    OracleFeedMismatch,

    #[msg("Pyth price is stale — refresh the collateral price before borrowing")]
    StalePrice,

    #[msg("Pyth price confidence interval is too wide to trust")]
    PriceConfidenceTooWide,

    #[msg("Pyth price is non-positive")]
    InvalidPythPrice,

    #[msg("This collateral is not driven by an oracle")]
    OracleNotEnabled,

    #[msg("Pyth price is unverified (Partial update with too few guardian signatures)")]
    PythPriceUnverified,

    #[msg("A borrower cannot liquidate their own loan")]
    SelfLiquidation,

    #[msg("This lending market requires oracle-priced collateral; manual price is forbidden")]
    OracleRequired,
}
