# AgroGlobalDex — Evaluación de preparación pre-auditoría (Solana / Anchor 0.31.1)

*Assessment read-only. Fecha: 2026-06-14. Sirve como base para la auditoría externa
y como lista de issues conocidos a entregar al auditor.*

---

## 0. Estado de remediación (actualizado 2026-06-16)

Tras el assessment se remediaron, **con verificación on-chain en CI (anchor-test)**,
los siguientes hallazgos. Esta sección es el resumen ejecutivo; el detalle original
de cada uno se conserva intacto abajo como contexto para el auditor.

| Hallazgo | Sev | Estado | Commit / nota |
|----------|-----|--------|---------------|
| **H-2** Bypass de acreditación P2P | 🟠 Alto | ✅ Corregido (CI verde) | Hook exige `accredited_investor` en clases restringidas (`AccreditationRequired`) |
| **C-1** `paused` no cubría lending | 🔴 Crítico | ✅ Corregido (CI verde) | `marketplace` + `!paused` en deposit/withdraw/repay/liquidate |
| **M-1** mint sin pause/KYC | 🟡 Medio | ✅ Corregido (CI verde) | `mint_token` exige `!paused` + KYC del issuer |
| **H-1** Precio manual sin oráculo | 🟠 Alto | ✅ Corregido (CI verde) | `require_oracle_for_loans` + `OracleRequired` |
| **M-3** Liquidación de bad debt | 🟡 Medio | ✅ Corregido (CI verde) | Liquidación parcial + `BadDebtRealized` (LPs absorben pro-rata) |
| **C-2** Authority single-wallet | 🔴 Crítico | ✅ Mitigado (procedimiento) | *Go-live gate* multisig+timelock documentado en `SECURITY.md` |
| **H-3** Parseo manual de Pyth | 🟠 Alto | ⏳ Documentado (mitigado) | Ya tiene owner-check + discriminador + feed binding + umbral firmas + staleness + confidence; migración a SDK queda para que el auditor decida |
| M-2, M-4, L-1…L-5 | 🟡/⚪ | ⏳ Pendientes | Ver detalle abajo |

> **Punto para el auditor (M-3 / M-4):** en el path underwater, `liquidate` hace
> `total_borrowed -= debt` (principal + interés), pero en este código `total_borrowed`
> solo contabiliza *principal* (M-4: el interés nunca se suma ahí). Con un préstamo es
> inofensivo; en un pool multi-préstamo resta de más un delta de interés (nivel polvo).
> La alternativa estrictamente conservadora es `-= principal` con pérdida
> `principal - repaid`. **Decisión de redondeo a validar por la auditoría.**

> Nota: el veredicto de la §1 era correcto en su fecha. A 2026-06-16, los bloqueantes
> de **código** críticos/altos están remediados y verificados en CI; el go-live real
> con dinero sigue requiriendo, en paralelo, **auditoría externa + licencia CASP/MiCA**
> (capital + legal), inalterado.

---

## 1. Veredicto de mainnet-readiness (en la fecha del assessment)

**No. Hoy no es seguro mover dinero real en mainnet, y el propio repo lo dice**
(`SECURITY.md` — "PoC/Demo… NO usar con valor real en mainnet"; lista la auditoría
profesional como bloqueante). El código está sorprendentemente bien para un PoC:
muchas defensas reales ya están implementadas (separación authority/compliance_signer,
re-derivación de PDAs en el hook, MINIMUM_LIQUIDITY_SHARES, gate de verification_level
de Pyth, devolución de remanente en liquidación, guard anti-self-liquidation). Pero
quedan defectos concretos que un atacante o un regulador explotaría: (a) **el circuit
breaker no cubre el módulo de lending**; (b) **bypass de acreditación documentado y
real** vía transferencia P2P de tokens-valor; (c) **`authority` es una sola wallet**
sin multisig forzado en código; (d) **parseo manual de Pyth por offsets de bytes**;
(e) **modo de precio manual** sin staleness ni oráculo. Súmese que el producto mueve
**valores regulados (InvestmentOffering = security bajo MiCA/MiFID II)**, lo que exige
licencia/registro CASP en la UE *antes* de operar con público.

---

## 2. Hallazgos por severidad

### CRITICAL

#### C-1 — El circuit breaker (`paused`) NO cubre el módulo de lending
- **Dónde:** `lending.rs` — `repay_loan`, `liquidate`, `deposit_liquidity`,
  `withdraw_liquidity` **no cargan ni leen `marketplace`**. Solo `open_loan` lo
  verifica (`constraint = !marketplace.paused`).
- **Qué está mal:** Si se descubre un exploit en vivo, `set_paused(true)` **no detiene**
  depósitos, retiros, repagos ni liquidaciones. El "kill switch" es ilusorio para el
  subsistema que mueve más capital.
