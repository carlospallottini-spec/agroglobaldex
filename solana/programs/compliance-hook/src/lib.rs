//! # AgroGlobalDex Compliance Transfer Hook
//!
//! Token-2022 `TransferHook` program enforcing KYC + jurisdiction policy on
//! every transfer of an AgroGlobalDex-issued mint.
//!
//! ## Design
//!
//! 1. Per-mint setup. When the main `agroglobaldex` program registers a new
//!    asset and creates its Token-2022 mint, it CPIs into this program's
//!    `initialize_extra_account_meta_list` instruction. That call:
//!      - persists a `HookConfig` PDA (`[b"hook_config", mint]`) carrying the
//!        marketplace pubkey + the agroglobaldex program id so we can later
//!        resolve `ComplianceRecord` and `JurisdictionPolicy` PDAs;
//!      - writes the `ExtraAccountMetaList` (`[b"extra-account-metas", mint]`)
//!        declaring the additional accounts Token-2022 must pass on every
//!        transfer.
//!
//! 2. On every transfer. Token-2022 invokes `execute(amount)` after the source
//!    balance has been debited; we receive (in this order):
//!       0. source token account
//!       1. mint
//!       2. destination token account
//!       3. owner of the source (authority)
//!       4. extra_account_meta_list PDA (Token-2022 appends it from the TLV)
//!       Then the extra accounts declared in the TLV:
//!       5. hook_config PDA
//!       6. marketplace account (referenced from hook_config)
//!       7. jurisdiction_policy PDA
//!       8. source ComplianceRecord PDA
//!       9. destination ComplianceRecord PDA
//!
//!    We verify both compliance records (`kyc_verified == true`,
//!    `jurisdiction` not in `policy.blocked`). If anything fails the transfer
//!    is aborted.
//!
//! ## Caveats (PoC scope)
//!
//! - Mints created without a `ComplianceRecord` on the destination side will
//!   simply fail to transfer to that wallet, which is exactly the intended
//!   behaviour for a regulated venue.
//! - We do NOT enforce class-specific rules (e.g. accredited investor for
//!   `HarvestFraction`/`InvestmentOffering`) inside the hook — they are
//!   enforced by the marketplace's `buy_asset` instruction. The hook is the
//!   universal floor: KYC + jurisdiction. Class-specific gates are surface-
//!   level: enforced in IXs where the asset class is known.
//! - NOTE: this is a Proof-of-Concept. Not audited.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use spl_discriminator::SplDiscriminate;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

declare_id!("GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL");

// PDA seeds shared with the main program. Keep these byte-identical to the
// constants defined in `agroglobaldex::state` so the resolver can construct
// `ComplianceRecord` and `JurisdictionPolicy` PDAs.
pub const HOOK_CONFIG_SEED: &[u8] = b"hook_config";
pub const EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"extra-account-metas";
pub const COMPLIANCE_RECORD_SEED: &[u8] = b"compliance_record";
pub const JURISDICTION_POLICY_SEED: &[u8] = b"jurisdiction_policy";

#[program]
pub mod compliance_hook {
    use super::*;

