# 07 — Metrics & KPIs

**Principio:** medí pocas cosas, medilas bien, todas las semanas. Si no las podés calcular en 10 minutos los lunes, no las medís.

---

## North-star metric (1 sola)

**Volumen de RWA tokenizado listado o en LOI en AgroGlobalDex (USD equivalente).**

- Por qué: captura el resultado final (somos un marketplace; sin assets listados no hay producto).
- Cómo medir: suma del valor nominal de tokens emitidos por originadores con los que firmamos LOI/MOU/listing.
- Meta día 90: USD 5M-10M en LOIs (no en trading real todavía).
- Meta día 180: USD 25M en LOIs, USD 1M en trading real.
- Meta día 365: USD 100M en LOIs, USD 10M en trading real.

[VERIFICAR si estos targets son consistentes con el modelo financiero del founder.]

---

## Leading indicators (lo que sí podés controlar semana a semana)

### Outreach
| KPI | Meta semanal | Meta 90 días |
|---|---|---|
| Touches enviados (cold emails + DMs personalizados) | 30 | 350+ |
| Reply rate | > 15% | — |
| Calls agendadas | 3-5 | 30+ |
| Calls hechas | 3-5 | 25+ |

### Pipeline
| KPI | Meta semanal | Meta 90 días |
|---|---|---|
| Conversaciones activas (= 2+ touches con respuesta) | +2 | 20+ |
| LOIs en negociación | — | 5+ |
| LOIs firmados | — | 1 (día 30), 3 (día 90) |
| Pilotos firmados | — | 1 (día 60) |

### Producto / Demo
| KPI | Meta semanal | Meta 90 días |
|---|---|---|
| Sesiones de demo trackeadas (DocSend / Notion) | 5+ | 50+ |
| Tiempo promedio en deck | > 60 seg | — |
| Solana testnet uptime | — | > 95% post-launch |

### Investor pipeline
| KPI | Meta 90 días |
|---|---|
| Inversores Tier 1 contactados | 20+ |
| Calls VC realizadas | 10+ |
| Inversores en data room | 5+ |
| Term sheets recibidos | 1+ |

---

## Métricas de calidad (cualitativas pero trackeables)

- **Top 3 objeciones repetidas esta semana** (de calls). Si la #1 se repite 3 semanas, hay que refactorizar el deck.
- **Top 3 elogios espontáneos** (qué les gustó). Reforzar en outreach futuro.
- **NPS interno** del founder: 1-10, "¿cómo me siento sobre la tracción esta semana?" — proxy de burnout y dirección.

---

## Plantilla de tracking semanal (Sheet o Notion)

### Sheet 1: Pipeline

| Fecha 1er touch | Empresa | Persona | Rol | Tier | Canal | Template usado | Última actividad | Próximo paso | Owner | Estado |
|---|---|---|---|---|---|---|---|---|---|---|
| 2026-05-13 | Agrotoken | E. Novillo Astrada | CEO | 1 | Email | T1-ES | Cold sent | Wait 7d → FU1 | CP | Sent |
| ... | ... | ... | ... | ... | ... | ... | ... | ... | ... | ... |

**Estados posibles:** Sent · Replied · Call scheduled · Call done · Proposal sent · LOI negotiating · LOI signed · Lost · Cold-storage

### Sheet 2: Weekly review (cada lunes 9:00)

```
SEMANA: [#]  | FECHA: [YYYY-MM-DD]

Outreach esta semana:    [#]  (meta: 30)
Replies recibidas:       [#]  ([%])
Calls agendadas:         [#]
Calls realizadas:        [#]
Pipeline activo:         [#] conversaciones

LOIs en negociación:     [#]
LOIs firmados acumulados:[#]
Pilotos firmados:        [#]

Top 3 objeciones:
1.
2.
3.

Top 3 elogios:
1.
2.
3.

Decisión más importante esta semana:

Próxima semana — top 3 prioridades:
1.
2.
3.

NPS del founder (1-10):
Comentario libre:
```

### Sheet 3: Investor tracker (a partir de semana 9)

| Fondo | Persona | Stage | 1er touch | Última activ. | Next step | Notas |
|---|---|---|---|---|---|---|
| ... | ... | First call / 2nd call / DD / TS / Closed | ... | ... | ... | ... |

---

## Definiciones operativas (para no engañarte solo)

- **"Touch"**: mensaje 1:1 personalizado. Newsletter blast NO cuenta.
- **"Conversación activa"**: hubo al menos 1 respuesta de la otra parte. Sin respuesta = no es conversación.
- **"LOI"**: documento escrito (1-2 páginas), firmado por ambas partes, con scope mínimo (qué piloto, qué métricas). Un email de "sí, vamos para adelante" NO es un LOI.
- **"Piloto firmado"**: contrato (puede ser MOU) con: scope, tiempo, métricas de éxito, responsabilidades técnicas, IP. Sin contrato firmado = no es piloto.
- **"Volumen en LOI"**: valor nominal acordado por escrito. Promesas verbales NO cuentan.

---

## Anti-patterns (señales de que las métricas te están mintiendo)

- **Reply rate alto pero 0 calls**: los replies son "no gracias" educados. Mensaje o target mal.
- **Calls altas pero 0 segundos pasos**: el pitch no convierte. Iterar deck.
- **Pipeline grande pero estancado**: hay vanity-leads. Limpiar el sheet: si llevamos 30 días sin actividad, va a cold-storage.
- **Mucho time-in-Twitter, poco time-in-outbox**: si una semana < 30 touches, hay que sentarse y revisar dónde se fue el tiempo.

---

## Reporting trimestral (día 90)

Generá un mini-reporte de 1 página (público o para inversores) con:

- North-star: USD en LOIs
- 5 leading indicators (cuáles cumpliste, cuáles no)
- Top 3 lecciones aprendidas
- Plan trimestre siguiente (3 objetivos máximo)
- Pedido específico (warm intros, capital, hires)

---

## Herramientas sugeridas (mínimo viable)

| Función | Herramienta | Costo | Crítico? |
|---|---|---|---|
| CRM ligero | Notion / Airtable / Google Sheets | Free-low | Sí |
| Email tracking | DocSend / HubSpot free | Free-30€ | Sí |
| Email finder | Hunter.io / Apollo | 50€/mo | Sí |
| Scheduling | Calendly | Free | Sí |
| Analytics demo (web) | Plausible / Vercel Analytics | 10€/mo | Medio |

No hace falta más en los primeros 90 días. Si te ves comprando un Salesforce, parálo.
