# AgroGlobalDex — Colosseum Hackathon Submission

> Para submission al **Solana Breakout Hackathon** (Colosseum).
> URL: https://www.colosseum.org/hackathon
> Categorías target: **DePIN** + **Consumer** + **Infra** + **RWA**.

## TL;DR

**AgroGlobalDex** is the compliance-first marketplace for tokenized agricultural real-world assets (RWAs) on Solana. Two use cases in one app:

1. **Direct tokenization** — farmers tokenize kilos/liters of physical production (grains, meat, wine, oil, dairy, fruits) as SPL Token-2022 with on-chain KYC enforcement.
2. **Global aggregator** — lists tokens from other agro-tokenization platforms (Agrotoken, Topaz, Centrifuge…) in a single cross-country, cross-sector marketplace under one MiCA-aligned UX.

**Why it matters:** USD 12T global agricultural market with <0.05% crypto penetration. Existing platforms are single-sector or single-country. AgroGlobalDex is the first horizontal + MiCA-first venue.

## Live demo

- **URL**: [tu URL en Vercel/Netlify de la web pública apuntando a devnet]
- **GitHub**: https://github.com/carlospallottini-spec/agroglobaldex
- **Demo video** (90s): [link a YouTube/Loom]

## What we built (during/before hackathon period)

### On-chain (2 Anchor programs)

| Program | Program ID | Instructions |
|---|---|---|
| `agroglobaldex` (marketplace) | `G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a` | 17 |
| `compliance_hook` (Transfer Hook) | `GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL` | 2 |

**Asset classes supported (`AssetClass` enum):**
- `Grain` (soy, corn, wheat, other)
- `CarbonCredit` (VCS, Gold Standard, EU ETS)
- `HarvestFraction` (yield-share NFTs)
- `InvestmentOffering` (yield-bearing — security tokens, accredited-only)
- `Commodity` (NEW) — generic bucket with 9 sectors: **Meat / Wine / Oil / Dairy / Fruit / Vegetable / Fiber / GrainSpecial / Other**. Each token = N grams of physical product, with country of origin + vintage year.

### Solana primitives used (showcase)

- ✅ **SPL Token-2022** (not classic SPL) — using extensions:
  - **TransferHook** → CPIs into `compliance_hook` on every transfer, enforcing KYC + jurisdiction policy on source AND destination
  - **MetadataPointer** + **TokenMetadata** → Phantom/Solflare/Backpack render the token with human name ("Viñedo Rioja 2026 Reserva"), symbol (AGRO-WINE, AGRO-MEAT, etc.) and URI
- ✅ **Anchor 0.31.1** with `init-if-needed`, ATA helpers, Interface accounts
- ✅ **USDC settlement** (not SOL) with treasury PDA accumulating protocol fees
- ✅ **Multi-program workspace** (marketplace + compliance-hook deploy independently)
- ✅ **Mutable on-chain `JurisdictionPolicy`** (no redeploy to add/remove sanctioned countries)
- ✅ **Separate `compliance_signer` from `authority`** (multisig-aware, rotatable)
- ✅ **Pause circuit breaker** (`set_paused` — gates all write paths)

### Frontend (PWA + Capacitor Android APK)

- 9 HTML pages with consistent design system (neón verde `#00FF6A` + black)
- Multi-wallet adapter: **Phantom + Solflare + Backpack + Glow** with modal picker
- **Mobile Wallet Adapter (MWA)** helper for Saga / Seed Vault on Android
- PWA installable (`manifest.webmanifest` + service worker + 192/512/maskable icons)
- Capacitor scaffold ready to `npm run build:apk` → Android Play Store-ready APK
- Bottom tab nav for mobile · install prompt on Chrome/Edge/Android
- Mobile-web deep-links to wallet in-app browsers (Phantom/Solflare/Backpack universal-links)

### Tests + validation

- **11 mocha test cases** in `solana/tests/agroglobaldex.ts` covering happy + sad paths:
  - KYC signed by authority (must fail) vs by compliance_signer (must succeed)
  - InvestmentOffering yield > 5000 bps must fail (`InvalidYield`)
  - Paused state gates writes
  - Compliance signer rotation
  - Aggregator SPL + cross-chain
