# 04 — Marco KYC / AML

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Los umbrales y procesos deben ajustarse al riesgo de la operación y a la regulación nacional aplicable. Validar con DPO / abogado / oficial AML.

---

## 1. Marco normativo aplicable

| Norma | Contenido |
|---|---|
| Directiva (UE) 2015/849 (4AMLD) | Marco AML base |
| Directiva (UE) 2018/843 (5AMLD) | Incluye VASPs como obliged entities |
| Directiva (UE) 2018/1673 (6AMLD penal) | Tipos penales de blanqueo |
| **Reg. (UE) 2024/1624 (AMLR)** | Reglamento AML directamente aplicable. Plena aplicación: **10 julio 2027** [VERIFICAR] |
| **Directiva (UE) 2024/1640 (6AMLD revisada)** | Sustituye 4AMLD/5AMLD. Trasposición hasta 10 julio 2027 [VERIFICAR] |
| **Reg. (UE) 2024/1620** | Crea **AMLA** (autoridad UE AML), supervisión directa para entidades de alto riesgo desde 2028 |
| **Reg. (UE) 2023/1113** | "Travel Rule" para transferencias de fondos y criptoactivos. Aplicable desde 30 dic 2024 |
| MiCA art. 68 | Obligaciones AML para CASPs |
| Sanctions: Reg. (UE) 2580/2001, 881/2002, sanciones contra Rusia, etc. | Listas consolidadas |

---

## 2. Tiers de KYC propuestos

El diseño siguiente es una propuesta de tres niveles basada en riesgo. Los umbrales son **orientativos** y deben ajustarse al **risk-based approach** (RBA) requerido por AMLD/AMLR y a la guía de la NCA. [VERIFICAR con oficial AML].

### Tier 0 — Acceso de exploración
- **Datos recolectados**: email verificado, IP/geo, device fingerprint, aceptación de T&Cs y aviso de privacidad.
- **Permite**: navegar, ver precios, simular compras. **No** depósitos ni operaciones.
- **Justificación**: no hay relación de negocio ni transacción → fuera del ámbito CDD.

### Tier 1 — Operaciones de bajo importe
- **Trigger**: usuario quiere depositar o transaccionar.
- **Datos**: nombre completo, fecha y lugar de nacimiento, nacionalidad, domicilio declarado, documento de identidad oficial (DNI/pasaporte) con verificación biométrica (liveness check), datos de contacto.
- **Verificación**: documento de identidad + **liveness** + screening (sanciones + PEP).
- **Límites operativos sugeridos** (orientativos, ajustar por RBA):
  - Depósito/retiro acumulado **<€1.000/mes**.
  - Operación individual **<€1.000**.
- **Vigencia**: re-KYC cada 36 meses si bajo riesgo.

### Tier 2 — Operaciones de importe medio/alto
- **Trigger**: usuario supera el umbral Tier 1 o quiere acceder a productos restringidos.
- **Datos adicionales**: prueba de domicilio (utility bill / extracto bancario <3 meses), **proof of source of funds** (recibo de salario, declaración fiscal, comprobante de venta de activo), beneficial ownership (si actúa por cuenta de PJ), propósito y naturaleza de la relación de negocio.
- **Verificación**: documental + screening reforzado + entrevista (videollamada) si trigger alto riesgo.
- **Sin límite operativo** (sujeto a monitorización continua).
- **Vigencia**: re-KYC cada 12–24 meses según riesgo.

### Tier 3 — Enhanced Due Diligence (EDD)
Obligatorio para (AMLR art. 28–36 / [VERIFICAR numeración]):
- **PEPs** (Politically Exposed Persons), familiares y allegados.
- Clientes de **jurisdicciones de alto riesgo** (lista FATF + lista UE de terceros países de alto riesgo, Reg. Delegado (UE) 2016/1675 y actualizaciones).
- **Beneficial owners** opacos / estructuras complejas.
- Transacciones inusualmente grandes o sin propósito económico aparente.
- **Acciones**: aprobación senior, source of wealth (no solo of funds), monitoreo reforzado, revisión periódica más frecuente.

