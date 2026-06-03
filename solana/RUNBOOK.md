# RUNBOOK — AgroGlobalDex on-chain operations

Procedimientos operativos para los **2 programas Anchor**
(`agroglobaldex` + `compliance_hook`) deployados en Solana.

**Aplica a**: devnet pública (`devnet`) y mainnet beta una vez que se complete
la auditoría profesional. Cualquier acción destructiva en mainnet requiere
checklist `careful` (ver `gstack-skills/careful/SKILL.md`).

> **Estado al 2026-06-02**: NO deployado a mainnet. Auditoría pendiente. NO
> usar con valor real.

---

## 0. Roles & wallets

| Rol | Wallet | Capacidad | Quién es |
|---|---|---|---|
| `authority` | hardware wallet o multisig Squads | Update fee_bps, pause, rotate compliance signer, withdraw treasury, update jurisdiction policy, update_external_asset | Founder + co-founders (multisig 2-of-3 a partir de mainnet) |
| `compliance_signer` | hot wallet en servicio KYC | Stamp `ComplianceRecord`, `revoke_kyc` | Servicio KYC (Sumsub / Veriff webhook) |
| `issuer` | wallet del productor | `register_asset`, `mint_token`, `settle_investment_offering`, `update_metadata` (pre-mint) | Cada bodega / frigorífico / cooperativa por separado |
| `holder` | wallet del inversor / comprador | `buy_asset`, `redeem`, listing CRUD | Cualquier wallet con `ComplianceRecord` KYC-verified |
| `curator` | igual a `authority` | `aggregate_external_asset`, `update_external_asset` | Founder team |

**Reglas duras**:
1. `authority` NUNCA debe ser igual a `compliance_signer` (separación de
   poderes — un compromise del KYC service no debe poder drenar tesorería).
2. `authority` debe ser Squads multisig en mainnet (single-sig en devnet es
   tolerable para desarrollo).
3. Hardware wallet o passphrase-protected keypair para `authority`. **NUNCA**
   keypair JSON sin cifrar para mainnet.

---

## 1. Circuit breaker — pausar marketplace

**Cuándo**: bug crítico detectado, hack en curso, regulator request, error de
deploy.

**Efecto on-chain**: `marketplace.paused = true` bloquea:
- `register_asset`, `buy_asset`, `buy_external_asset`, `list_asset`,
  `update_listing_price`, `cancel_listing` (vía constraint), `update_metadata`,
  `aggregate_external_asset`, `redeem`.

**Efecto on-chain — NO bloquea**: `update_kyc`, `revoke_kyc`,
`set_compliance_signer`, `set_paused` (obvio), `treasury_withdraw`. El
authority sigue operativo para resolver el incidente.

```bash
# Pausar
anchor run pause --provider.cluster devnet

# Resumir
anchor run resume --provider.cluster devnet
```

**SLA**: 5 minutos desde detección a `paused=true`. La cuenta `authority`
debe poder firmar desde cualquier laptop del equipo con el hardware wallet.

---

## 2. Rotación de `compliance_signer`

**Cuándo**: compromise del servicio KYC, salida de equipo, rotación periódica
(recomendado cada 90 días).

**Procedimiento**:

1. Generar nuevo keypair en el HSM del servicio KYC nuevo.
2. Backup encriptado del keypair viejo (en caso de necesidad forense).
3. `authority` firma `set_compliance_signer(new_signer)`.
4. Verificar evento `ComplianceSignerRotated`.
5. Confirmar en frontend: las nuevas KYC stamps llegan firmadas por el nuevo
   signer.

