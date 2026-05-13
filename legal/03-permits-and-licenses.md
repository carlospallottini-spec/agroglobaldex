# 03 — Permisos y licencias necesarios

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Los plazos, costes y requisitos son **orientativos**; los exactos dependen del Estado miembro y de la NCA. Validar con abogado matriculado UE.

---

## 1. Mapa general

AgroGlobalDex, en función del diseño actual, requiere un combo regulatorio que combina:

1. **Autorización CASP MiCA** (núcleo de la operación).
2. **Registro AML/CFT** como obliged entity (paralelo a CASP, normalmente integrado).
3. **(Eventual) Autorización investment firm + prospecto** si se mantienen fracciones de cosechas como securities.
4. **Cumplimiento GDPR**, con eventual nombramiento de DPO.
5. **Cumplimiento sectorial**: carbono (EUDR / CRCF / claims ambientales), commodities (regulación nacional de almacenes / warrants).
6. **Registro mercantil** y autorizaciones administrativas generales de la entidad.

---

## 2. Detalle de cada permiso

### 2.1. Autorización CASP bajo MiCA

| Concepto | Detalle |
|---|---|
| Norma | Reg. UE 2023/1114 (MiCA) Título V, art. 59–74 |
| Regulador (NCA) | Autoridad nacional del Estado miembro de origen (AMF/ACPR Francia, BaFin Alemania, CNMV España, AFM/DNB Países Bajos, Banco de Lituania, MFSA Malta, Central Bank Irlanda) |
| Pasaporte | Sí — válida en toda la UE/EEE tras notificación cross-border (art. 65) |
| Capital mínimo | Anexo IV MiCA: Clase 1 €50.000 / Clase 2 €125.000 / Clase 3 €150.000 según servicios. [VERIFICAR cifras en anexo vigente] |
| Plazo objetivo | ~3 meses legales (completitud + decisión). En la práctica: **6–12 meses** total incluyendo preparación. |
| Coste estimado | Legal + consultoría: **€80.000–€300.000**. Tasas NCA variables. [VERIFICAR] |
| Documentación clave | Programa de actividades, gobierno interno, plan de continuidad, políticas AML, política de custodia y segregación, plan operativo, fit & proper de directivos y socios significativos (>10–20%), descripción técnica, ciberseguridad, externalización |
| Bloqueante para lanzar | **SÍ** |

### 2.2. Régimen AML/CFT — obliged entity

| Concepto | Detalle |
|---|---|
| Norma | Directiva (UE) 2015/849 (4AMLD), modificada por (UE) 2018/843 (5AMLD); paquete AML 2024: **Reg. (UE) 2024/1624 (AMLR)**, **Directiva (UE) 2024/1640 (6AMLD)**, **Reg. (UE) 2024/1620** creando la **AMLA** (autoridad europea). |
| Aplicabilidad | Toda CASP es obliged entity directamente. La aplicación plena del paquete AML 2024 se produce el **10 de julio de 2027** [VERIFICAR fecha en versión vigente del AMLR]. Hasta entonces, transposición nacional de 4AMLD/5AMLD/6AMLD. |
| Obligaciones | KYC/CDD, EDD para alto riesgo, monitoreo continuo, sanctions screening, PEP screening, reporting de operaciones sospechosas (SAR) a la **FIU** nacional, registros 5–10 años |
| Travel Rule | **Reg. (UE) 2023/1113** sobre información que acompaña a las transferencias de fondos y determinados criptoactivos — aplicable desde 30 dic 2024. Toda transferencia de criptoactivos (cualquier importe, no hay umbral mínimo significativo) debe llevar datos de originador y beneficiario. |
| Plazo | Integrado en la solicitud CASP; no es licencia separada en la mayoría de Estados |
| Coste | Stack tecnológico (KYC + sanctions + transaction monitoring): **€30.000–€150.000/año** [VERIFICAR según volumen] |
| Bloqueante | **SÍ** |

### 2.3. Autorización investment firm + prospecto (solo si mantenemos cosechas como security)

