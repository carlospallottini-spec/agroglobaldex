# Cómo pasar el pitch deck a visual (sin diseñador, sin gastar)

> Tenés `marketing/pitch-deck-full.md` con 14 slides de texto listo.
> Acá las 4 formas más rápidas de convertirlo a un deck visual presentable.
> Recomendadas por orden de **menor tiempo → mejor resultado**.

---

## OPCIÓN 1 — Gamma.app (RECOMENDADA · 15 minutos · gratis)

**La más rápida.** Gamma usa IA: pegás el markdown y genera un deck visual con imágenes, layouts y branding consistente.

### Paso a paso

1. Andá a **https://gamma.app** y registrate gratis (login con Google).
2. Click **"Create new"** → **"Generate"** → **"Paste in text"**.
3. Abrí `marketing/pitch-deck-full.md` y **copiá todo el contenido** (Ctrl+A, Ctrl+C).
4. Pegalo en el textarea de Gamma.
5. Antes de generar:
   - **Style**: elegí "Default" o "Modern" → más sobrio para inversores.
   - **Number of cards**: 14 (el deck tiene 14 slides + apéndice).
   - **Theme**: dark / minimal con accent verde (mejor para combinar con tu paleta neón).
   - Si Gamma pide un prompt extra, pegá: *"Investor pitch deck for AgroGlobalDex, a compliance-first tokenized agricultural marketplace on Solana. Dark theme with neon green accent. Keep tone serious, no hype. Add data viz for tables."*
6. Click **"Generate"**. Espera 30-60 segundos.
7. Editá cada slide tocando el contenido directamente — Gamma permite drag, swap layouts y cambiar imágenes con un click.
8. **Export**: botón arriba derecha → "Export" → "PDF" (para mandar por email) o "Figma" (si querés seguir editando en Figma).

### Costo

- **Gratis**: 400 créditos al registrarte, suficiente para 3-5 decks.
- **Plus** ($10/mes): unlimited cards, exports sin watermark, custom domain.

### Tips

- Si una slide sale con texto cortado, achicá el contenido directamente en Gamma — no hace falta volver al .md.
- Las **imágenes de stock** vienen de Unsplash integrado. Buscá: "vineyard", "cattle ranch", "wheat field", "blockchain agriculture".
- **Para slides con tablas** (slide 7 modelo de negocio, slide 13 financials): pegá el markdown table tal cual; Gamma lo renderiza limpio.

---

## OPCIÓN 2 — Pitch.com (30 minutos · gratis para starters)

Más control que Gamma, menos automático. Templates de calidad VC.

### Paso a paso

1. **https://pitch.com** → registro gratis.
2. **Templates** → buscá **"Investor pitch deck"** o **"Crypto startup"**.
3. Elegí uno con 14-15 slides. Recomendadas: "Notion-style minimal", "Brex investor deck", "Linear pitch".
4. **Click cada slide** y reemplazá el contenido por el del .md slide por slide.
5. **Branding**: en "Theme", configurá:
   - Primary color: `#00FF6A` (neón verde AgroGlobalDex)
   - Background: `#05080A` (negro)
   - Font: DM Serif Display (titulares) + Outfit (body)
6. **Export**: PDF directo, o link compartible (con analytics tipo DocSend).

### Costo

- **Gratis**: hasta 3 presentaciones públicas, link sharing.
- **Pro** ($10/mes): unlimited, custom branding, analytics avanzados.

---

## OPCIÓN 3 — Figma directo (1-2 horas · gratis · resultado más profesional)

Si querés **el deck más customizable** y tenés 1-2 horas, hacerlo en Figma directo.

### Paso a paso

1. **https://figma.com** → cuenta gratis.
2. **Community → Templates**: buscá "pitch deck" — los más bajados son **Y Combinator Pitch Deck** y **Sequoia template**.
3. Click **"Duplicate"** sobre el template.
4. **Cada frame del template = una slide**. Reemplazá texto copiándolo del .md.
5. **Paleta**: cambiá los colores del template en Inspector → Colors:
   - Primario: `#00FF6A` (neón)
   - Background: `#05080A`
   - Texto: `#ECF0EC`
   - Gold accent (para sección Yield/Invest): `#C8A84B`
6. **Fonts**: importá DM Serif Display + Outfit (gratis Google Fonts).
7. **Export**: File → Export → PDF (cada frame).

