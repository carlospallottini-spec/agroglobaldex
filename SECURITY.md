# Security policy — AgroGlobalDex

> **Estado al 2026-06-02**: PoC/Demo. Auditoría profesional PENDIENTE.
> NO usar con valor real en mainnet.

## Reporte de vulnerabilidades

### Para hallazgos críticos (loss-of-funds, KYC bypass, drain de tesorería)

Email cifrado a: **security@agroglobaldex** (PGP key publicada en
[`legal/security-pgp-key.txt`](legal/security-pgp-key.txt) — pendiente).

**NO abrir issue público en GitHub para criticals**.

### Para hallazgos no críticos

Abrir issue en GitHub con tag `security` describiendo:
1. Vector de ataque.
2. Impacto estimado.
3. Reproducción mínima (preferentemente test mocha o seed script modificado).
4. Mitigación propuesta (opcional).

## Disclosure policy

- **Critical**: 90 días desde reporte. Si no hay respuesta en 14 días, el
  reporter puede disclose público.
- **High/Medium/Low**: 30-60 días dependiendo de severidad.
- Reporters reconocidos en CHANGELOG del fix (a menos que pidan anonimato).
- **Bounty**: pendiente de definir post-mainnet. PoC no tiene bounty oficial.

## Scope

### En scope
- Programs Anchor: `agroglobaldex` (`G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a`)
- Programs Anchor: `compliance_hook` (`GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL`)
- Frontend `web 2.0/` y `agrochain-electron/`
- CI workflows `.github/workflows/`

### Fuera de scope
- Wallets de terceros (Phantom, Solflare, Backpack — reportar al proveedor).
- Infraestructura de proveedores RPC (Helius, etc).
- Servicios KYC externos (reportar al proveedor).
- Bugs cosméticos o de UX sin impacto en seguridad / fondos.

## Defensas implementadas (PoC)

Detalle completo en [`solana/RUNBOOK.md`](solana/RUNBOOK.md).

### On-chain

1. **Separación de autoridades**: `authority` ≠ `compliance_signer`.
   `compliance_signer` puede stampear KYC pero NO drenar tesorería.