| Concepto | Detalle |
|---|---|
| Norma | MiFID II (Dir. 2014/65/UE), IFR/IFD (Reg. 2019/2033 / Dir. 2019/2034), Prospectus Reg. (UE) 2017/1129 |
| Regulador | NCA del Estado miembro de origen |
| Capital mínimo | Depende de la clase de ESI: desde €75.000 hasta €750.000 (MTF/OTF). [VERIFICAR cifras IFR vigente] |
| Plazo | **12–18 meses** |
| Coste | **€300.000–€1.500.000** considerando prospecto auditado + legal + tecnológico |
| Bloqueante | **SÍ si no rediseñamos las cosechas a utility token** |

### 2.4. DLT Pilot Regime (opcional)

| Concepto | Detalle |
|---|---|
| Norma | Reg. (UE) 2022/858 |
| Aplicabilidad | Plataformas de negociación y liquidación basadas en DLT para securities tokenizados, con sandbox regulatorio |
| Plazo | Aplica desde marzo 2023; vigente |
| Útil para | Operar securities tokenizados con exenciones puntuales de MiFID II/CSDR |
| Bloqueante | NO (es opcional, alternativa al régimen estándar) |

### 2.5. GDPR / Privacidad

| Concepto | Detalle |
|---|---|
| Norma | Reg. (UE) 2016/679 (GDPR) + leyes nacionales (LOPDGDD España, etc.) |
| Designación DPO | Obligatoria si: (a) tratamientos a gran escala de datos personales, (b) monitorización sistemática, (c) datos sensibles. Para una marketplace fintech con KYC: **muy probable obligatoria** (GDPR art. 37). |
| Registro de actividades de tratamiento | Obligatorio si >250 empleados, o tratamiento no ocasional, o datos sensibles. Para CASP: **obligatorio en la práctica**. |
| DPIA | Obligatoria para tratamientos de alto riesgo (art. 35). KYC + AML monitoring = alto riesgo. |
| Plazo | Antes de iniciar el tratamiento |
| Coste | DPO externo: €20.000–€60.000/año; DPIA inicial: €5.000–€20.000 [VERIFICAR] |
| Bloqueante | **SÍ** |

### 2.6. Carbono — específico

| Concepto | Detalle |
|---|---|
| EU ETS (si entráramos) | Cuenta en el Registro de la Unión — solo si tokenizáramos EUAs. **Recomendación: NO en PoC.** |
| Mercados voluntarios | Cuenta de cuenta-titular en registros Verra (VCS), Gold Standard, etc. **No requiere autorización pública**, pero sí KYC del operador del registro |
| EUDR (Reg. UE 2023/1115) | Diligencia debida sobre cadena de suministro de productos de riesgo (incluye carbono forestal). Aplicación general aplazada — [VERIFICAR fecha actualizada]. |
| CRCF (Reg. UE 2024/3012) | Marco voluntario de certificación de absorciones de carbono. En implementación. |
| Green Claims | Directiva (UE) 2024/825 + propuesta Green Claims Directive: claims ambientales verificables. |
| Bloqueante | **Parcial**: solo si comercializamos carbono. |

### 2.7. Comercio de commodities físicos — específico

| Concepto | Detalle |
|---|---|
| Warehouse warrants | Régimen nacional de cada Estado miembro (ej. Francia: warrants agricoles; España: warrants de almacén general de depósito) |
| Almacenes certificados | Acuerdos con almacenes que cumplan estándares nacionales/EUREX para entrega física |
| FCD MiFID II carve-out | Operación de commodity físico sin componente derivado no es instrumento financiero (MiFID II Anexo I C(7) tiene matices con liquidación física vs. financiera) |
| Bloqueante | Operacional, no licencia |

### 2.8. Entidad jurídica y registro mercantil

| Concepto | Detalle |
|---|---|
| Forma | Sociedad limitada / SA / SAS según Estado miembro |
| Capital social mínimo | El requerido por la forma jurídica + por la licencia CASP (el mayor de los dos) |
| Registro | Registro Mercantil + Beneficial Ownership Register (Ley AML) |
| Plazo | 2–6 semanas |
| Coste | €2.000–€10.000 setup + capital |
| Bloqueante | **SÍ** |

### 2.9. Ciberseguridad — DORA