---

## 3. Travel Rule (Reg. UE 2023/1113)

Aplica a **todas las transferencias de criptoactivos** efectuadas por CASPs (sin umbral mínimo significativo, salvo simplificaciones para self-hosted wallets y transferencias <€1.000 entre CASPs UE en ciertos casos).

### 3.1. Datos del originador que deben transmitirse
- Nombre completo.
- Dirección de la cuenta de criptoactivos (wallet) del originador.
- Dirección postal, número de documento oficial, identificador de cliente, o fecha y lugar de nacimiento.
- Identificador legal de la entidad (LEI) si aplica.

### 3.2. Datos del beneficiario
- Nombre completo.
- Dirección de la cuenta de criptoactivos del beneficiario.

### 3.3. Caso self-hosted wallet (unhosted)
- Verificación de **control** sobre la wallet (firma de mensaje, micro-depósito, etc.) si el importe **>€1.000** acumulado en un periodo razonable [VERIFICAR umbral exacto].
- Posible aplicar **medidas reforzadas** si origen/destino es wallet unhosted.

### 3.4. Transferencias a/desde CASPs fuera UE
- Solo permitidas con CASPs que cumplan estándares equivalentes (FATF Recomendación 16).
- Bloqueo automático de wallets en listas de sanciones o mixers.

### 3.5. Implementación técnica
Protocolos: **TRP (Travel Rule Protocol)**, **TRISA**, **OpenVASP**, **Sumsub Travel Rule**, **Notabene**, **VerifyVASP**, **Veriscope (Shyft)**.

---

## 4. Sanctions screening

### 4.1. Listas que deben consultarse
- **EU Consolidated List** (mantenida por el SEAE / Comisión).
- **OFAC SDN List** (Tesoro EEUU).
- **UK HMT Consolidated List**.
- **ONU** — Consolidated UN Security Council Sanctions List.
- Listas nacionales del Estado miembro (p. ej. listas francesas DGT, alemanas Bundesbank).

### 4.2. Cuándo screenear
- **Onboarding**: nuevo cliente y todos los beneficial owners declarados.
- **Continuo**: re-screening diario contra cambios en listas.
- **Por transacción**: contraparte / wallet receptora.
- **Por modificación**: cuando el cliente actualiza datos.

### 4.3. Acciones ante match
- True positive: **freeze inmediato** de cuenta y fondos.
- **Reporte** a la autoridad nacional sancionadora (no es la FIU, son autoridades distintas: Tesoro/Ministerio Exteriores).
- No comunicación al cliente ("tipping off" prohibido).

---

## 5. PEP screening

- Identificar **PEPs domésticos, extranjeros y de organizaciones internacionales**, así como familiares directos y close associates.
- Tratar como **alto riesgo automáticamente** → EDD.
- Aprobación senior obligatoria para iniciar/mantener relación.
- Re-screening continuo (las personas cambian de estatus PEP).

---

## 6. Transaction monitoring

### 6.1. Reglas mínimas sugeridas
- **Umbral absoluto**: cualquier operación >€10.000 → alerta (alineado con umbrales de reporte en efectivo de varios EM).
- **Cumulative**: >€15.000 en 24h, >€50.000 en 30 días → alerta. [VERIFICAR umbrales del RBA y la guía nacional].
- **Patrones de smurfing**: múltiples operaciones justo por debajo del umbral.
- **Velocidad anómala**: ramp-up rápido vs. perfil declarado.
- **Geografía**: operaciones desde/a jurisdicciones de alto riesgo.
- **Counterparty risk**: wallet vinculada a mixers (Tornado Cash sancionado), darknet markets, ransomware, hacks conocidos.
- **Cross-chain hopping**: uso de bridges para ofuscar origen.
- **Salida total inmediata** tras depósito grande.

### 6.2. Workflow
1. Alerta automática → triage por analista L1.
2. L1 cierra como false positive **o** escala a L2.
3. L2 investiga (Chainalysis / Elliptic / TRM), entrevista al cliente si aplica.
4. Decisión: cerrar, congelar, reportar SAR a FIU.

