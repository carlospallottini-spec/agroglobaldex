# AgroGlobalDex — Índice del Data Room

**Actualizado:** 2026-07-26 · **Owner:** Carlos Pallottini

> Propósito: cuando un inversor o socio pida información, este es el índice único. Todo lo listado en la sección 1 existe en el repositorio y es verificable. La sección 2 es la lista honesta de lo que **falta** y solo Carlos puede aportar.
>
> Protocolo de acceso por etapas (qué compartir en cada meeting) en [`marketing/data-room-structure.md`](../marketing/data-room-structure.md).

---

## 1. Documentos existentes en el repositorio

### 1.1 Visión y resumen ejecutivo

| Documento | Qué es |
|---|---|
| [`docs/INVESTOR_ONE_PAGER.md`](./INVESTOR_ONE_PAGER.md) | One-pager honesto: problema, solución, estado real, roadmap en 3 fases, uso de fondos |
| [`marketing/one-pager.md`](../marketing/one-pager.md) | Versión corta (~300 palabras) para mail/DM previo al meeting |
| [`marketing/pitch-deck-full.md`](../marketing/pitch-deck-full.md) | Deck completo (14 slides + apéndice) en texto, listo para pasar a visual |
| [`marketing/faq-investors.md`](../marketing/faq-investors.md) | 20 preguntas difíciles de inversores con respuestas honestas |
| [`README.md`](../README.md) | Portada del repo: qué hay, cómo probarlo, disclaimers |

### 1.2 Producto y tecnología

| Documento | Qué es |
|---|---|
| [`docs/ARCHITECTURE.md`](./ARCHITECTURE.md) | Diagrama del sistema (web → wallet → programas Solana → compliance hook → Pyth) y descripción de cada componente |
| [`docs/whitepaper-v0.1.md`](./whitepaper-v0.1.md) | Whitepaper técnico v0.1 — **ojo: parcialmente desactualizado** respecto al código (menciona Switchboard; el contrato usa Pyth). Revisar antes de compartir |
| [`solana/README.md`](../solana/README.md) | Detalle técnico de los 2 programas Anchor, 30 instrucciones, 5 clases de activo |
| [`solana/RUNBOOK.md`](../solana/RUNBOOK.md) | Runbook operativo del contrato: procedimientos de incidente, pausa, llaves |
| [`web 2.0/`](../web%202.0/) | La web/PWA publicada en agroglobaldex.com (11+ páginas, demo devnet) |
| [`CHANGELOG.md`](../CHANGELOG.md) | Historial de desarrollo versión a versión — demuestra cadencia real de trabajo |

### 1.3 Seguridad (lo que un inversor técnico va a pedir primero)

| Documento | Qué es |
|---|---|
| [`AUDIT_READINESS.md`](../AUDIT_READINESS.md) | Auditoría interna completa: hallazgos por severidad, estado de remediación (críticos/altos de código corregidos y verificados en CI), invariantes del protocolo, shortlist de firmas auditoras con costes estimados |
| [`SECURITY.md`](../SECURITY.md) | Política de seguridad: defensas implementadas, limitaciones conocidas, gestión de llaves, gate de go-live (multisig + timelock), canal de reporte |
| [`MAINNET.md`](../MAINNET.md) | Checklist de mainnet-readiness por gates (auditoría, CASP, hardening, tests, infra, capital, GTM) con el camino crítico de 9–15 meses |
| CI del repo ([`.github/workflows/`](../.github/workflows/)) | `anchor test` (47 tests), clippy, fmt, cargo-audit, guard de deriva del IDL — todo verde en cada commit |

### 1.4 Legal y regulatorio (borradores — requieren validación de abogado)

| Documento | Qué es |
|---|---|
| [`legal/README.md`](../legal/README.md) | Índice del paquete legal + disclaimer (nada de esto es asesoría legal) |
| [`legal/01-mica-analysis.md`](../legal/01-mica-analysis.md) | Análisis MiCA aplicado al producto |
| [`legal/02-asset-classification.md`](../legal/02-asset-classification.md) | Calificación de cada clase de activo (commodity vs security) — decisión clave del negocio |
| [`legal/03-permits-and-licenses.md`](../legal/03-permits-and-licenses.md) | Mapa de permisos: CASP, AML, prospecto, DORA, GDPR |
| [`legal/04-kyc-aml-framework.md`](../legal/04-kyc-aml-framework.md) | Marco KYC/AML (AMLD6, travel rule, screening de sanciones) |
| [`legal/05-jurisdictional-strategy.md`](../legal/05-jurisdictional-strategy.md) | Comparativa de jurisdicciones UE — recomendación: Francia (alternativa: Países Bajos) |
| [`legal/06-terms-of-service.draft.md`](../legal/06-terms-of-service.draft.md) | Borrador de T&C |
| [`legal/07-privacy-policy.draft.md`](../legal/07-privacy-policy.draft.md) | Borrador de política de privacidad (GDPR) |
| [`legal/08-white-paper-template.md`](../legal/08-white-paper-template.md) | Template de white paper MiCA (Art. 6 + Anexo I) |
| [`legal/09-risk-disclosures.md`](../legal/09-risk-disclosures.md) | Disclosures de riesgo obligatorios |
| [`legal/10-compliance-checklist.md`](../legal/10-compliance-checklist.md) | Checklist accionable pre-lanzamiento con resumen económico (~€400k–1.8M total compliance, *estimación*) y plazos |
| [`legal/11-safe-template-pre-seed.md`](../legal/11-safe-template-pre-seed.md) | Template SAFE post-money para el pre-seed |

