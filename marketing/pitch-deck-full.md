# AgroGlobalDex — Pitch Deck (full text, 14 slides)

> Cada slide listo para copiar a Figma/Pitch/Slidebean.
> Estilo: tono founder serio, sin hype, datos verificables, claim claro por slide.

---

## Slide 1 — Cover

**Título:** AgroGlobalDex
**Subtítulo:** The compliance-first marketplace for tokenized agricultural assets.
**Tagline:** *Tokenize. Aggregate. Connect global capital to real food.*
**Footer:** Pre-seed · Solana · 2026 · MiCA-first

---

## Slide 2 — Problem

**Headline:** El capital global no llega al productor agro porque la infraestructura financiera no fue diseñada para él.

**Body (3 puntos):**
- Productores en mercados emergentes (LATAM, África, sudeste asiático) **dependen de prefinanciación cara** (tasas reales 25–60%) porque no acceden a capital institucional global.
- Inversores institucionales que SÍ quieren exposición a agro no pueden hacerla on-chain de forma compliance-aware: las plataformas existentes están fragmentadas por país, sector y standard.
- Los compradores (traders, retailers, exportadores) no tienen **trazabilidad on-chain auditable** del origen, cantidad y certificaciones — pierden tiempo y dinero en reconciliación.

**Soporte:** mercado global agro = USD 12 trillions/año (FAO 2024). Penetración crypto en agro <0.05%.

---

## Slide 3 — Solution

**Headline:** Una sola plataforma para tokenizar, agregar y tradear cualquier activo agropecuario, con compliance MiCA-grade.

**Body — dos casos de uso:**

**1. Tokenización directa.** Productores tokenizan kg/litros de su producto físico — granos, **carnes, vinos, aceites, lácteos, frutas, vegetales** — en Solana con Token-2022 + Transfer Hook. Cada token representa una unidad física con certificado off-chain verificable on-chain (hash SHA-256).

**2. Agregador global.** AgroGlobalDex es **single front-end** que lista tokens de otras plataformas (Agrotoken, Topaz, RIPE, Centrifuge…) por sector y país, dándoles distribución y a los inversores **una sola interfaz** para invertir cross-border.

**KYC + jurisdicción enforced on-chain en cada transfer** (Transfer Hook). MiCA-aligned desde día uno.

---

## Slide 4 — Why now

**Headline:** Tres ventanas alineándose en los próximos 18 meses.

**Tres puntos:**
1. **MiCA en vigor** (Dec 2024). Por primera vez Europa tiene un marco regulatorio unificado para crypto-assets. Tokenizar bajo MiCA da legitimidad institucional que pre-2024 no existía.
2. **Token-2022 maduro.** Las extensiones de SPL Token-2022 (Transfer Hook, Metadata Pointer, Permanent Delegate) acaban de salir de beta. AgroGlobalDex es de las primeras implementaciones agro production-ready.
3. **Climate-tech capital saturado de "carbon-only".** Los fondos ESG buscan exposición agro real — comida, no solo créditos. AgroGlobalDex es el único marketplace cross-sector + cross-país que les abre ese acceso.

---

## Slide 5 — Product (con captura/mockup)

**Headline:** Cuatro flows en una sola app web + móvil instalable.

**Bullet points** (con screenshots de la web — tomar de /marketplace, /tokenize, /invest, /aggregate):
- **Marketplace.** Lista de activos nativos + agregados, filtros por sector / país / clase.
- **Tokenizar.** Wizard 4 pasos para productores. Calcula SHA-256 del certificado en el navegador, firma con wallet Solana, mintea Token-2022.
- **Invertir.** Yield offerings y commodities físicos, modal de detalle con disclosures MiCA.
- **Agregador.** Admin curated. Tokens RWA-agro de terceros aparecen junto a los nativos.

**Stack:** Solana mainnet (target Q3 2026) · Anchor 0.31 · Token-2022 · PWA + Capacitor Android · multi-wallet (Phantom/Solflare/Backpack).

---

## Slide 6 — Market

**Headline:** TAM masivo, fragmentado, mal servido.

