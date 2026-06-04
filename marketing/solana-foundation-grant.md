# Solana Foundation Builder Grant — application draft

> **Estado**: Draft v1 listo para copy-paste al form de aplicación.
> Última actualización: alineado con repo en commit posterior a v0.5.0
> (TradeReceipt + lending market + devnet bootstrap idempotente).
>
> **Para enviar**: https://solana.org/grants-funding/

---

## Project name

**AgroGlobalDex** — Compliance-first marketplace for tokenized agricultural
real-world assets on Solana, with on-chain lending against tokenized harvest
collateral.

## One-line pitch

The global, MiCA-aligned venue where producers tokenize their harvest,
collateralize it for instant USDC credit, and investors trade fractional
exposure to real agricultural commodities — all settled on Solana in seconds
with KYC enforced at the token level.

## Project URL

- GitHub: https://github.com/carlospallottini-spec/agroglobaldex
- Web (devnet PoC): https://agroglobaldex.io *[VERIFICAR cuando esté el DNS]*
- Demo desktop app: see GitHub Releases (Windows .exe, macOS .dmg, Linux .AppImage)

## Funding requested

**USD 50,000** (within the Builder Grant range of USD 25k–100k).

Justification for 50k specifically: it covers ~33% of the Sec3 audit
quote (USD ~50k for two-program scope) plus ~10% buffer for the verifiable-
build infrastructure (Docker reproducibility + `solana-verify`). The
remaining audit cost will be matched by the pre-seed round. This is the
smallest amount that meaningfully unblocks the audit gate, which is the
single biggest barrier between us and a mainnet deployment.

## Use of funds (line-item)

| Allocation | Amount | Why this matters for Solana ecosystem |
|---|---|---|
| Sec3 security audit (~33% of total quote) | USD 20k | First audited compliance-hook Token-2022 program for agro RWA — reusable reference for other agritech projects on Solana |
| MLRO + DPO contracting (France) — first 3 months | USD 15k | Unlocks CASP MiCA filing, which becomes the regulatory template every EU-facing Solana RWA project will eventually need |
| Devnet → mainnet ops: Squads multisig, verifiable build (Docker), monitoring (Helius webhooks, Solscan alerts) | USD 10k | Production-grade ops checklist documented publicly in RUNBOOK.md — directly reusable by other Solana RWA teams |
| Producer onboarding pilots (Spain wines + Venezuela meats + Argentina cooperatives travel) | USD 5k | First real on-chain agro production data sets, shareable as case studies for Solana Foundation marketing |

## What is the project? (detailed)

AgroGlobalDex is a Solana program (Anchor 0.31.1) that lets agricultural
producers tokenize their physical output — grains, meat, wine, olive oil,
dairy, fruit — and sells fractional exposure to global investors with
**compliance enforced at the token-transfer level** via SPL Token-2022's
TransferHook extension.

Beyond tokenization, we built two differentiators that no other crypto-agro
project on Solana offers:

**1. Lending market on-chain.** Producers lock their tokenized harvest as
collateral and receive USDC instantly from a community-funded pool. Fixed
APR (snapshot at open), max LTV 50%, liquidation threshold 80%, liquidator
bonus 5%. The full primitive: `init_lending_market`, `set_collateral_config`
(authority oracle relay), `deposit_liquidity`, `open_loan`, `repay_loan`,
`liquidate`. This closes the capital → production → settlement cycle that
banks have monopolized at predatory rates for decades.

**2. Proof-of-trade ledger.** Every `buy_asset` creates a `TradeReceipt`
PDA — structured immutable data (buyer, seller, asset_mint, amount,
unit_price, fees, buyer_jurisdiction snapshot, settled_at, global
trade_index) — not just an NFT with metadata. Auditors, regulators and
counterparties can reconstruct the full supply-chain provenance with
memcmp filters, no JSON parsing required.

## Why Solana (and not another L1)

| Capability we need | Solana | Ethereum L1 | Polygon | Other L1s |
|---|---|---|---|---|
| Settlement < 5 seconds for retail USDC payments | ✓ | ✗ (12s + finality) | ✓ | varies |
| Per-tx cost < $0.001 for small-ticket producer onboarding | ✓ | ✗ ($5-50) | ~$0.01 | varies |
| **Token-2022 TransferHook for on-chain KYC enforcement** | ✓ (native) | ✗ (would need ERC20 wrappers + middleware) | ✗ | ✗ |
| TokenMetadata extension (Phantom/Solflare display) | ✓ | ✗ | ✗ | ✗ |
| Mobile-first wallet ecosystem (Solana Mobile Stack, Phantom Mobile) | ✓ | partial | partial | poor |
| Active RWA developer ecosystem (Topaz, Honey Finance precedents) | ✓ | crowded | minimal | minimal |

The TransferHook bit is the killer one. Our compliance-hook program
intercepts every transfer of an AgroGlobalDex-issued mint and validates:
(a) source wallet has KYC stamped, (b) destination wallet has KYC stamped,
(c) neither jurisdiction is in the on-chain mutable blocklist, (d) PDA
addresses are re-derived from trusted inputs (no account substitution).
This level of compliance enforcement is **only possible on Solana** because
Token-2022 is the only mainstream token standard with native transfer
hooks. On Ethereum, equivalent enforcement requires custom wrapper
contracts that break composability.

## Technical state (verifiable)

- **27 instructions** across 2 Anchor programs:
  - `agroglobaldex` (`G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a`) — marketplace, lending, tokenization, compliance, aggregator
  - `compliance_hook` (`GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL`) — Token-2022 transfer hook with re-derived PDA validation