### Templates Figma gratis recomendados (links directos)

- [YC Pitch Deck Template](https://www.figma.com/community/file/1015521434097420935) — clásico, va al hueso.
- [Notion-style Pitch](https://www.figma.com/community/file/892101050953192319) — minimalist.
- [Crypto pitch deck](https://www.figma.com/community/file/1167093447019773795) — temática blockchain.

---

## OPCIÓN 4 — Canva (45 minutos · gratis · más fácil)

Si nunca usaste Figma, Canva es más simple y tiene mejores templates "agro".

### Paso a paso

1. **https://canva.com** → cuenta gratis.
2. **Search**: "pitch deck" → filtrar por gratis.
3. Recomendados: "Black Modern Pitch Deck", "Green Agriculture Pitch", "Minimal Startup Deck".
4. Duplicar template → reemplazar texto del .md slide por slide.
5. Cambiar paleta a los colores de AgroGlobalDex.
6. **Imágenes**: Canva tiene biblioteca free de fotos agro. Buscá "wheat", "vineyard", "cattle", "farm tech".
7. **Export**: Descargar como PDF.

### Costo

- **Gratis** suficiente para 14 slides.
- **Pro** ($13/mes): más templates, magic resize, branding kit.

---

## OPCIÓN 5 — Slidev (developer mode · 1 hora · gratis)

Si querés controlar todo desde código (markdown a slides reveal.js). Para los muy puristas.

```bash
npm init slidev@latest
# Pegar el contenido de pitch-deck-full.md adaptado a sintaxis Slidev
# (separador "---" entre slides)
npm run dev
# Export: npm run export → PDF
```

---

## Comparativa rápida

| Opción | Tiempo | Costo | Resultado | Para quién |
|---|---|---|---|---|
| **Gamma.app** | 15 min | Gratis | 8/10 | **Empezá por acá** |
| **Pitch.com** | 30 min | Gratis | 8.5/10 | Control + templates VC |
| **Figma** | 1-2 h | Gratis | 10/10 | Querés customizar todo |
| **Canva** | 45 min | Gratis | 7/10 | Nunca usaste Figma |
| **Slidev** | 1 h | Gratis | 8/10 | Sos developer |

---

## Mi recomendación operativa

**Hacé esto en este orden:**

1. **Hoy** (15 min): pasalo a Gamma → PDF v1. Útil para mandar **mañana** a los productores de vinos España y carnes Venezuela junto al LOI.
2. **Esta semana** (1-2 h): refiná en Figma o Pitch para los inversores serios.
3. **Cuando tengas LOIs firmadas** (10 min): actualizá el slide 8 de Traction con los nombres y vuelvé a exportar.

---

## Capturas que necesitás incluir en el deck

Estas screenshots no las puedo generar yo, las hacés vos desde la web local:

- **Slide 5 (Product)**: 4 capturas — `/marketplace.html`, `/tokenize.html`, `/invest.html`, `/aggregate.html`. Cada una a 1280x720 px.
- **Slide 8 (Traction)**: screenshot de Solscan mostrando una tx del seed (`anchor build` + `seed-localnet.ts` que ya validamos).
- **Slide A2 (Architecture)** del apéndice: el diagrama ASCII del README de Solana, convertido a imagen con https://asciiflow.com o https://www.draw.io.

## Color palette y assets

Para que cualquier herramienta tenga los assets correctos, en el repo está:

- **Logo SVG**: `web 2.0/icons/icon.svg` (lo subís a Gamma/Pitch/Figma).
- **Logo PNG 512**: `web 2.0/icons/icon-512.png`.
- **Colores en CSS variables** (copialos):
  - Background: `#05080A`
  - Background 2: `#0A0F0C`
  - Neon green primary: `#00FF6A`
  - Gold accent: `#C8A84B`
  - Text: `#ECF0EC`
  - Muted text: `#7A9080`
  - Border: `#131D14`
- **Fuentes**: DM Serif Display + Outfit (ambas Google Fonts gratis).

## Si te frenás en algún paso

Mandame screenshot al chat o gh issue. Lo más común que te puede frenar:
- Gamma pide login con Google → ok, es gratis.
- Figma "no se pueden editar templates Community" → click "Duplicate" primero.
- Canva pone marca de agua en el PDF → usá el free trial de 30 días.