- **End-to-end seed validated** against local validator (`scripts/seed-localnet.ts`):
  - 11 transactions confirmed
  - 2 native assets (Grain + InvestmentOffering "Viñedo Rioja 2026 Reserva")
  - 2 external aggregated (Agrotoken SPL + Centrifuge cross-chain Ethereum)
  - JurisdictionPolicy mutable with 5 blocked countries
  - Token-2022 with TransferHook wired to compliance-hook program

## Innovation highlights (judge-friendly)

1. **First production-shaped MiCA-aligned RWA marketplace** on Solana with on-chain KYC enforcement via Token-2022 TransferHook.
2. **Aggregator pattern** for RWA-agro tokens — turns competitors into distribution partners. Solves the "every agro tokenization platform is an island" problem.
3. **`Commodity` asset class** designed to cover the full agricultural taxonomy in one type: meat, wine, oil, dairy, fruit, vegetable, fiber, special grains. Sector + sub_kind + country + vintage + grams_per_token captures any physical agri-RWA.
4. **Compliance-hook program is reusable** — any other Solana protocol can deploy our hook independently to add KYC + jurisdiction policy to their Token-2022 mints. Building a public good.
5. **Mobile-first**: PWA + Capacitor APK + MWA. Most RWA dApps are desktop-only.

## Real-world traction (during hackathon)

- 🇪🇸 **Spanish DOC wine producer** sitting at the table — LOI in negotiation
- 🇻🇪 **Venezuelan beef rancher** — LOI in negotiation
- ✅ Submitting to Colosseum (this submission)
- ✅ Apply pending to Solana Foundation Grant Tier 1

## Business model (one slide)

| Stream | % | Trigger |
|---|---|---|
| Trading fee | 0.5% | every `buy_asset` / `buy_external_asset` |
| Listing fee | Flat USD (Y2+) | every `register_asset` |
| Aggregator rev-share | % (Y3+) | monthly on referred volume |

## Why we win this hackathon

1. **Real product**, not vapor. Code compiles, deploys, seed runs end-to-end on devnet.
2. **Public good**: compliance-hook program is reusable infrastructure for the Solana ecosystem.
3. **Massive TAM** in an underbuilt vertical (agro RWA).
4. **Regulatory thoughtfulness**: MiCA analysis done, security-token classification analyzed.
5. **Founder ships fast**: 17 instructions, 2 programs, full web, PWA, mobile, legal pack, marketing pack all in the open.

## How to run locally (judges)

```bash
# Toolchain (one-time)
sh -c "$(curl -sSfL https://release.anza.xyz/v3.0.0/install)"
cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked

# Clone
git clone https://github.com/carlospallottini-spec/agroglobaldex
cd agroglobaldex/solana
anchor keys sync
anchor build

# Local validator
ulimit -n 65536          # CRITICAL — default 4096 kills the validator
solana-test-validator --reset --rpc-port 8899 &
solana config set --url http://127.0.0.1:8899
solana airdrop 200

# Deploy both programs
solana program deploy --program-id target/deploy/agroglobaldex-keypair.json target/deploy/agroglobaldex.so
solana program deploy --program-id target/deploy/compliance_hook-keypair.json target/deploy/compliance_hook.so

# Seed end-to-end (creates Marketplace, KYC, registers Grain + InvestmentOffering,
# mints 50 tons, aggregates Agrotoken + Centrifuge, updates jurisdiction policy,
# rotates compliance_signer)
npm install
npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts

# Serve web
cd ..
python3 -m http.server 8000 --directory "web 2.0"
# Open http://localhost:8000/investors.html
```

## Team

- **Carlos Pallottini** — Founder & Solana developer
- (Hiring with the round: 2 eng + 1 BD/regulatory + co-founder)

## Ask post-hackathon

- **Grant prize (any tier)** → funds professional audit
- **Solana Foundation BUIDL grant** (separate track, USD 5-25k) → CASP filing
- **Helius / Triton dev credits** → mainnet RPC infrastructure

## License

MIT (code) · CC-BY-4.0 (docs).

## Contact

**Carlos Pallottini** · carlos@agroglobaldex.io · [LinkedIn] · [Twitter]
