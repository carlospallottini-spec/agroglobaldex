# AgroGlobalDex — Solana Program

Programa Anchor (Rust) que da soporte al marketplace de RWA agro tokenizado.
Soporta tres clases nativas (granos, créditos de carbono, fracciones de cosechas
futuras) **y agrega** tokens emitidos por otras plataformas (Agrotoken, Topaz,
RIPE, Centrifuge…).

> **Estado:** beta production-shaped. Compila contra Anchor 0.31.1 + Solana 2.1
> con un toolchain de Rust suficientemente moderno (`cargo ≥ 1.82`). No auditado.
> Devnet sí, mainnet **no**.

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
├── AssetRegistry[index]                       seeds: asset_registry, marketplace, index_le
│   └── Mint[asset]                           seeds: asset_mint, asset_registry  ← Token-2022 + TransferHook
│       └── MarketplaceListing[seller]        seeds: listing, asset_registry, seller
│           └── escrow ATA                    ATA Token-2022, owner=listing PDA
├── ExternalAssetRegistry[index]               seeds: external_asset, marketplace, index_le
└── ComplianceRecord[wallet]                   seeds: compliance_record, marketplace, wallet
```

## Buildear y deployar

Necesitás un toolchain Rust reciente (cargo ≥ 1.82). El cargo embebido en
Solana CLI ≤ 1.18 NO funciona — usá Solana 2.1+ con cargo del sistema.

```bash
cd solana

anchor keys sync           # primera vez: alinea declare_id! con target/deploy/*-keypair.json
anchor build
./scripts/deploy-devnet.sh # build + deploy + copia el IDL a web 2.0/js/idl/
```

## Decisiones de diseño relevantes

- **Token-2022 con TransferHook**. La única manera de imponer KYC on-chain en
  *cada* transferencia, no sólo en `buy_asset`. El hook se deploya como
  programa separado (TODO).
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
