# AgroGlobalDex — Solana/Anchor smart contracts

PoC scaffold for the **AgroGlobalDex** marketplace: a MiCA-aligned venue for
tokenized agricultural RWAs (Real-World Assets) on Solana.

> **WARNING — Proof of Concept.** This code is **not audited** and **must not
> be deployed to mainnet with real value**. It is intended as the starting
> point for a human developer to extend toward a production system.

---

## What it does

Three classes of agro RWA can be issued, listed, traded and redeemed:

| Class             | Backing                                  | Unit       | Redeemable? |
|-------------------|------------------------------------------|------------|-------------|
| `Grain`           | Warehouse receipt (soy / corn / wheat)   | tons       | yes (physical delivery) |
| `CarbonCredit`    | VCS / Gold-Standard / EU-ETS issuance    | kg CO2eq   | yes (retirement) |
| `HarvestFraction` | Notarized land + crop plan, future year  | hectares   | no (cash settlement off-chain) |

Each issuance is represented by an **SPL Token-2022 mint** whose authority is
the program. Every wallet that wants to buy/hold a regulated token must first
have a `ComplianceRecord` PDA stamped by the marketplace's
`compliance_authority`.

---

## Project layout

```
solana/
├── Anchor.toml                # Anchor + cluster + test validator config
├── Cargo.toml                 # Rust workspace
├── package.json               # TS test deps
├── tsconfig.json
├── programs/agroglobaldex/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs             # #[program] entry + declare_id!
│       ├── state.rs           # Marketplace, AssetRegistry, ComplianceRecord, MarketplaceListing, events
│       ├── errors.rs          # AgroError enum
│       └── instructions/
│           ├── mod.rs
│           ├── initialize.rs
│           ├── register_asset.rs
│           ├── mint_token.rs
│           ├── update_kyc.rs   (+ enforce_compliance helper)
│           ├── list_asset.rs
│           ├── buy_asset.rs
│           └── redeem.rs
└── tests/agroglobaldex.ts     # Mocha/Anchor happy-path tests
```

---

## Quick start

Prerequisites: Solana CLI 1.18+, Anchor 0.30.1, Node 18+, Yarn.

```bash
# from solana/
yarn install
anchor build
anchor keys sync     # writes the real program id back into Anchor.toml + lib.rs
anchor test          # spins up a local validator, runs tests/agroglobaldex.ts
```

Deploy to devnet:

```bash
solana config set --url devnet
solana airdrop 2
anchor build
anchor deploy --provider.cluster devnet
```

The placeholder program id in `Anchor.toml` and `declare_id!` is
`AGRoG1obA1Dex11111111111111111111111111111`. **Replace it** with the keypair
output by `anchor keys sync` before deploying.

---

## Architecture

```
                       +----------------------------+
                       |        Marketplace         |  PDA[ "marketplace", authority ]
                       |  authority, fee_bps,       |
                       |  compliance_authority      |
                       +-------------+--------------+
                                     |
              +----------------------+----------------------+
              |                                             |
              v                                             v
  +-----------+-----------+                  +--------------+--------------+
  | ComplianceRecord       |  PDA[ "compliance_record",     |
  | wallet, kyc_verified,  |       marketplace, wallet ]    |
  | jurisdiction,          |                                |
  | accredited_investor    |                                |
  +-----------+-----------+                                  |
              |  (read by buy_asset & future transfer hook)  |
              v                                              v
  +-----------+----------+      mint_authority      +--------+--------+
  |    AssetRegistry     |<-------------------------+   Token-2022    |
  | issuer, mint,        |                          |   Mint (PDA)    |
  | asset_class,         |                          +--------+--------+
  | oracle_attestation,  |                                   |
  | white_paper_uri,     |                                   |
  | total/minted_supply, |                                   |
  | redeemable           |                                   |
  +-----------+----------+                                   |
              |                                              |
              v                                              v
  +-----------+----------+       escrow ATA owned by listing |
  |  MarketplaceListing  +------> +-------------------+      |
  | seller, price, qty,  |        |  Token Account    |<-----+
  | escrow, active       |        +-------------------+
  +----------------------+
```

### Flows

**Issue**
1. Producer calls `register_asset` → creates `AssetRegistry` + Token-2022 mint.
2. Producer calls `mint_token` → tokens land in producer's ATA, registry tracks `minted_supply`.

**Trade (compliance-gated)**
1. Producer calls `list_asset` → tokens move into a listing-owned escrow ATA.
2. Marketplace operator KYCs the buyer via `update_kyc` → stamps `ComplianceRecord`.
3. Buyer calls `buy_asset`:
   - Loads `buyer_compliance` PDA (fails if missing).
   - `enforce_compliance` checks KYC, jurisdiction blocklist, accredited-investor for `HarvestFraction`.
   - Pays seller + marketplace fee in SOL.
   - Tokens transfer from escrow → buyer's ATA.

**Redeem**
1. Holder calls `redeem` → burns tokens, emits `AssetRedeemed`.
2. Off-chain workflow watches the event and physically delivers / retires the asset.

