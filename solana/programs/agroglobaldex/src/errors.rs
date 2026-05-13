use anchor_lang::prelude::*;

#[error_code]
pub enum AgroError {
    #[msg("Caller is not the marketplace authority")]
    UnauthorizedMarketplaceAuthority,

    #[msg("Caller is not the compliance authority")]
    UnauthorizedComplianceAuthority,

    #[msg("Caller is not the asset issuer")]
    UnauthorizedIssuer,

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
}
