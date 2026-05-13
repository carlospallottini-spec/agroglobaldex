# 01 — Ecosystem Research: Tokenización Agro & RWA Agro (2026)

**Objetivo:** mapear el ecosistema actual para identificar (a) potenciales partners para listar en el marketplace, (b) competidores directos, y (c) infraestructura sobre la que apoyarnos (oráculos, custody, compliance).

**Método:** desk research basado en conocimiento público hasta principios 2026. Donde la info pueda haber cambiado o no esté confirmada, marco `[VERIFICAR]`.

---

## Tabla maestra (25 proyectos)

| # | Nombre | País / HQ | Qué tokeniza | Blockchain | Modelo de negocio | Relación con AGD | Fuente / Link |
|---|---|---|---|---|---|---|---|
| 1 | **Agrotoken** | Argentina / Brasil | Granos (soja, maíz, trigo) — stablecoins respaldadas por commodities | Ethereum, Polygon, Algorand | Tokenización 1:1 + crédito/pagos B2B | Complementario (onboardear) | agrotoken.io |
| 2 | **RIPE Money** | LATAM (HQ Miami) | Commodities agro reales en stablecoin | Ethereum | Stablecoin asset-backed | Complementario | ripe.money [VERIFICAR estado 2026] |
| 3 | **Topaz** | Brasil | NFTs / tokens de ativos rurais (CPR tokenizada) | Solana | Custody + tokenización para bancos BR | Complementario fuerte (mismo stack Solana) | topaz.io |
| 4 | **Cropper Finance** | LATAM | DeFi sobre commodities tokenizados | Solana | DEX + yield sobre agro tokens | Potencial integración técnica | [VERIFICAR estado] |
| 5 | **AgriDex** | UK / Global | Marketplace agro on-chain (commodities físicas) | Solana | Marketplace + settlement | **Competidor directo** | agridex.com |
| 6 | **GrainChain** | USA / México | Contratos forward de granos + logística | Hyperledger / propietario | SaaS contractual + escrow | Complementario (data layer) | grainchain.io |
| 7 | **Demeter Protocol** (Sora) | Internacional | RWA agro y biodiversidad | Substrate (Sora) | Parachain RWA | Complementario nicho | [VERIFICAR] |
| 8 | **Cargill blockchain pilots** | USA | Trazabilidad granos / cocoa | Hyperledger Fabric | Internal supply chain | No es target directo, pero referencia | Cargill.com press 2022-2024 |
| 9 | **Bayer / Climate FieldView** | USA / DE | Datos agronómicos (no tokenización pura, pilots) | Multiple | SaaS + data | Posible data partner | climate.com [VERIFICAR pilots cripto] |
| 10 | **Nori** | USA | Créditos de carbono agro (soil carbon) | Polygon | Marketplace de carbon removal | Complementario fuerte (listar créditos) | nori.com |
| 11 | **Indigo Ag (Carbon)** | USA | Carbon credits de prácticas regenerativas | Off-chain principalmente | Programa de créditos + venta | Complementario | indigoag.com |
| 12 | **Regrow Ag** | USA / AU | MRV de carbono agro | Off-chain (data) | SaaS MRV | Infra/data partner | regrow.ag |
| 13 | **Pachama** | USA | Carbon credits forestales (con datos satelitales) | Web2 + parcial on-chain | Marketplace + MRV | Complementario | pachama.com |
| 14 | **Toucan Protocol** | UE (Alemania) | Bridge de carbon credits a on-chain (BCT, NCT) | Polygon, Celo | Infra de tokenización de carbono | **Partner de infra** (clave para UE) | toucan.earth |
| 15 | **Moss.Earth** | Brasil | Carbon credits tokenizados (MCO2) | Ethereum | Marketplace de carbono | Complementario | moss.earth |
| 16 | **KlimaDAO** | DAO global | Treasury de carbon credits tokenizados | Polygon | DAO + token | Complementario (liquidez carbono) | klimadao.finance |
| 17 | **Veritree** | Canadá | Tokenización de proyectos de reforestación | Cardano | MRV + tokenización | Complementario nicho | veritree.com |
| 18 | **Agrology** | USA | Sensores IoT agro + datos (no tokenización core) | Off-chain | SaaS sensores | Data partner potencial | agrology.ag |
| 19 | **Carbonbase** | Hong Kong | Carbon offsets retail + tokenización | Polygon | App B2C carbono | Complementario | carbonbase.co |
| 20 | **Sustainabl** | Singapur | Carbon credits agro Asia | [VERIFICAR] | Marketplace regional | Complementario regional | [VERIFICAR URL] |
| 21 | **Centrifuge** | UE / Global | RWA general (incluye algunos pools agro) | Polkadot, Ethereum | Pool de RWA en DeFi | **Infra/competidor parcial** | centrifuge.io |
| 22 | **Goldfinch** | USA | Crédito a economías emergentes (incluye agro) | Ethereum | Lending pools | Complementario crédito | goldfinch.finance |
| 23 | **Pyth Network** | Global | Oráculos de precios commodities | Solana + cross-chain | Oráculo | **Infra crítica** (oracle de precios) | pyth.network |
| 24 | **Chainlink CCIP / Functions** | Global | Oráculos + cross-chain (incluye agri data) | Multi-chain | Oráculo | **Infra crítica** | chain.link |
| 25 | **dClimate** | USA | Datos climáticos on-chain | Multi-chain | Oráculo climate data | Infra | dclimate.net |

