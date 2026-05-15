# AgroGlobalDex — Financial Model (5y projections)

> Modelo conservador. Todos los números marcados [ASSUMPTION] son discutibles
> con el inversor — el deck los presenta como rango, este doc los detalla.

## Resumen ejecutivo

- **Year 1:** runway burn de USD 500k, GMV "demo" de USD 500k (3 pilotos).
- **Year 5:** GMV USD 280M, revenue USD 1.96M, **near breakeven** (target USD 2M ARR).
- **Take-rate efectiva:** 0.5% Y1 (solo trading fee) → 0.7% Y5 (trading + listing + aggregator rev-share).

## Tabla principal

| Año | Pilotos cooperativos | Plataformas agregadas | GMV USD | Take-rate | Revenue USD | OPEX USD | EBITDA USD |
|---:|---:|---:|---:|---:|---:|---:|---:|
| Y1 | 3 | 1 | 500,000 | 0.5% | 2,500 | 500,000 | (497,500) |
| Y2 | 12 | 3 | 8,000,000 | 0.6% | 48,000 | 900,000 | (852,000) |
| Y3 | 30 | 5 | 50,000,000 | 0.7% | 350,000 | 1,400,000 | (1,050,000) |
| Y4 | 60 | 8 | 140,000,000 | 0.7% | 980,000 | 1,800,000 | (820,000) |
| Y5 | 100 | 12 | 280,000,000 | 0.7% | 1,960,000 | 2,000,000 | (40,000) |

## Revenue breakdown (Year 5)

| Stream | Monto USD | % | Notas |
|---|---:|---:|---|
| Trading fee (0.5% buy) | 1,400,000 | 71% | Sobre USD 280M GMV |
| Listing fee | 200,000 | 10% | 100 pilotos × USD 2,000 |
| Aggregator rev-share | 360,000 | 18% | 12 plataformas × USD 30k/año promedio |
| **TOTAL** | **1,960,000** | **100%** | |

## OPEX breakdown (Year 5)

| Categoría | Monto USD | Detalle |
|---|---:|---|
| Salarios + benefits | 1,200,000 | 8 FTE (founder, 3 eng, 2 BD, 1 ops, 1 compliance) |
| Hosting + infra | 60,000 | RPC providers (Helius/Triton), CDN, monitoring |
| Audits (annual) | 100,000 | Re-audit anual de smart contracts |
| Legal + compliance ongoing | 200,000 | Counsel UE + ongoing CASP reporting |
| Marketing + sales | 300,000 | Conferencias, content, ads, BD travel |
| Insurance (D&O + cyber) | 60,000 | Standard fintech tier |
| G&A | 80,000 | Accounting, banking, office |
| **TOTAL** | **2,000,000** | |

## Assumptions detalladas

### 1. Cooperative pilots growth [ASSUMPTION]
- Y1 → 3 (España + Argentina + Brasil, una por país).
- Y2 → 12 (cuadruplicamos con primer producto market fit).
- Y3 → 30 (escalando con BD team).
- Y4 → 60.
- Y5 → 100.
- **Each pilot avg GMV:** Y1 = USD 100k, Y5 = USD 2M (productores grandes).

### 2. Aggregator platforms [ASSUMPTION]
- Y1: Agrotoken (SPL) integrado.
- Y2: + Topaz + RIPE + Centrifuge (cross-chain).
- Y3-Y5: scaling con 1-2 nuevas plataformas por año.
- **Each adds:** USD 5M GMV avg Y3, USD 15M avg Y5.

### 3. Take-rate evolution
- Y1: 0.5% trading fee solo (no cobramos listing al principio).
- Y2-Y3: 0.6% (agregamos USD 1k listing fee).
- Y4-Y5: 0.7% (agregamos rev-share aggregator).

### 4. Headcount
- Y1: 3 FTE (founder + 2 eng).
- Y2: 5 FTE (+1 BD + 1 ops).
- Y3: 7 FTE (+1 compliance + 1 eng).
- Y4: 8 FTE.
- Y5: 10 FTE.

## Comparables (sanity check)

| Comparable | GMV anual | Revenue | Multiple |
|---|---|---|---|
| Agrotoken (estimate, AR/BR) | ~USD 30M TVL → ~USD 60M GMV anual | ~USD 600k | 100× revenue valuation USD 60M (2024 ronda) |
| Centrifuge (RWA agnostic) | USD 400M TVL | ~USD 4M | Token-backed valuation ~USD 200M |
| Goldfinch (agro/DeFi) | ~USD 100M | ~USD 1M | Token cap ~USD 80M |

**Implicación:** un AgroGlobalDex maduro (Y5) con USD 280M GMV y USD 2M ARR puede valuar en rango **USD 40-80M** vía revenue multiples (20–40×), ofreciendo retorno **80-160×** a los pre-seed Y1 entrants @ USD 5M cap.

## Sensitivity table

¿Qué pasa si los pilotos crecen más lento?

| Y5 cooperative pilots | Y5 GMV | Y5 revenue | Estado |
|---:|---:|---:|---|
| 50 (50% del plan) | 130M | 910k | Break-even Y6 |
| 100 (plan) | 280M | 1.96M | Break-even Y5 |
| 200 (upside) | 560M | 3.92M | Profitable Y4 |

## Funding plan

| Round | Cuándo | Monto USD | Vehicle | Valuation cap |
|---|---|---|---|---|
| Pre-seed (current) | Q2 2026 | 500k | SAFE post | USD 5M |
| Seed | Q1 2027 (after audit + pilots) | 2.5M | Priced equity | USD 15-20M |
| Series A | Q1 2028 (after USD 50M+ GMV) | 8-12M | Priced equity | USD 60-80M |

## Disclaimer

Estos números son **proyecciones**, no pronósticos. Sensitivity table arriba muestra los rangos. El inversor debe asumir variance de ±50% en GMV de cualquier año dado.
