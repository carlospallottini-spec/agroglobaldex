# AgroGlobalDex — Solana Program

Programa Anchor (Rust) que da soporte al marketplace de RWA agro tokenizado.
Soporta tres clases nativas (granos, créditos de carbono, fracciones de cosechas
futuras) **y agrega** tokens emitidos por otras plataformas (Agrotoken, Topaz,
RIPE, Centrifuge…).

> **Estado:** beta production-shaped, **buildeado y verificado end-to-end** en
> un validator local con seed completo (init → KYC → register Grain →
> mintToken → aggregate Agrotoken-like + Centrifuge cross-chain). Compila contra
> Anchor 0.31.1 + Solana 3.0.0 + platform-tools v1.54 (cargo 1.89, rustc 1.89).
> No auditado. Devnet/local sí, mainnet **no**.

## Stack

- `anchor-lang` / `anchor-spl` 0.31.1
- `spl-token-2022` 6.x (transfer hook extension)
- USDC SPL clásico para settlement (mint configurable por marketplace)

## Estructura

```
solana/
├── Anchor.toml
├── Cargo.toml                       # workspace
├── package.json                     # tests TS (mocha)
├── tsconfig.json
├── programs/
│   └── agroglobaldex/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs               # entry + #[program]
│           ├── state.rs             # cuentas, enums, eventos
│           ├── errors.rs            # AgroError
│           └── instructions/
│               ├── mod.rs
│               ├── initialize.rs    # marketplace + treasury USDC ATA
│               ├── register_asset.rs# AssetRegistry + mint Token-2022 con TransferHook
│               ├── mint_token.rs    # issuer-only, capped a total_supply
│               ├── update_kyc.rs    # KYC + jurisdicción + accredited
│               ├── list_asset.rs    # listado nativo en USDC
│               ├── buy_asset.rs     # compra en USDC con fee al treasury
│               ├── redeem.rs        # burn → emit AssetRedeemed
│               └── aggregate.rs     # aggregator (SPL Solana o cross-chain)
├── scripts/
│   └── deploy-devnet.sh             # anchor keys sync + build + deploy + IDL copy
└── tests/
    └── agroglobaldex.ts             # happy path
```

## Diagrama de cuentas (PDA tree)

```
Marketplace[authority]                       seeds: marketplace, authority
├── ComplianceAuthority                       seeds: compliance_authority, marketplace
├── Treasury                                   seeds: treasury, marketplace
│   └── treasury_usdc_ata                     ATA USDC, owner=treasury
├── JurisdictionPolicy                         seeds: jurisdiction_policy, marketplace  ← mutable blocklist
├── AssetRegistry[index]                       seeds: asset_registry, marketplace, index_le
│   └── Mint[asset]                           seeds: asset_mint, asset_registry  ← Token-2022 + TransferHook
│       └── MarketplaceListing[seller]        seeds: listing, asset_registry, seller
│           └── escrow ATA                    ATA Token-2022, owner=listing PDA
├── ExternalAssetRegistry[index]               seeds: external_asset, marketplace, index_le
└── ComplianceRecord[wallet]                   seeds: compliance_record, marketplace, wallet

compliance-hook program (SEPARATE, GFFp2bTh…CL8tL)
├── HookConfig[mint]                           seeds: hook_config, mint
└── ExtraAccountMetaList[mint]                 seeds: extra-account-metas, mint  ← Token-2022 TLV
```

## Buildear y deployar

Toolchain validado: **Solana 3.0.0 + Anchor 0.31.1 + platform-tools v1.54**
(cargo 1.89, rustc 1.89). Versiones más viejas (Solana ≤ 2.2, platform-tools
≤ 1.53) fallan al compilar deps con `edition2024`.

> **Importante: `ulimit -n 65536`** antes de correr el validator local.
> El default (4096) hace que el validator muera silenciosamente en
> "Waiting for fees to stabilize 3...". Esto se documentó tras corregir
> el problema en este sandbox.

```bash
# Instalar Solana 3.0
sh -c "$(curl -sSfL https://release.anza.xyz/v3.0.0/install)"
# Instalar Anchor 0.31.1
cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked

cd solana

anchor keys sync           # primera vez: alinea declare_id! con target/deploy/*-keypair.json
anchor build               # genera 2 .so + IDLs en target/

# Local validator (rápido para probar todo end-to-end)
ulimit -n 65536            # CRÍTICO — sin esto el validator muere
solana-test-validator --reset --rpc-port 8899 --faucet-port 9900 &
solana config set --url http://127.0.0.1:8899
solana airdrop 200
solana program deploy --program-id target/deploy/agroglobaldex-keypair.json target/deploy/agroglobaldex.so
solana program deploy --program-id target/deploy/compliance_hook-keypair.json target/deploy/compliance_hook.so
npm install
npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts

# Deploy a devnet (requiere wallet con SOL devnet)
./scripts/deploy-devnet.sh
```