**Datos (cite source):**
- **TAM agro global:** USD 12T producción primaria anual (FAO, 2024).
- **SAM tokenizable inicial:** USD 280B mercado UE/año (Eurostat 2024) — pilot region.
- **SOM 5y target:** 0.1% del SAM UE = **USD 280M GMV anual** vía AgroGlobalDex en año 5.
- **Mercados conocidos hoy:** Agrotoken (USD 30M+ TVL LATAM granos), Topaz (Solana RWA), Centrifuge (Ethereum RWA, $400M+ TVL agnostic). Ningún player cubre cross-sector + cross-país.

---

## Slide 7 — Business model

**Headline:** Tres fuentes de ingreso, ya implementadas on-chain.

**Tabla:**

| Fuente | % | Quién paga | Cuándo |
|---|---|---|---|
| **Trading fee** | 0.5% (50 bps) | Buyer | En cada `buy_asset` / `buy_external_asset` |
| **Listing fee** (futuro) | Flat USD | Productor | Al `register_asset` |
| **Aggregator partnership** (B2B) | Rev-share % | Plataforma agregada | Mensual sobre volumen referido |

**Take-rate proyectada año 5:** 0.7% promedio efectivo sobre GMV.

**Caso 1 (tokenización directa):** captamos la fee + listing fee.
**Caso 2 (agregador):** captamos rev-share + trading fee del flow on-platform.

---

## Slide 8 — Traction

**Headline:** Producto técnico listo, pilotos reales en hot.

**Demostrado:**
- Programa on-chain con **27 instrucciones** auditables (GitHub público).
- 2 programas Anchor (agroglobaldex + compliance-hook) compilando contra Solana 3.0 / Anchor 0.31.1.
- Web instalable como PWA, APK Android via Capacitor.
- Aggregator funcional: ya prueba listar Agrotoken (SPL Solana) + Centrifuge (cross-chain) en seed.
- Análisis legal MiCA completo (carpeta `/legal` en repo).
- **Colosseum Solana Hackathon: aplicación enviada** (Q2 2026).

**Pilotos en negociación (LOIs en proceso):**
- 🇪🇸 **Vinos DOC España** — bodega/cooperativa en negociación. Tokenización de litros/botellas de vino reserva con D.O.P. verificada.
- 🇻🇪 **Carnes Venezuela** — finca pecuaria en negociación. Tokenización de kg de carne vacuna con certificación INSAI, dirigida a inversores internacionales acreditados.

**[Actualizar a "LOIs firmadas" cuando se cierren]**

**Por qué importa:** dos pilotos en países distintos (UE + LATAM), sectores distintos (Wine + Meat), validan el modelo cross-país + cross-sector desde día uno.

---

## Slide 9 — Competition

**Headline:** Nadie ataca cross-sector + cross-país con compliance MiCA-first.

**Mapa 2×2** (cuadrantes: vertical=geographic scope, horizontal=sector coverage):

| | Single-sector | Multi-sector |
|---|---|---|
| **Single-country** | Agrotoken (AR granos), RIPE (USA) | — |
| **Cross-country** | AgriDex (UK, multi-commodity, no compliance-first), Toucan/Klima (carbono solo), Verra TR² | **AgroGlobalDex** |

**Differentiator clave:**
- Otros son **vertical-thin** (un sector) o **geo-thin** (un país).
- AgroGlobalDex es horizontal + global + compliance-first.
- Los demás son **competidores complementarios**: los agregamos.

**Diferenciadores técnicos específicos vs AgriDex (referente directo)**:

| Capacidad | AgriDex | AgroGlobalDex |
|---|---|---|
| Tokenización RWA agro sobre Solana | ✅ | ✅ |
| Settlement USDC instantáneo | ✅ | ✅ |
| Compliance MiCA-first (jurisdiction policy on-chain) | ❌ | ✅ |
| **Proof-of-trade**: receipt por trade | NFT con metadata | **PDA estructurada queryable** |
| KYC enforcement on-chain (TransferHook) | ❌ | ✅ Token-2022 |
| **Yield offerings** (security tokens MiFID II) | ❌ | ✅ con settlement on-chain receipt + epochs |
| **Aggregator** de tokens de otras plataformas | ❌ | ✅ (Agrotoken, Topaz, Centrifuge…) |
| Revocación de KYC + transferencia de issuer | ❌ | ✅ ix dedicadas |
| White paper MiCA Art. 6 templates legales | ❌ | ✅ pack legal completo |
| Foco geográfico | UK/global commodity | UE-first (Francia AMF/ACPR) + LATAM |