**Defensa nuestra**: el handler rechaza `new_signer == Pubkey::default()` y
`new_signer == old_signer` (issue #13 del audit).

---

## 3. Update de `JurisdictionPolicy`

**Cuándo**: cambio de sanctions list (OFAC add/remove), nuevo emisión de
regulator, decisión de board de habilitar nueva jurisdicción.

**Procedimiento**:

1. `authority` firma `update_jurisdiction_policy(blocked, requires_accredited)`.
2. El cambio aplica inmediatamente a **TODOS** los `buy_asset` y
   `buy_external_asset` siguientes (validación on-chain).
3. El cambio aplica también a **TODOS** los transfers Token-2022 que pasan
   por el `compliance_hook::execute` (el hook lee el policy en cada transfer).
4. Confirmar evento `JurisdictionPolicyUpdated`.

**Importante**: a partir del fix del issue #1 (audit), NO existe lista
hardcoded. Lo que está on-chain ES lo que aplica.

---

## 4. Revocación de KYC (sanctions hit / fraud / regulatory request)

**Cuándo**: sanctions screening positivo, alerta de monitoring, regulator
order.

**Procedimiento**:

```
compliance_signer firma revoke_kyc(reason_code)
  reason_code:
    0 = manual
    1 = sanctions hit (OFAC / EU consolidated list / UN)
    2 = fraud detection
    3 = regulatory request
    4 = wallet request (self-revocation)
```

**Efecto on-chain**: la wallet revocada NO puede más:
- `buy_asset` / `buy_external_asset` (enforce_compliance falla)
- recibir tokens Token-2022 (compliance_hook::execute falla)

**Efecto on-chain — NO afecta**: tokens ya en wallet (no hay clawback en
PoC). Para clawback se requiere PermanentDelegate extension (roadmap).

**SLA**: 1 minuto desde el alert al ack on-chain.

---

## 5. Treasury withdraw

**Cuándo**: distribución de protocol fees a tesorería operativa.

**Procedimiento**:

1. Confirmar balance del treasury ATA via `solana balance`.
2. `authority` firma `treasury_withdraw(amount, destination)`.
3. Verificar evento `TreasuryWithdrawn`.
4. Conciliar contra ledger contable del SPV.

**Recomendación**: en mainnet, el `authority` debe ser Squads multisig 2-of-3
con timelock de 24h (audit #10). Esto bloquea drenajes de un solo signer
compromised.

---

## 6. Deploy a devnet pública (procedimiento desde cero)

```bash
export PATH="$HOME/.cargo/bin:$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd solana/

# 1. anchor keys sync (asegura declare_id consistente)
anchor keys sync

# 2. Build verificable (audit #26)
anchor build --verifiable

# 3. Capturar IDL hash para comparar contra commit
sha256sum target/idl/agroglobaldex.json target/idl/compliance_hook.json | tee IDL.lock

# 4. Configurar devnet
solana config set --url https://api.devnet.solana.com

# 5. Deploy ambos programas
solana program deploy --program-id target/deploy/agroglobaldex-keypair.json target/deploy/agroglobaldex.so
solana program deploy --program-id target/deploy/compliance_hook-keypair.json target/deploy/compliance_hook.so

# 6. Verificar programs deployadas
solana program show G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a
solana program show GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL

# 7. Copiar IDL al frontend
cp target/idl/agroglobaldex.json "../web 2.0/js/idl/agroglobaldex.json"
cp target/idl/compliance_hook.json "../web 2.0/js/idl/compliance_hook.json"

# 8. Inicializar marketplace
anchor run initialize-devnet
```

**Verificable build (audit #25, #26)**: `anchor build --verifiable` usa Docker
con toolchain fijo para que `solana-verify` pueda chequear que el .so
deployado corresponde al commit publicado. Esto es prerequisito para
auditoría profesional.

---

## 7. Incident response — runbook por categoría

### A. Bug crítico detectado en buy_asset

1. `authority` ejecuta `set_paused(true)`. SLA 5 min.
2. Publicar comunicado en Discord/Twitter con timestamp on-chain.
3. Reproducir on-chain en localnet con seed script.
4. Patch + tests + `anchor build --verifiable`.
5. `authority` ejecuta `solana program deploy --upgrade-authority` para el
   bugfix. Verificar IDL no cambió (o publicar IDL nuevo si la API cambió).
6. `set_paused(false)`. Comunicado de all-clear.

### B. Compliance signer comprometido (KYC service hack)

1. `authority` ejecuta `set_compliance_signer(temporary_authority)` para
   bloquear nuevas stamps maliciosas.
2. Para CADA wallet stampeada en la ventana de exposición (last_updated_at >
   t_compromise), `revoke_kyc(reason_code=3)`.
3. Re-stamp manualmente con el nuevo signer las wallets legítimas.
4. Post-mortem público dentro de 72h.

### C. Drain de tesorería (single-sig compromised)

1. `set_paused(true)` — bloquea nuevos buys (no genera más fees).
2. Migrar el `authority` a Squads multisig URGENTE (`set_compliance_signer`
   no es suficiente — necesitás cambiar el authority, que requiere redeploy
   con nuevo authority key).
3. Considerar fork del marketplace si los fondos ya están drenados.

---

## 7b. Lending — setup y operaciones

El módulo de crédito colateralizado requiere un setup inicial y monitoreo
continuo del health de los préstamos.

### Setup inicial (una vez)

```
1. authority → init_lending_market(apr_bps, max_ltv_bps,
   liquidation_threshold_bps, liquidation_bonus_bps)
   Recomendado conservador: apr=1200 (12%), max_ltv=5000 (50%),
   liq_threshold=8000 (80%), liq_bonus=500 (5%).

2. compliance_signer → update_kyc(vault_authority_pda, kyc=true,
   jurisdiction="XX" interno, accredited=true)
   CRÍTICO: el vault PDA recibe colateral Token-2022, y el TransferHook
   exige ComplianceRecord en el destino. Sin este paso, NINGÚN open_loan
   funciona. El `vault_authority` PDA = [b"lending_vault", lending_market].

3. authority → set_collateral_config(asset_registry, price_usdc_per_token,
   enabled=true) por cada asset aceptado como colateral.

4. Fondear el pool: cualquiera → deposit_liquidity(amount). El treasury
   o un LP institucional aporta el USDC inicial.
```

### Operación continua

**Actualizar precios de colateral** (oráculo):
- `authority → set_collateral_config(asset, nuevo_precio, true)` cada vez
  que el spot price del commodity se mueve materialmente. Producción debe
  wirear un price feed firmado (audit #12) en lugar de la authority manual.

**Monitorear health de préstamos**:
- Indexar el event `LoanOpened`. Para cada loan activo, calcular
  `LTV = (principal + accrued_interest) / (collateral_amount * price)`.
- Si `LTV > liquidation_threshold`, el loan es liquidable.
- Alertar al equipo + permitir que liquidators (KYC'd) ejecuten `liquidate`.

**Liquidación**:
- Cualquier wallet KYC-verified puede llamar `liquidate(loan)` si el loan
  superó el threshold. El liquidator paga la deuda (principal+interés) al
  pool y se lleva TODO el colateral (el bonus es implícito: el colateral
  vale más que la deuda al threshold).
- En un crash de precio del commodity, bajá `set_collateral_config` ANTES
  de que los loans queden bajo-colateralizados, o el pool come pérdidas.

### Riesgos del módulo lending

| Riesgo | Mitigación |
|---|---|
| Precio de colateral stale → loan bajo-colateralizado | Price feed firmado + heartbeat; pausar si stale |
| Pool sin liquidez para nuevos loans | `deposit_liquidity` de LPs; cap de `total_borrowed` |
| Oráculo comprometido infla precio → over-borrow | Multi-sig de oráculos (3-de-5) en mainnet |
| Liquidación no rentable (gas > bonus) | Subir `liquidation_bonus_bps` |
| Vault PDA sin KYC → loans rotos | Checklist de setup paso 2 (arriba) |

---

## 8. Checklist mensual de salud on-chain

- [ ] `solana balance` del authority > 1 SOL (para fees de pause emergency)
- [ ] `solana balance` del compliance_signer > 1 SOL (para stamps)
- [ ] Treasury balance reconciliado contra ledger contable
- [ ] `paused == false` (sanity)
- [ ] `usdc_mint` apunta al mint correcto (mainnet: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`)
- [ ] IDL en `web 2.0/js/idl/` coincide con `target/idl/` del último deploy
  (hash check)
- [ ] `JurisdictionPolicy.blocked` revisada contra última OFAC/EU update
- [ ] Squads multisig signers todos accesibles (rotación de hardware wallets)

---

## 9. Comandos canónicos

```bash
# Validador local (CRÍTICO: ulimit -n 65536 o muere en 60s)
ulimit -n 65536
nohup prlimit --nofile=65536 -- solana-test-validator --reset \
    --rpc-port 8899 --faucet-port 9900 --bind-address 127.0.0.1 \
    --limit-ledger-size 50000000 > /tmp/validator.log 2>&1 &

# Logs de un programa
solana logs G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a

# Inspeccionar una cuenta
solana account <PUBKEY> --output json-compact

# Fetch IDL de devnet
anchor idl fetch G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a -o /tmp/agroglobaldex-deployed.json
diff <(jq -S . target/idl/agroglobaldex.json) <(jq -S . /tmp/agroglobaldex-deployed.json)
```

---

## 10. Contacto de emergencia

| Caso | Contacto | Canal |
|---|---|---|
| Bug on-chain | dev lead | Discord `@onchain-oncall` |
| Sanctions hit | MLRO | mlro@agroglobaldex (cifrado) |
| Hack en curso | security lead + autoridad legal | Signal, número en data room |
| Regulator request | counsel + DPO | bufete designado + DPO en France |

---

**Versionado**: este runbook se versionará junto al programa. Cada vez que se
agrega/quita una instrucción, el runbook se actualiza en el mismo PR.