- **5 asset classes**: Grain, CarbonCredit, HarvestFraction, InvestmentOffering, Commodity (with 9 sectors: Meat, Wine, Oil, Dairy, Fruit, Vegetable, Fiber, GrainSpecial, Other)
- **28 mocha tests** covering happy + sad + boundary paths
- **Internal audit done**: 30 issues identified, 3 criticals + 8 HIGH/MED/LOW already fixed — documented in CHANGELOG
- **11 web pages** + PWA + Android Capacitor + Electron desktop (Windows/macOS/Linux)
- **Build OK**: `anchor build` produces 711 KB `agroglobaldex.so` + 244 KB `compliance_hook.so`
- **Devnet bootstrap script**: `scripts/initialize-devnet.ts` is idempotent — one command sets up marketplace + jurisdiction policy + lending market with safe defaults

## Differentiation vs existing Solana RWA projects

| | AgriDex (UK) | Agrotoken (AR) | Topaz (BR) | **AgroGlobalDex** |
|---|---|---|---|---|
| Solana-based | ✓ | ✗ (multi-chain) | ✓ | ✓ |
| Multi-sector (grain + meat + wine + ...) | ✓ | ✗ (grains only) | partial | ✓ |
| Multi-country | partial | AR only | BR only | ✓ |
| KYC enforced at token-transfer level (TransferHook) | ✗ | ✗ | ✗ | ✓ |
| **On-chain lending against tokenized agro collateral** | ✗ | ✗ | ✗ | ✓ |
| **Proof-of-trade PDA ledger (auditable without parser)** | NFT metadata | ✗ | ✗ | ✓ |
| MiCA-first compliance documentation | ✗ | ✗ | ✗ | ✓ |
| White paper templates + jurisdictional analysis (Francia AMF/ACPR target) | ✗ | ✗ | ✗ | ✓ |
| Yield offerings (MiFID II security tokens) with on-chain settlement receipts | ✗ | ✗ | ✗ | ✓ |
| Aggregates other platforms' tokens (non-competitive positioning) | ✗ | ✗ | ✗ | ✓ |

We are not trying to compete with these projects — our aggregator
intentionally includes them. Our wedge is **EU compliance + lending +
proof-of-trade**, three things none of them prioritize.

## Roadmap (next 12 months, contingent on grant + pre-seed)

**Q3 2026**:
- Sec3 audit complete (the grant unblocks this).
- Devnet public, stable for 30+ days with real producer wallets onboarded.
- 2 LOIs signed (Spain wines + Venezuela meats, both in late negotiation).

**Q4 2026**:
- MLRO + DPO hired in France.
- CASP MiCA notification submitted to AMF/ACPR.
- Mainnet deployment with Squads multisig 2-of-3 + 24h timelock.
- First live producer mint (Vineyard Rioja 2026 harvest fraction).

**Q1 2027**:
- Lending pool seeded with USD 100k+ liquidity.
- First on-chain loan opened (target: olive grove ES + soja AR).
- Reach out to public-sector validators (ICEX España, EIT Food, BPI France)
  — deferred until post-audit so we have credibility to bring.

**Q2 2027**:
- USD 280k GMV / month target (per financial model).
- Series A conversations.

## Founder

**Carlos Pallottini** — Founder & CEO. *[VERIFICAR background completo en
LinkedIn antes de enviar: experiencia previa, dominio agrotech, exposición
crypto / Solana, ubicación, idiomas].*

Single-founder execution to date — all code, design, legal analysis,
pitch materials in the public repo. Hiring CTO + Head of Compliance
post-funding.

## Ecosystem contribution

What we plan to open-source / share with Solana Foundation as part of the
grant deliverable:

1. **Compliance-hook reference implementation** — a Token-2022 TransferHook
   program with PDA re-derivation, jurisdiction policy, KYC enforcement.
   Other RWA / regulated-token projects on Solana can fork this.
2. **MiCA white paper template** — `legal/08-white-paper-template.md` is
   already public, fillable by any Solana team needing CASP notification.
3. **Lending-against-RWA-collateral primitive** — `LendingMarket` +
   `CollateralConfig` + `LoanPosition` accounts, no aggressive liquidation
   logic, conservative defaults — a starter kit for any "borrow against
   tokenized X" project (real estate, invoice factoring, etc.).
4. **Public RUNBOOK.md and SECURITY.md** — operations + disclosure policy
   templates other Solana programs can adapt.

All under MIT (code) and CC-BY-4.0 (docs).

## Risks (honest assessment)

- **Audit may surface critical issues we didn't catch internally.** Our
  internal audit found and fixed 11 of 30 identified issues; some of the
  remaining 19 are out-of-scope for PoC (Squads integration, PermanentDelegate
  for clawback). External audit might reveal genuinely new criticals.
- **MiCA CASP process is slow** (6-12 months in France). Grant alone won't
  shorten this; it's a parallel track.
- **Single-founder bus factor** until first hire closes.
- **Producer adoption is the hardest piece** — productores agro grandes
  don't trust crypto. We mitigate via LOIs signed before mainnet, not
  after.

## Disclaimer

This is a Proof-of-Concept. AgroGlobalDex is **not** authorized as a
Crypto-Asset Service Provider under MiCA at the date of this application.
This document does **not** constitute an offer of crypto-assets or
investment services in any jurisdiction. No yields are guaranteed.
Programs are not yet professionally audited and are not deployed to
Solana mainnet. The figures above are projections and roadmap targets,
not commitments. All factual claims about the codebase are verifiable
in the public GitHub repository at the commit hash submitted with this
application.
