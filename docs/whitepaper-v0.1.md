# AgroGlobalDex — Technical Whitepaper v0.1

**Versión:** 0.1 (borrador para hackathon, sujeto a revisión legal y técnica)
**Fecha:** Mayo 2026
**Autor:** Carlos Pallottini — Founder
**Estado:** Pre-MVP · Proof of Concept

> Este documento describe la arquitectura técnica y legal **prevista** para AgroGlobalDex. **Ningún componente está en producción a fecha de publicación.** Toda referencia a vías regulatorias, smart contracts, integraciones o coste presupone validación previa por abogado colegiado en ejercicio.

---

## 1. Problema

El mercado agropecuario español mueve **>€50.000M/año** de producción final, con cooperativas y productores familiares como núcleo del sector. Tres problemas estructurales:

1. **Plazos de cobro de 90–180 días** desde la entrega a la gran distribución (Mercadona, Carrefour, Lidl, Día). El productor financia el ciclo de explotación con su propio capital o asume factoring caro.
2. **Coste real del adelanto de circulante**: 8 %–18 % anual vía factoring tradicional, condicionado a aval personal en muchos casos. La banca tradicional ha replegado su división agro (BBVA cerró la suya en 2023).
3. **Sin acceso del inversor minorista** a la producción real. Los fondos agrarios institucionales tienen tickets de €100K+. El inversor de €100–€5.000 no puede invertir en una cosecha, una explotación lechera o un lote de wagyu.

Mientras tanto, el inversor minorista español:
- Tiene cuentas remuneradas al 2,1 % nominal.
- Recibe oferta cripto al 4–5 % vía stables en CEX, sin entender el riesgo de contraparte.
- No conoce vías para invertir directamente en activos reales del sector primario español.

**AgroGlobalDex se propone resolver ambos lados** con un marketplace tokenizado bajo estructura SPV regulada.

---

## 2. Solución a alto nivel

Cada producción tokenizada en AgroGlobalDex tendrá:

1. **Una SPV** (sociedad limitada española o cooperativa) propietaria/usufructuaria del activo físico.
2. **Un contrato vinculante** entre la SPV y el productor con obligaciones de entrega, custodia, seguros y derechos de inspección.
3. **Un token Solana (Token-2022)** representativo de cuotas o participaciones de la SPV, con `TransferHook` que impone allowlist KYC.
4. **Un oráculo (Switchboard On-Demand)** que publica precio spot del commodity desde fuentes validadas independientes.
5. **Un programa Anchor de distribución de yield** que recibe USDC del productor al cierre del ciclo y distribuye pro-rata a holders.

El inversor compra el token con EUR (vía on-ramp SEPA → USDC) o con cripto (BTC/ETH/SOL/USDC). Al final del ciclo de producción (3–18 meses según activo), recibe su participación en USDC, convertible a EUR si lo prefiere.

---

## 3. Arquitectura técnica

### 3.1 Contratos Solana

#### a) `agroglobaldex_asset_factory` (Anchor program)

Responsable de instanciar cada producción tokenizada como un Token-2022 con extensiones específicas.

```
PDA: ["asset", spv_pubkey, asset_id]
Estado:
  - producer_id: Pubkey (firmante de la SPV)
  - mint: Pubkey (Token-2022 con extensions)
  - mint_authority: PDA
  - cycle_start: i64
  - cycle_end: i64
  - asset_class: enum (Lacteo, Carnico, Cosecha, Olivar, Ganaderia)
  - oracle_feed: Pubkey (Switchboard On-Demand)
  - kyc_allowlist: Pubkey (Civic gateway o lista interna)
  - status: enum (Initialized, Active, Distributing, Closed)
```

Extensiones Token-2022 activas obligatoriamente:
- **`MetadataPointer`** → puntero a Arweave con: nombre producto, productor, DOP/IGP si aplica, certificados sanitarios, fotografías georeferenciadas, contrato PDF.
- **`TransferHook`** → invoca programa hook que valida que el receptor está en la `kyc_allowlist`.
- **`PermanentDelegate`** → permite a la SPV congelar/recuperar tokens por orden judicial (exigible bajo MiCA art. 68).
- **`MintCloseAuthority`** → permite cerrar el mint al final del ciclo.
- **`InterestBearing`** *(condicional)* → solo para activos con yield continuo verificable (leche en streaming mensual). NO para activos one-shot (cosecha, lote de wagyu).

#### b) `agroglobaldex_revenue_distribution`