**El "trade receipt NFT" de AgriDex es metadata semi-estructurada**. El nuestro
es una `TradeReceipt` PDA con campos tipados (buyer, seller, jurisdiction
snapshot, gross, fee, trade_index global, settled_at). Indexers, auditores y
regulators consultan el ledger sin parsear metadata JSON.

---

## Slide 10 — Team

**Headline:** Founder técnico + advisors (al cierre del round).

**Carlos Pallottini** — Founder & CEO.
[Background bullet, en 2 líneas. Verificable en LinkedIn.]

**[Hiring con el round]:**
- Co-founder agro/regulatorio UE (CRO).
- 2 ingenieros (1 Solana senior + 1 frontend/mobile).

**Advisors target** (en negociación):
- Ex-CTO de Agrotoken o Topaz (technical).
- Ex-MFIN/CNV o consultora MiCA (regulatorio).
- Cooperativa agro grande (commercial).

---

## Slide 11 — Roadmap

**Headline:** Mainnet en 9 meses post-funding.

**Timeline:**

```
Mes 1-2:  Audit profesional (Trail of Bits / OtterSec) — USD 30-80k
Mes 2-4:  CASP authorization filing (Francia AMF/ACPR)
Mes 3:    Deploy a Solana devnet pública + primer piloto productor
Mes 4-6:  Onboarding 3-5 cooperativas piloto (Caso 1)
Mes 5-7:  Partnership con 2-3 plataformas existentes (Caso 2)
Mes 6-9:  Squads multisig setup + bug bounty Immunefi
Mes 9:    Mainnet launch, primer GMV real
Mes 10-12: Mobile apps en Google Play + App Store
Año 2:    Expansión LATAM (Argentina, Brasil)
```

---

## Slide 12 — Use of funds

**Headline:** USD 500k a un runway de 12-15 meses con foco product-market fit.

**Bullets:**
- 40% **Team** (USD 200k): contratar 2 ingenieros + 1 BD/regulatorio
- 16% **Audit + security** (USD 80k)
- 30% **Legal CASP + capital regulatorio** (USD 150k)
- 6% **Mobile stores + design** (USD 30k)
- 4% **Marketing + ops** (USD 20k)
- 4% **Contingencia** (USD 20k)

---

## Slide 13 — Financials (proyección 5 años)

**Headline:** Path a USD 2M ARR año 5 sobre GMV de USD 280M.

**Tabla resumida** (detalle en `marketing/financials-5y.md`):

| Año | GMV (USD) | Take-rate | Revenue (USD) | OPEX | EBITDA |
|---|---|---|---|---|---|
| Y1 | 500k | 0.5% | 2.5k | 500k | (497k) |
| Y2 | 8M | 0.6% | 48k | 900k | (852k) |
| Y3 | 50M | 0.7% | 350k | 1.4M | (1.05M) |
| Y4 | 140M | 0.7% | 980k | 1.8M | (820k) |
| Y5 | 280M | 0.7% | 1.96M | 2.0M | (40k) — breakeven |

**Assumptions:** GMV grows on 5 cooperative pilots Y1 → 30 pilots + 5 aggregated platforms Y3 → 100 partners Y5.

---

## Slide 14 — Ask

**Headline:** USD 500k pre-seed @ USD 5M cap (SAFE).

**Bullets:**
- **Ticket size:** USD 25k–100k.
- **Vehicle:** SAFE post-money (YC standard) — convertible al primer priced round.
- **Use of funds:** ver slide 12.
- **Close target:** 90 días post-launch del round.
- **Lead committed [VERIFICAR antes de presentar].**

**Contact:**
Carlos Pallottini · carlos@agroglobaldex.io · [LinkedIn] · [Calendly link]

