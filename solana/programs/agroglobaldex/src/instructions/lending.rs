//! # Lending module — collateralized USDC loans against tokenized agro-RWA
//!
//! The killer ag-finance primitive: a producer with tokenized grain (or any
//! AgroGlobalDex asset) deposits it as collateral and borrows USDC instantly,
//! without a bank. Mirrors how Agrotoken unlocked credit for LATAM farmers,
//! but generalized cross-sector + cross-country.
//!
//! Flow:
//!   1. `init_lending_market` (authority) — set APR, max LTV, liquidation
//!      threshold + bonus, create the USDC pool ATA.
//!   2. `deposit_liquidity` (anyone) — fund the pool with USDC.
//!   3. `set_collateral_config` (authority/oracle) — price + enable an asset.
//!   4. `open_loan` (borrower) — lock collateral Token-2022, receive USDC up
//!      to `max_ltv` of the collateral value.
//!   5. `repay_loan` (borrower) — pay principal + accrued interest, unlock
//!      collateral.
//!   6. `liquidate` (anyone) — if the loan is past its health threshold,
//!      repay the debt and seize the collateral (with a bonus).
//!
//! ## Compliance note
//! The collateral vault ATA is owned by the `lending_vault` authority PDA.
//! Because collateral mints are Token-2022 with the compliance TransferHook,
//! the vault PDA MUST have a KYC-verified `ComplianceRecord` (stamped once by
//! the compliance signer with a non-blocked jurisdiction) before any loan can
//! be opened. See RUNBOOK §11.
//!
//! ## Interest
//! Linear accrual: `interest = principal * apr_bps * elapsed_s /
//! (10_000 * SECONDS_PER_YEAR)`. Simple, auditable, no compounding surprises.

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::{transfer as usdc_transfer, Transfer as UsdcTransfer};
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::AgroError;
use crate::state::*;

/// Accrue linear interest into a loan's `accrued_interest_usdc` up to `now`.
fn accrue_interest(loan: &mut LoanPosition, now: i64) -> Result<()> {
    if !loan.active || loan.principal_usdc == 0 {
        loan.last_accrued_at = now;
        return Ok(());
    }
    let elapsed = now.saturating_sub(loan.last_accrued_at).max(0) as u128;
    if elapsed == 0 {
        return Ok(());
    }
    // interest = principal * apr_bps * elapsed / (10_000 * SECONDS_PER_YEAR)
    let numerator = (loan.principal_usdc as u128)
        .checked_mul(loan.apr_bps as u128)
        .ok_or(AgroError::PriceOverflow)?
        .checked_mul(elapsed)
        .ok_or(AgroError::PriceOverflow)?;
    let denom = 10_000u128
        .checked_mul(SECONDS_PER_YEAR as u128)
        .ok_or(AgroError::PriceOverflow)?;
    let new_interest = (numerator / denom) as u64;
    loan.accrued_interest_usdc = loan
        .accrued_interest_usdc
        .checked_add(new_interest)
        .ok_or(AgroError::PriceOverflow)?;
    loan.last_accrued_at = now;
    Ok(())
}

// ===========================================================================
// 1. init_lending_market
// ===========================================================================

#[derive(Accounts)]
pub struct InitLendingMarket<'info> {
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
        init,
        payer = authority,
        space = 8 + LendingMarket::INIT_SPACE,
        seeds = [LENDING_MARKET_SEED, marketplace.key().as_ref()],
        bump
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    /// CHECK: PDA authority that owns the USDC pool + collateral vaults.
    #[account(
        seeds = [LENDING_VAULT_SEED, lending_market.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(address = marketplace.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault_authority,
    )]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn init_lending_market_handler(
    ctx: Context<InitLendingMarket>,
    apr_bps: u16,
    max_ltv_bps: u16,
    liquidation_threshold_bps: u16,
    liquidation_bonus_bps: u16,
) -> Result<()> {
    require!(apr_bps <= 10_000, AgroError::InvalidLendingParams);
    require!(
        max_ltv_bps > 0 && max_ltv_bps < liquidation_threshold_bps,
        AgroError::InvalidLendingParams
    );
    require!(
        liquidation_threshold_bps <= 10_000,
        AgroError::InvalidLendingParams
    );
    require!(liquidation_bonus_bps <= 5_000, AgroError::InvalidLendingParams);

    let lm = &mut ctx.accounts.lending_market;
    lm.marketplace = ctx.accounts.marketplace.key();
    lm.usdc_mint = ctx.accounts.usdc_mint.key();
    lm.usdc_pool = ctx.accounts.usdc_pool.key();
    lm.apr_bps = apr_bps;
    lm.max_ltv_bps = max_ltv_bps;
    lm.liquidation_threshold_bps = liquidation_threshold_bps;
    lm.liquidation_bonus_bps = liquidation_bonus_bps;
    lm.total_liquidity = 0;
    lm.total_borrowed = 0;
    lm.loan_count = 0;
    lm.bump = ctx.bumps.lending_market;
    lm.vault_authority_bump = ctx.bumps.vault_authority;

    emit!(LendingMarketInitialized {
        lending_market: lm.key(),
        marketplace: lm.marketplace,
        apr_bps,
        max_ltv_bps,
    });
    Ok(())
}