- **Fix:** Añadir `marketplace` con `constraint = !marketplace.paused` a las 4
  instrucciones de lending.

#### C-2 — `authority` es una única wallet; centralización total del tesoro y de los parámetros de riesgo
- **Dónde:** `initialize.rs`, `treasury_withdraw.rs`, `set_collateral_config`,
  `set_paused.rs`. Nada en el código obliga a multisig.
- **Qué está mal:** Una sola clave comprometida puede vaciar el tesoro, fijar precios
  de colateral arbitrarios, des-bloquear jurisdicciones y des-pausar.
- **Fix:** Hacer del Squads multisig un requisito operativo verificado + timelock 24h.
  Separar rol "risk admin" de "treasury admin".

### HIGH

#### H-1 — Modo de precio manual de colateral sin oráculo ni staleness
- **Dónde:** `lending.rs` `set_collateral_config_handler` fija `price_usdc_per_token`
  a mano (`oracle_enabled = false`); en `open_loan`/`liquidate` la verificación de
  staleness se salta cuando `oracle_enabled == false`.
- **Qué está mal:** `authority` puede asignar cualquier precio y prestar/liquidar contra
  él. Vector clásico de oracle-manipulation reducido a "confiar en la authority".
- **Fix:** En mainnet deshabilitar el path manual o restringirlo a multisig con límites;
  exigir `oracle_enabled == true` para colateral con liquidez real.

#### H-2 — Bypass de acreditación por transferencia P2P de tokens-valor
- **Dónde:** El hook valida solo KYC + jurisdicción; *no* lee `accredited_investor`. El
  gate de acreditación solo corre en `buy_asset`/`buy_external_asset`.
- **Qué está mal:** `HarvestFraction`/`InvestmentOffering` son securities. Un acreditado
  compra y transfiere directamente a una wallet KYC'd pero no acreditada; el hook lo
  permite. Violación regulatoria directa.
- **Fix:** Pasar `asset_class`/`requires_accredited` al hook vía ExtraAccountMetaList y
  validar `dest.accredited_investor`; o PermanentDelegate. *(En curso — Agente 1.)*

#### H-3 — Parseo manual de `PriceUpdateV2` por offsets de bytes
- **Dónde:** `oracle.rs` `parse_price_update_v2`, layout y discriminador hardcodeados.
- **Mitigantes presentes (correctos):** owner-check, discriminador, feed_id binding,
  gate verification_level/num_signatures, future-skew, staleness, confidence.
- **Fix:** Migrar a `pyth-solana-receiver-sdk` o congelar versión + tests de regresión
  del layout. Documentar el riesgo como invariante auditable.

### MEDIUM

- **M-1 — `mint_token` no respeta `paused` ni exige KYC del issuer.** Añadir ambos checks.
- **M-2 — `withdraw_liquidity`** puede revertir con `InsufficientLiquidity` cuando el
  valor pedido supera la liquidez ociosa (fondos "atrapados", no robo). Documentar
  invariante o implementar retiro parcial.
- **M-3 — Liquidación de posiciones muy bajo el agua** (`collateral < seize_units`)
  desincentiva al liquidador → bad debt latente. Definir liquidación parcial o backstop.
- **M-4 — Interés lineal** no acumulado en `total_borrowed` entre repagos → asimetría
  temporal en el valor del share de LP. Documentar o acumular globalmente.

### LOW

- **L-1** `network-config.js` apunta a `devnet`; rotar `NETWORK`/`PROGRAM_ID`/
  `COMPLIANCE_HOOK_PROGRAM_ID` y añadir `[programs.mainnet]` a `Anchor.toml`.
- **L-2** Keypairs de programa committeadas (devnet-only); verificar no-reuso en mainnet.
- **L-3** `set_collateral_config` no limpia `oracle_feed_id`/`max_staleness_secs` (stale state).
- **L-4** `check_confidence` divide por `price`; caller garantiza `price > 0` (ok), pero
  la función pública no lo revalida.
- **L-5** Falta `anchor build --verifiable` + `solana-verify` (build reproducible).

---

## 3. Invariantes del protocolo (a documentar para el auditor)

1. `total_shares == MINIMUM_LIQUIDITY_SHARES + Σ(LiquidityProvider.shares)`.
2. `usdc_pool.amount >= total_liquidity` siempre (donaciones directas no alteran shares).
3. `total_liquidity + total_borrowed` solo crece por depósitos/intereses; cada open_loan
   mueve liquidity→borrowed; repay/liquidate lo revierte.