### 1.5 Mercado, negocio y ronda

| Documento | Qué es |
|---|---|
| [`marketing/01-ecosystem-research.md`](../marketing/01-ecosystem-research.md) | Panorama competitivo RWA-agro |
| [`marketing/02-target-list.md`](../marketing/02-target-list.md) | Lista priorizada de targets de outreach (redactar antes de compartir externamente) |
| [`marketing/05-positioning-and-messaging.md`](../marketing/05-positioning-and-messaging.md) | Posicionamiento y mensajes |
| [`marketing/financials-5y.md`](../marketing/financials-5y.md) | Modelo financiero a 5 años (proyecciones + sensibilidad + comparables) — todo proyección, sin datos reales aún |
| [`marketing/06-launch-plan-90days.md`](../marketing/06-launch-plan-90days.md) | Plan de lanzamiento 90 días |
| [`marketing/07-metrics-and-kpis.md`](../marketing/07-metrics-and-kpis.md) | KPIs que se medirán (hoy no hay métricas de usuarios: pre-lanzamiento) |
| [`marketing/cap-table-template.md`](../marketing/cap-table-template.md) | Template de cap table pre-seed → Series A (es template, no la cap table real) |
| [`marketing/loi-templates.md`](../marketing/loi-templates.md) | Templates de LOI para pilotos (España vinos, Venezuela carnes) — **no firmados** |
| [`marketing/grant-applications.md`](../marketing/grant-applications.md) / [`solana-foundation-grant.md`](../marketing/solana-foundation-grant.md) | Borradores de aplicaciones a grants (Solana Foundation, Horizon Europe) — **no enviadas** |
| [`marketing/demo-video-script.md`](../marketing/demo-video-script.md) | Guion del demo de 90 segundos — **video no grabado aún** |
| [`LAUNCH.md`](../LAUNCH.md) | Checklist maestro de lanzamiento con estado real de cada ítem |

---

## 2. Lo que FALTA — solo Carlos puede aportarlo

Checklist de piezas que ningún documento del repo sustituye. Sin ellas el data room está incompleto para una due diligence seria:

- [ ] **Constitución de la sociedad / SPV.** Hoy no existe entidad legal. Sin sociedad no hay con quién firmar un SAFE. Decidir jurisdicción del vehículo (la recomendación regulatoria es Francia u Holanda para la OpCo CASP; el vehículo del SAFE puede ser otro) y constituirla con un abogado. *(Estimación: €5k–30k en notaría/registro/tasas + honorarios.)*
- [ ] **Cap table real.** El repo solo tiene un template. Completarla con la participación actual (100% founder, presumiblemente) y el escenario post-SAFE.
- [ ] **Presupuesto operativo y uso de fondos detallado.** Convertir los rangos estimados del one-pager en un presupuesto mensual de 18–24 meses (runway) con nombres de partidas concretas.
- [ ] **Term sheet / SAFE con condiciones definitivas.** Existe el template (`legal/11-safe-template-pre-seed.md`); faltan las condiciones firmables validadas por abogado (cap, descuento, MFN, jurisdicción del contrato).
- [ ] **CV / bio del founder y advisors.** El data room estándar (`08_TEAM`) lo pide siempre. Incluir experiencia relevante en agro y por qué Carlos es la persona para esto.
- [ ] **Cuentas y accesos corporativos:** emails del dominio (security@/legal@/hello@ hoy son placeholders según `SECURITY.md`), cuenta bancaria de la entidad, Google Play Console (~USD 25 una vez) y Apple Developer (~USD 99/año) para publicar la app.
- [ ] **Abogado especializado contratado.** Todo el paquete `legal/` es borrador generado internamente; la primera factura real del proyecto debería ser la validación legal de `legal/01`, `02` y `05`.
- [ ] **LOIs firmados con productores/cooperativas.** Hay templates; no hay firmas. Uno o dos LOIs reales cambian la conversación con inversores más que cualquier documento técnico.
- [ ] **Video demo de 90 segundos grabado y publicado.** El guion está listo (`marketing/demo-video-script.md` y `LAUNCH.md`); falta grabarlo. Es la pieza de mayor impacto/coste-cero pendiente.
- [ ] **Decisión de producto clave (con abogado): rediseñar o no las fracciones de cosecha** para evitar el régimen MiFID II (ahorro estimado €500k–1.5M y 12+ meses según `legal/10-compliance-checklist.md`).

---

## 3. Higiene antes de compartir

- Unificar el número de tests y fechas en todos los docs (fuente de verdad: la suite en `solana/tests/` — 47 tests hoy).
- El whitepaper v0.1 menciona Switchboard y una arquitectura anterior; actualizarlo o marcar la sección 3 como "diseño original, ver ARCHITECTURE.md para lo implementado".
- La fecha objetivo de mainnet aparece como Q3 2026, Q4 2026 y Q1 2027 según la página. Unificar al mensaje honesto: **9–15 meses después del cierre del funding** *(estimación)*, condicionado a auditoría + licencia.
- Nunca compartir `marketing/02-target-list.md` sin redactar nombres.
