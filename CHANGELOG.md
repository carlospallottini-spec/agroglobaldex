# CHANGELOG

All notable changes to AgroGlobalDex are documented here. Follows
[Keep a Changelog](https://keepachangelog.com/) + [SemVer](https://semver.org/).

Repository: https://github.com/carlospallottini-spec/agroglobaldex

## [Unreleased] — 0.4.0 (proof-of-trade ledger, inspirado en AgriDex)

### Programa Solana — Feature TradeReceipt

- **Nuevo account `TradeReceipt`** — proof-of-trade inmutable creado en cada
  `buy_asset` y `buy_external_asset`. Seed `[trade_receipt, marketplace,
  trade_index]`. Campos: `marketplace`, `listing`, `asset_mint`, `buyer`,
  `seller`, `source` (Native/External), `amount`, `unit_price_usdc`,
  `gross_usdc`, `fee_usdc`, `buyer_jurisdiction` (snapshot), `trade_index`,
  `settled_at`. Equivalente estructurado del "trade receipt NFT" de
  AgriDex, queryable sin parsear metadata JSON.
- **`Marketplace.trade_count: u64`** — contador global monotónico, derivar
  el siguiente receipt PDA.
- **Evento `TradeReceiptCreated`** — para indexers.

### Programa Solana — Settlement history persistido

- **`AssetRegistry` agrega 3 campos**: `last_settled_epoch: u32`,
  `total_yield_paid_usdc: u64`, `last_settled_at: i64`. Antes
  `settle_investment_offering` solo emitía event; ahora persiste el
  cursor on-chain.
- **`EpochNotMonotonic`** — la ix rechaza epochs que no son estrictamente
  crecientes. Esquema queryable sin indexer.

### Frontend

- **`receipts.html`** nueva — ledger público read-only de comprobantes.
  4 stats (trades, volumen, fees, jurisdicciones), filtro por wallet
  (memcmp), emoji flags por buyer_jurisdiction. Link en nav de las 8
  páginas públicas.
- **`invest.html`** — cada offering card muestra "Último settlement ·
  epoch N · USD X acum." si hubo distribución, "Sin distribuciones aún"
  si nunca se settleó.
- **Client**: `fetchAllTradeReceipts({ buyer, seller })` con memcmp,
  `findTradeReceiptPda`. `buyAsset` y `buyExternalAsset` pasan el
  receipt PDA derivado de `mp.tradeCount`.

### Marketing

- **Slide 9 pitch deck** — tabla comparativa específica vs AgriDex
  (10 capacidades). Claim diferencial: nuestro receipt es PDA tipada,
  no metadata NFT.

### Schema changes (breaking on-chain)

`Marketplace` y `AssetRegistry` crecieron en bytes. Solo afecta
devnet/localnet (mainnet sin deployar todavía). Requiere `--reset` del
validator local. CI / seed script intactos.

---

## [Unreleased] — 0.3.0 (audit pre-mainnet)

### Programa Solana — Critical fixes (audit interno)

- **CRITICAL #3** `register_asset` ahora CPIs a
  `compliance_hook::initialize_extra_account_meta_list`. Sin este fix
  NINGÚN token Token-2022 nativo era transferible — el resolver del
  TransferHook fallaba al no encontrar la lista de cuentas extra.
  Bloqueante absoluto detectado y resuelto.
- **CRITICAL #2** `compliance_hook::execute` valida los 3 PDAs externos
  (`jurisdiction_policy`, `source_compliance`, `destination_compliance`)
  por re-derivación con `find_program_address`, no solo `owner` check.
  Antes un atacante podía pasar la `ComplianceRecord` de otro usuario
  KYC-verified para bypassear su propio KYC. Resuelto.
- **CRITICAL #1** Eliminado `BLOCKED = [KP, IR, SY, CU]` hardcodeado en
  `enforce_compliance_basic`. Ahora `enforce_compliance(_basic)` recibe
  `&JurisdictionPolicy` y lee `blocked` + `requires_accredited` on-chain.
  `update_jurisdiction_policy` aplica realmente (antes era teatro).

### Programa Solana — Hardening (audit HIGH/MED/LOW)

- **HIGH #4** Re-check defensivo `fee_bps <= 10_000` al inicio de
  `buy_asset` y `buy_external_asset`.
- **HIGH #5** `update_listing_price` valida `listing.source_registry ==
  asset_registry.key()`.
- **HIGH #6** `cancel_listing` cierra el escrow ATA via `close_account`
  para evitar rent griefing.
- **HIGH #7** `aggregate_external_asset` setea `active = false` por
  defecto. Curator debe activar explícitamente con `update_external_asset`.
- **MED #8** `buy_asset` y `buy_external_asset` agregan seeds explícitos
  en `asset_registry` / `external_asset` (defensa en profundidad).
- **MED #9** `redeem` agrega `marketplace` account con `!paused` constraint.
  Circuit breaker ahora cubre TODOS los write paths.
- **MED #11** `compliance_hook` valida los 8 bytes de discriminator de
  `ComplianceRecord` y `JurisdictionPolicy` antes de parsear.
- **MED #18** `AssetRegistry` agrega `redeemed_supply: u64`. `redeem`
  lo incrementa por amount burned.
- **LOW #13** `set_compliance_signer` rechaza `Pubkey::default()` y
  `new == old`.

### Programa Solana — Instrucciones nuevas

- **HIGH #15** `revoke_kyc(reason_code)` — compliance signer revoca KYC
  ante sanctions / fraud / regulatory request. Emite `ComplianceRevoked`
  con reason_code (0=manual, 1=sanctions, 2=fraud, 3=regulatory, 4=request).
- **HIGH #16** `settle_investment_offering(epoch, yield_paid_usdc, attestation)`
  — issuer registra on-chain el receipt de cada distribución de yield
  off-chain. Solo aplica a `AssetClass::InvestmentOffering`.
- **HIGH #20** `update_metadata(name, metadata_uri, white_paper_uri)` —
  issuer actualiza metadata BEFORE el primer mint. Después de
  `mint_token`, `frozen_metadata = true` y revierte con `MetadataFrozen`.
- **HIGH #19** `transfer_issuer()` — cede el rol issuer a otra wallet
  KYC-verified. Útil para venta de SPV, rotación de keypair, disaster
  recovery.

Total instrucciones: **21** (era 17). 5 `AssetClass` variants:
`Grain`, `CarbonCredit`, `HarvestFraction`, `InvestmentOffering`,
`Commodity` (9 sectores).

### Tests

- 28 tests mocha (era 11). Cubren happy + sad + boundary de las 21
  instrucciones.
- 5 fuzz tests boundary (audit #23): `total_supply=0`, `white_paper_uri=''`,
  `product_name > 64`, `origin_country` non-ASCII-uppercase,
  `duration_months > 120`.
- **Gap conocido**: `list_asset`/`buy_asset`/`cancel_listing` end-to-end no
  cubiertos. El TransferHook requiere `ComplianceRecord` para el destination
  owner (la listing PDA), que ningún compliance signer puede stampear sin
  comprometer la separación de poderes. Decisión de producto pendiente
  (audit #21-22).

### Frontend

- **Bug fix CRITICAL**: `agroglobaldex-client.js#registerAsset` y `buyAsset`
  estaban rotos — me faltaba pasar `hookConfig`, `extraAccountMetaList`
  y `jurisdictionPolicy` al cambiar el programa. Ningún register o buy
  funcionaba desde la web hasta este fix.
- 4 nuevas funciones wrappeadas: `revokeKyc`, `settleInvestmentOffering`,
  `updateMetadata`, `transferIssuer`.
- Nuevos PDA helpers: `findHookConfigPda`, `findExtraAccountMetaListPda`,
  `findJurisdictionPolicyPda`.
- **`admin.html`** nueva: UI para las 4 ix nuevas con file-upload de
  attestation, role badges (compliance_signer / issuer), error toasts
  contextuales (MetadataFrozen, KycNotVerified).

### Docs

- `RUNBOOK.md` (solana/) — procedimientos operativos: circuit breaker,
  rotación signers, update policy, revocación KYC, treasury withdraw,
  deploy devnet, incident response 3 escenarios (bug crítico, signer
  hack, treasury drain).
- `SECURITY.md` (raíz) — disclosure policy, scope, 11 defensas on-chain
  + 4 off-chain, pendientes pre-mainnet, contactos.
- `MAINNET.md` (raíz) — 7 gates pre-mainnet (audit, CASP, hardening,
  tests, infra, capital, GTM). Camino crítico 9-15 meses.
- `AGENTS.md` (raíz) — orquestador de los 3 agentes IA del proyecto
  + 10 skills de proceso reusados de gstack (Garry Tan / YC, MIT).
- `agents/{solana-dev,marketing-bd,legal-compliance}/SKILL.md` — perfiles
  específicos por dominio.

### Scripts

- `seed-localnet.ts` ahora demuestra TODAS las 21 ix end-to-end (15 steps).
  Incluye `update_metadata`, `settle_investment_offering`, `revoke_kyc +
  re-stamp`, `transfer_issuer` con nueva wallet ES.
- `deploy-devnet.sh` hashea ambos IDLs post-copy y emite SHA-256 en log
  (audit #28).

### Schema changes (breaking on-chain)

`AssetRegistry` agrega `redeemed_supply: u64`. Cuentas existentes en devnet
NO son compatibles — requiere `--reset` del validator. Sin impacto en
mainnet (no deployado todavía).

---

## [0.2.2] — 2026-05-30

### Fixed
- Electron app black screen on navigation: service worker kill-switch
  para context `file://` + `clearStorageData` antes de `createWindow`.

## [0.2.1] — 2026-05-29

### Fixed
- Electron app black screen on navigation: primer intento (incompleto,
  resuelto en 0.2.2).

## [0.2.0] — 2026-05-28

### Added
- CI GitHub Actions: build Windows `.exe` + macOS `.dmg` + Linux
  `.AppImage` en paralelo. Auto-release on tag `v*` o `workflow_dispatch`.
- Electron wrapper hardened: single-instance lock, native menu, window
  state persistence.
- Capacitor scaffold para Android APK.
- Pre-seed materials: pitch deck (14 slides + apéndice A6 Risk Mitigation),
  financials 5y, FAQ, cap table, demo script, grant apps, LOI templates
  España (vinos) + Venezuela (carnes), Colosseum submission.

### Solana
- 17 instrucciones + 2 programas Anchor (`agroglobaldex` + `compliance-hook`).
- 5 AssetClass variants. Commodity 9-sector.
- Token-2022 con TransferHook + MetadataPointer + TokenMetadata.

## [0.1.0] — 2026-04 (initial scaffolding)

- Repo creado.
- Estructura inicial: Solana program scaffold, 9 HTML pages, IDL pipeline.

---

## Licencias

- Código y docs propios: MIT (código) · CC-BY-4.0 (docs)
- `gstack-skills/` contiene archivos derivados de
  [gstack](https://github.com/garrytan/gstack) bajo MIT
