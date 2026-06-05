# AgroGlobalDex

> **Marketplace global de RWA agropecuarios tokenizados sobre Solana.**
> Compliance-first (MiCA-aligned) · Aggregator cross-país y cross-sector · Demo/PoC.

⚠️ **Estado: Demo / Proof of Concept.** Repositorio en fase de diseño y desarrollo. NO es plataforma operativa, NO constituye oferta de valores ni servicios sobre criptoactivos, y NO está autorizada como CASP MiCA al día de hoy.

---

## 📥 Descargar la app (sin compilar nada)

### **➡️ https://github.com/carlospallottini-spec/agroglobaldex/releases/latest**

| Plataforma | Archivo | Notas |
|---|---|---|
| 🪟 Windows 10/11 | `AgroGlobalDex-Setup-2.3.0.exe` | Instalador NSIS — click → Next → Next |
| 🪟 Windows portable | `AgroGlobalDex-2.3.0-portable.exe` | Sin instalar — doble click y corre |
| 🍎 macOS | `AgroGlobalDex-2.3.0.dmg` | Intel + Apple Silicon |
| 🐧 Linux | `AgroGlobalDex-2.3.0.AppImage` | `chmod +x` y ejecutar |
| 🤖 Android | `AgroGlobalDex-2.3.0-debug.apk` | Debug-signed APK. Habilitar "Orígenes desconocidos" → instalar. Play Store post-funding. |
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

**11 páginas listas:**
- `/index.html` — landing
- `/marketplace.html` — listings on-chain
- `/tokenize.html` — wizard para productores (4 pasos)
- `/invest.html` — yield offerings (con settlement history on-chain)
- **`/borrow.html`** — lending market: USDC contra colateral tokenizado, APR fijo, LTV 50%
- **`/receipts.html`** — ledger público de TradeReceipts (proof-of-trade)
- `/aggregate.html` — admin / curador (cross-platform tokens)
- `/admin.html` — operaciones avanzadas (revoke_kyc, settle, update_metadata, transfer_issuer)
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

**30 instrucciones · 2 programas Anchor · 5 AssetClass (Grain · CarbonCredit · HarvestFraction · InvestmentOffering · Commodity con 9 sectores) · oráculo Pyth para colateral.** Detalles en [`solana/README.md`](solana/README.md).

### Cambios v0.3 (audit pre-mainnet)

Audit interno completo (30 issues identificados). 3 críticos + 8 HIGH/MED/LOW fixeados:

| Audit # | Fix | Impacto |
|---|---|---|
| #1 CRITICAL | `JurisdictionPolicy` on-chain real (no hardcoded BLOCKED) | `update_jurisdiction_policy` ahora aplica realmente |
| #2 CRITICAL | TransferHook valida PDAs por derivación, no solo owner | Cierra account-substitution attack |
| #3 CRITICAL | CPI a `compliance_hook::initialize_extra_account_meta_list` en `register_asset` | Sin este fix NINGÚN token nativo era transferible |
| #4 HIGH | Fee_bps re-checkeado en buy_asset/buy_external_asset | Defensa en profundidad |
| #5 HIGH | `update_listing_price` valida source_registry | Bypass del check antes posible |
| #6 HIGH | `cancel_listing` cierra escrow ATA | Sin rent griefing |
| #8 MED | Seeds explícitos en asset_registry / external_asset (buy paths) | Defensa en profundidad |
| #9 MED | `redeem` respeta `marketplace.paused` | Circuit breaker completo |
| #11 MED | Compliance-hook valida discriminator antes de parsear | Cierra type-confusion attack |
| #13 LOW | `set_compliance_signer` rechaza default / same signer | Sanity |

Y 4 nuevas instrucciones (HIGH: #15, #16, #19, #20):
- `revoke_kyc(reason_code)` — compliance signer revoca KYC ante sanctions/fraud/regulatory
- `settle_investment_offering(epoch, yield_paid, attestation)` — receipt on-chain de yield distribuido off-chain
- `update_metadata(name, uri, wp_uri)` — issuer actualiza metadata pre-mint
- `transfer_issuer()` — ceder el rol issuer al nuevo wallet (KYC-verified)

Además: `AssetRegistry.redeemed_supply` tracking (audit #18), `aggregate.active=false` por default (audit #7).

23 tests mocha. Run book operativo en [`solana/RUNBOOK.md`](solana/RUNBOOK.md).
Security policy en [`SECURITY.md`](SECURITY.md).

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