### Seed run validado (output REAL del seed-localnet.ts contra validator local)

```
Program ID         : G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a
Hook program ID    : GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL
Authority          : BCFbE1nawxJxPTce48w2jMsgT8Dsuoiz7jesvwseNbZU
Compliance signer  : vdDXdseynoQch2HYwK73cHHPPDvvMHCDZELy3EharEa  (SEPARATE)

[1]  fake USDC mint creado
[2]  initialize marketplace (authority y compliance_signer separados) ✓
[3]  init_jurisdiction_policy (defaults KP/IR/SY/CU) ✓
[4]  update_kyc firmado por compliance_signer (no authority) ✓
[5]  register_asset Grain (Soja 100 ton, mint Token-2022 + TransferHook → compliance-hook real) ✓
[6]  mint_token 50 ton ✓
[7]  register_asset InvestmentOffering "Viñedo Rioja 2026 Reserva" — 12 meses, 9% ROI ✓
[8]  aggregate_external_asset SPL (Agrotoken) ✓
[9]  aggregate_external_asset cross-chain (Centrifuge / Ethereum) ✓
[10] update_jurisdiction_policy (agregar AF al blocklist) ✓
[11] set_compliance_signer (rotate) ✓

[VERIFY] Estado on-chain:
  Marketplace assetCount=2, externalAssetCount=2, feeBps=50
  JurisdictionPolicy blocked: KP, IR, SY, CU, AF
  AssetRegistry: "Viñedo Rioja 2026 Reserva" (investmentOffering) + "Soja AR 2026 Q1" (grain)
  ExternalAssetRegistry: Agrotoken + Centrifuge
  ComplianceRecord: 1
  Marketplace.complianceSigner ROTADO a la nueva pubkey ✓
```

## Decisiones de diseño relevantes

- **compliance-hook como programa Anchor SEPARADO** (`programs/compliance-hook/`).
  Implementa SPL Transfer Hook Interface y se ejecuta en cada transfer de
  Token-2022 para verificar KYC + jurisdicción de source y destination
  contra `JurisdictionPolicy`. Sin esto, cualquier holder podía transferir
  libremente tras el primer mint — el hook cierra ese agujero.
- **Token-2022 con TransferHook**. Único mecanismo para enforce compliance
  en *cada* transferencia, no sólo en `buy_asset`.
- **`compliance_signer` separado de `authority`**. Service account para
  stampar KYC sin tocar treasury. Rotable via `set_compliance_signer`.
- **`JurisdictionPolicy` mutable on-chain**. Sin redeploy para agregar/quitar
  países bloqueados.
- **USDC para settlement** (no SOL). `price_usdc` en base units (6 decimales).
  Fee va a un PDA treasury en USDC.
- **Index reconstruible**. `AssetRegistry` y `ExternalAssetRegistry` guardan
  su `index: u64` y las seeds incluyen `index.to_le_bytes()` para que cualquier
  cliente reconstruya el PDA sin depender de input no validado.
- **Una mint por lote**. Cada lote físico es su propia mint. Sin fungibilidad
  entre vintages.
- **ListingSource discriminator**. Native vs External comparten la misma
  cuenta `MarketplaceListing` con campo `source`. En el PoC `buy_asset` sólo
  soporta `Native`; los externals son display-only on-chain.
- **`enforce_compliance` único**. Helper compartido entre `buy_asset` y el
  futuro transfer hook. Bloquea KP/IR/SY y exige `accredited_investor` para
  `HarvestFraction`.

## Roadmap a mainnet (TODO crítico)

1. **Compliance-Hook program separado**. Hoy `register_asset.rs` ya wirea la
   TransferHook extension al mint, pero apuntando al propio programa
   (placeholder). Deployar un programa real con `execute(amount)` +
   `initialize_extra_account_meta_list`.
2. **Buy externos SPL**. Soportar `ListingSource::External` en `buy_asset.rs`
   con un adapter que respete la compliance del mint externo.
3. **Cancel listing**. Hoy un listing se cierra solo al agotarse `remaining`.
4. **Treasury withdraw**. Instrucción para que la authority retire fees.
5. **Multisig + oráculo KYC**. Hoy `compliance_authority == marketplace.authority`.
   Producción debería separar y soportar un firmante por jurisdicción.
6. **Pyth/Switchboard** para precios spot de commodities.
7. **Audit** (Trail of Bits / OtterSec / Halborn) antes de mainnet.
8. **MiCA white paper aprobado** por la NCA — ver
   `legal/05-jurisdictional-strategy.md` (recomendación: Francia AMF).

## Tests

```bash
yarn install
anchor test
```

Cobertura mínima: happy path init → register → mint → kyc → list → buy.
Falta cobertura de transfer hook y aggregator.

## Disclaimer

PoC. No auditado. No usar en mainnet con valor real. Cualquier deployment
productivo requiere audit profesional, multisig de authority, oráculos KYC
reales y autorización CASP bajo MiCA — ver carpeta `legal/` en el root.
