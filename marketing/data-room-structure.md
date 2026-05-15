# AgroGlobalDex — Data Room (estructura)

> Estructura recomendada para Notion / DocSend / Google Drive compartido con
> inversores durante due diligence. **No compartir el data room antes del
> meeting #2** — pre-meeting solo manda el one-pager y el deck.

```
DATA ROOM/
├── 01_OVERVIEW/
│   ├── one-pager.pdf                       (de marketing/one-pager.md)
│   ├── pitch-deck.pdf                      (de marketing/pitch-deck-full.md → Figma → PDF)
│   ├── demo-video.mp4                      (90s, ver marketing/demo-video-script.md)
│   └── faq-investors.pdf                   (de marketing/faq-investors.md)
│
├── 02_PRODUCT/
│   ├── live-demo-url.txt                   (URL a la web en devnet pública)
│   ├── github-readme.pdf                   (snapshot del README principal)
│   ├── architecture-diagram.png            (de solana/README.md ASCII → diagrama visual)
│   ├── compliance-hook-explainer.pdf       (cómo funciona el Transfer Hook)
│   └── screenshots/                        (capturas de las 8 páginas)
│
├── 03_TECH/
│   ├── github-repo-link.txt                (https://github.com/carlospallottini-spec/agroglobaldex)
│   ├── tech-stack-summary.md
│   ├── solana-program-instructions.md      (lista de las 17 ix + 14 events)
│   ├── token-2022-features.md              (TransferHook, MetadataPointer, TokenMetadata)
│   ├── multi-wallet-support.md             (Phantom + Solflare + Backpack + Glow + MWA)
│   ├── pwa-mobile-apk.md                   (capabilities del wrapper Capacitor)
│   └── security-roadmap.md                 (audit plan, multisig plan, monitoring)
│
├── 04_LEGAL/
│   ├── mica-analysis.pdf                   (de legal/01-mica-analysis.md)
│   ├── asset-classification.pdf            (de legal/02-asset-classification.md)
│   ├── permits-and-licenses.pdf            (de legal/03-permits-and-licenses.md)
│   ├── kyc-aml-framework.pdf               (de legal/04-kyc-aml-framework.md)
│   ├── jurisdictional-strategy.pdf         (de legal/05-jurisdictional-strategy.md)
│   ├── terms-of-service.draft.pdf          (de legal/06-terms-of-service.draft.md)
│   ├── privacy-policy.draft.pdf            (de legal/07-privacy-policy.draft.md)
│   ├── white-paper-template.pdf            (de legal/08-white-paper-template.md)
│   ├── risk-disclosures.pdf                (de legal/09-risk-disclosures.md)
│   └── compliance-checklist.pdf            (de legal/10-compliance-checklist.md)
│
├── 05_MARKET/
│   ├── ecosystem-research.pdf              (de marketing/01-ecosystem-research.md)
│   ├── target-list.pdf                     (de marketing/02-target-list.md, redacted)
│   ├── positioning-and-messaging.pdf       (de marketing/05-positioning-and-messaging.md)
│   ├── launch-plan-90days.pdf              (de marketing/06-launch-plan-90days.md)
│   └── metrics-and-kpis.pdf                (de marketing/07-metrics-and-kpis.md)
│
├── 06_BUSINESS_MODEL/
│   ├── two-use-cases-explainer.md          (Caso 1 + Caso 2 deep dive)
│   ├── revenue-streams.md                  (trading + listing + aggregator fees)
│   ├── take-rate-evolution.xlsx
│   └── pricing-strategy.md
│
├── 07_FINANCIALS/
│   ├── financials-5y.xlsx                  (de marketing/financials-5y.md → spreadsheet real)
│   ├── unit-economics.xlsx                 (per pilot: CAC, LTV, payback)
│   ├── sensitivity-analysis.xlsx
│   ├── runway-calculator.xlsx
│   └── comparables-valuation.md
│
├── 08_TEAM_AND_CAPTABLE/
│   ├── founder-cv.pdf                      (Carlos Pallottini)
│   ├── advisor-bios.pdf                    (cuando estén firmados)
│   ├── cap-table.xlsx                      (de marketing/cap-table-template.md)
│   ├── vesting-schedules.md
│   └── hiring-plan-12mo.md
│
├── 09_GO_TO_MARKET/
│   ├── outreach-templates.md               (de marketing/04-outreach-templates.md)
│   ├── pilot-pipeline.md                   (estado de cada cooperativa contactada)
│   ├── partnerships-pipeline.md            (estado de cada plataforma aggregable)
│   └── conferences-events.md               (qué hackathons/agro events vamos a hacer)
│
└── 10_ROUND_DETAILS/
    ├── safe-template.pdf                   (YC SAFE post-money)
    ├── term-sheet-summary.md
    ├── use-of-funds-detailed.xlsx
    ├── investor-rights.md
    ├── existing-commitments.md             (lead + soft circles)
    └── timeline-and-close-date.md
```

## Protocolo de acceso

1. **Pre-meeting**: one-pager (PDF público).
2. **Post-meeting #1**: enviar deck + faq (vía link DocSend para track).
3. **Meeting #2 (interested)**: dar acceso al data room SOLO `01_OVERVIEW`, `02_PRODUCT`, `06_BUSINESS_MODEL`, `10_ROUND_DETAILS`.
4. **Meeting #3 (due diligence)**: dar acceso completo, watermarked.
5. **Soft commit**: open Slack channel para Q&A directo.
6. **Term sheet signed**: full transparency, monthly board updates.

## Tooling sugerido

- **DocSend** ($10/mes) — para track quién abrió qué, cuánto tiempo, qué slides. Crítico para identificar warm leads.
- **Notion** (gratis para pequeños teams) — para colaborar internamente en los docs.
- **Google Drive** — para los archivos pesados (videos, spreadsheets).
- **Calendly** — para scheduling sin friction.

## Update cadence

- **Pre-round**: data room frozen al cierre del round target.
- **Post-round**: monthly updates a inversores (email + KPI dashboard).
- **Continuous**: actualizar `09_GO_TO_MARKET/pilot-pipeline.md` semanal.