---

## 7. Reporte de operaciones sospechosas (SAR / STR)

- **Destino**: FIU del Estado miembro (TRACFIN Francia, FIU.NL Países Bajos, SEPBLAC España, FIU Lituania, FIU Malta, FIU Irlanda, FIU Alemania).
- **Plazo**: "sin demora" (típicamente <24–72h desde detección, [VERIFICAR plazo nacional]).
- **Contenido**: identidad cliente, descripción operación, motivos de sospecha, documentación soporte.
- **Confidencialidad**: tipping-off prohibido (delito en la mayoría de EM).
- **Conservación**: registros 5 años desde fin de la relación (10 años en algunos EM). [VERIFICAR].

---

## 8. Gobernanza AML interna

- **AML Officer / MLRO** (Money Laundering Reporting Officer) — persona de alto nivel responsable, comunicada a la NCA.
- **Comité AML** trimestral.
- **Risk assessment empresarial** anual + por producto + por cliente.
- **Manual AML** aprobado por consejo, revisado anualmente.
- **Formación AML** anual obligatoria para todo el personal con responsabilidad relevante.
- **Auditoría interna** AML al menos anual; auditoría externa cada 1–3 años.

---

## 9. Stack tecnológico sugerido

| Función | Opciones | Pros | Cons |
|---|---|---|---|
| **KYC / Identity Verification** | Sumsub | Cobertura global, multi-doc, biometría, automatización alta | Coste; UX puede ser pesada en mercados emergentes |
|  | Onfido | UX pulida, fuerte en UE/UK, partner de muchos bancos | Precio por verificación elevado |
|  | Veriff | Excelente verificación documental, soporte 40+ idiomas | Menos AML reforzado integrado |
|  | iDenfy | Coste competitivo, sede UE | Cobertura geográfica más limitada |
| **AML / Transaction Monitoring on-chain** | Chainalysis (KYT / Reactor) | Estándar de mercado, mejor cobertura, partners regulatorios | Coste alto; menos sensible a chains nicho |
|  | Elliptic (Navigator / Lens) | Excelente UI, fuerte UK/UE | Coste alto |
|  | TRM Labs | Buena cobertura Solana/Cosmos, precio competitivo | Menor footprint regulatorio que Chainalysis |
|  | Crystal Intelligence | Fuerte en EU/CIS | Marca menos consolidada |
| **Travel Rule** | Notabene | Más adopción VASP, multi-protocolo | Coste |
|  | Sumsub Travel Rule | Integrado con KYC mismo proveedor | Menos integraciones cross-VASP |
|  | VerifyVASP | Fuerte en Asia/UE | Coste |
| **Sanctions / PEP / Adverse Media** | Refinitiv World-Check | Standard de mercado | Caro |
|  | ComplyAdvantage | API-first, buen pricing fintech | Cobertura PEP algo más débil que WC |
|  | Dow Jones Risk & Compliance | Premium | Caro, integraciones más lentas |
| **Case management / SAR** | Hummingbird, Unit21, Alessa | Workflow AML completo | Coste |
|  | In-house sobre Jira/Linear | Barato, flexible | Menos auditabilidad |

### 9.1. Recomendación de stack inicial (PoC → producción)
1. **Sumsub** (KYC + AML básico + travel rule en bundle) — barato de empezar, todo en uno.
2. **Chainalysis KYT** o **TRM Labs** para on-chain monitoring (Chainalysis si presupuesto, TRM si optimizar coste).
3. **ComplyAdvantage** para sanctions/PEP/adverse media continuo.
4. **Case management** en **Unit21** o sobre Notion/Jira para PoC.

**Coste estimado total stack**: €40k–€150k año 1, escalando con volumen. [VERIFICAR cotizaciones reales].

---

## 10. Aviso final

El RBA (risk-based approach) es **el principio rector**: los umbrales y procesos deben **calibrarse al riesgo real** del producto, base de clientes y geografías. No copiar literalmente este documento; usar como base y ajustar con el oficial AML y la NCA.

---

*Fin del documento 04.*