// ===========================================================================
// 2. deposit_liquidity
// ===========================================================================

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, lending_market.marketplace.as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(address = lending_market.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        address = lending_market.usdc_pool,
    )]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = provider,
    )]
    pub provider_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = provider,
        space = 8 + LiquidityProvider::INIT_SPACE,
        seeds = [LIQUIDITY_PROVIDER_SEED, lending_market.key().as_ref(), provider.key().as_ref()],
        bump
    )]
    pub liquidity_provider: Box<Account<'info, LiquidityProvider>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_liquidity_handler(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    require!(
        ctx.accounts.provider_usdc_ata.amount >= amount,
        AgroError::InsufficientFunds
    );

    usdc_transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.provider_usdc_ata.to_account_info(),
                to: ctx.accounts.usdc_pool.to_account_info(),
                authority: ctx.accounts.provider.to_account_info(),
            },
        ),
        amount,
    )?;

    let lm = &mut ctx.accounts.lending_market;
    lm.total_liquidity = lm
        .total_liquidity
        .checked_add(amount)
        .ok_or(AgroError::PriceOverflow)?;
    let total_liquidity = lm.total_liquidity;
    let lending_market_key = lm.key();

    // Record/increment this provider's net principal in the pool.
    let lp = &mut ctx.accounts.liquidity_provider;
    lp.lending_market = lending_market_key;
    lp.provider = ctx.accounts.provider.key();
    lp.deposited_usdc = lp
        .deposited_usdc
        .checked_add(amount)
        .ok_or(AgroError::PriceOverflow)?;
    lp.bump = ctx.bumps.liquidity_provider;

    emit!(LiquidityDeposited {
        lending_market: lending_market_key,
        provider: ctx.accounts.provider.key(),
        amount,
        total_liquidity,
    });
    Ok(())
}

// ===========================================================================
// 2b. withdraw_liquidity
// ===========================================================================

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, lending_market.marketplace.as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(
        mut,
        seeds = [LIQUIDITY_PROVIDER_SEED, lending_market.key().as_ref(), provider.key().as_ref()],
        bump = liquidity_provider.bump,
        constraint = liquidity_provider.provider == provider.key()
            @ AgroError::UnauthorizedIssuer,
        constraint = liquidity_provider.lending_market == lending_market.key()
            @ AgroError::ListingMismatch,
    )]
    pub liquidity_provider: Box<Account<'info, LiquidityProvider>>,

    #[account(address = lending_market.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = lending_market.usdc_pool)]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: PDA authority that owns the USDC pool.
    #[account(
        seeds = [LENDING_VAULT_SEED, lending_market.key().as_ref()],
        bump = lending_market.vault_authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = provider,
    )]
    pub provider_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_liquidity_handler(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, AgroError::InvalidAmount);
    require!(
        amount <= ctx.accounts.liquidity_provider.deposited_usdc,
        AgroError::ExceedsDeposit
    );
    // Can't withdraw USDC that's currently lent out.
    require!(
        amount <= ctx.accounts.lending_market.total_liquidity,
        AgroError::InsufficientLiquidity
    );

    // ---- Move USDC pool -> provider --------------------------------------
    let lm_key = ctx.accounts.lending_market.key();
    let vault_bump = ctx.accounts.lending_market.vault_authority_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LENDING_VAULT_SEED,
        lm_key.as_ref(),
        std::slice::from_ref(&vault_bump),
    ]];
    usdc_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.usdc_pool.to_account_info(),
                to: ctx.accounts.provider_usdc_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    let lp = &mut ctx.accounts.liquidity_provider;
    lp.deposited_usdc = lp
        .deposited_usdc
        .checked_sub(amount)
        .ok_or(AgroError::ExceedsDeposit)?;

    let lm = &mut ctx.accounts.lending_market;
    lm.total_liquidity = lm
        .total_liquidity
        .checked_sub(amount)
        .ok_or(AgroError::InsufficientLiquidity)?;

    emit!(LiquidityWithdrawn {
        lending_market: lm_key,
        provider: ctx.accounts.provider.key(),
        amount,
        total_liquidity: lm.total_liquidity,
    });
    Ok(())
}