Recibe USDC del productor (o de la SPV) y distribuye pro-rata a holders.

```
Instrucciones:
  - register_payment(amount: u64, asset: Pubkey)
  - claim_share(holder: Pubkey)
  - close_cycle(asset: Pubkey)

Patrón: pull (no push) para no pagar rent de N transacciones.
Snapshot de holders: via Helius DAS API on-demand, almacenado on-chain como Merkle root.
Verification: holder provee Merkle proof al hacer claim.
```

Seguridad:
- `checked_add` / `checked_sub` / `checked_mul` everywhere.
- Reentrancy guard vía estado `is_distributing: bool`.
- CPI signer validation estricta antes de transferir USDC.

#### c) `agroglobaldex_kyc_hook`

Implementa el Transfer Hook del Token-2022.

```
on_transfer(src, dst, amount):
  - require!(allowlist.contains(dst), Unauthorized);
  - require!(!sanctions.contains(dst), SanctionedAddress);
  - emit TransferAllowed { src, dst, amount, ts };
```

La `allowlist` se sincroniza desde el backend (Helius Webhooks → Anchor instruction `update_allowlist`) cuando el KYC provider (Sumsub) confirma la verificación del inversor.

### 3.2 Oráculo de precios

**Switchboard On-Demand** (no Pyth — Pyth no cubre commodities regionales como leche cooperativa española, wagyu europeo, cosechas concretas).

Para cada activo tokenizado se define un **job custom** firmado por:
1. La cooperativa productora.
2. Un agregador independiente (Mercolleida para porcino, SOEX para cereales, Lonja de Albacete para olivar).
3. Un auditor externo (boutique tipo Agroconsulting).

El precio publicado on-chain es la **mediana TWAP 24h** de las 3 fuentes. Circuit breaker: si `now - last_update > 86400s`, las operaciones se pausan automáticamente.

### 3.3 Custodia técnica

- **Wallets institucionales:** Fireblocks o Anchorage. Hot wallet operativa con cap diario, cold storage para >€50K. Seguro de custodia ≥ €5M.
- **Wallets de usuario:** Privy embedded wallet (email/social login con recovery propio) o conexión a wallet propia (Phantom, Solflare).
- **Segregación**: cumplimiento estricto art. 70 MiCA — los activos de cliente NUNCA en wallet de la operadora.

### 3.4 Identidad y KYC

- **Civic Pass** on-chain (Solana-native) para verificación básica (€1–2/usuario, latencia segundos).
- **Sumsub** para enhanced due diligence en tickets > €1.000 (residencia, sanciones, PEP). Bridge a Solana Attestation Service.
- **Travel Rule** (Reg. UE 2023/1113): integración con Notabene o Sumsub Travel Rule.

### 3.5 On-ramp / Off-ramp fiat

Inversor europeo paga en EUR → recibe USDC en su wallet → compra token AgroGlobalDex.

Proveedores planificados:
- **Banxa** o **MoonPay** para tarjeta.
- **Bit2Me Pay** o **Bitvavo** vía SEPA para tickets >€500.
- Salida: liquidación en USDC, conversión a EUR a través del mismo bridge SEPA.

### 3.6 Frontend

- **Next.js** (sustituyendo el HTML estático actual una vez se implemente).
- **`@solana/wallet-adapter`** + **Privy** para onboarding.
- **TanStack Query** para data fetching, **Helius DAS API** como backend de lectura.
- **Sentry** para error monitoring.
- **PostHog** para producto/analytics anonimizado, GDPR-compliant.

### 3.7 Infraestructura

- **Helius Business RPC** (tier estándar al inicio, $500/mes; Business cuando MAU > 10K).
- **Triton One** como fallback RPC.
- **Arweave (Irys SDK)** para almacenamiento permanente de certificados y metadata.
- **Vercel** o **Fly.io** para frontend + APIs.
- **Cloudflare** para CDN y DDoS protection.
- **Datadog** o **Grafana Cloud** para observability.

---

## 4. Arquitectura legal (resumen — el documento legal lo lleva el equipo de compliance)

### 4.1 Estructura societaria

```
AgroGlobalDex Group, S.L.  (HoldCo · Madrid)
├── AgroGlobalDex Markets, S.A.  (OpCo CASP MiCA · capital €125K)
│   └── Custodia y onboarding
├── Asset SPV #1 — Ej: "Lacteos Asturias 2026 Q3, S.L."
├── Asset SPV #2 — Ej: "Cereales Castilla 2026 Q4, Cooperativa"
└── Asset SPV #N — una por producción
```

