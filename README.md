# AgroGlobalDex — AgroChain

> **El marketplace global de activos agropecuarios tokenizados.**
> Invierte en ganado, cosechas y producción real con BTC, ETH, SOL, USDC o EUR. Sin bancos, sin intermediarios — todo on-chain con retornos verificados.

---

## La idea

AgroChain conecta a **productores agropecuarios** que necesitan capital con **inversores globales** que buscan exposición a activos reales del sector agro. Tokenizamos cosechas, ganado y producción láctea/cárnica para que cualquier persona pueda invertir desde **100 €** y recibir un retorno verificable a través de blockchain.

- **$280B** de mercado agro UE al año
- **<0.5 %** de comisión por transacción
- **~5 s** de liquidación on-chain
- **14 países** activos en el marketplace

## Descarga rápida (Windows)

👉 **[Descargar AgroChain (.exe) desde Releases](../../releases/latest)**

| Archivo | Para qué |
|---|---|
| `AgroChain-Portable.exe` | Doble clic. No requiere instalación. **Recomendado.** |
| `AgroChain-Setup.exe` | Instalador con acceso directo en escritorio y menú inicio. |

> Windows mostrará un aviso de SmartScreen la primera vez porque el `.exe` no está firmado digitalmente. Pulsa **Más información → Ejecutar de todos modos**.

## Estructura del repositorio

| Carpeta / archivo | Descripción |
|---|---|
| [`web 2.0/`](web%202.0/) | **Aplicación web principal.** 5 páginas: Inicio, Nosotros, Marketplace, Equipo, Contacto. HTML5 + CSS3 + JS vanilla, sin frameworks. |
| [`web agrochaindex/`](web%20agrochaindex/) | Versión alternativa con CSS/JS separados en archivos. |
| [`agrochain-electron/`](agrochain-electron/) | Wrapper Electron que empaqueta la web como app de escritorio `.exe` para Windows. |
| [`AgroChain_App_Final.html`](AgroChain_App_Final.html) | Prototipo single-file de la app. |
| `AgroChain-Logo.*` | Logo del producto en SVG, PNG y ICO. |

## Cómo ejecutarlo

### Opción 1 — Descarga el .exe

Ve a la sección [**Releases**](../../releases) y descarga `AgroChain-Portable.exe`. Doble clic y abre.

### Opción 2 — Abrir en el navegador

Abre cualquier HTML directamente:

```
web 2.0/index.html
```

### Opción 3 — Compilar el .exe tú mismo

Requiere Node.js 20+.

```bash
cd agrochain-electron
npm install
npm run build
# El .exe queda en agrochain-electron/dist/
```

## Stack técnico

- **Frontend:** HTML5, CSS3, JavaScript vanilla (sin frameworks pesados)
- **Mapas:** Leaflet con tiles de CartoDB dark
- **Desktop:** Electron 33 (Chromium + Node.js)
- **Empaquetado:** electron-builder 25 (portable + NSIS installer)
- **Iconos:** SVG vectorial → ICO multi-resolución (16/24/32/48/64/128/256 px) con `sharp` + `png-to-ico`

## Identidad visual

- Verde neón principal: `#00FF6A`
- Fondo oscuro: `#05080A`
- Tipografía display (look financiero / blockchain)

## Licencia

MIT — uso libre para revisión técnica y fines del hackathon.