// ===========================================================================
// 3. set_collateral_config
// ===========================================================================

#[derive(Accounts)]
pub struct SetCollateralConfig<'info> {
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
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
    )]
    pub asset_registry: Box<Account<'info, AssetRegistry>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + CollateralConfig::INIT_SPACE,
        seeds = [COLLATERAL_CONFIG_SEED, lending_market.key().as_ref(), asset_registry.key().as_ref()],
        bump
    )]
    pub collateral_config: Box<Account<'info, CollateralConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn set_collateral_config_handler(
    ctx: Context<SetCollateralConfig>,
    price_usdc_per_token: u64,
    enabled: bool,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let cfg = &mut ctx.accounts.collateral_config;
    cfg.lending_market = ctx.accounts.lending_market.key();
    cfg.asset_registry = ctx.accounts.asset_registry.key();
    cfg.mint = ctx.accounts.asset_registry.mint;
    cfg.price_usdc_per_token = price_usdc_per_token;
    cfg.enabled = enabled;
    cfg.updated_at = now;
    cfg.bump = ctx.bumps.collateral_config;
    // This instruction is the manual-relay path: setting a price by hand
    // switches the collateral back to manual mode. Use `set_collateral_oracle`
    // to (re-)bind a Pyth feed.
    cfg.oracle_enabled = false;

    emit!(CollateralConfigured {
        lending_market: cfg.lending_market,
        asset_registry: cfg.asset_registry,
        price_usdc_per_token,
        enabled,
    });
    Ok(())
}

// ===========================================================================
// 4. open_loan
// ===========================================================================