### 4.2 Clasificación del token

El token AgroGlobalDex representa derechos económicos sobre una producción real, lo que muy probablemente lo califica como **valor negociable** ex art. 2.1.b/c Ley 6/2023 (Mercados de Valores y Servicios de Inversión) y por tanto queda **excluido de MiCA** (art. 2.4.a Reg. UE 2023/1114).

Vía regulatoria prevista: **Folleto simplificado Art. 1.4 Reg. UE 2017/1129** (ofertas ≤ €8M en 12 meses con documento informativo presentado a CNMV) + autorización CASP MiCA en paralelo para servicios accesorios. Decisión a confirmar con abogado colegiado.

### 4.3 KYC/AML

Sujeto obligado bajo Ley 10/2010 PBC/FT desde día 1. KYC obligatorio para todos los usuarios. Travel Rule activa para transferencias ≥ €1.000. Reportes SEPBLAC y DAC8 a AEAT.

### 4.4 Privacidad

Datos personales únicamente off-chain (cumplimiento RGPD). On-chain solo hashes y wallets. DPIA obligatoria. DPO designado.

---

## 5. Tokenomics — sin token propio AGRO

**Decisión explícita: no hay token nativo AGRO.**

- Solo existen **tokens por activo** (uno por producción tokenizada), todos respaldados por su SPV y su contrato.
- No hay airdrop, ni TGE, ni vesting de fundador, ni allocation a equipo.
- Modelo de ingreso de AgroGlobalDex: **take-rate 2,0–3,5 %** sobre transacciones primarias + **0,5–1 % management fee anual** sobre AUM tokenizado. Cero rentas extractivas vía token propio.

Esto elimina el 80 % del riesgo regulatorio asociado a token issuance y posiciona a AgroGlobalDex como **infraestructura, no como activo especulativo**.

---

## 6. Roadmap de ingeniería

| Sprint | Sem | Entregable verificable |
|---|---|---|
| S0 | 1–2 | Anchor workspace, CI con `anchor test`, borrador whitepaper v1.0 |
| S1 | 3–4 | `asset_factory` Anchor en devnet con tests mint/transfer |
| S2 | 5–6 | `kyc_hook` integrado con Civic Pass, allowlist sync |
| S3 | 7–8 | Oracle Switchboard On-Demand custom para 1 activo |
| S4 | 9–10 | `revenue_distribution` con snapshot + pull-claim |
| S5 | 11–12 | Frontend Next.js + wallet-adapter + Privy + 1 asset devnet E2E |
| S6 | 13–14 | Pool Meteora DLMM secondary market |
| S7 | 15–16 | Audit prep + fuzz testing + handoff a OtterSec/Halborn |

Post-audit (semanas 17–22): fixes, mainnet beta cerrado (50 wallets, cap €50K), apertura progresiva.

---

## 7. Coste y captación

**Runway hasta MVP mainnet:** €220–280K (mínimo) / €450–550K (cómodo).

Vías de financiación priorizadas:
1. **Solana Foundation Grant** — $50–75K, milestone-based.
2. **CDTI Neotec** — €325K subvención a fondo perdido (España).
3. **ENISA** — €25–300K préstamo participativo.
4. **Hyperdrive (Colosseum)** — $25K preseed + demo day.
5. **Ecosystem funds**: RockawayX (EU), Helius Ventures, Hashed, Big Brain.
6. **Angels cripto-ES** una vez con devnet funcional.

---

## 8. Riesgos top

| # | Riesgo | Mitigación |
|---|---|---|
| 1 | Discordancia on-chain vs físico (productor incumple) | SPV con seguro Agroseguro + auditor físico trimestral + IoT attestation |
| 2 | Recalificación regulatoria como valor (MiFID estricto) | Análisis con abogado, vía Art. 1.4 + consulta previa CNMV |
| 3 | Oracle manipulation / stale feed | Multi-fuente TWAP + circuit breaker + monitoreo 24/7 |
| 4 | Hack smart contract | Auditoría OtterSec/Halborn + fuzz testing + bug bounty |
| 5 | KYC bypass vía Transfer Hook | Auditoría específica del hook + tests adversarios |

---

## 9. Aviso

Este whitepaper es un **borrador técnico**. Su contenido no constituye oferta de valores, asesoramiento financiero, ni invitación a participar en operación alguna. Ninguna decisión vinculante se tomará sin previa validación legal, técnica y regulatoria por terceros independientes colegiados.

— Fin del documento —
