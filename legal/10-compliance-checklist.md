# 10 — Compliance Checklist Pre-Lanzamiento

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Las estimaciones de tiempo y coste son orientativas; ajustar tras consulta con abogado UE y proveedores.
>
> **Cómo usar**: cada ítem tiene **responsable sugerido**, marca de **bloqueante (S/N)** para el lanzamiento, y **estimación de tiempo**. Marcar avance en la columna **Estado** (Todo / In Progress / Done / Blocked).

---

## Leyenda

- **Resp.**: rol responsable sugerido (Founder, COO, Legal externo, AML Officer, DPO, CTO, etc.).
- **Bloq.**: ¿bloqueante para lanzamiento? (S/N).
- **Tiempo**: estimación de duración total (preparación + tramitación).
- **Estado**: Todo / In Progress / Done / Blocked.

---

## A. Setup de entidad legal (8 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| A1 | Decidir Estado miembro UE de domicilio (recomendación: Francia) | Founder | S | 2 sem | Todo |
| A2 | Constituir sociedad UE (SAS/GmbH/SL) con capital social | Founder + notario | S | 4–8 sem | Todo |
| A3 | Abrir cuenta bancaria operativa UE (KYC del banco) | COO | S | 4–12 sem | Todo |
| A4 | Aportar capital social + capital regulatorio CASP (Anexo IV MiCA) | Founder | S | 1 sem (tras A3) | Todo |
| A5 | Registrar beneficial ownership en el Beneficial Ownership Register nacional | Legal externo | S | 2 sem | Todo |
| A6 | Designar órgano de administración + fit & proper docs (CVs, certificados antecedentes, declaración patrimonial) | Legal externo | S | 3 sem | Todo |
| A7 | Solicitar **LEI** (Legal Entity Identifier) | COO | S | 1 sem | Todo |
| A8 | Contratar seguro D&O y responsabilidad profesional | COO | N | 4 sem | Todo |

## B. Licenciamiento (10 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| B1 | Pre-application meeting informal con la NCA elegida | Founder + Legal | N | 2 sem | Todo |
| B2 | Preparar **Programa de Operaciones** detallado (servicios CASP a prestar) | COO + Legal | S | 6 sem | Todo |
| B3 | Preparar **Plan de Negocio** y proyecciones financieras 3 años | Founder + CFO | S | 4 sem | Todo |
| B4 | Preparar **Gobernanza interna y políticas** (RACI, controles, three lines of defence) | COO | S | 6 sem | Todo |
| B5 | Preparar **Plan de Continuidad y Recuperación** (BCP/DRP) | CTO + COO | S | 4 sem | Todo |
| B6 | Preparar **Política AML/CFT** completa y manual del MLRO | AML Officer | S | 6 sem | Todo |
| B7 | Preparar **Política de Custodia y Segregación** | CTO + Legal | S | 4 sem | Todo |
| B8 | Preparar **Política de Mejor Ejecución y Conflictos de Interés** | Legal | S | 3 sem | Todo |
| B9 | **Presentar solicitud de autorización CASP** ante la NCA | Legal externo | S | Día 0 + 6–12 meses tramitación | Todo |
| B10 | Si aplica securities: presentar autorización investment firm + prospecto | Legal externo | S (si aplica) | 12–18 meses | Todo |

## C. AML / KYC operativo (8 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| C1 | Designar **MLRO** (Money Laundering Reporting Officer) y comunicar a NCA | Founder | S | 1 sem | Todo |
| C2 | Realizar **Business-wide Risk Assessment** AML | AML Officer | S | 4 sem | Todo |
| C3 | Contratar proveedor KYC (Sumsub / Onfido / Veriff) e integrar | CTO | S | 6 sem | Todo |
| C4 | Contratar proveedor on-chain (Chainalysis / TRM / Elliptic) | CTO + AML | S | 4 sem | Todo |
| C5 | Contratar Sanctions/PEP screening continuo (ComplyAdvantage / WorldCheck) | AML Officer | S | 3 sem | Todo |
| C6 | Implementar **Travel Rule** (Notabene / Sumsub TR / VerifyVASP) | CTO | S | 8 sem | Todo |
| C7 | Establecer canal y procedimiento de **SAR/STR** con la FIU local | AML Officer | S | 2 sem | Todo |
| C8 | Plan de **formación AML** anual para todo el personal | AML Officer | S | 2 sem | Todo |

