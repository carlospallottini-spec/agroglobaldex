# Mainnet readiness checklist — AgroGlobalDex

> Checklist exacto que tiene que estar 100% antes de deployar a mainnet beta.
> Estado al **2026-06-02**: ~40% completo. Estimado plazo restante: **9-15 meses**.

---

## 🔒 Gate 1 — Auditoría profesional (BLOQUEANTE absoluto)

- [ ] Cotizar 3 firmas: **Sec3** (recomendada para Anchor/Token-2022),
      **OtterSec**, **Halborn**.
- [ ] Presupuesto USD 50k-150k según severidad y profundidad.
- [ ] Audit final report sin **Critical** ni **High** sin resolver.
- [ ] Audit report publicado en `legal/audit/` (PDF + commit hash auditado).
- [ ] Re-audit incremental antes de cada upgrade post-mainnet.

**Pre-requisitos para empezar el audit**:
- [x] Audit interno propio (30 issues, 11 fixeados al 2026-06-02)
- [x] Tests mocha que cubran las 21 instrucciones (23 tests, parcial)
- [ ] `anchor build --verifiable` reproducible (Docker fija toolchain)
- [ ] `solana-verify` workflow funcionando contra commit pinneado
- [ ] RUNBOOK + SECURITY publicados (✅ hecho)

---

## ⚖️ Gate 2 — CASP MiCA (BLOQUEANTE legal)

Detalle completo en [`legal/03-permits-and-licenses.md`](legal/03-permits-and-licenses.md).

### Jurisdicción
- [x] Elegida: **Francia (AMF/ACPR)**. Razón en
      [`legal/05-jurisdictional-strategy.md`](legal/05-jurisdictional-strategy.md).

### Personal con sustancia local
- [ ] MLRO (Money Laundering Reporting Officer) designado, residente
      en Francia, con cualificación AML reconocida.
- [ ] DPO (Data Protection Officer) certificado para GDPR.
- [ ] Conducting officer Senior con experiencia financiera UE.

### Capital regulatorio MiCA Anexo IV
- [ ] **€50k-150k** depositado en cuenta bancaria UE de la entidad
      (proporcional al servicio CASP autorizado).
- [ ] Cuenta bancaria UE para entidad crypto (4-12 semanas de apertura).

### Documentación
- [ ] White paper MiCA Art. 6 + Anexo I (template en
      [`legal/08-white-paper-template.md`](legal/08-white-paper-template.md)).
- [ ] Notificación AMF del white paper (Art. 8 MiCA).
- [ ] T&Cs operativos (no solo draft) (
      [`legal/06-terms-of-service.draft.md`](legal/06-terms-of-service.draft.md)).
- [ ] Privacy Policy GDPR-compliant
      ([`legal/07-privacy-policy.draft.md`](legal/07-privacy-policy.draft.md)).

### Procesos
- [ ] Sumsub / Veriff / equivalente integrado con `update_kyc` API.
- [ ] OFAC + EU consolidated + UN sanctions screening automatizado en cada
      KYC stamp.
- [ ] Travel rule (AMLD6) compliance — provider seleccionado (Notabene/
      TRM Labs).
- [ ] Disaster recovery plan (DORA Art. 6).

---

## 🛡️ Gate 3 — Hardening on-chain

### Multisig
- [ ] **Squads multisig 2-of-3** para `marketplace.authority` con
      timelock 24h.
- [ ] Hardware wallets para los 3 signers (Ledger / Trezor).
- [ ] Procedimiento documentado para perder 1 signer sin perder el control.

### Token-2022 PermanentDelegate (clawback)
- [ ] Decidir si el mainnet usa PermanentDelegate o no.
- [ ] Si SÍ: el `marketplace.authority` o `compliance_signer` puede
      transferir tokens fuera de wallets sancionadas post-revoke_kyc.
- [ ] Si SÍ: comunicado público explicando el trade-off de censorship-resistance
      a cambio de compliance.

