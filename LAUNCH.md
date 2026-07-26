# AgroGlobalDex — Checklist maestro de lanzamiento

> **Actualizado: 2026-07-26.** Documento vivo. Este es EL documento que Carlos abre
> para saber "qué me falta". Leyenda:
> **✅ hecho** (con fecha) · **🔜 listo, requiere acción de Carlos** (con coste estimado) · **⬜ viene después**
>
> Todos los costes son **rangos estimados de mercado**, no presupuestos cerrados.
> Detalle económico completo del compliance en [`legal/10-compliance-checklist.md`](legal/10-compliance-checklist.md).

---

## 1. ✅ HECHO (verificable en el repo)

| Ítem | Fecha | Evidencia |
|---|---|---|
| ✅ Contrato Solana feature-complete (2 programas Anchor, 30 instrucciones, 5 clases de activo) | 2026-06 | `solana/`, CHANGELOG v0.5 |
| ✅ Suite de tests en CI verde: 47 tests de integración (happy + sad + fuzz + adversarios del hook) | 2026-06-16 | `solana/tests/agroglobaldex.ts`, workflow `anchor-test.yml` |
| ✅ Gates de CI: clippy + fmt + cargo-audit + guard de deriva del IDL | 2026-06 | `.github/workflows/` |
| ✅ Hardening interno completo: críticos y altos de código remediados y verificados en CI (kill-switch cubre lending, bypass de acreditación P2P cerrado, oráculo exigible, liquidación justa, LP shares) | 2026-06-16 | `AUDIT_READINESS.md` §0, `SECURITY.md` |
| ✅ Documentación de seguridad para el auditor: assessment interno, invariantes, runbook, política de seguridad | 2026-06-14/16 | `AUDIT_READINESS.md`, `SECURITY.md`, `solana/RUNBOOK.md` |
| ✅ Web/PWA publicada con deploy automático (merge → Cloudflare) | 2026-06-13 | agroglobaldex.com |
| ✅ Apps de escritorio con builds automáticos por release (Win/mac/Linux) | 2026-06 | `.github/workflows/electron-build.yml`, Releases |
| ✅ Wrapper móvil Capacitor listo para compilar | 2026-06-13 | `mobile/` |
| ✅ Paquete legal borrador (11 docs: MiCA, clasificación de activos, licencias, KYC/AML, jurisdicción, T&C, privacidad, white paper template, riesgos, checklist, SAFE) | 2026-06 | `legal/` |
| ✅ Paquete de fundraising (deck, one-pager, FAQ, financials 5y, templates LOI/outreach, guiones) | 2026-06 | `marketing/` |
| ✅ Data room indexado + one-pager honesto + doc de arquitectura | 2026-07-26 | `docs/DATA_ROOM.md`, `docs/INVESTOR_ONE_PAGER.md`, `docs/ARCHITECTURE.md` |
| ✅ Script de deploy mainnet con traspaso de upgrade authority a multisig | 2026-06 | `solana/scripts/deploy-mainnet.sh` |

---

## 2. 🔜 LISTO PERO REQUIERE ACCIÓN DE CARLOS

Ordenado por impacto/urgencia. Nada de esto lo puede hacer un agente ni está en el código: son decisiones, firmas, cuentas y dinero.

### 2.1 Coste cero — hacer esta semana

- [ ] 🔜 **Grabar el video demo de 90s** (guion listo abajo, §5). Es la pieza que destraba todo el fundraising. Coste: tiempo.
- [ ] 🔜 **Publicar el video** en YouTube + LinkedIn y ponerlo en investors.html.
- [ ] 🔜 **Completar el one-pager**: LinkedIn real, link de agenda (cal.com), y decidir qué email usar (los `@agroglobaldex` de SECURITY.md son placeholders hasta tener dominio de email operativo).
- [ ] 🔜 **Unificar la fecha de mainnet en la web**: hoy conviven Q3 2026 / Q4 2026 / Q1 2027 en distintas páginas. Mensaje honesto único: "9–15 meses post-funding *(estimación)*".
- [ ] 🔜 **Escribir su bio/CV de founder** para el data room (ver `docs/DATA_ROOM.md` §2).

### 2.2 Fundraising (semanas 1–12)

- [ ] 🔜 **Enviar aplicaciones a grants** — borradores listos en `marketing/grant-applications.md` y `marketing/solana-foundation-grant.md`. Coste: USD 0. Potencial: USD 25k–75k *(estimación)* no dilutivo.
- [ ] 🔜 **Arrancar outreach a inversores** con `marketing/02-target-list.md` + templates de `marketing/outreach-vc-emails.md`. Round objetivo: USD 500k pre-seed @ USD 5M cap (SAFE).
- [ ] 🔜 **Perseguir 1–2 LOIs firmados** con cooperativas/productores (templates en `marketing/loi-templates.md`). Cambian la ronda más que cualquier feature.
- [ ] 🔜 **Contratar abogado especializado cripto/UE** para validar `legal/01`, `02` y `05` y preparar el SAFE firmable. Primera consulta: €2k–10k *(estimación)*; paquete legal completo del lanzamiento: €80k–300k *(estimación, post-funding)*.
- [ ] 🔜 **Constituir la sociedad** (sin entidad no hay con quién firmar el SAFE). Notaría + registro + tasas: €5k–30k *(estimación)*. Decisión de jurisdicción con el abogado (recomendación del análisis: Francia u Holanda para la OpCo; ver `legal/05`).
- [ ] 🔜 **Cap table real** (hoy solo existe el template).

### 2.3 Cuentas y accesos (requieren tarjeta/identidad de Carlos)