#[derive(Accounts)]
pub struct OpenLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.authority.as_ref()],
        bump = marketplace.bump,
        constraint = !marketplace.paused @ AgroError::Paused,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, marketplace.key().as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(
        seeds = [COLLATERAL_CONFIG_SEED, lending_market.key().as_ref(), asset_registry.key().as_ref()],
        bump = collateral_config.bump,
        constraint = collateral_config.enabled @ AgroError::CollateralNotEnabled,
    )]
    pub collateral_config: Box<Account<'info, CollateralConfig>>,

    #[account(
        seeds = [
            ASSET_REGISTRY_SEED,
            marketplace.key().as_ref(),
            &asset_registry.index.to_le_bytes(),
        ],
        bump = asset_registry.bump,
        constraint = asset_registry.marketplace == marketplace.key()
            @ AgroError::ListingMismatch,
    )]
    pub asset_registry: Box<Account<'info, AssetRegistry>>,

    /// Borrower must be KYC-verified to take a loan.
    #[account(
        seeds = [COMPLIANCE_RECORD_SEED, marketplace.key().as_ref(), borrower.key().as_ref()],
        bump = borrower_compliance.bump,
        constraint = borrower_compliance.kyc_verified @ AgroError::KycNotVerified,
    )]
    pub borrower_compliance: Box<Account<'info, ComplianceRecord>>,

    #[account(
        mut,
        address = asset_registry.mint,
    )]
    pub collateral_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = borrower,
        associated_token::token_program = collateral_token_program,
    )]
    pub borrower_collateral_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: PDA authority over vaults.
    #[account(
        seeds = [LENDING_VAULT_SEED, lending_market.key().as_ref()],
        bump = lending_market.vault_authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = collateral_token_program,
    )]
    pub collateral_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    // ---- USDC side ----
    #[account(address = lending_market.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = lending_market.usdc_pool)]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = borrower,
        associated_token::token_program = usdc_token_program,
    )]
    pub borrower_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = borrower,
        space = 8 + LoanPosition::INIT_SPACE,
        seeds = [LOAN_SEED, lending_market.key().as_ref(), borrower.key().as_ref(), asset_registry.key().as_ref()],
        bump
    )]
    pub loan: Box<Account<'info, LoanPosition>>,

    pub collateral_token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn open_loan_handler(
    ctx: Context<OpenLoan>,
    collateral_amount: u64,
    borrow_amount: u64,
) -> Result<()> {
    require!(collateral_amount > 0 && borrow_amount > 0, AgroError::InvalidAmount);
    let cfg = &ctx.accounts.collateral_config;
    let price = cfg.price_usdc_per_token;
    require!(price > 0, AgroError::InvalidCollateralPrice);

    // Oracle-driven collateral must have a fresh price: refuse to lend against
    // a stale Pyth quote. Manual-relay collateral (oracle_enabled == false) is
    // unaffected.
    if cfg.oracle_enabled {
        let now = Clock::get()?.unix_timestamp;
        let age = now.saturating_sub(cfg.updated_at);
        require!(age <= cfg.max_staleness_secs, AgroError::StalePrice);
    }

    // collateral_value = collateral_amount * price (USDC base units)
    let collateral_value = (collateral_amount as u128)
        .checked_mul(price as u128)
        .ok_or(AgroError::PriceOverflow)?;
    // max_borrow = collateral_value * max_ltv_bps / 10_000
    let max_borrow = collateral_value
        .checked_mul(ctx.accounts.lending_market.max_ltv_bps as u128)
        .ok_or(AgroError::PriceOverflow)?
        / 10_000u128;
    require!((borrow_amount as u128) <= max_borrow, AgroError::ExceedsMaxLtv);
    require!(
        ctx.accounts.usdc_pool.amount >= borrow_amount,
        AgroError::InsufficientLiquidity
    );

    // ---- Move collateral borrower -> vault (Token-2022, hook fires) -------
    let decimals = ctx.accounts.collateral_mint.decimals;
    transfer_checked(
        CpiContext::new(
            ctx.accounts.collateral_token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.borrower_collateral_ata.to_account_info(),
                mint: ctx.accounts.collateral_mint.to_account_info(),
                to: ctx.accounts.collateral_vault.to_account_info(),
                authority: ctx.accounts.borrower.to_account_info(),
            },
        ),
        collateral_amount,
        decimals,
    )?;

    // ---- Move USDC pool -> borrower --------------------------------------
    let lm_key = ctx.accounts.lending_market.key();
    let vault_bump = ctx.accounts.lending_market.vault_authority_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LENDING_VAULT_SEED,
        lm_key.as_ref(),
        std::slice::from_ref(&vault_bump),
    ]];
    usdc_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.usdc_token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.usdc_pool.to_account_info(),
                to: ctx.accounts.borrower_usdc_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        ),
        borrow_amount,
    )?;

    let now = Clock::get()?.unix_timestamp;
    let loan_index = ctx.accounts.lending_market.loan_count;

    let loan = &mut ctx.accounts.loan;
    loan.lending_market = lm_key;
    loan.borrower = ctx.accounts.borrower.key();
    loan.asset_registry = ctx.accounts.asset_registry.key();
    loan.collateral_mint = ctx.accounts.collateral_mint.key();
    loan.collateral_vault = ctx.accounts.collateral_vault.key();
    loan.collateral_amount = collateral_amount;
    loan.principal_usdc = borrow_amount;
    loan.accrued_interest_usdc = 0;
    loan.apr_bps = ctx.accounts.lending_market.apr_bps;
    loan.opened_at = now;
    loan.last_accrued_at = now;
    loan.loan_index = loan_index;
    loan.active = true;
    loan.bump = ctx.bumps.loan;

    let lm = &mut ctx.accounts.lending_market;
    lm.total_liquidity = lm.total_liquidity.saturating_sub(borrow_amount);
    lm.total_borrowed = lm
        .total_borrowed
        .checked_add(borrow_amount)
        .ok_or(AgroError::PriceOverflow)?;
    lm.loan_count = lm.loan_count.checked_add(1).ok_or(AgroError::PriceOverflow)?;

    emit!(LoanOpened {
        loan: loan.key(),
        lending_market: lm_key,
        borrower: loan.borrower,
        asset_registry: loan.asset_registry,
        collateral_amount,
        principal_usdc: borrow_amount,
        loan_index,
    });
    Ok(())
}