    /// Wire up the per-mint validation data. Called once per AgroGlobalDex
    /// mint, right after the mint itself is created. The caller is the main
    /// program (signing for the `asset_registry` PDA acting as the mint
    /// authority) — but for Anchor IDL friendliness the signer is the
    /// `authority` account, which we validate to be the mint authority on the
    /// client side (Token-2022 only checks the signature; the seeds bind the
    /// account to the mint).
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
        marketplace: Pubkey,
        agroglobaldex_program: Pubkey,
    ) -> Result<()> {
        // ---- Persist HookConfig ------------------------------------------------
        let cfg = &mut ctx.accounts.hook_config;
        cfg.mint = ctx.accounts.mint.key();
        cfg.marketplace = marketplace;
        cfg.agroglobaldex_program = agroglobaldex_program;
        cfg.bump = ctx.bumps.hook_config;

        // ---- Build the extra-account-meta list --------------------------------
        // Index layout when Execute is invoked by Token-2022:
        //   0 source, 1 mint, 2 destination, 3 owner (=source authority),
        //   4 extra_account_meta_list (we are here),
        //   5 hook_config            (this PDA, seeds [b"hook_config", mint])
        //   6 marketplace            (literal pubkey from cfg)
        //   7 jurisdiction_policy    (PDA on agroglobaldex_program)
        //   8 source_compliance      (PDA on agroglobaldex_program)
        //   9 destination_compliance (PDA on agroglobaldex_program)
        //
        // Indices used in Seed::AccountKey below reference the FULL accounts
        // list (instruction accounts + extra accounts resolved so far).
        let metas: Vec<ExtraAccountMeta> = vec![
            // 5: hook_config — PDA on THIS program
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: HOOK_CONFIG_SEED.to_vec(),
                    },
                    Seed::AccountKey { index: 1 }, // mint
                ],
                false,
                false,
            )
            .map_err(|_| HookError::AccountMetaConstructionFailed)?,
            // 6: marketplace — literal pubkey
            ExtraAccountMeta::new_with_pubkey(&marketplace, false, false)
                .map_err(|_| HookError::AccountMetaConstructionFailed)?,
            // 7: jurisdiction_policy — PDA on the EXTERNAL agroglobaldex program.
            // We pass the agroglobaldex program id as another literal account
            // for resolution purposes via `new_external_pda_with_seeds`. Since
            // ExtraAccountMeta cross-program PDA resolution needs the foreign
            // program ID to appear in the accounts list, we store it on the
            // hook_config (cfg.agroglobaldex_program) and have the off-chain
            // resolver / on-chain check_account_infos call build the PDA
            // manually. The simplest layout the TLV supports: declare
            // jurisdiction_policy with seeds derived through the **same**
            // resolver but pointing at the agroglobaldex program. For that we
            // include the agroglobaldex_program as an extra account first.
            //
            // ----- We split this into two metas for clarity:
            //   7: agroglobaldex_program (literal)
            //   8: jurisdiction_policy   (external PDA, program_index = 7)
            //   9: source_compliance     (external PDA, program_index = 7)
            //  10: destination_compliance(external PDA, program_index = 7)
            ExtraAccountMeta::new_with_pubkey(&agroglobaldex_program, false, false)
                .map_err(|_| HookError::AccountMetaConstructionFailed)?,
            // 8: jurisdiction_policy
            ExtraAccountMeta::new_external_pda_with_seeds(
                7, // program index in the accounts list
                &[
                    Seed::Literal {
                        bytes: JURISDICTION_POLICY_SEED.to_vec(),
                    },
                    Seed::AccountKey { index: 6 }, // marketplace
                ],
                false,
                false,
            )
            .map_err(|_| HookError::AccountMetaConstructionFailed)?,
            // 9: source compliance record — seeds [COMPLIANCE_RECORD_SEED, marketplace, source_owner]
            ExtraAccountMeta::new_external_pda_with_seeds(
                7,
                &[
                    Seed::Literal {
                        bytes: COMPLIANCE_RECORD_SEED.to_vec(),
                    },
                    Seed::AccountKey { index: 6 }, // marketplace
                    Seed::AccountKey { index: 3 }, // source owner
                ],
                false,
                false,
            )
            .map_err(|_| HookError::AccountMetaConstructionFailed)?,
            // 10: destination compliance record — we use the destination
            //     token account *owner*, but at the point the hook runs we
            //     only have the destination token account (index 2) as an
            //     `AccountInfo`. The compliance record is keyed by the
            //     destination wallet (its owner). We resolve that by reading
            //     the owner field (offset 32, len 32) of the destination
            //     token account data via `Seed::AccountData`.
            ExtraAccountMeta::new_external_pda_with_seeds(
                7,
                &[
                    Seed::Literal {
                        bytes: COMPLIANCE_RECORD_SEED.to_vec(),
                    },
                    Seed::AccountKey { index: 6 }, // marketplace
                    Seed::AccountData {
                        account_index: 2, // destination token account
                        data_index: 32,   // SPL token account `owner` field offset
                        length: 32,
                    },
                ],
                false,
                false,
            )
            .map_err(|_| HookError::AccountMetaConstructionFailed)?,
        ];

        // ---- Create the ExtraAccountMetaList account manually ---------------
        // We can't use Anchor `init` because the account size depends on the
        // number of metas and uses a TLV layout.
        let account_size = ExtraAccountMetaList::size_of(metas.len())
            .map_err(|_| HookError::AccountMetaConstructionFailed)?;
        let lamports = Rent::get()?.minimum_balance(account_size);

        let mint_key = ctx.accounts.mint.key();
        let bump = ctx.bumps.extra_account_meta_list;
        let signer_seeds: &[&[u8]] = &[
            EXTRA_ACCOUNT_METAS_SEED,
            mint_key.as_ref(),
            std::slice::from_ref(&bump),
        ];

        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.extra_account_meta_list.key,
                lamports,
                account_size as u64,
                ctx.program_id,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.extra_account_meta_list.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[signer_seeds],
        )?;

        let mut data = ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)
            .map_err(|_| HookError::AccountMetaListInitFailed)?;

        msg!(
            "ComplianceHook::initialize_extra_account_meta_list mint={} marketplace={} agroglobaldex={}",
            mint_key,
            marketplace,
            agroglobaldex_program,
        );
        Ok(())
    }

    /// Token-2022 transfer hook entrypoint. Verifies both source and
    /// destination wallets have valid `ComplianceRecord`s and that neither
    /// jurisdiction is in the policy blocklist.
    pub fn execute(ctx: Context<Execute>, _amount: u64) -> Result<()> {
        // ---- Validate external PDAs (defense in depth) ------------------------
        // The Execute accounts struct only owner-checks jurisdiction_policy,
        // source_compliance and destination_compliance. An attacker could pass
        // a different `JurisdictionPolicy` (e.g. from another marketplace) or
        // somebody else's `ComplianceRecord` (e.g. a KYC'd user's record to
        // bypass the source's own KYC). We defend by re-deriving each PDA from
        // the trusted inputs (marketplace pubkey, agroglobaldex program id,
        // source owner, destination token account owner) and asserting key
        // equality.
        let marketplace = ctx.accounts.marketplace.key();
        let agroglobaldex = ctx.accounts.agroglobaldex_program.key();

        // jurisdiction_policy = PDA([JURISDICTION_POLICY_SEED, marketplace], agroglobaldex)
        let (expected_policy, _) = Pubkey::find_program_address(
            &[JURISDICTION_POLICY_SEED, marketplace.as_ref()],
            &agroglobaldex,
        );
        require_keys_eq!(
            ctx.accounts.jurisdiction_policy.key(),
            expected_policy,
            HookError::JurisdictionPolicyMismatch
        );

        // source_compliance = PDA([COMPLIANCE_RECORD_SEED, marketplace, source_owner], agroglobaldex)
        let source_owner = ctx.accounts.owner.key();
        let (expected_src, _) = Pubkey::find_program_address(
            &[
                COMPLIANCE_RECORD_SEED,
                marketplace.as_ref(),
                source_owner.as_ref(),
            ],
            &agroglobaldex,
        );
        require_keys_eq!(
            ctx.accounts.source_compliance.key(),
            expected_src,
            HookError::SourceComplianceMismatch
        );

        // destination_compliance is keyed by the DESTINATION TOKEN ACCOUNT's
        // owner (the wallet), not by the destination token account pubkey.
        // Read the owner field (offset 32, len 32) from the SPL token account
        // raw data.
        let dest_owner = {
            let data = ctx.accounts.destination.try_borrow_data()?;
            require!(data.len() >= 64, HookError::DeserializationFailed);
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&data[32..64]);
            Pubkey::new_from_array(buf)
        };
        let (expected_dest, _) = Pubkey::find_program_address(
            &[
                COMPLIANCE_RECORD_SEED,
                marketplace.as_ref(),
                dest_owner.as_ref(),
            ],
            &agroglobaldex,
        );
        require_keys_eq!(
            ctx.accounts.destination_compliance.key(),
            expected_dest,
            HookError::DestComplianceMismatch
        );

        // ---- Read jurisdiction policy ----------------------------------------
        // The policy account belongs to the agroglobaldex program — Anchor
        // can't deserialize it as a typed `Account<JurisdictionPolicy>` from
        // here, so we parse the discriminator-prefixed Borsh layout manually.
        let policy_data = ctx.accounts.jurisdiction_policy.try_borrow_data()?;
        let blocked_list = parse_blocked_jurisdictions(&policy_data)?;
        drop(policy_data);

        // ---- Source compliance -----------------------------------------------
        let source_data = ctx.accounts.source_compliance.try_borrow_data()?;
        let (src_kyc, src_jur) = parse_compliance_record(&source_data)?;
        drop(source_data);
        require!(src_kyc, HookError::SourceKycNotVerified);
        require!(
            !blocked_list.iter().any(|j| j == &src_jur),
            HookError::SourceJurisdictionBlocked
        );

        // ---- Destination compliance ------------------------------------------
        let dest_data = ctx.accounts.destination_compliance.try_borrow_data()?;
        let (dest_kyc, dest_jur) = parse_compliance_record(&dest_data)?;
        drop(dest_data);
        require!(dest_kyc, HookError::DestKycNotVerified);
        require!(
            !blocked_list.iter().any(|j| j == &dest_jur),
            HookError::DestJurisdictionBlocked
        );

        msg!(
            "ComplianceHook::execute OK src_jur={:?} dest_jur={:?}",
            src_jur,
            dest_jur,
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Token-2022 mint these metas apply to.
    /// CHECK: validated only by being the seed of `extra_account_meta_list`.
    pub mint: UncheckedAccount<'info>,

    /// Per-mint config carrying marketplace pubkey + the agroglobaldex program id.
    #[account(
        init,
        payer = payer,
        space = 8 + HookConfig::INIT_SPACE,
        seeds = [HOOK_CONFIG_SEED, mint.key().as_ref()],
        bump
    )]
    pub hook_config: Account<'info, HookConfig>,

    /// The ExtraAccountMetaList account. Created manually below to avoid the
    /// fixed-size limitation of Anchor `init`.
    /// CHECK: created and initialized in the handler.
    #[account(
        mut,
        seeds = [EXTRA_ACCOUNT_METAS_SEED, mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    /// CHECK: source token account; Token-2022 has already debited it.
    pub source: UncheckedAccount<'info>,

    /// CHECK: the Token-2022 mint.
    pub mint: UncheckedAccount<'info>,

    /// CHECK: destination token account.
    pub destination: UncheckedAccount<'info>,

    /// CHECK: source authority (owner / delegate of the source token account).
    pub owner: UncheckedAccount<'info>,

    /// ExtraAccountMetaList — Token-2022 ALWAYS appends this account at index
    /// 4 of the Execute instruction. Validation that the resolved extra
    /// accounts match the TLV layout happens implicitly via Token-2022's
    /// resolver before we get here.
    /// CHECK: PDA validated by seeds.
    #[account(
        seeds = [EXTRA_ACCOUNT_METAS_SEED, mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [HOOK_CONFIG_SEED, mint.key().as_ref()],
        bump = hook_config.bump,
        has_one = mint @ HookError::HookConfigMintMismatch,
    )]
    pub hook_config: Account<'info, HookConfig>,

    /// The marketplace account — only its key is checked against hook_config.
    /// CHECK: address-checked against `hook_config.marketplace`.
    #[account(address = hook_config.marketplace @ HookError::MarketplaceMismatch)]
    pub marketplace: UncheckedAccount<'info>,

    /// The agroglobaldex program — needed by the TLV resolver so PDA seeds
    /// resolve against it.
    /// CHECK: address-checked against `hook_config.agroglobaldex_program`.
    #[account(address = hook_config.agroglobaldex_program @ HookError::ProgramMismatch)]
    pub agroglobaldex_program: UncheckedAccount<'info>,

    /// JurisdictionPolicy account, owned by `agroglobaldex_program`.
    /// CHECK: deserialized manually inside the handler. Owner-checked.
    #[account(owner = hook_config.agroglobaldex_program @ HookError::AccountOwnerMismatch)]
    pub jurisdiction_policy: UncheckedAccount<'info>,

    /// Source's ComplianceRecord, owned by `agroglobaldex_program`.
    /// CHECK: deserialized manually inside the handler. Owner-checked.
    #[account(owner = hook_config.agroglobaldex_program @ HookError::AccountOwnerMismatch)]
    pub source_compliance: UncheckedAccount<'info>,

    /// Destination's ComplianceRecord, owned by `agroglobaldex_program`.
    /// CHECK: deserialized manually inside the handler. Owner-checked.
    #[account(owner = hook_config.agroglobaldex_program @ HookError::AccountOwnerMismatch)]
    pub destination_compliance: UncheckedAccount<'info>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