- [ ] 🔜 **Google Play Console**: ~USD 25 una sola vez *(tarifa oficial)*. La app Android está lista para compilar.
- [ ] 🔜 **Apple Developer Program**: ~USD 99/año *(tarifa oficial)* — solo si se decide iOS.
- [ ] 🔜 **Email corporativo del dominio** (Google Workspace ~USD 6–12/usuario/mes *(estimación)*): activa security@/legal@/hello@ reales.
- [ ] 🔜 **Probar la app móvil en un dispositivo Android real** (falta `cap add android` + prueba física; instrucciones en `mobile/README.md`).

### 2.4 Con capital en mano (los dos bloqueantes duros)

- [ ] 🔜 **Auditoría externa del contrato** — bloqueante absoluto de mainnet. Cotizar 3 firmas (OtterSec, Neodyme, Zellic, Sec3, Halborn — ver `AUDIT_READINESS.md` §5). Coste: **USD 30k–80k** *(estimación, rango de mercado para 2 programas Anchor ~3–4k líneas con Token-2022 + hook custom + Pyth + lending; máxima profundidad puede llegar a ~USD 150k)*. Plazo: 4–10 semanas incluyendo cola. **Reservar slot con antelación.**
- [ ] 🔜 **Camino CASP MiCA (Francia recomendada)** — bloqueante legal para operar en UE. Incluye: capital regulatorio **€50k–150k depositado** *(según servicio, MiCA Anexo IV)*, MLRO + DPO con sustancia local (búsqueda 2–3 meses), cuenta bancaria UE (4–12 semanas), white paper MiCA notificado. Detalle por gates en `MAINNET.md` y `legal/03`.
- [ ] 🔜 **Decisión de producto con el abogado: fracciones de cosecha como security o rediseño** para evitar MiFID II — ahorro potencial €500k–1.5M y 12+ meses *(estimación, `legal/10`)*.

---

## 3. ⬜ DESPUÉS (post-funding, camino a mainnet)

Secuencia detallada por gates en [`MAINNET.md`](MAINNET.md). Resumen:

- [ ] ⬜ Pre-auditoría técnica restante: build reproducible (`anchor build --verifiable` + `solana-verify`), E2E completo list→buy con hook, fuzzing adicional, migración del parseo Pyth al SDK oficial (o decisión del auditor).
- [ ] ⬜ Remediar los hallazgos de la auditoría externa (sin Critical/High abiertos) y publicar el informe en el data room.
- [ ] ⬜ Multisig Squads 2-de-3 + timelock 24h para `authority` y upgrade authority (gate de go-live en `SECURITY.md`); hardware wallets; ceremonia de llaves con `deploy-mainnet.sh`.
- [ ] ⬜ Decisión PermanentDelegate (clawback de wallets sancionadas) + comunicado del trade-off.
- [ ] ⬜ Integración KYC comercial (Sumsub/Veriff) + screening automático OFAC/UE/ONU + travel rule.
- [ ] ⬜ Infra de producción: RPC dedicado (Helius/Triton), monitoreo y alertas, backups de llaves en frío, plan DR.
- [ ] ⬜ `require_oracle_for_loans = true` en mainnet (el precio manual queda solo para devnet).
- [ ] ⬜ Deploy mainnet beta cerrado → apertura progresiva. **Estimación total realista: 9–15 meses desde el cierre del funding.**

---

## 4. La secuencia (por qué en este orden)

```
HOY ──► Video demo + outreach ──► Capital (pre-seed/grants) ──► Auditoría + CASP (en paralelo) ──► Mainnet
        (coste cero, ya posible)   (con la demo en mano)         (meses + capital)                (producto real)
```

El lanzamiento que SÍ se puede hacer hoy, sin auditoría ni licencia, es el de la
**narrativa**: demo en video del producto funcionando en devnet. Es lo que destraba
el capital que paga la auditoría y el legal. Prometer operación con dinero real sin
auditoría ni CASP es el error que mata la ronda (y es ilegal). La transparencia es el activo.

---

## 5. Guion del video demo (90 segundos, devnet)

1. **0–10s — El problema.** "El productor no puede vender ni acceder a crédito.
   €280B de mercado agro UE, cero liquidez." (hero de index.html).
2. **10–30s — Tokenizar.** Conectar wallet (Phantom devnet) → tokenizar una
   cosecha en `tokenize.html`. Mostrar la tx en Solscan.
3. **30–55s — Crédito instantáneo.** En `borrow.html`: depositar el token como
   colateral → recibir USDC al instante. Mostrar el cálculo LTV en vivo.
4. **55–75s — Marketplace + inversión.** Comprar una fracción en
   `marketplace.html`/`invest.html`. KYC gate visible.
5. **75–90s — Cierre.** "On-chain, en segundos, sobre Solana. Esto ya funciona
   en devnet hoy." Logo + "mainnet: post-auditoría y licencia".

**Pre-grabación (1–2 días):**
- [ ] Fondear una wallet devnet con SOL + USDC de faucet.
- [ ] Verificar que el `PROGRAM_ID` desplegado en devnet coincide con el del cliente.
- [ ] Pre-cargar 2–3 assets de ejemplo en el marketplace.
- [ ] Grabar **con el banner DEMO visible** (es honesto y legalmente obligatorio) y mostrar Solscan en cada tx.

---

## 6. Reglas duras (nunca romper)

1. No deploy a mainnet sin auditoría externa completa + Critical/High resueltos.
2. No promesas de yield garantizado en ningún canal (fraude de valores).
3. No operar en UE sin autorización CASP; el banner Demo/PoC es obligatorio hasta entonces.
4. No presentar proyecciones como métricas: hoy hay **0 usuarios reales y 0 ingresos** — decirlo así suma credibilidad.
5. `authority` de mainnet siempre multisig, nunca una sola wallet.
