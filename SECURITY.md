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
- **CI**: gates de `clippy`, `cargo fmt --check`, `cargo audit` (report-only) y
  un **IDL drift guard** (el IDL del frontend no puede divergir del programa).

### Limitaciones conocidas (documentadas)

- **Acreditación bypasseable por transferencia P2P**: el TransferHook valida
  KYC + jurisdicción pero **no** `accredited_investor`. Un token
  `InvestmentOffering`/`HarvestFraction` puede re-transferirse a un wallet
  KYC'd no acreditado. Fix futuro: pasar el `asset_class` al hook o usar la
  PermanentDelegate extension.
- **Precio manual** (`set_collateral_config`) confía en `authority` sin
  staleness — en producción usar siempre oráculo + multisig.

## Gestión de llaves — DEVNET vs MAINNET

- Las keypairs en `solana/target/deploy/*.json` son **DEVNET-ONLY**,
  committeadas a propósito para program IDs deterministas en CI/devnet. La
  dirección es pública; su única sensibilidad es ser *upgrade authority*.
- **Mainnet**: NUNCA reusarlas. Generar keys frescas **offline** y pasar el
  upgrade authority a un **Squads multisig** con
  [`solana/scripts/deploy-mainnet.sh`](solana/scripts/deploy-mainnet.sh).

## Pendiente pre-mainnet

1. **Auditoría profesional** (Sec3 / OtterSec / Halborn) — bloqueante.
2. **Squads multisig 2-of-3** para `authority` con timelock 24h (audit #10).
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