### Oracle attestation
- [ ] Ed25519 verification en `register_asset` (audit #12) si el oracle
      es centralizado.
- [ ] O multi-sig de oracles (3-of-5 atestadores independientes) si
      descentralizado.

### Treasury defense
- [ ] `max_withdraw_per_epoch` cap en `treasury_withdraw` (parametrizable
      por authority).
- [ ] Alertas off-chain en cada `TreasuryWithdrawn` event (Slack /
      PagerDuty).

---

## 🧪 Gate 4 — Test coverage completa

- [x] 23 tests mocha cubriendo 21 instrucciones — parcial (faltan e2e).
- [ ] **list_asset → buy_asset full E2E con TransferHook** (issue #21-22).
      Necesita: stamping KYC para listing PDA o modificación del hook para
      skippear escrows del marketplace.
- [ ] Fuzzing con `proptest` para parámetros del registry (issue #23):
      total_supply=u64::MAX, fee_bps=u16::MAX, URIs en límite.
- [ ] `anchor verify` test que asserta el .so deployado == commit pinneado.
- [ ] Stress test con 1000 transfers concurrentes via TransferHook.

---

## 🏗️ Gate 5 — Infra & operations

### Deploy
- [ ] Squads multisig usado para deployar (no single-sig).
- [ ] `anchor build --verifiable` con Docker + toolchain congelada.
- [ ] IDL hashes registrados en `IDL.lock` (script ya tiene logging,
      falta el commit del lock).

### Monitoring
- [ ] Helius / Triton dedicated RPC.
- [ ] Solscan / SolanaFM webhook → Slack on `marketplace.paused` change.
- [ ] Sentry / Datadog para errores del frontend.
- [ ] On-chain analytics dashboard (Dune query).

### Backup & DR
- [ ] Backup de keypairs `authority`, `compliance_signer` en cold
      storage (HSM físico + paper backup encriptado).
- [ ] Procedimiento de incidente de cada escenario A/B/C del RUNBOOK.

---

## 💰 Gate 6 — Capitalización

- [ ] Pre-seed cerrado (USD 500k @ USD 5M cap o equivalente).
- [ ] 2 LOIs firmados (España vinos + Venezuela carnes).
- [ ] Aplicación a Solana Foundation Grant + Horizon Europe enviada y
      respondida (sí/no).
- [ ] Cap table actualizada y firmada por todos los SAFE holders.
- [ ] SPV constituido (Francia / Holanda / Luxemburgo TBD).

---

## 📣 Gate 7 — Go-to-market

- [ ] Web pública con disclaimer Demo/PoC quitado (porque ya NO es PoC).
- [ ] Pitch deck visual (Gamma → PNG → PDF) actualizado con métricas reales
      de devnet uso.
- [ ] 5-10 productores onboardeados a devnet (no solo LOIs, datos reales).
- [ ] Demo grabado (90s) publicado en YouTube + LinkedIn.
- [ ] Cobertura prensa: 1-2 medios crypto (CoinDesk, The Block) + 1 medio
      agro (Reuters Markets, Bloomberg ag).

---

## 🚦 Resumen de bloqueantes

Al 2026-06-02:

| Bloqueante | Tipo | ETA |
|---|---|---|
| Audit profesional | Técnico | 4-8 semanas tras pago |
| MLRO + DPO | Legal/HR | 2-3 meses de búsqueda + contratación |
| Cuenta bancaria UE | Operativo | 4-12 semanas |
| Capital regulatorio €50k-150k | Cash | Inmediato post-funding |
| White paper notificado | Legal | 1-2 meses con counsel |
| Squads multisig setup | Técnico | 1 semana |
| LOIs firmados | Comercial | 4-12 semanas (en negociación) |

**Camino crítico**: funding → counsel UE → MLRO/DPO → cuenta bancaria → white
paper → CASP filing → audit → deploy mainnet. Total realista: **9-15 meses**.

---

## ⚠️ Reglas duras (NUNCA romper)

1. **No deploy a mainnet sin auditoría completa** + remediación de Critical/High.
2. **No promesas de yield garantizado** en ningún channel oficial (securities fraud).
3. **No US persons sin Reg D filing** (bloqueado on-chain también).
4. **No tokenizar activos sin custodial-verify off-chain**: warehouse receipt
   firmado, D.O.C. validada, certificado INSAI, etc.
5. **No singularizar la `authority`**: siempre Squads multisig en mainnet.
6. **No `--no-verify` en commits**: pre-commit hooks son load-bearing.
7. **No `git push --force` a main / branch productiva**.

---

## Próxima review

Cada **2 semanas** durante pre-seed; cada **mensual** post-seed.
Owner: founder (Carlos Pallottini).
