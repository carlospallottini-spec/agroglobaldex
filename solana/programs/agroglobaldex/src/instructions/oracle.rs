//! # Price oracle — Pyth pull-feed integration for lending collateral
//!
//! Replaces the "authority types in a price" relay with a real, permissionless
//! price source. The marketplace authority binds a Pyth price-feed id to a
//! collateral via [`set_collateral_oracle`]; after that ANYONE can crank
//! [`refresh_collateral_price`], which reads the on-chain Pyth `PriceUpdateV2`
//! account, validates it (owner, discriminator, feed id, staleness, confidence)
//! and caches the price into `CollateralConfig.price_usdc_per_token`. The rest
//! of the lending logic (`open_loan`, `liquidate`) keeps reading that cached
//! field, so the hot path stays cheap — but `open_loan` now refuses to lend
//! against a stale oracle price (see `lending.rs`).
//!
//! ## Why we parse the account by hand
//! We deliberately avoid adding the `pyth-solana-receiver-sdk` crate: pinning
//! it against `solana-program 2.3` / `anchor 0.31` is fragile and a heavy
//! dependency for a PoC. Instead we read the well-documented `PriceUpdateV2`
//! byte layout directly, after asserting the account owner and Anchor
//! discriminator — which is what makes the read safe.
//!
//! `PriceUpdateV2` layout (after the 8-byte Anchor discriminator):
//! ```text
//! write_authority : Pubkey                       (32 bytes)
//! verification_level : enum                       (1 byte tag; +1 if Partial)
//!     Partial { num_signatures: u8 } => [0, n]
//!     Full                           => [1]
//! price_message :
//!     feed_id          : [u8; 32]                 (32)
//!     price            : i64                       (8)
//!     conf             : u64                       (8)
//!     exponent         : i32                       (4)
//!     publish_time     : i64                       (8)
//!     prev_publish_time: i64                       (8)
//!     ema_price        : i64                       (8)
//!     ema_conf         : u64                       (8)
//! posted_slot : u64                                (8)
//! ```

use anchor_lang::prelude::*;

use crate::errors::AgroError;
use crate::state::*;

/// Pyth pull-oracle "receiver" program. Every `PriceUpdateV2` account is owned
/// by this program on both devnet and mainnet-beta.
pub const PYTH_RECEIVER_PROGRAM_ID: Pubkey = pubkey!("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");

/// Anchor discriminator of `PriceUpdateV2` (= sha256("account:PriceUpdateV2")[..8]).
pub const PRICE_UPDATE_V2_DISCRIMINATOR: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];

/// Minimum guardian signatures we accept for a `Partial` Pyth price. A `Full`
/// update (all guardians) is always accepted. Pyth's default is 5.
pub const MIN_PYTH_SIGNATURES: u8 = 5;

/// How far in the future a Pyth `publish_time` may be (clock skew tolerance)
/// before we reject it as bogus. Prevents a future-dated price from defeating
/// the staleness check via `saturating_sub` underflow-to-zero.
pub const MAX_PYTH_FUTURE_SKEW_SECS: i64 = 60;

/// Minimal, validated view of a Pyth price message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PythPrice {
    pub feed_id: [u8; 32],
    pub price: i64,
    pub conf: u64,
    pub expo: i32,
    pub publish_time: i64,
    /// 1 = Full (all guardian sigs verified), 0 = Partial.
    pub verification_level: u8,
    /// Number of guardian signatures when `verification_level == 0` (Partial).
    pub num_signatures: u8,
}

/// A price is trustworthy if it is `Full` or a `Partial` with at least
/// [`MIN_PYTH_SIGNATURES`] guardian signatures. A `Partial { 0 }` (an update
/// the receiver wrote WITHOUT verifying signatures) is rejected.
pub fn is_price_verified(verification_level: u8, num_signatures: u8) -> bool {
    verification_level == 1 || num_signatures >= MIN_PYTH_SIGNATURES
}