## D. Tech & seguridad (DORA + smart contracts) (8 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| D1 | **ICT Risk Framework** conforme a DORA | CTO + CISO | S | 8 sem | Todo |
| D2 | **Auditoría de seguridad de smart contracts** (Trail of Bits / OpenZeppelin / Halborn) | CTO | S | 6–12 sem | Todo |
| D3 | **Pentest** infraestructura + frontend | CISO | S | 4 sem | Todo |
| D4 | Implementar **MFA, segregación de roles, key management** (HSM o MPC) | CTO | S | 6 sem | Todo |
| D5 | Plan de **gestión de incidentes** y reporte a NCA (DORA) | CISO | S | 3 sem | Todo |
| D6 | **Registro de terceros TIC** críticos (DORA art. 28) | COO | S | 2 sem | Todo |
| D7 | **Tests de resiliencia** (TLPT) — programar para post-launch | CISO | N | continuo | Todo |
| D8 | Plan de **monitoring** de redes blockchain usadas (Solana) y plan de contingencia ante outage | CTO | S | 2 sem | Todo |

## E. Datos personales / GDPR (6 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| E1 | Designar **DPO** (interno o externo) y comunicar a autoridad de control | Founder | S | 2 sem | Todo |
| E2 | Elaborar **Registro de Actividades de Tratamiento (RoPA)** | DPO | S | 4 sem | Todo |
| E3 | Realizar **DPIA** (Data Protection Impact Assessment) para KYC + AML monitoring | DPO | S | 4 sem | Todo |
| E4 | Implementar **mecanismo de consentimiento de cookies** y banner | CTO + DPO | S | 2 sem | Todo |
| E5 | Implementar **portal de derechos del titular** (acceso, rectificación, supresión, portabilidad) | CTO + DPO | S | 4 sem | Todo |
| E6 | Firmar **SCCs / DPAs** con todos los encargados del tratamiento | DPO | S | 6 sem | Todo |

## F. Documentación contractual y disclosures (6 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| F1 | **Términos y Condiciones** finalizados (ES, EN + idiomas EM target) | Legal externo | S | 4 sem | In Progress |
| F2 | **Política de Privacidad** finalizada | Legal externo + DPO | S | 3 sem | In Progress |
| F3 | **Política de Cookies** | DPO | S | 1 sem | Todo |
| F4 | **Risk Disclosures** finalizados e integrados en el flujo de onboarding | Legal externo + AML | S | 3 sem | In Progress |
| F5 | **White paper(s)** OCA para cada token + notificación a NCA (MiCA art. 8) | Legal + Founder | S | 6 sem | In Progress |
| F6 | **Política de quejas y resolución de disputas** | Legal externo | S | 2 sem | Todo |

## G. Producto: clasificación de activos (4 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| G1 | Decisión estratégica: rediseñar o no las "fracciones de cosechas" para evitar MiFID II | Founder + Legal | S | 2 sem | Todo |
| G2 | Acuerdos con **warehouses certificados** para granos | COO | S | 8–12 sem | Todo |
| G3 | Acuerdos con **registros voluntarios de carbono** (Verra / Gold Standard) | COO | S (si carbono) | 4–8 sem | Todo |
| G4 | Implementar **prueba de reservas** (PoR) auditable on-chain para tokens RWA | CTO | S | 6 sem | Todo |

## H. Marketing y comunicaciones (5 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| H1 | Política de **marketing communications** (MiCA art. 7, 13) | Marketing + Legal | S | 2 sem | Todo |
| H2 | Plantillas con advertencias obligatorias integradas | Marketing | S | 1 sem | Todo |
| H3 | Procedimiento de **aprobación previa** de toda pieza promocional | COO | S | 1 sem | Todo |
| H4 | Registro/notificación a NCA si la jurisdicción lo exige | Legal | S (si aplica) | 2 sem | Todo |
| H5 | Política de **influencers** y disclosure de patrocinios | Marketing + Legal | S | 1 sem | Todo |