---

## Clasificación estratégica

### A. Complementarios — onboardear o partnership (15)

Estos NO compiten con AGD. Emiten/originan tokens y necesitan liquidez global y un canal UE-compliant. Son target #1 de BD.

- **Agrotoken** — el más grande de LATAM en stablecoins de granos. Listarlos sería un signal enorme.
- **Topaz** — Solana + Brasil. Stack idéntico. Potencial co-marketing.
- **Moss.Earth** — carbon LATAM con presencia UE.
- **Toucan Protocol** — infra de tokenización de carbono UE. Aliado clave para MiCA narrative.
- **Nori, Pachama, KlimaDAO, Veritree** — todos pueden listar sus créditos.
- **Goldfinch, Centrifuge** — RWA infra; posible co-listing de pools agro.
- **GrainChain, Regrow, dClimate, Agrology** — data/MRV partners.
- **RIPE Money, Cropper, Demeter** — DeFi sobre agro; potencial integración.

### B. Competidores directos (2-3)

- **AgriDex** — marketplace agro on-chain, también en Solana, también global. **Es el competidor #1.** Diferenciación AGD: foco UE/MiCA, modelo agregador (no emisor), neutral por chain del emisor.
- **Centrifuge** — más general (RWA broad), pero podría capturar agro si quisiera. Diferenciación: AGD es vertical agro puro, no horizontal RWA.

### C. Infra sobre la que apoyarse (5)

- **Pyth** (precios commodities en Solana — match perfecto)
- **Chainlink** (CCIP para cross-chain settlement)
- **dClimate** (data climática on-chain)
- **Toucan** (bridge de carbon credits)
- Wallets/custody: Fireblocks, Anchorage, Copper [VERIFICAR cuál es el match correcto para Solana + MiCA]

---

## Señales de mercado a 2026 (para usar en pitch)

- **Tokenization RWA cruzó USD 30B+ on-chain** (mayoría US Treasuries + commodities). [VERIFICAR número exacto en rwa.xyz a la fecha de uso]
- **MiCA en vigor desde dic 2024** para crypto-assets en UE, dando claridad regulatoria a tokens respaldados por activos.
- **CBAM (Carbon Border Adjustment Mechanism)** UE en transición → demanda creciente por créditos de carbono verificables y trazables. Tailwind para agro carbon tokenizado.
- **LATAM exporta USD 100B+ en granos/año** ([VERIFICAR cifra USDA/FAO 2025]) — gran TAM si capturamos fracción.
- **Solana TPS y costo** = stack viable para settlement agro (vs Ethereum L1 caro).

---

## Gaps de research

- Estado real 2026 de RIPE Money, Cropper, Demeter, Sustainabl — marcados `[VERIFICAR]`.
- No tengo data confirmada de pilots actuales de Bayer/Cargill en tokenización pura (post-2024).
- Falta mapear players específicos de UE-Este (Polonia, Rumania son grandes productores de trigo/maíz) — gap geográfico.
- Falta mapear el ángulo islámico/halal-compliant (mercado MENA es relevante para granos LATAM).