/// Parse + validate the raw bytes of a `PriceUpdateV2` account. Caller MUST
/// have already verified the account owner is [`PYTH_RECEIVER_PROGRAM_ID`].
pub fn parse_price_update_v2(data: &[u8]) -> Result<PythPrice> {
    // 8 disc + 32 authority + 1 vlevel tag = 41 bytes minimum before the
    // price message even starts.
    require!(data.len() >= 41, AgroError::InvalidOracleAccount);
    require!(
        data[..8] == PRICE_UPDATE_V2_DISCRIMINATOR,
        AgroError::InvalidOracleAccount
    );

    // verification_level is a borsh enum right after the 32-byte authority.
    let vlevel_tag = data[40];
    let (msg_start, num_signatures) = match vlevel_tag {
        0 => (42, data[41]), // Partial { num_signatures: u8 } — 2 bytes consumed
        1 => (41, 0u8),      // Full — 1 byte consumed
        _ => return err!(AgroError::InvalidOracleAccount),
    };

    // feed_id(32) + price(8) + conf(8) + expo(4) + publish_time(8) = 60 bytes.
    let end = msg_start + 60;
    require!(data.len() >= end, AgroError::InvalidOracleAccount);

    let mut feed_id = [0u8; 32];
    feed_id.copy_from_slice(&data[msg_start..msg_start + 32]);
    let mut o = msg_start + 32;

    let price = i64::from_le_bytes(read8(data, &mut o));
    let conf = u64::from_le_bytes(read8(data, &mut o));
    let expo = i32::from_le_bytes(read4(data, &mut o));
    let publish_time = i64::from_le_bytes(read8(data, &mut o));

    Ok(PythPrice {
        feed_id,
        price,
        conf,
        expo,
        publish_time,
        verification_level: vlevel_tag,
        num_signatures,
    })
}

#[inline]
fn read8(data: &[u8], off: &mut usize) -> [u8; 8] {
    let mut b = [0u8; 8];
    b.copy_from_slice(&data[*off..*off + 8]);
    *off += 8;
    b
}

#[inline]
fn read4(data: &[u8], off: &mut usize) -> [u8; 4] {
    let mut b = [0u8; 4];
    b.copy_from_slice(&data[*off..*off + 4]);
    *off += 4;
    b
}

/// Convert a Pyth (price, expo) USD quote into USDC base units (6 decimals).
///
/// `usd = price * 10^expo`, so `usdc_6dp = price * 10^(expo + 6)`.
/// Returns `InvalidPythPrice` if the price is non-positive, and `PriceOverflow`
/// on out-of-range scaling.
pub fn pyth_to_usdc_6dp(price: i64, expo: i32) -> Result<u64> {
    require!(price > 0, AgroError::InvalidPythPrice);
    let p = price as i128;
    let net = expo + 6; // shift into USDC 6-decimal space

    let scaled: i128 = if net >= 0 {
        // Guard absurd exponents to keep the shift bounded.
        require!(net <= 24, AgroError::PriceOverflow);
        let factor = 10i128
            .checked_pow(net as u32)
            .ok_or(AgroError::PriceOverflow)?;
        p.checked_mul(factor).ok_or(AgroError::PriceOverflow)?
    } else {
        let down = (-net) as u32;
        require!(down <= 24, AgroError::PriceOverflow);
        let divisor = 10i128.checked_pow(down).ok_or(AgroError::PriceOverflow)?;
        p / divisor
    };

    require!(scaled > 0, AgroError::InvalidPythPrice);
    require!(scaled <= u64::MAX as i128, AgroError::PriceOverflow);
    Ok(scaled as u64)
}

/// Reject prices whose confidence band is wider than `max_confidence_bps`
/// relative to the price. `max_confidence_bps == 0` disables the check.
pub fn check_confidence(price: i64, conf: u64, max_confidence_bps: u16) -> Result<()> {
    if max_confidence_bps == 0 {
        return Ok(());
    }
    let ratio_bps = (conf as u128)
        .checked_mul(10_000)
        .ok_or(AgroError::PriceOverflow)?
        / (price as u128);
    require!(
        ratio_bps <= max_confidence_bps as u128,
        AgroError::PriceConfidenceTooWide
    );
    Ok(())
}

// ===========================================================================
// set_collateral_oracle — bind a Pyth feed to a collateral (authority-only)
// ===========================================================================

#[derive(Accounts)]
pub struct SetCollateralOracle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.authority == authority.key()
            @ AgroError::UnauthorizedMarketplaceAuthority,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        seeds = [LENDING_MARKET_SEED, marketplace.key().as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(
        mut,
        seeds = [COLLATERAL_CONFIG_SEED, lending_market.key().as_ref(), collateral_config.asset_registry.as_ref()],
        bump = collateral_config.bump,
    )]
    pub collateral_config: Box<Account<'info, CollateralConfig>>,
}

