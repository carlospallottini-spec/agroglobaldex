# AgroGlobalDex — Grant Applications (drafts)

> Borradores para aplicar a grants NO-DILUTIVOS antes / en paralelo al pre-seed.

## 1. Solana Foundation Grants

**URL**: https://solana.org/grants-funding
**Ticket realista**: USD 5,000–50,000 (Tier 1) o USD 50k-150k (Tier 2 con tracción).
**Tiempo de respuesta**: 4-8 semanas.

### Application text (draft)

**Project name:** AgroGlobalDex

**One-line description:** The compliance-first marketplace for tokenized agricultural assets (grains, meat, wine, oil, dairy) on Solana, with cross-sector aggregator for existing platforms.

**Problem we're solving:** Agricultural producers globally lack access to institutional capital. Existing RWA tokenization platforms (Agrotoken, RIPE, Centrifuge) are single-sector and single-country, with inconsistent compliance. Investors looking for agro exposure can't access it cross-border in a compliance-aware way.

**Why Solana:** Token-2022 extensions (Transfer Hook + Metadata Pointer) are non-negotiable for our regulatory model (MiCA-aligned). Solana is the only chain with these production-ready. Throughput and cost are critical for small agricultural producers — Ethereum L1 economics don't work for a USD 100 ticket.

**Technical scope:** 2 Anchor programs (17 instructions). 1) Main marketplace program: register/mint/list/buy/redeem flows with USDC settlement and treasury PDA. 2) `compliance-hook` Transfer Hook program implementing SPL Transfer Hook Interface for on-chain KYC + jurisdiction enforcement on every transfer. Both compile against Anchor 0.31.1 / Solana 3.0 / platform-tools v1.54. Source: https://github.com/carlospallottini-spec/agroglobaldex

**Stage:** PoC complete, end-to-end seed validated against local validator. Pre-mainnet (no audit yet).

**Asks (Tier 1, USD 25k):**
- USD 15k: Professional security audit (OtterSec or equivalent, partial).
- USD 5k: Devnet/mainnet deployment infrastructure (Helius RPC, monitoring).
- USD 5k: Marketing for cooperative pilot acquisition.

**Milestones (3 months):**
- M1: Audit kick-off.
- M2: Deploy to Solana devnet pública con seed completo.
- M3: First cooperative pilot signed + audit report received.

**Why we deserve this grant:** 100% of the technical work is in the open. The compliance-hook program is one of the first production-ready implementations of Token-2022 Transfer Hooks in agro RWA. We're betting on Solana as the institutional layer for tokenized real-world commodities — this grant helps us prove that bet faster.

**Team:** Carlos Pallottini (founder, full-stack + Solana). [Co-founder hiring in progress.]

**Contact:** carlos@agroglobaldex.io · GitHub @carlospallottini-spec

---

## 2. Horizon Europe — agritech / digitalisation calls

**URL**: https://ec.europa.eu/info/funding-tenders/opportunities/portal/
**Programa target**: Horizon Europe Cluster 6 (Food, Bioeconomy, Natural Resources, Agriculture and Environment) — calls específicas sobre digitalisation y traceability.
**Ticket**: EUR 500k–2M (multi-year, multi-partner).
**Tiempo**: 6-12 meses paperwork. Submission de propuesta + competencia abierta.
**Requisito clave**: tener un **consortium** con al menos 3 organizaciones de 3 países UE distintos.

### Plan de aplicación

1. **Buscar consortium partner**:
   - Universidad agraria UE (Wageningen, AgroParisTech, Universidad Politécnica de Madrid).
   - Research institute (INRAE en Francia, JRC).
   - Cooperative federation (Copa-Cogeca tiene seed funding pre-call).
2. **Encontrar call activa**: filtrar por "agritech", "tokenization", "traceability", "digital agriculture".
3. **Posición de AgroGlobalDex**: technical lead. Implementación de la blockchain layer. Otros partners aportan: agro domain expertise, regulatory legal, end-user pilots.

### Borrador de pitch para outreach a partners (texto)

**Asunto:** Horizon Europe — agro RWA tokenization consortium

> Estimado/a [nombre],
>
> Soy Carlos Pallottini, founder de AgroGlobalDex, un marketplace de
> commodities agro tokenizados sobre Solana con compliance MiCA-first.
>
> Estamos preparando una aplicación a Horizon Europe Cluster 6 sobre
> traceability + digital agriculture, y necesitamos un consortium partner
> con perfil [agronomic research / cooperative federation / regulatory legal].
>
> Nuestro aporte: technical lead. Programa Anchor ya compilando, audit en
> curso, MiCA analysis completo. Github público (link).
>
> ¿Sería posible una llamada de 30 minutos para explorar fit? Adjunto
> one-pager.
>
> Saludos,
> Carlos

**Próximos pasos concretos:** identificar 5 partners potenciales esta semana. Mandar el email arriba.

---

## 3. SBIR (USA) — no aplica directamente pero referenciar

Si en el futuro abrimos US presence, SBIR Phase I (USDA / NSF) son USD 250k–500k no dilutivos. Requiere LLC US y partnership con federal lab. Posponer hasta Y2-Y3 cuando tengamos tracción UE.

---

## 4. Hackathon prizes (próximos 6 meses)

| Hackathon | Cuándo | Ticket | Notas |
|---|---|---|---|
| **Colosseum / Solana Breakout Hackathon** | rolling | USD 50k–250k bounties | Submission ready: programa actual + pitch demo. |
| **Solana Foundation BUIDL** | rolling | USD 5k–25k | Smaller bounties, faster turnaround. |
| **EthGlobal / Solana track partnerships** | quarterly | varies | Si se hace partnership con un agro corporate. |

**Acción concreta:** preparar `submission/` folder con README + demo video + screenshots. Submission para próximo Colosseum.

---

## 5. Helius / Triton One developer grants

RPC providers que dan crédito gratuito a proyectos serios. Hablar con ambos:
- Helius: hasta USD 5k/mes en RPC credits + co-marketing.
- Triton One: similar.

**Acción**: email a developer-relations de cada uno, con repo público y plan de deploy.

---

## Tabla de prioridad

| Fuente | Esfuerzo de aplicación | Ticket | Plazo | ROI por hora |
|---|---|---|---|---|
| Solana Foundation Grant | Bajo (10-20h) | USD 25k | 4-8 sem | **MÁS ALTO** |
| Hackathons | Medio (40-60h) | USD 5-250k | rolling | Alto |
| Helius/Triton credits | Bajo (5h) | USD 60k/año equivalente | 2-4 sem | Alto |
| Horizon Europe | Muy alto (200h+) | EUR 500k-2M | 12+ meses | Medio |
| SBIR USA | Alto, requiere US entity | USD 250k+ | 6+ meses | Bajo (corto plazo) |

**Recomendación operativa:** **arrancar con Solana Foundation grant + Helius credits + 1 hackathon esta semana**. Eso puede traer USD 30-50k en 8-12 semanas sin diluir equity, mientras corre el pre-seed en paralelo.