// ===========================================================================
// 5. repay_loan
// ===========================================================================

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, lending_market.marketplace.as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(
        mut,
        seeds = [LOAN_SEED, lending_market.key().as_ref(), borrower.key().as_ref(), loan.asset_registry.as_ref()],
        bump = loan.bump,
        constraint = loan.active @ AgroError::LoanInactive,
        constraint = loan.borrower == borrower.key() @ AgroError::UnauthorizedIssuer,
    )]
    pub loan: Box<Account<'info, LoanPosition>>,

    #[account(mut, address = loan.collateral_mint)]
    pub collateral_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = borrower,
        associated_token::token_program = collateral_token_program,
    )]
    pub borrower_collateral_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: PDA authority over vaults.
    #[account(
        seeds = [LENDING_VAULT_SEED, lending_market.key().as_ref()],
        bump = lending_market.vault_authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut, address = loan.collateral_vault)]
    pub collateral_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = lending_market.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = lending_market.usdc_pool)]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = borrower,
        associated_token::token_program = usdc_token_program,
    )]
    pub borrower_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub collateral_token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Program<'info, Token>,
}

pub fn repay_loan_handler(ctx: Context<RepayLoan>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    accrue_interest(&mut ctx.accounts.loan, now)?;

    let principal = ctx.accounts.loan.principal_usdc;
    let interest = ctx.accounts.loan.accrued_interest_usdc;
    let total_due = principal
        .checked_add(interest)
        .ok_or(AgroError::PriceOverflow)?;
    require!(
        ctx.accounts.borrower_usdc_ata.amount >= total_due,
        AgroError::InsufficientFunds
    );

    // ---- USDC borrower -> pool (principal + interest) --------------------
    usdc_transfer(
        CpiContext::new(
            ctx.accounts.usdc_token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.borrower_usdc_ata.to_account_info(),
                to: ctx.accounts.usdc_pool.to_account_info(),
                authority: ctx.accounts.borrower.to_account_info(),
            },
        ),
        total_due,
    )?;

    // ---- Collateral vault -> borrower ------------------------------------
    let collateral_amount = ctx.accounts.loan.collateral_amount;
    let lm_key = ctx.accounts.lending_market.key();
    let vault_bump = ctx.accounts.lending_market.vault_authority_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LENDING_VAULT_SEED,
        lm_key.as_ref(),
        std::slice::from_ref(&vault_bump),
    ]];
    let decimals = ctx.accounts.collateral_mint.decimals;
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.collateral_token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.collateral_vault.to_account_info(),
                mint: ctx.accounts.collateral_mint.to_account_info(),
                to: ctx.accounts.borrower_collateral_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        ),
        collateral_amount,
        decimals,
    )?;

    let lm = &mut ctx.accounts.lending_market;
    lm.total_liquidity = lm
        .total_liquidity
        .checked_add(total_due)
        .ok_or(AgroError::PriceOverflow)?;
    lm.total_borrowed = lm.total_borrowed.saturating_sub(principal);

    let loan = &mut ctx.accounts.loan;
    loan.active = false;
    loan.principal_usdc = 0;
    loan.accrued_interest_usdc = 0;

    emit!(LoanRepaid {
        loan: loan.key(),
        borrower: loan.borrower,
        principal_usdc: principal,
        interest_usdc: interest,
        collateral_returned: collateral_amount,
    });
    Ok(())
}

