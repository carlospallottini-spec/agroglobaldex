# AgroGlobalDex — One-Pager para inversores

**Fecha:** 2026-07-26 · **Autor:** Carlos Pallottini (Founder) · **Estado del proyecto:** Pre-lanzamiento (demo funcional en devnet)

> Este documento no constituye oferta de valores ni asesoramiento financiero. Todas las cifras marcadas como *(estimación)* son rangos orientativos, no compromisos. AgroGlobalDex no está autorizada como CASP bajo MiCA a la fecha de este documento.

---

## El problema

- Los productores agropecuarios cobran a 90–180 días y financian su circulante con factoring caro (8–18% anual, *estimación de mercado*) o no acceden a crédito en absoluto.
- El inversor minorista no tiene vía para invertir en producción agro real: los vehículos institucionales exigen tickets altos y las plataformas de tokenización existentes son de un solo país y un solo sector.
- El mercado agro de la UE mueve ~€280B/año (Eurostat) y el global ~USD 12T/año (FAO); casi nada de ese valor es líquido ni accesible on-chain.

## La solución

**AgroGlobalDex** es un marketplace global de activos agropecuarios tokenizados sobre Solana, **compliance-first**: el cumplimiento no es una promesa off-chain, está programado dentro del token.

- **Tokenización** de producción real (granos, carnes, vinos, aceites, créditos de carbono, fracciones de cosecha, ofertas de inversión) como tokens Token-2022.
- **Compliance on-chain**: cada transferencia pasa por un *Transfer Hook* que verifica KYC, jurisdicción permitida y —para clases restringidas (securities)— acreditación del receptor. Un token no puede llegar a una wallet no verificada, ni siquiera por transferencia directa P2P.
- **Crédito colateralizado**: el productor deposita su cosecha tokenizada como colateral y recibe USDC on-chain al instante, sin banco. Los proveedores de liquidez ganan el interés pro-rata.
- **Oráculo Pyth** para el precio del colateral (con verificación de firmas, staleness y confianza), en lugar de precios fijados a mano.
- **Trazabilidad**: cada operación deja un `TradeReceipt` inmutable on-chain.

## Cómo funciona (en una frase por pieza)

1. El productor tokeniza su activo → se crea un mint Token-2022 con el hook de compliance adjunto.
2. El comprador/inversor pasa KYC (sellado on-chain por un firmante de compliance separado de la administración) y compra en el marketplace.
3. El productor puede, en vez de vender, **pedir prestado USDC** contra su token; el precio del colateral lo fija el oráculo Pyth y la liquidación es parcial y justa (devuelve el excedente al deudor).

## Estado actual (real, verificable en el repo)

| Qué | Estado |
|---|---|
| Smart contracts (2 programas Anchor, 30 instrucciones) | ✅ Feature-complete, desplegable en devnet |
| Tests | ✅ 47 tests de integración mocha en CI verde (`anchor test`), + gates de clippy, fmt, cargo-audit y control de deriva del IDL |
| Hardening de seguridad | ✅ Auditoría interna documentada (`AUDIT_READINESS.md`): hallazgos críticos y altos de código remediados y verificados en CI |
| Web (PWA, 11+ páginas) | ✅ Publicada en agroglobaldex.com con deploy automático |
| App de escritorio (Windows/macOS/Linux) | ✅ Builds automáticos por release (Electron) |
| App móvil (Capacitor/Android) | ✅ Lista para compilar; falta prueba en dispositivo y cuentas de stores |
| Auditoría externa | ⬜ Pendiente — bloqueante absoluto para mainnet |
| Licencia CASP MiCA | ⬜ Pendiente — bloqueante legal para operar en UE |
| Mainnet | ⬜ No desplegado; no se mueve dinero real hoy |

**Lo que NO hay todavía (honestidad primero):** usuarios reales, ingresos, partnerships firmados ni operación con dinero real. El proyecto está en fase demo/devnet por decisión deliberada: operar sin auditoría y sin licencia sería ilegal e irresponsable.

## Roadmap honesto en 3 fases

| Fase | Qué | Depende de |
|---|---|---|
| **(a) Hoy** | Demo completa funcionando en devnet: tokenización, marketplace, lending, compliance hook, oráculo. Web pública y apps compilables. | Nada — ya está |
| **(b) Con capital** | Auditoría externa del contrato + camino regulatorio (CASP MiCA, jurisdicción recomendada: Francia) + multisig Squads + contrataciones clave (MLRO/DPO) | Cierre del pre-seed |
| **(c) Mainnet** | Deploy con ceremonia de llaves, oráculo obligatorio activado, liquidez inicial, primeros productores onboardeados | Auditoría aprobada + autorización regulatoria. Camino crítico realista: **9–15 meses post-funding** *(estimación)* |

## Uso de fondos (pre-seed USD 500k @ USD 5M cap, SAFE) *(estimación)*

| Partida | Rango estimado | Nota |
|---|---|---|
| Auditoría externa del contrato | USD 30k–80k | Rango de mercado para 2 programas Anchor (~3–4k líneas Rust con Token-2022 + hook + Pyth + lending); firmas top o máxima profundidad pueden llegar a ~USD 150k |
| Legal y licencias (CASP MiCA, UE) | €80k–300k legal externo + €50k–150k de capital regulatorio depositado | Detalle en `legal/10-compliance-checklist.md`; el capital regulatorio se deposita, no se gasta |
| Equipo | ~40% del round | Desarrollo, compliance (MLRO/DPO), operaciones |
| Liquidez inicial + infraestructura | Resto | Pool de lending semilla, RPC dedicado, monitoreo, marketing |

## Por qué ahora

1. **MiCA** en vigor desde diciembre 2024: primer marco unificado UE — la barrera regulatoria se convierte en foso competitivo para quien la cruce primero.
2. **Token-2022** maduro: Transfer Hooks estables hacen posible el compliance programado en el propio token.
3. Las plataformas RWA-agro existentes son single-country/single-sector y sin enfoque MiCA.

## Materiales disponibles

Data room completo indexado en [`docs/DATA_ROOM.md`](./DATA_ROOM.md) · Pitch deck en [`marketing/pitch-deck-full.md`](../marketing/pitch-deck-full.md) · Arquitectura en [`docs/ARCHITECTURE.md`](./ARCHITECTURE.md) · Repo público con CI verde.

## Contacto

**Carlos Pallottini** — Founder · carlos@agroglobaldex.io · Web: agroglobaldex.com