pub fn set_collateral_oracle_handler(
    ctx: Context<SetCollateralOracle>,
    oracle_feed_id: [u8; 32],
    max_staleness_secs: i64,
    max_confidence_bps: u16,
    enabled: bool,
) -> Result<()> {
    require!(max_staleness_secs > 0, AgroError::InvalidLendingParams);
    let cfg = &mut ctx.accounts.collateral_config;
    cfg.oracle_enabled = enabled;
    cfg.oracle_feed_id = oracle_feed_id;
    cfg.max_staleness_secs = max_staleness_secs;
    cfg.max_confidence_bps = max_confidence_bps;
    // Force a fresh crank before this collateral can back a new loan: an
    // oracle-driven config is considered stale until refresh_collateral_price
    // runs at least once.
    if enabled {
        cfg.updated_at = 0;
    }

    emit!(CollateralOracleSet {
        lending_market: cfg.lending_market,
        asset_registry: cfg.asset_registry,
        oracle_feed_id,
        max_staleness_secs,
        max_confidence_bps,
        enabled,
    });
    Ok(())
}

// ===========================================================================
// refresh_collateral_price — permissionless crank from the Pyth feed
// ===========================================================================

#[derive(Accounts)]
pub struct RefreshCollateralPrice<'info> {
    /// Anyone can crank the price; no authority required.
    pub cranker: Signer<'info>,

    #[account(
        mut,
        seeds = [COLLATERAL_CONFIG_SEED, collateral_config.lending_market.as_ref(), collateral_config.asset_registry.as_ref()],
        bump = collateral_config.bump,
    )]
    pub collateral_config: Box<Account<'info, CollateralConfig>>,

    /// CHECK: validated by owner + discriminator + feed id inside the handler.
    #[account(owner = PYTH_RECEIVER_PROGRAM_ID @ AgroError::InvalidOracleAccount)]
    pub price_update: UncheckedAccount<'info>,
}

pub fn refresh_collateral_price_handler(ctx: Context<RefreshCollateralPrice>) -> Result<()> {
    let cfg = &mut ctx.accounts.collateral_config;
    require!(cfg.oracle_enabled, AgroError::OracleNotEnabled);

    let data = ctx.accounts.price_update.try_borrow_data()?;
    let pyth = parse_price_update_v2(&data)?;

    // Reject unverified prices: a `Partial { num_signatures: 0 }` update is one
    // the receiver wrote WITHOUT verifying guardian signatures — accepting it
    // would let anyone set an arbitrary collateral price.
    require!(
        is_price_verified(pyth.verification_level, pyth.num_signatures),
        AgroError::PythPriceUnverified
    );
    // Positive price first — guards the confidence-ratio division below and the
    // USDC conversion.
    require!(pyth.price > 0, AgroError::InvalidPythPrice);

    // The price MUST come from the feed this collateral is bound to.
    require!(
        pyth.feed_id == cfg.oracle_feed_id,
        AgroError::OracleFeedMismatch
    );

    let now = Clock::get()?.unix_timestamp;
    // Reject future-dated prints (clock skew tolerated) so a crafted
    // `publish_time > now` can't defeat the staleness gate via saturating_sub.
    require!(
        pyth.publish_time <= now.saturating_add(MAX_PYTH_FUTURE_SKEW_SECS),
        AgroError::StalePrice
    );
    let age = now.saturating_sub(pyth.publish_time);
    require!(age <= cfg.max_staleness_secs, AgroError::StalePrice);

    check_confidence(pyth.price, pyth.conf, cfg.max_confidence_bps)?;

    let price_usdc = pyth_to_usdc_6dp(pyth.price, pyth.expo)?;
    cfg.price_usdc_per_token = price_usdc;
    cfg.updated_at = now;

    emit!(CollateralPriceRefreshed {
        lending_market: cfg.lending_market,
        asset_registry: cfg.asset_registry,
        price_usdc_per_token: price_usdc,
        pyth_price: pyth.price,
        pyth_expo: pyth.expo,
        pyth_conf: pyth.conf,
        publish_time: pyth.publish_time,
    });
    Ok(())
}