// ===========================================================================
// 6. liquidate
// ===========================================================================

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_MARKET_SEED, lending_market.marketplace.as_ref()],
        bump = lending_market.bump,
    )]
    pub lending_market: Box<Account<'info, LendingMarket>>,

    #[account(
        seeds = [COLLATERAL_CONFIG_SEED, lending_market.key().as_ref(), loan.asset_registry.as_ref()],
        bump = collateral_config.bump,
    )]
    pub collateral_config: Box<Account<'info, CollateralConfig>>,

    #[account(
        mut,
        seeds = [LOAN_SEED, lending_market.key().as_ref(), loan.borrower.as_ref(), loan.asset_registry.as_ref()],
        bump = loan.bump,
        constraint = loan.active @ AgroError::LoanInactive,
    )]
    pub loan: Box<Account<'info, LoanPosition>>,

    /// Liquidator must be KYC-verified to receive the seized collateral.
    #[account(
        seeds = [COMPLIANCE_RECORD_SEED, lending_market.marketplace.as_ref(), liquidator.key().as_ref()],
        bump = liquidator_compliance.bump,
        constraint = liquidator_compliance.kyc_verified @ AgroError::KycNotVerified,
    )]
    pub liquidator_compliance: Box<Account<'info, ComplianceRecord>>,

    #[account(mut, address = loan.collateral_mint)]
    pub collateral_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = liquidator,
        associated_token::token_program = collateral_token_program,
    )]
    pub liquidator_collateral_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: PDA authority over vaults.
    #[account(
        seeds = [LENDING_VAULT_SEED, lending_market.key().as_ref()],
        bump = lending_market.vault_authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut, address = loan.collateral_vault)]
    pub collateral_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = lending_market.usdc_mint @ AgroError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = lending_market.usdc_pool)]
    pub usdc_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = liquidator,
        associated_token::token_program = usdc_token_program,
    )]
    pub liquidator_usdc_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub collateral_token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Program<'info, Token>,
}

pub fn liquidate_handler(ctx: Context<Liquidate>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    accrue_interest(&mut ctx.accounts.loan, now)?;

    let price = ctx.accounts.collateral_config.price_usdc_per_token;
    require!(price > 0, AgroError::InvalidCollateralPrice);

    let principal = ctx.accounts.loan.principal_usdc;
    let interest = ctx.accounts.loan.accrued_interest_usdc;
    let debt = principal
        .checked_add(interest)
        .ok_or(AgroError::PriceOverflow)?;

    // Health check: current LTV = debt / collateral_value. Liquidatable when
    // debt * 10_000 >= collateral_value * liquidation_threshold_bps.
    let collateral_value = (ctx.accounts.loan.collateral_amount as u128)
        .checked_mul(price as u128)
        .ok_or(AgroError::PriceOverflow)?;
    let lhs = (debt as u128)
        .checked_mul(10_000u128)
        .ok_or(AgroError::PriceOverflow)?;
    let rhs = collateral_value
        .checked_mul(ctx.accounts.lending_market.liquidation_threshold_bps as u128)
        .ok_or(AgroError::PriceOverflow)?;
    require!(lhs >= rhs, AgroError::LoanHealthy);

    // Liquidator repays the full debt into the pool.
    require!(
        ctx.accounts.liquidator_usdc_ata.amount >= debt,
        AgroError::InsufficientFunds
    );
    usdc_transfer(
        CpiContext::new(
            ctx.accounts.usdc_token_program.to_account_info(),
            UsdcTransfer {
                from: ctx.accounts.liquidator_usdc_ata.to_account_info(),
                to: ctx.accounts.usdc_pool.to_account_info(),
                authority: ctx.accounts.liquidator.to_account_info(),
            },
        ),
        debt,
    )?;

    // Liquidator seizes ALL collateral (bonus is implicit: they paid `debt`
    // but receive collateral worth >= debt / threshold). Simple + safe.
    let collateral_amount = ctx.accounts.loan.collateral_amount;
    let lm_key = ctx.accounts.lending_market.key();
    let vault_bump = ctx.accounts.lending_market.vault_authority_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        LENDING_VAULT_SEED,
        lm_key.as_ref(),
        std::slice::from_ref(&vault_bump),
    ]];
    let decimals = ctx.accounts.collateral_mint.decimals;
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.collateral_token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.collateral_vault.to_account_info(),
                mint: ctx.accounts.collateral_mint.to_account_info(),
                to: ctx.accounts.liquidator_collateral_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        ),
        collateral_amount,
        decimals,
    )?;

    let lm = &mut ctx.accounts.lending_market;
    lm.total_liquidity = lm
        .total_liquidity
        .checked_add(debt)
        .ok_or(AgroError::PriceOverflow)?;
    lm.total_borrowed = lm.total_borrowed.saturating_sub(principal);

    let loan = &mut ctx.accounts.loan;
    loan.active = false;
    loan.principal_usdc = 0;
    loan.accrued_interest_usdc = 0;

    emit!(LoanLiquidated {
        loan: loan.key(),
        borrower: loan.borrower,
        liquidator: ctx.accounts.liquidator.key(),
        debt_repaid_usdc: debt,
        collateral_seized: collateral_amount,
    });
    Ok(())
}