| Concepto | Detalle |
|---|---|
| Norma | Reg. (UE) 2022/2554 — Digital Operational Resilience Act |
| Aplicabilidad | **Sí** — los CASP están incluidos en el ámbito subjetivo de DORA (art. 2(1)(o)) |
| Vigencia | 17 de enero de 2025 |
| Obligaciones | Gestión de riesgos TIC, gestión de incidentes, tests de resiliencia, gestión de riesgo de terceros TIC, intercambio de información |
| Plazo | Antes de operar |
| Coste | €30.000–€200.000 setup; auditorías anuales |
| Bloqueante | **SÍ** |

### 2.10. Protección al consumidor

| Concepto | Detalle |
|---|---|
| Norma | Directiva 2011/83/UE (Consumer Rights Directive), Directiva 2005/29/CE (UCPD), Directiva 93/13/CEE (Unfair Contract Terms) |
| Aplicabilidad | A usuarios retail (no profesionales) |
| Obligaciones | Información precontractual, derecho de retracto (MiCA art. 14 establece 14 días para criptoactivos OCA), cláusulas no abusivas, geo-blocking compliance |
| Bloqueante | **SÍ** |

---

## 3. Tabla resumen accionable

| # | Permiso / Cumplimiento | Regulador | Plazo (preparación + tramitación) | Coste estimado | Bloqueante |
|---|---|---|---|---|---|
| 1 | Constitución entidad UE | Registro Mercantil | 4–8 semanas | €5–10k + capital | **SÍ** |
| 2 | Capital social + cuenta bancaria | Banco UE | 4–12 semanas (KYC banco) | Capital mínimo €50–150k | **SÍ** |
| 3 | Autorización CASP MiCA | NCA Estado miembro | 6–12 meses | €80–300k legal | **SÍ** |
| 4 | Registro AML obliged entity | FIU + NCA | Integrado en CASP | Incluido | **SÍ** |
| 5 | Stack KYC/AML/Travel Rule | Privado (Sumsub, Chainalysis...) | 2–3 meses integración | €30–150k/año | **SÍ** |
| 6 | DPO + GDPR compliance | AEPD/CNIL/etc. | 1–2 meses | €20–60k/año | **SÍ** |
| 7 | DORA — resiliencia TIC | NCA | 3–6 meses | €30–200k | **SÍ** |
| 8 | White paper OCA (granos, carbono) | NCA (notificación) | 2–4 meses | €15–50k | **SÍ** |
| 9 | T&Cs + Política privacidad + risk disclosures | Privado | 1–2 meses | €10–30k | **SÍ** |
| 10 | (Si securities) Investment firm + prospecto | NCA | 12–18 meses | €300k–1.5M | **SÍ** (si aplica) |
| 11 | (Si carbono) Cuentas registros voluntarios | Verra / Gold Standard | 1–3 meses | €5–20k | Parcial |
| 12 | (Si carbono) Verificación claims ambientales | Tercero independiente | Continuo | €10–50k/año | Parcial |
| 13 | Marca / propiedad intelectual | EUIPO | 4–6 meses | €1–3k | NO |
| 14 | Acuerdos con warehouses | Privado | 1–6 meses | Variable | Operacional |
| 15 | Auditoría smart contracts | Privado (Trail of Bits, OpenZeppelin...) | 1–3 meses | €30–150k | **SÍ** |

**Total compliance pre-lanzamiento estimado**: ver `10-compliance-checklist.md` para resumen económico y de plazos.

---

## 4. Atajos posibles

1. **Asociarse con un CASP ya autorizado** y operar bajo su licencia (modelo "regulatory umbrella"): acelera time-to-market pero crea dependencia.
2. **Adquirir una entidad con licencia preexistente** (régimen transitorio MiCA art. 143).
3. **Limitar perimetralmente la oferta inicial** a (a) inversores cualificados, (b) jurisdicciones no UE en fase PoC, (c) un Estado miembro único sin pasaporte.
4. **Sandbox regulatorios** disponibles: AMF (Francia), BaFin (Alemania), CNMV (España), MFSA (Malta), Bank of Lithuania. Permiten operar bajo supervisión piloto.

---

*Fin del documento 03.*