---

## Apéndice (slides reserva, fuera de los 14 principales)

- **A1 — Regulatory deep-dive.** MiCA classification por AssetClass.
- **A2 — Tech architecture.** Diagrama de cuentas Solana + flow KYC.
- **A3 — Risk factors.** Smart contract, regulatory, counterparty, market.
- **A4 — Exit landscape.** Comparables M&A en RWA / agritech (precios y múltiplos).
- **A5 — Pipeline de pilotos.** Lista de cooperativas/empresas en contacto.
- **A6 — Risk Mitigation.** Cómo cubrimos cada riesgo (detallado abajo).

---

## Slide A6 — Risk Mitigation (slide profundo para preguntas de DD)

**Headline:** Para cada riesgo identificable, tenemos un plan concreto de mitigación.

**Tabla principal:**

| Riesgo | Probabilidad | Impacto | Mitigación |
|---|:-:|:-:|---|
| **Smart-contract exploit** | Media | Crítico | Audit profesional (Trail of Bits / OtterSec, USD 30-80k) + Squads multisig como upgrade authority + bug bounty Immunefi USD 5-25k pool + circuit breaker on-chain (`set_paused`) ya implementado |
| **Regulatorio MiCA** | Media | Alto | Diseño MiCA-first desde día uno. CASP filing en Francia (AMF) en mes 2-4 post-funding. `JurisdictionPolicy` mutable on-chain para adaptarnos sin redeploy. Análisis legal completo (`/legal`) firmado por counsel UE |
| **Counterparty (productor fraude)** | Baja | Medio | SPV legal española propietaria del activo físico (no el productor) + warehouse receipt notariado on-chain (SHA-256) + curator network para tokens externos + KYC enforced on-chain en cada transfer |
| **Market price commodity** | Alta | Bajo (en take-rate) | **No garantizamos retornos** — somos marketplace, no fondo. El riesgo de precio lo asume el comprador del token, no AgroGlobalDex. Transparencia 100% on-chain |
| **Liquidez secundaria** | Alta | Medio | Empezamos illiquid (hold-to-redeem). Plan Y2-Y3: integración con Whirlpool/Raydium AMMs para mercado secundario. Y aggregator hace que el universo de tokens listados sea grande desde día uno |
| **FX / Cambiario** | Media | Bajo | Settlement 100% en USDC (no fiat local). Productores y compradores quedan en stablecoin. Off-ramp a EUR/local vía Circle USDC ↔ banking integration (mes 6-12 post-funding) |
| **Operational (server / RPC down)** | Baja | Bajo | Helius + Triton One como RPCs dedicados con SLA. Monitoreo on-chain con tx confirmation tracking. Pause-flag para incidentes |
| **Key compromise (authority)** | Baja | Crítico | Squads multisig 3-of-5 desde mainnet day-1. `compliance_signer` separado de `authority` ya implementado (rotación sin tocar fees / treasury) |
| **Founder key-person** | Media | Alto | Hire de co-founder agro/regulatorio en mes 1-3. Documentación técnica + legal 100% open source. Capital regulatorio en banco UE (no en wallet) |
| **Competencia (Agrotoken/etc.)** | Alta | Medio | Estrategia aggregator NO competimos head-to-head: los listamos. Network effect bilateral los hace mejores partners que competidores |
| **Sanciones internacionales (Venezuela)** | Media | Crítico para esa jurisdicción | OFAC/EU/ONU screening on-chain en cada KYC stamp. `JurisdictionPolicy` mutable permite bloquear país en segundos. LOI Venezuela tiene Cláusula CUARTA específica de riesgos sancionatorios |

**Resumen para el inversor:** los 3 riesgos materiales (audit + regulatorio + key-person) están en el roadmap explícito con plazos y costos. Los demás son mitigables vía diseño on-chain ya implementado.

**Lo que NO mitigamos:** el riesgo de precio del commodity subyacente. Eso es del comprador del token, no nuestro. Y no podemos eliminar el riesgo regulatorio macro — si MiCA cambia drásticamente, adaptamos.