4. No under-collateralization en open: `borrow <= collateral * price * max_ltv_bps / 1e4`,
   con `max_ltv_bps < liquidation_threshold_bps`.
5. Liquidación solo si `debt*1e4 >= collateral_value * liquidation_threshold_bps`.
6. Conservación de colateral en liquidación: `seize + remainder == collateral_amount`.
7. Solo un PriceUpdateV2 verificado (Full o Partial≥5 sigs), feed correcto, no futuro,
   no stale, con confidence aceptable, refresca el precio.
8. Ninguna transferencia de mint nativo sin `kyc_verified` en source+dest y jurisdicción
   permitida; PDAs re-derivados de inputs trusteados.
9. `authority != compliance_signer`.
10. **(ROTO — fijar C-1):** "todo path que mueve fondos respeta `paused`".

---

## 4. Checklist pre-auditoría (priorizado)

**Bloqueantes (código):**
1. [ ] C-1: cobertura de `paused` en deposit/withdraw/repay/liquidate.
2. [ ] H-2: cerrar bypass de acreditación. *(En curso — Agente 1.)*
3. [ ] H-1: restringir/deshabilitar precio manual en mainnet; exigir oráculo.
4. [ ] C-2: forzar `authority` = Squads multisig (verificación post-deploy + runbook + timelock).
5. [ ] M-1: `mint_token` respeta `paused` + KYC del issuer.
6. [ ] M-3: definir liquidación parcial / bad debt.

**Robustez:**
7. [ ] H-3: migrar Pyth a SDK oficial o congelar versión + tests de layout.
8. [ ] L-5: `anchor build --verifiable` + workflow `solana-verify`.
9. [ ] Tests de propiedad/fuzz sobre mate de shares y liquidación.
10. [ ] `SPEC.md` con los 10 invariantes.

**Operacional:**
11. [ ] L-1/L-2: rotar IDs/NETWORK para mainnet; `[programs.mainnet]`; no reusar keypairs.
12. [ ] `ComplianceRecord` válido del `lending_vault` PDA antes de cualquier préstamo.
13. [ ] Runbook de incidente: quién pausa, con qué llaves, en cuánto tiempo.

> Razón de hacerlo **antes** de pagar: un auditor cobra por encontrar lógica sutil, no
> por reportar "el pause no cubre lending". Entregar los bloqueantes ya cerrados
> maximiza el ROI de la auditoría.

---

## 5. Auditoría externa

| Firma | Notas |
|---|---|
| **OtterSec** | Fuerte en Anchor + Token-2022. |
| **Neodyme** | Profundos en runtime Solana; lending/oráculos. |
| **Zellic** | Track record en DeFi y mate de protocolos. |
| **Sec3 / FuzzLand** | Manual + fuzzing automatizado (útil para mate de shares). |
| **Halborn** | Cobertura amplia (mencionada en SECURITY.md). |

- **Alcance:** ~2 programas, ~3-4k líneas Rust con Token-2022 + TransferHook + Pyth +
  lending → mediano-complejo.
- **Costo:** ~USD 30k–80k según firma/profundidad (Token-2022 + hook custom encarece).
- **Lead time:** 4–10 semanas (cola 2–6 sem + revisión 2–4 sem). Reservar slot con antelación.
- **Artefactos:** repo congelado en tag + build reproducible, doc de arquitectura +
  invariantes (sección 3) + threat model, tests, lista de issues conocidos (este informe),
  direcciones/roles/PDAs, dependencias pinneadas.

---

## 6. Camino legal en paralelo (MiCA/CASP, UE)

**No es posterior a la auditoría — corre en paralelo y debe estar resuelto antes del go-live:**

- **InvestmentOffering / HarvestFraction son securities** → caen bajo **MiFID II /
  Prospectus Regulation**: prospecto aprobado o exención válida (colocación privada a
  cualificados), y autorización de empresa de inversión para intermediar.
- **Grain/Commodity/CarbonCredit + marketplace** → probable MiCA: **white paper**
  conforme (`white_paper_uri` ya existe) + **autorización CASP** para operar el venue.
  El **lending de RWA** añade ángulos de crédito/AML.
- **AML/KYC con sustancia:** MLRO + DPO designados, screening OFAC/UN/EU en cada KYC,
  `JurisdictionPolicy` on-chain como control complementario.
- **Por qué precede a operar:** operar sin CASP/MiFID expone a sanciones, prohibición de
  operar y responsabilidad personal de los administradores — riesgo que ninguna auditoría
  de smart contracts mitiga.

**Recomendación:** "auditoría técnica" y "autorización regulatoria" son dos workstreams
paralelos bloqueantes. El go-live mainnet con dinero real requiere *ambos* verdes, además
de los fixes Alta/Crítica de la sección 2.
