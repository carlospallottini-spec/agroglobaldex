# AgroGlobalDex — Demo Video Script (120 segundos)

> Para grabar con OBS + screen recording de la web local. Voice-over en español
> con subtítulos EN. Output target: 1080p, 120s, formato cuadrado 1:1 para
> LinkedIn + horizontal 16:9 para YouTube.
>
> Estructura: HOOK (10s) → PROBLEM (15s) → SOLUTION (60s) → PROOF (25s) → CTA (10s).
>
> v2 (Jun 2026) incorpora las dos features nuevas que cierran el moat vs
> AgriDex: **crédito al productor** (lending market on-chain) y
> **comprobantes de trade** (TradeReceipt PDA inmutable).

---

## 0:00–0:10 — Hook (cámara al founder, b-roll de campo)

> "Argentina exporta soja por miles de millones cada año. Sus productores cobran
> tarde, mal, y con tasas reales del 40 por ciento. Esto se puede cambiar
> ahora."

**On-screen text:** "El productor agro paga 40% por capital. El inversor global no llega."

---

## 0:10–0:25 — Problem (b-roll: granos, contratos, papeles)

> "Hoy un productor de soja en Mendoza, un viticultor en Rioja y un ganadero
> en Uruguay viven los mismos tres problemas: capital caro, intermediarios
> sin transparencia, y mercados financieros que no fueron diseñados para
> ellos."

**On-screen text:** "3 problemas · 1 solución blockchain."

---

## 0:25–1:25 — Solution (screen recording de la web)

**Sub-segmento A (0:25-0:40):** Mostrar `/` con el mapa satélite real del mundo
(featured shot — el founder hace zoom in sobre Mendoza, Rioja, Toscana — los
markers neón pulsantes señalan dónde hay producción tokenizada).

> "AgroGlobalDex es el marketplace global de activos agropecuarios tokenizados.
> Listamos granos, carnes, vinos, aceites y lácteos. Cada token es un kilo
> físico, con certificado verificable on-chain."

**Sub-segmento B (0:40-0:53):** Mostrar `/tokenize` wizard 4 pasos.

> "Un productor tokeniza en cuatro pasos. Elige qué producto. Carga el
> certificado. Firma con su wallet. Listo: su producción aparece en el
> marketplace, accesible globalmente."

**Sub-segmento C (0:53-1:08):** Mostrar `/borrow` — esta es la diapositiva clave.

> "Y acá está lo que ningún competidor tiene. Ese token de su cosecha lo usa
> como colateral. Lockea, recibe USDC en segundos, sin banco, sin papeleo.
> Devuelve cuando vende. Cierra el ciclo capital, producción y venta on-chain.
> Cero intermediarios."

**On-screen text durante /borrow:** "Crédito al productor · APR 12% fijo · LTV 50%"

**Sub-segmento D (1:08-1:18):** Mostrar `/invest` con yield offerings + badge
"Último settlement · epoch 2 · USD 450 acum.".

> "Un inversor en Madrid compra desde un kilo de carne argentina hasta una
> participación en la cosecha 2026 de un viñedo de Rioja. Cada distribución
> de yield queda registrada on-chain."

**Sub-segmento E (1:18-1:25):** Mostrar `/receipts` con la tabla del ledger
público.

> "Y cada operación deja un comprobante inmutable que cualquiera puede
> auditar. Esto es trazabilidad supply-chain de verdad, no una promesa
> de marketing."

---

## 1:25–1:50 — Proof (screen recording de wallet + tx en Solscan)

> "Está construido y funcionando hoy en devnet pública de Solana. Veintisiete
> instrucciones, dos programas Anchor, Token-2022 con transfer hook que
> enforza KYC y jurisdicción en cada transferencia. Un lending market
> completo. Ledger de proof-of-trade. El código está en GitHub. Auditoría
> profesional contratada. Y la aplicación web ya funciona como PWA y como
> app móvil en Android."

**On-screen text:** "27 ix on-chain · lending · proof-of-trade · MiCA-aligned · open source"

---

## 1:50–2:00 — CTA (cámara al founder)

> "Estamos levantando 500 mil dólares para llevarlo a mainnet con audit y
> autorización CASP MiCA. Si esto te interesa, escríbeme."

**On-screen text + audio outro:** "agroglobaldex.io · carlos@agroglobaldex.io"

---

## Notas de producción

- **Música**: cinematic, mid-tempo, sin lyrics. Recomendado: Epidemic Sound
  → "Future Innovation" o similar. Budget USD 10-30 license.
- **B-roll necesario** (sin gastar — Pexels / Unsplash free stock):
  - Cosechadora en campo de soja (10s)
  - Manos contando billetes (3s)
  - Bodega de vino (5s)
  - Vaca / feedlot (3s)
  - Manos firmando contrato (3s)
- **Screen recording**: usar OBS Studio (free) en Chrome con ventana 1280×720.
  Grabar cada flow por separado, ensamblar en DaVinci Resolve (free).
- **Shot list para grabar** (en orden):
  1. `index.html` con el mapa cargado, hacer zoom in sobre Mendoza/Rioja
     (~5s cada uno) — el mapa satélite real es el money shot del demo.
  2. `tokenize.html` paso 1 al 4 del wizard (~13s total).
  3. `borrow.html` con liquidez del pool ya cargada — hacer click en
     "Abrir préstamo", mostrar el LTV preview ($collateral × price → max),
     firmar la tx (~15s).
  4. `invest.html` con la card de Viñedo Rioja mostrando "Último settlement".
  5. `receipts.html` con la tabla poblada — scroll lento, hover sobre una
     fila para que se ilumine.
  6. Solscan abierto en otra tab mostrando un trade real (`AssetPurchased` event).
- **Cuándo correr `seed-localnet.ts` antes de grabar**: necesitás datos demo
  poblados para que las pantallas no aparezcan vacías. Corré primero el seed
  para tener 2 assets registrados + un settlement de yield + algunos trades.
- **Voice-over**: grabar con micrófono USB decente (USD 50-100) o pagar a
  freelancer en Fiverr/Voice123 (USD 50-150 para 90s en español).
- **Subtitulos EN**: generar con Whisper API o submagic.co (USD 10-20).
- **Total budget**: USD 100-300 si tercerizás voz; USD 30-50 si grabás vos.

## Versiones a generar

- **Master 120s 16:9** — YouTube, embed en web, demo days.
- **Cuadrado 1:1 120s** — LinkedIn feed.
- **Vertical 9:16 60s** — TikTok / Instagram Reels (cortar a sub-segmentos
  C "crédito" + D "invest" + CTA — el credito al productor es lo que mas
  retiene en short-form).
- **GIF 6s loop** — Twitter cover / DM teaser (mapa satelite + un marker
  pulsante neon sobre Rioja → "tokenizá tu cosecha").