#[derive(InitSpace)]
pub struct HookConfig {
    /// Mint this config refers to.
    pub mint: Pubkey,
    /// AgroGlobalDex marketplace account this mint lives under. Used as a
    /// seed component when resolving compliance/policy PDAs.
    pub marketplace: Pubkey,
    /// AgroGlobalDex main program id — used to resolve external PDAs.
    pub agroglobaldex_program: Pubkey,
    pub bump: u8,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[error_code]
pub enum HookError {
    #[msg("Source wallet has not passed KYC")]
    SourceKycNotVerified,
    #[msg("Destination wallet has not passed KYC")]
    DestKycNotVerified,
    #[msg("Source jurisdiction is blocked by the marketplace policy")]
    SourceJurisdictionBlocked,
    #[msg("Destination jurisdiction is blocked by the marketplace policy")]
    DestJurisdictionBlocked,
    #[msg("Failed to construct extra-account-meta entry")]
    AccountMetaConstructionFailed,
    #[msg("Failed to initialize the ExtraAccountMetaList")]
    AccountMetaListInitFailed,
    #[msg("HookConfig mint mismatch")]
    HookConfigMintMismatch,
    #[msg("Marketplace mismatch")]
    MarketplaceMismatch,
    #[msg("AgroGlobalDex program mismatch")]
    ProgramMismatch,
    #[msg("Account is not owned by the agroglobaldex program")]
    AccountOwnerMismatch,
    #[msg("Failed to deserialize the compliance/policy account")]
    DeserializationFailed,
    #[msg("JurisdictionPolicy account is not the expected PDA for this marketplace")]
    JurisdictionPolicyMismatch,
    #[msg("Source ComplianceRecord is not the expected PDA for the source owner")]
    SourceComplianceMismatch,
    #[msg("Destination ComplianceRecord is not the expected PDA for the destination owner")]
    DestComplianceMismatch,
}

// ---------------------------------------------------------------------------
// Manual deserializers
// ---------------------------------------------------------------------------
//
// We cannot import the agroglobaldex crate from here (would create a cyclic
// build dependency). Instead we parse the Borsh-encoded fields we care about
// by their byte offsets relative to the Anchor discriminator (first 8 bytes).
//
// `ComplianceRecord` layout (after the 8-byte Anchor discriminator):
//   wallet            : Pubkey [32]
//   marketplace       : Pubkey [32]
//   kyc_verified      : bool   [1]
//   jurisdiction      : [u8;2] [2]
//   ... (rest ignored)
//
// `JurisdictionPolicy` layout (after the 8-byte Anchor discriminator):
//   marketplace        : Pubkey [32]
//   blocked            : Vec<[u8;2]> -> 4-byte LE length + N * 2 bytes
//   requires_accredited: Vec<[u8;2]> -> 4-byte LE length + N * 2 bytes
//   bump               : u8

const DISCRIMINATOR_LEN: usize = 8;
const PUBKEY_LEN: usize = 32;

fn parse_compliance_record(data: &[u8]) -> Result<(bool, [u8; 2])> {
    // 8 (discr) + 32 (wallet) + 32 (marketplace) = 72
    let kyc_off = DISCRIMINATOR_LEN + PUBKEY_LEN + PUBKEY_LEN;
    let jur_off = kyc_off + 1;
    require!(data.len() >= jur_off + 2, HookError::DeserializationFailed);
    let kyc = data[kyc_off] != 0;
    let jur: [u8; 2] = data[jur_off..jur_off + 2]
        .try_into()
        .map_err(|_| HookError::DeserializationFailed)?;
    Ok((kyc, jur))
}

fn parse_blocked_jurisdictions(data: &[u8]) -> Result<Vec<[u8; 2]>> {
    // skip discriminator + marketplace pubkey
    let mut off = DISCRIMINATOR_LEN + PUBKEY_LEN;
    require!(data.len() >= off + 4, HookError::DeserializationFailed);
    let len = u32::from_le_bytes(
        data[off..off + 4]
            .try_into()
            .map_err(|_| HookError::DeserializationFailed)?,
    ) as usize;
    off += 4;
    require!(
        data.len() >= off + len * 2,
        HookError::DeserializationFailed
    );
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let s = off + i * 2;
        out.push(
            data[s..s + 2]
                .try_into()
                .map_err(|_| HookError::DeserializationFailed)?,
        );
    }
    Ok(out)
}