2. **Circuit breaker**: `set_paused(true)` bloquea write paths en 5 minutos
   (audit #9: incluye `redeem`).
3. **JurisdictionPolicy on-chain**: `update_jurisdiction_policy` re-emite la
   lista de bloqueo. No hay lista hardcoded (audit #1 fixed).
4. **TransferHook con PDA validation**: el hook re-deriva `jurisdiction_policy`,
   `source_compliance` y `destination_compliance` desde inputs trusteados
   para evitar account substitution (audit #2 fixed).
5. **Compliance hook init via CPI en register_asset**: sin esto NINGÚN
   token nativo es transferible (audit #3 fixed).
6. **Discriminator check on-chain**: el hook valida los primeros 8 bytes de
   `ComplianceRecord` y `JurisdictionPolicy` antes de parsear (audit #11).
7. **Defensa en profundidad — PDA seeds explícitos**: `buy_asset` y
   `buy_external_asset` re-derivan `asset_registry` / `external_asset` via
   seeds (audit #8).
8. **Fee_bps re-check**: `buy_asset` y `buy_external_asset` revalidan
   `fee_bps <= 10_000` aunque `initialize` ya capea en 1000 (audit #4).
9. **Escrow close en cancel_listing**: cierra el ATA escrow para evitar
   rent griefing (audit #6).
10. **revoke_kyc**: el `compliance_signer` puede revocar KYC ante sanctions
    hit / fraud sin necesidad de autoridad multisig (audit #15).
11. **transfer_issuer con KYC requerido**: rotar el rol issuer requiere que
    el nuevo issuer tenga ComplianceRecord verificado (audit #19).
12. **Acreditación enforced en el TransferHook** (audit #20, ex-limitación):
    al registrar un mint de clase restringida (`HarvestFraction` /
    `InvestmentOffering`) `register_asset` marca `requires_accredited` en el
    `HookConfig` del compliance-hook. En **cada** transferencia Token-2022 el
    hook lee `accredited_investor` del `ComplianceRecord` del **destino** y
    aborta con `AccreditationRequired` si el mint es restringido y el receptor
    no está acreditado. Cierra el bypass de acreditación por re-transferencia
    P2P: un token security ya no puede moverse a un wallet KYC'd-pero-no-
    acreditado fuera del path `buy_asset`. KYC + jurisdicción siguen siendo el
    piso universal para todos los mints. Cobertura: tests 44 (happy) / 45 (sad).

### Off-chain

1. **Disclaimer Demo/PoC obligatorio** en toda comunicación pública.
2. **No promesa de yield garantizado** (securities fraud).
3. **Bloqueo de US persons** sin Reg D filing (a nivel `JurisdictionPolicy`).
4. **OFAC/EU/UN screening** obligatorio en cada KYC stamp.

## Hardening aplicado (pase de auditoría interna v0.6)

Tras un diagnóstico interno se corrigieron, con cobertura de tests on-chain:

- **Oráculo Pyth no forjable**: `refresh_collateral_price` exige
  `verification_level == Full` (o `Partial` con ≥ `MIN_PYTH_SIGNATURES`),
  rechaza precios futuros (skew), valida `price > 0` antes del cálculo de
  confianza, y `liquidate` verifica staleness del oráculo.
- **Liquidación justa**: incauta solo `deuda × (1 + bonus)` y **devuelve el
  excedente al deudor** (antes confiscaba el 100% y `liquidation_bonus_bps`
  no se usaba). Guard `SelfLiquidation` (`borrower != liquidator`).
- **LP con shares**: pool_value = ocioso + prestado, así el **interés se
  distribuye pro-rata** y un LP no puede retirar el principal de otro; lock
  `MINIMUM_LIQUIDITY_SHARES` contra el inflation attack del primer depositante.
- **Separación de poderes**: `initialize` exige `compliance_signer != authority`.
- **Kill-switch completo (audit C-1)**: el flag `paused` ahora cubre **todo el
  módulo de lending** — `deposit_liquidity`, `withdraw_liquidity`, `repay_loan`
  y `liquidate` revierten `Paused` cuando el mercado está pausado (antes solo lo
  hacía `open_loan`). `set_paused(true)` es un freno real ante un exploit en vivo.
- **Minteo con compliance (audit M-1)**: `mint_token` exige marketplace
  no-pausado y **KYC vigente del issuer** (`issuer_compliance.kyc_verified`); un
  issuer con KYC revocado o un mercado pausado no puede mintear.
- **Oráculo exigible (audit H-1)**: el `LendingMarket` tiene
  `require_oracle_for_loans` (setter authority-only
  `set_lending_oracle_requirement`); cuando está activo, `open_loan`/`liquidate`
  exigen colateral con oráculo (`OracleRequired`), cerrando el vector de precio
  manual manipulable en mainnet.
- **CI**: gates de `clippy`, `cargo fmt --check`, `cargo audit` (report-only) y
  un **IDL drift guard** (el IDL del frontend no puede divergir del programa).

### Limitaciones conocidas (documentadas)

- **Acreditación bypasseable por transferencia P2P** — ✅ **CORREGIDO**
  (ver "Defensas implementadas" #12). El TransferHook ahora enforced
  `accredited_investor` en el destino para clases restringidas
  (`HarvestFraction` / `InvestmentOffering`) vía el flag `requires_accredited`
  del `HookConfig` (seteado por `register_asset`). Una re-transferencia P2P de
  un token security a un wallet KYC'd-no-acreditado revierte con
  `AccreditationRequired`. La extensión PermanentDelegate sigue pendiente solo
  para clawback de wallets sancionadas post-mint (pre-mainnet #3), no para la
  acreditación.
- **Precio manual** (`set_collateral_config`) confía en `authority` sin
  staleness — ✅ **MITIGADO** (audit H-1): activando `require_oracle_for_loans`
  el mercado rechaza colateral sin oráculo en `open_loan`/`liquidate`. **Es un
  requisito del go-live de mainnet** (ver gate abajo): el path manual queda solo
  para devnet/PoC.

## Gestión de llaves — DEVNET vs MAINNET

- Las keypairs en `solana/target/deploy/*.json` son **DEVNET-ONLY**,
  committeadas a propósito para program IDs deterministas en CI/devnet. La
  dirección es pública; su única sensibilidad es ser *upgrade authority*.
- **Mainnet**: NUNCA reusarlas. Generar keys frescas **offline** y pasar el
  upgrade authority a un **Squads multisig** con
  [`solana/scripts/deploy-mainnet.sh`](solana/scripts/deploy-mainnet.sh).

### Go-live gate de autoridad y timelock (audit C-2) — BLOQUEANTE de mainnet

El programa es agnóstico a quién sea `authority`: acepta una hot wallet igual
que un multisig. Por eso, la separación de poderes se garantiza por
**procedimiento verificado**, no solo por código. Antes de habilitar dinero
real, **todos** estos puntos deben estar verdes y verificados on-chain:

1. **`upgrade authority` (ambos programas) = Squads multisig** (lo hace
   `deploy-mainnet.sh`). Verificar:
   `solana program show <PROGRAM_ID>` → *Upgrade Authority* == vault Squads.
2. **`marketplace.authority` = Squads multisig** (NO una hot wallet). Controla
   tesorería (`treasury_withdraw`), parámetros de riesgo (`set_collateral_config`,
   `set_lending_oracle_requirement`), `set_paused` y políticas de jurisdicción.
   Una sola clave comprometida aquí drena el tesoro y fija precios → debe ser
   multisig **2-de-3 como mínimo**.
3. **`compliance_signer` = clave SEPARADA** de `authority` (ya forzado on-chain
   por `initialize`), idealmente otro firmante/HSM.
4. **Timelock de 24 h** sobre las acciones del multisig que cambian parámetros
   de riesgo o mueven tesorería (config del Squads, fuera de la cadena de este
   programa), para dar ventana de reacción ante una propuesta maliciosa.
5. **`require_oracle_for_loans = true`** en el `LendingMarket` (audit H-1): sin
   esto el path de precio manual sigue activo.
6. **Runbook de incidente** escrito: quién pausa (`set_paused`), con qué llaves
   del multisig, y en cuánto tiempo. El kill-switch (audit C-1) ya cubre todo el
   lending; lo que falta es el procedimiento humano.

> Verificación post-deploy (mínima): `solana program show` para ambos IDs, y un
> fetch del `Marketplace` + `LendingMarket` confirmando `authority` /
> `compliance_signer` / `require_oracle_for_loans`. Mientras `marketplace.authority`
> sea una hot wallet, el sistema **no** está listo para dinero real aunque la
> upgrade authority ya sea multisig.

## Pendiente pre-mainnet

1. **Auditoría profesional** (Sec3 / OtterSec / Halborn) — bloqueante.
2. **Squads multisig 2-of-3** para `authority` con timelock 24h (audit C-2) —
   ver el *go-live gate de autoridad y timelock* arriba; es procedimiento
   verificado on-chain, bloqueante.
3. **PermanentDelegate Token-2022 extension** para clawback de tokens en
   wallets sancionadas post-mint (out of PoC scope).
4. **`anchor build --verifiable`** + `solana-verify` workflow (audit #26).
5. **Oracle attestation Ed25519** verificada on-chain (audit #12).
6. **MLRO + DPO** designados con sustancia local en Francia (compliance
   requirement, no técnica).

## Contacto

| Caso | Email | Tiempo de respuesta |
|---|---|---|
| Security incident en curso | security@agroglobaldex | <2h |
| Vulnerabilidad responsable | security@agroglobaldex | <72h |
| Compliance / regulatory | legal@agroglobaldex | <5d |
| General | hello@agroglobaldex | best effort |

> Los emails arriba son placeholders mientras el SPV no esté constituido.
> Para reportes urgentes durante PoC, contactar al founder en LinkedIn
> (Carlos Pallottini).
