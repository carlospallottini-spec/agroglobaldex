# AgroGlobalDex

> **Marketplace global de RWA agropecuarios tokenizados sobre Solana.**
> Compliance-first (MiCA-aligned) · Aggregator cross-país y cross-sector · Demo/PoC.

⚠️ **Estado: Demo / Proof of Concept.** Repositorio en fase de diseño y desarrollo. NO es plataforma operativa, NO constituye oferta de valores ni servicios sobre criptoactivos, y NO está autorizada como CASP MiCA al día de hoy.

---

## 📥 Descargar la app (sin compilar nada)

### **➡️ https://github.com/carlospallottini-spec/agroglobaldex/releases/latest**

| Plataforma | Archivo | Notas |
|---|---|---|
| 🪟 Windows 10/11 | `AgroGlobalDex-Setup-2.0.0.exe` | Instalador NSIS — click → Next → Next |
| 🪟 Windows portable | `AgroGlobalDex-2.0.0-portable.exe` | Sin instalar — doble click y corre |
| 🍎 macOS | `AgroGlobalDex-2.0.0.dmg` | Intel + Apple Silicon |
| 🐧 Linux | `AgroGlobalDex-2.0.0.AppImage` | `chmod +x` y ejecutar |
| 📱 Mobile (PWA) | Web installable | Abrí la web en el móvil → "Añadir a pantalla de inicio" |

Cada vez que tagueamos `v*` el workflow buildea Windows + macOS + Linux en paralelo y los publica como Release. Tamaño ~90 MB.

---

## 🌐 Probar en el navegador (sin instalar)

```bash
git clone https://github.com/carlospallottini-spec/agroglobaldex
cd agroglobaldex
python3 -m http.server 8000 --directory "web 2.0"
# Abrir http://localhost:8000/index.html
```

**8 páginas listas:**
- `/index.html` — landing
- `/marketplace.html` — listings on-chain
- `/tokenize.html` — wizard para productores (4 pasos)
- `/invest.html` — yield offerings
- `/aggregate.html` — admin / curador
- `/investors.html` — fundraising pre-seed
- `/about.html`, `/contact.html`, `/team.html`

PWA installable desde mobile.

---

## 🔗 Solana smart contracts

```bash
cd solana
sh -c "$(curl -sSfL https://release.anza.xyz/v3.0.0/install)"
cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked

anchor build
ulimit -n 65536   # CRÍTICO para que el validator no muera
solana-test-validator --reset &
solana config set --url http://127.0.0.1:8899 && solana airdrop 200
solana program deploy --program-id target/deploy/agroglobaldex-keypair.json target/deploy/agroglobaldex.so
solana program deploy --program-id target/deploy/compliance_hook-keypair.json target/deploy/compliance_hook.so
npm install
npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts
```

**17 instrucciones · 2 programas Anchor · 5 AssetClass (Grain · CarbonCredit · HarvestFraction · InvestmentOffering · Commodity con 9 sectores).** Detalles en [`solana/README.md`](solana/README.md).

---

## 📁 Estructura

| Carpeta | Qué contiene |
|---|---|
| [`solana/`](solana/) | Programa Anchor principal + `compliance-hook` (Transfer Hook). Build OK. |
| [`web 2.0/`](web%202.0/) | App web (8 páginas) + PWA + cliente Anchor JS + multi-wallet. |
| [`agrochain-electron/`](agrochain-electron/) | Wrapper Electron para builds desktop. Mirror de `web 2.0/`. |
| [`mobile/`](mobile/) | Capacitor scaffold para APK Android. |
| [`marketing/`](marketing/) | Pitch deck, financials, FAQ inversores, LOI templates, Colosseum submission, guías. |
| [`legal/`](legal/) | Análisis MiCA, T&Cs draft, Privacy, white paper template, checklist. |
| [`docs/`](docs/) | Whitepaper. |
| [`.github/workflows/`](.github/workflows/) | CI builds automáticos del `.exe`/`.dmg`/`.AppImage` por tag. |

---

## 🚀 Para inversores

Servir [`/investors.html`](web%202.0/investors.html) local o ver:

| Documento | Para qué sirve |
|---|---|
| [Pitch Deck completo](marketing/pitch-deck-full.md) | 14 slides + apéndice |
| [Cómo pasar el deck a visual](marketing/pitch-deck-to-figma-guide.md) | Gamma 15min · Pitch · Figma · Canva · Slidev |
| [Financial model 5y](marketing/financials-5y.md) | Proyecciones + sensitivity + comparables |
| [One-pager](marketing/one-pager.md) | 300 palabras para mail/DM |
| [FAQ inversores](marketing/faq-investors.md) | 20 preguntas honestas |
| [Cap table template](marketing/cap-table-template.md) | Pre-seed → Seed → Series A |
| [Demo video script](marketing/demo-video-script.md) | 90s listo para grabar |
| [Grant applications](marketing/grant-applications.md) | Solana Foundation + Horizon Europe |
| [LOI templates](marketing/loi-templates.md) | España (vinos) + Venezuela (carnes) |
| [Colosseum submission](marketing/colosseum-submission.md) | README hackathon Solana |
| [Data room structure](marketing/data-room-structure.md) | DD folder |

---

## ⚖️ Disclaimer

PoC. Sin audit, sin operación en mainnet, sin autorización CASP MiCA al día de hoy. Cualquier integración productiva requiere completar el audit, la autorización regulatoria, capital regulatorio y el resto del compliance roadmap documentado en [`legal/`](legal/).

## License

MIT (código) · CC-BY-4.0 (docs).

## Contacto

**Carlos Pallottini** — Founder
carlos@agroglobaldex.io