// ===========================================================================
// Unit tests — pure conversion / parsing logic
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usdc_conversion_typical_expo() {
        // $5.50 at expo -8 => 550000000 raw. usdc_6dp = 5_500_000.
        assert_eq!(pyth_to_usdc_6dp(550_000_000, -8).unwrap(), 5_500_000);
    }

    #[test]
    fn usdc_conversion_expo_minus_six_is_identity() {
        // expo == -6 => price already in USDC 6dp.
        assert_eq!(pyth_to_usdc_6dp(1_234_567, -6).unwrap(), 1_234_567);
    }

    #[test]
    fn usdc_conversion_positive_net_expo() {
        // price 3 at expo -5 => $0.00003 => usdc_6dp = 3 * 10^1 = 30.
        assert_eq!(pyth_to_usdc_6dp(3, -5).unwrap(), 30);
    }

    #[test]
    fn usdc_conversion_rejects_non_positive() {
        assert!(pyth_to_usdc_6dp(0, -8).is_err());
        assert!(pyth_to_usdc_6dp(-1, -8).is_err());
    }

    #[test]
    fn verification_gate() {
        assert!(is_price_verified(1, 0)); // Full always ok
        assert!(is_price_verified(0, 5)); // Partial w/ >= MIN sigs ok
        assert!(is_price_verified(0, 9));
        assert!(!is_price_verified(0, 0)); // Partial { 0 } rejected
        assert!(!is_price_verified(0, 4)); // below MIN
    }

    #[test]
    fn parse_exposes_verification_fields() {
        // Full
        let mut full = vec![0u8; 8 + 32 + 1 + 60 + 8];
        full[..8].copy_from_slice(&PRICE_UPDATE_V2_DISCRIMINATOR);
        full[40] = 1;
        let p = parse_price_update_v2(&full).unwrap();
        assert_eq!(p.verification_level, 1);
        assert_eq!(p.num_signatures, 0);
        // Partial { 3 }
        let mut part = vec![0u8; 8 + 32 + 2 + 60 + 8];
        part[..8].copy_from_slice(&PRICE_UPDATE_V2_DISCRIMINATOR);
        part[40] = 0;
        part[41] = 3;
        let p2 = parse_price_update_v2(&part).unwrap();
        assert_eq!(p2.verification_level, 0);
        assert_eq!(p2.num_signatures, 3);
        assert!(!is_price_verified(p2.verification_level, p2.num_signatures));
    }

    #[test]
    fn confidence_gate() {
        // conf 1% of price, bound 2% => ok; bound 0.5% => reject.
        assert!(check_confidence(1_000_000, 10_000, 200).is_ok());
        assert!(check_confidence(1_000_000, 10_000, 50).is_err());
        // zero bound disables the check.
        assert!(check_confidence(1_000_000, 999_999, 0).is_ok());
    }

    #[test]
    fn parse_rejects_bad_discriminator() {
        let data = [0u8; 200];
        assert!(parse_price_update_v2(&data).is_err());
    }

    #[test]
    fn parse_full_verification_level_roundtrip() {
        // Build a synthetic Full PriceUpdateV2 and read it back.
        let mut data = vec![0u8; 8 + 32 + 1 + 60 + 8];
        data[..8].copy_from_slice(&PRICE_UPDATE_V2_DISCRIMINATOR);
        data[40] = 1; // Full
        let msg = 41;
        let feed = [7u8; 32];
        data[msg..msg + 32].copy_from_slice(&feed);
        data[msg + 32..msg + 40].copy_from_slice(&650_000_000i64.to_le_bytes());
        data[msg + 40..msg + 48].copy_from_slice(&1_000_000u64.to_le_bytes());
        data[msg + 48..msg + 52].copy_from_slice(&(-8i32).to_le_bytes());
        data[msg + 52..msg + 60].copy_from_slice(&1_700_000_000i64.to_le_bytes());

        let p = parse_price_update_v2(&data).unwrap();
        assert_eq!(p.feed_id, feed);
        assert_eq!(p.price, 650_000_000);
        assert_eq!(p.conf, 1_000_000);
        assert_eq!(p.expo, -8);
        assert_eq!(p.publish_time, 1_700_000_000);
        assert_eq!(pyth_to_usdc_6dp(p.price, p.expo).unwrap(), 6_500_000);
    }

    #[test]
    fn parse_partial_verification_level_offsets() {
        // Partial { num_signatures } shifts the message by one byte.
        let mut data = vec![0u8; 8 + 32 + 2 + 60 + 8];
        data[..8].copy_from_slice(&PRICE_UPDATE_V2_DISCRIMINATOR);
        data[40] = 0; // Partial
        data[41] = 3; // num_signatures
        let msg = 42;
        let feed = [9u8; 32];
        data[msg..msg + 32].copy_from_slice(&feed);
        data[msg + 32..msg + 40].copy_from_slice(&100i64.to_le_bytes());
        data[msg + 40..msg + 48].copy_from_slice(&5u64.to_le_bytes());
        data[msg + 48..msg + 52].copy_from_slice(&(-2i32).to_le_bytes());
        data[msg + 52..msg + 60].copy_from_slice(&42i64.to_le_bytes());

        let p = parse_price_update_v2(&data).unwrap();
        assert_eq!(p.feed_id, feed);
        assert_eq!(p.price, 100);
        assert_eq!(p.expo, -2);
        assert_eq!(p.publish_time, 42);
    }
}