---

## Why SPL Token-2022 (not classic SPL Token)?

Token-2022 ships first-class **mint extensions**, three of which we need (or
will need) for a regulated RWA:

| Extension              | What we use it for                                                                 |
|------------------------|------------------------------------------------------------------------------------|
| **Transfer Hook**      | On-chain CPI into AgroGlobalDex on **every** transfer to verify `ComplianceRecord`. |
| **Default Account State** | Mints can be issued frozen-by-default so accounts must be thawed after KYC.        |
| **Permanent Delegate** | Compliance authority can claw back tokens in case of fraud / sanctions order.      |
| **Metadata Pointer**   | Point at the MiCA white paper without a side-program like Metaplex.                |

The classic SPL token cannot enforce transfer-time compliance on-chain — the
best you can do is freeze accounts manually, which doesn't scale and leaks
states. Token-2022 is therefore the only realistic choice for a MiCA-compliant
RWA on Solana.

> **PoC scope:** the scaffold initializes a vanilla Token-2022 mint and
> enforces compliance only at `buy_asset`. Wiring the actual transfer-hook
> program is a TODO — see roadmap.

---

## Compliance model (MiCA-style)

- `Marketplace.compliance_authority` is a PDA derived from the marketplace; it
  is the only signer authorized to write `ComplianceRecord`s.
- A `ComplianceRecord` carries: `kyc_verified`, `jurisdiction` (ISO-3166-α2),
  `accredited_investor`, `updated_at`.
- `enforce_compliance` (in `update_kyc.rs`) is the single chokepoint reused by
  every regulated instruction. It:
  - Requires KYC verified.
  - Blocks a hard-coded sanctioned list (KP, IR, SY). **Replace** with a
    mutable on-chain list for production.
  - Requires accredited investor status for `HarvestFraction` (yield-bearing).
- `AssetRegistry.white_paper_uri` satisfies MiCA Art. 6 (white paper required
  for asset-referenced tokens). `oracle_attestation` is a SHA-256 hash of the
  off-chain backing certificate.

---

## Design decisions (why these choices)

1. **Token-2022 over classic SPL** — only path to enforceable on-chain compliance.
2. **One mint per asset lot** (not per asset *class*) — keeps fungibility scoped
   to a single warehouse receipt / carbon project, mirrors how the off-chain
   world works (each VCS issuance has its own serial range).
3. **Lamports (SOL) as settlement currency in the PoC** — keeps the scaffold
   short. Production should accept a stablecoin (USDC) via a second token
   account on the listing escrow; trivial extension.
4. **Compliance authority = marketplace authority in PoC** — production should
   split these roles (multi-sig for marketplace ops, separate KYC oracle for
   compliance).
5. **`HarvestFraction` is not redeemable in-protocol** — it represents a
   future cash flow, not a deliverable good; settlement happens off-chain
   when the harvest is realized.

---

## Roadmap to production

- [ ] **Token-2022 Transfer Hook** sibling program that calls back into
      AgroGlobalDex to verify the *recipient*'s `ComplianceRecord` on every
      transfer (not just on `buy_asset`).
- [ ] **Default Account State = Frozen** on mint init, with thaw-on-KYC flow.
- [ ] **Permanent Delegate** wired to the compliance authority for sanctions clawback.
- [ ] **Metadata pointer** extension pointing at an on-chain MiCA metadata account.
- [ ] **Oracle integration** with Pyth / Switchboard for commodity price feeds
      (soy, corn, wheat futures) and FX (EUR/USD), so listings can be quoted
      in stable terms and AMM-style pools can be added.
- [ ] **Stablecoin settlement** (USDC) instead of SOL.
- [ ] **Listing cancel + partial-fill book** (current PoC supports partial
      fills but no explicit cancel — add a `cancel_listing` instruction that
      returns escrowed tokens to seller).
- [ ] **Per-marketplace allowlist/blocklist** stored on-chain, governed by the
      `compliance_authority`, replacing the hard-coded `BLOCKED` constant.
- [ ] **Separate compliance signer** from marketplace operator (multi-sig +
      independent KYC oracle).
- [ ] **Add `index: u64` field to `AssetRegistry`** so PDA seeds can be
      reconstructed deterministically without trusting the client (see
      `mint_token.rs` TODO).
- [ ] **Audit** — Trail of Bits / OtterSec / Halborn before any mainnet move.
- [ ] **Fuzzing** with `cargo-fuzz` and Anchor-friendly property tests.
- [ ] **Off-chain indexer + event-driven settlement service** for redemptions.
- [ ] **Web frontend integration** — replace the static HTML demo under
      `../web 2.0/` with a real Solana wallet adapter UI hitting these programs.

---

## License

MIT — see repository root.

## Disclaimer

This is a **proof of concept**. It has not been audited. The on-chain logic
is intentionally simplified. **Do not deploy to mainnet** or expose to real
user funds without a professional security audit and legal review of the
MiCA / local-jurisdiction implications.