## I. Operativa día 1 (5 ítems)

| # | Ítem | Resp. | Bloq. | Tiempo | Estado |
|---|---|---|---|---|---|
| I1 | Equipo de atención al cliente entrenado y operativo | COO | S | 4 sem | Todo |
| I2 | Procedimiento de retiros y redenciones operativo (tests end-to-end) | COO + CTO | S | 4 sem | Todo |
| I3 | Plan de **gestión de crisis** y comunicación pública | COO + PR | S | 2 sem | Todo |
| I4 | Plan de **price-feed / oracle redundancy** para precios on-chain | CTO | S | 3 sem | Todo |
| I5 | Cobertura legal en jurisdicciones donde haya usuarios "passporteados" | Legal externo | N | continuo | Todo |

## J. Compliance continuo (post-launch) (6 ítems)

| # | Ítem | Resp. | Bloq. | Frecuencia | Estado |
|---|---|---|---|---|---|
| J1 | **Reporte anual** a NCA conforme a MiCA | COO | N | Anual | — |
| J2 | **Auditoría AML** externa | Auditor externo | N | Anual–trienal | — |
| J3 | **Re-screening** continuo de sanciones, PEP y adverse media | AML | N | Diario | — |
| J4 | **Re-KYC** clientes existentes (refresh) | AML | N | 12–36 meses | — |
| J5 | **Auditoría de smart contracts** tras cada upgrade significativo | CTO | N | Por evento | — |
| J6 | **Risk assessment** anual actualizado | AML + COO | N | Anual | — |

---

## Resumen económico estimado pre-lanzamiento

| Categoría | Rango bajo | Rango alto |
|---|---|---|
| Capital regulatorio CASP (depositado, no gastado) | €50.000 | €150.000 |
| Legal externo (CASP + GDPR + docs) | €80.000 | €300.000 |
| Stack KYC/AML/Travel Rule (año 1) | €40.000 | €150.000 |
| Auditoría smart contracts | €30.000 | €150.000 |
| DORA / Pentest / CISO setup | €30.000 | €200.000 |
| DPO externo (año 1) | €20.000 | €60.000 |
| Seguros (D&O, cyber, responsabilidad) | €15.000 | €80.000 |
| Notaría, registros, tasas administrativas | €5.000 | €30.000 |
| Marketing legal (revisiones, claims verificación) | €10.000 | €40.000 |
| Personal compliance interno (MLRO + analistas, año 1) | €120.000 | €400.000 |
| Contingencia (15%) | — | — |
| **Total estimado (sin contingencia)** | **~€400.000** | **~€1.560.000** |
| **Total con contingencia 15%** | **~€460.000** | **~€1.800.000** |

**Nota**: si se añade el régimen MiFID II (securities + investment firm + prospecto), sumar **€300.000–€1.500.000** adicionales.

---

## Resumen de plazos estimados pre-lanzamiento

- **Setup entidad + capital**: 2–4 meses.
- **Solicitud CASP completa y presentada**: +3–4 meses adicionales.
- **Autorización CASP otorgada**: +6–12 meses adicionales.
- **Total mínimo realista**: **9–15 meses** desde decisión a operación productiva en UE.

---

## Top 5 ítems críticos para empezar **YA**

1. **A1 — Decidir jurisdicción** (Francia recomendada).
2. **G1 — Decidir si rediseñamos las fracciones de cosechas** para evitar MiFID II (ahorra €500k–€1.5M y 12+ meses).
3. **B1 — Pre-application meeting con la NCA** para feedback informal antes de invertir.
4. **A2/A3 — Constituir entidad + abrir banca** (la banca es el cuello de botella oculto: 4–12 semanas KYC bancario).
5. **C1/E1 — Designar MLRO y DPO** (sin estos roles no hay solicitud CASP posible).

---

*Fin del checklist. Revisar y completar con responsables nominados y fechas.*
