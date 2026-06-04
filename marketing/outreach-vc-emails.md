# Outreach email pack — AgroGlobalDex pre-seed

> Templates listos para enviar. Cada email tiene asunto + cuerpo + signature.
> Personalizá el primer párrafo con UN dato específico del destinatario
> (porque genérico = ignore).
>
> **NUNCA prometer yield garantizado.** Disclaimer + Demo/PoC obligatorios
> al final.

---

## Signature común (pegar al final de cada email)

```
Carlos Pallottini
Founder & CEO · AgroGlobalDex
carlos@agroglobaldex.io · linkedin.com/in/[VERIFICAR]
github.com/carlospallottini-spec/agroglobaldex

—
AgroGlobalDex is a Proof-of-Concept. Not authorized as a Crypto-Asset
Service Provider under MiCA at the date of this email. This message does
not constitute an offer of crypto-assets or investment services. No yields
guaranteed.
```

---

# Email A — VCs crypto-agro (top 5)

**Cuándo usar**: cold outbound a partners de fondos con thesis explícita en
agro / RWA / crypto-infra. Ticket esperado: USD 25k–250k.

**Asunto**: `AgroGlobalDex — MiCA-first agro RWA marketplace + on-chain lending (Solana)`

**Cuerpo** (parametrizado por VC abajo):

> Hola [Nombre],
>
> [PARRAFO PERSONALIZADO — uno de los 5 abajo]
>
> Construí **AgroGlobalDex** — un marketplace de RWA agropecuarios sobre
> Solana con compliance MiCA enforced on-chain (Token-2022 TransferHook),
> un **lending market on-chain** donde el productor lockea su cosecha
> tokenizada como colateral y recibe USDC en segundos, y un ledger
> público de comprobantes de trade (`TradeReceipt` PDAs auditables sin
> parser JSON).
>
> El stack es 27 instrucciones en 2 programas Anchor, 28 tests, audit
> interno completo (3 críticos + 8 HIGH/MED/LOW fixeados), repo
> público. Devnet running. Sec3 audit contratada para Q3.
>
> Estoy cerrando un **pre-seed USD 500k @ USD 5M cap (SAFE post)**.
> 2 LOIs en negociación: bodegas DOC España (vinos) y frigoríficos
> Venezuela (carnes). Ticket 25k-100k bienvenido.
>
> ¿Tenés 20 minutos esta semana? Te paso deck + cap table + acceso a
> devnet en el call.
>
> Carlos
>
> [signature común]

### Personalización por VC

**1. S2G Ventures** *(thesis: food + ag innovation)* — *[VERIFICAR partner con focus crypto]*
> "Vi que invirtieron en [Indigo/Cattle Beef/etc. — VERIFICAR portfolio
> reciente]. AgroGlobalDex extiende esa thesis a la capa financiera: el
> productor tokenizado por un proyecto que ustedes fondearon podría usar
> AgroGlobalDex como rail de settlement + crédito sin construir esa
> infra in-house."

**2. FoodLabs (DE)** *(thesis: food-tech early-stage UE)*
> "Su foco UE + alimentación me hace sentido directo. Soy founder
> argentino apuntando a CASP Francia (no Alemania, por velocidad
> ACPR) — me encantaría tu input sobre nuestra estrategia jurisdiccional
> antes de cerrarla."

**3. Distributed Global** *(thesis: crypto infra, RWA-curious)*
> "Su tesis pública de 'crypto-native infrastructure for traditional
> markets' es lo que estamos construyendo en el ag layer. Específicamente,
> nuestro `TradeReceipt` PDA estructurado vs los NFTs de metadata que
> dominan el espacio RWA — creo que esa diferencia técnica le va a
> interesar a su equipo."

**4. Multicoin Capital** *(thesis: Solana-native, focused on token-2022 use cases)*
> "Solana Token-2022 TransferHook es nuestro core differentiator vs
> equivalentes ETH. Nadie más en el espacio agro lo usa. Vi su position
> on Solana Token Extensions en [VERIFICAR tweet / blog post] — creo
> que somos exactamente el tipo de uso production-realistic de TransferHook
> que su tesis predice."

**5. Foundation Capital** *(thesis: enterprise + crypto)*
> "Su trabajo con [VERIFICAR portfolio compliance-tech] sugiere que
> el regulatory-first pitch les cierra. Nuestro fork-friendly compliance-hook
> program ya es referencia para 1-2 equipos Solana que están empezando
> RWA regulado — podemos charlar de ese efecto red de devs."

---

# Email B — EIT Food (excepción institucional)

**Cuándo usar**: cold outbound a innovation manager de EIT Food. Único
público-sector / quasi-público que vale la pena pichar ahora (ciclos
3-6 meses, €80M+ deployando, foco agritech explícito).

**Asunto**: `Compliance-first agricultural tokenization on Solana — partnership inquiry`

**Cuerpo**:

> Dear [Innovation Manager Name — VERIFICAR via LinkedIn],
>
> I'm reaching out from AgroGlobalDex, a project I've built over the
> past 12 months to address a specific gap in EU agricultural finance:
> the inability of small and mid-sized producers (vineyards, olive
> growers, livestock cooperatives) to access working capital without
> going through opaque intermediary financing at 30%+ effective rates.
>
> Our approach combines three elements that to my knowledge aren't
> integrated elsewhere in the EU agritech space:
>
> 1. **Tokenization of physical agricultural output** (grains, wine,
>    oil, meat, dairy, fruit) as compliance-aware tokens on Solana,
>    with KYC and jurisdiction enforcement on-chain via SPL Token-2022
>    Transfer Hook extensions.
>
> 2. **An on-chain lending market** where producers lock their
>    tokenized harvest as collateral and receive immediate USDC
>    settlement against it — closing the capital-to-production cycle
>    without bank intermediation.
>
> 3. **MiCA-first regulatory posture**: our jurisdictional strategy
>    document (in our public legal pack) selects France (AMF/ACPR)
>    for CASP authorization, and we are building toward audited
>    mainnet deployment with MLRO + DPO contracted locally.
>
> The full technical and legal documentation is public at
> github.com/carlospallottini-spec/agroglobaldex. We are running on
> Solana devnet today and closing a USD 500k pre-seed.
>
> I'm curious whether EIT Food's RisingFoodStars or one of your
> dedicated agritech programmes might be aligned. I'd value a 30-minute
> exploratory conversation — not pitching for grant funds at this stage,
> but rather sense-checking our roadmap against your network of producer
> partners and your view on where compliance-first crypto agritech
> fits into the EU innovation landscape.
>
> Available any time this or next week.
>
> Best regards,
> Carlos Pallottini
> Founder & CEO · AgroGlobalDex
>
> [signature común]

---

# Email C — Productores agro (5 sectores)

**Cuándo usar**: outreach a productores grandes específicos — el target es
firmar 2 LOIs (los actuales en negociación) y agregar 2-3 más antes del
pre-seed close. Tono cercano, sin jerga crypto en el primer párrafo.

**Asunto** (variar por sector): `Comercializá tu [vino/carne/aceite/café/leche] directo al inversor global — sin intermediarios`

## C1 — Bodega DOC España (vinos)

> Estimado/a [Nombre del responsable comercial — VERIFICAR],
>
> Le escribo desde AgroGlobalDex, una plataforma digital que estamos
> lanzando para conectar bodegas DOC con inversores europeos directamente.
> La idea es simple: usted ofrece un porcentaje de la producción de una
> añada concreta (por ejemplo, Reserva 2026), el inversor compra una
> fracción on-line, y al momento de la venta cobra su parte automáticamente.
> Sin intermediarios financieros, sin comisión de distribución
> tradicional, sin 90 días de cobro.
>
> Estamos en fase de pilotos. Ya tenemos un Letter of Intent con
> [BODEGA REFERENCIADA — VERIFICAR si aplica] y nos interesaría
> conversar con su bodega como segundo piloto antes del lanzamiento
> público (Q4 2026).
>
> Le dejo nuestro one-pager: [adjunto / link]. Si le interesa una llamada
> de 20 minutos esta o la próxima semana, agendamos. Le aclaro de
> antemano: no le pedimos exclusividad ni que firme nada vinculante en
> esta etapa.
>
> Atte.,
> Carlos
>
> [signature común]

## C2 — Frigorífico Venezuela / Argentina (carnes)

> Estimado/a [Nombre],
>
> Soy Carlos Pallottini, founder de AgroGlobalDex — una plataforma para
> que productores ganaderos puedan vender directo al comprador
> internacional en USDC, con liquidación inmediata.
>
> Para frigoríficos en Venezuela específicamente, el approach resuelve
> dos cosas concretas: (1) cobro en USD digital sin pasar por SWIFT
> (que cada vez es más restrictivo con la jurisdicción), y (2) acceso
> a compradores institucionales europeos que hoy no pueden operar
> directo por compliance.
>
> Nuestro pilot está estructurado con arbitraje ICC Madrid como sede
> neutral y una cláusula específica sobre OFAC/BCV/SUNACRIP que ya
> validamos con nuestro counsel.
>
> ¿Tendría 15 minutos esta semana para una llamada exploratoria?
>
> Atte.,
> Carlos
>
> [signature común]

## C3 — Cooperativa olivar España

> Estimados,
>
> Les escribo desde AgroGlobalDex. Estamos lanzando una infraestructura
> digital para que cooperativas olivareras puedan vender directamente
> aceite EVOO premium al consumidor final B2B (HoReCa, distribuidores
> especialistas) en EU + UK + LATAM, con cobro inmediato en USDC y
> trazabilidad on-chain del lote desde el molino al destino.
>
> Es complementario a su canal habitual — no compite con sus
> distribuidores tradicionales, sino que les abre el segmento premium
> internacional que hoy se les escapa por barreras de pago y logística
> de muestras.
>
> ¿Una llamada exploratoria? Sin compromiso.
>
> Atte.,
> Carlos
>
> [signature común]

## C4 — Cafetero Colombia / Brasil (café especial)

> Hola [Nombre],
>
> Soy Carlos, founder de AgroGlobalDex. Te escribo porque vi
> [PERSONALIZACIÓN — viste su finca en Instagram, leíste un perfil, un
> compañero te referenció — VERIFICAR contexto].
>
> Estamos construyendo una plataforma donde fincas de café especial
> (SCA 85+) tokenizan lotes específicos y los venden directo a
> coffee shops, roasters y aficionados premium en Europa y Asia. El
> precio lo ponés vos, el comprador paga upfront en USDC, vos despachás
> contra el ticket on-chain. Cero spread del intermediario.
>
> Tenemos un piloto pendiente con [REFERENCIA — VERIFICAR si aplica].
> Si te interesa, podemos charlar 20 min esta semana.
>
> Carlos
>
> [signature común]

## C5 — Lácteo Brasil / Argentina

> Estimado/a [Nombre],
>
> Soy Carlos Pallottini, founder de AgroGlobalDex. Estamos onboardeando
> productores lácteos a una plataforma para vender productos premium
> (quesos curados, mantequilla artesanal, yogur orgánico) directo al
> comprador internacional con cobro inmediato en USDC.
>
> El uso de blockchain es el medio, no el objetivo: lo importante para
> ustedes es cobro al día 0 vs los 60-90 días tradicionales del canal
> mayorista, y acceso a compradores institucionales europeos que hoy
> filtra el distribuidor.
>
> ¿15 minutos esta semana?
>
> Carlos
>
> [signature común]

---

# Email D — Agregadores (Agrotoken, Topaz, Centrifuge)

**Cuándo usar**: NO posicionarse como competidor. Posicionarse como su
canal de distribución UE bajo compliance MiCA.

## D1 — Agrotoken (Eduardo Novillo Astrada, AR)

**Asunto**: `Solana Token-2022 distribution layer for Agrotoken under MiCA — quick chat?`

> Eduardo,
>
> Soy Carlos Pallottini, founder de AgroGlobalDex. No te escribo a
> competir: te escribo a ofrecerte distribución regulada en EU.
>
> Construí una capa de agregación sobre Solana donde tokens RWA-agro
> de otras plataformas (la tuya incluida) aparecen en un marketplace
> único para el inversor UE, con KYC enforced on-chain via Token-2022
> TransferHook y compliance MiCA-first. Estamos targetando CASP
> Francia (AMF/ACPR).
>
> El upside para Agrotoken: distribución institucional UE sin que tengan
> que armar su propio CASP filing — los soja-coins de Agrotoken
> aparecerían en el marketplace para inversores europeos KYC'd. Nosotros
> manejamos el regulatory layer, ustedes mantienen el control del
> token y la relación con el productor.
>
> ¿Tendrías 30 min este mes para charlar de cómo se estructura técnicamente?
>
> Carlos
>
> [signature común]

## D2 — Topaz (BR, Solana RWA)

**Asunto**: `Same Solana stack, complementary scope — partnership chat?`

> Hi [Topaz founder/BD lead — VERIFICAR],
>
> I'm Carlos Pallottini from AgroGlobalDex. Same Solana + Anchor +
> Token-2022 stack as Topaz, but our scope is **cross-country agro
> assets with MiCA compliance as the wedge** (we're targeting CASP
> authorization in France) rather than the BR-domestic RWA broader
> focus.
>
> I think there's a clean complementary play: your tokens listed on our
> aggregator for EU investor access, our investors getting BR exposure
> through your issuance. No competition for liquidity, just additional
> distribution.
>
> 20-minute call this week or next?
>
> Carlos
>
> [signature común]

## D3 — Centrifuge (cross-chain RWA)

**Asunto**: `Solana-native distribution for Centrifuge agro pools?`

> Hi [Centrifuge BD — VERIFICAR],
>
> AgroGlobalDex is a Solana-native marketplace for agricultural RWAs.
> We aggregate cross-chain tokens (including Ethereum-based Centrifuge
> agro pools) as **display-only** assets in our marketplace, with
> direct links back to your platform for the actual purchase. This
> gives your pools visibility to the Solana-native investor base
> without you having to issue on Solana.
>
> No commercial ask in this email — just want to confirm we have the
> right contact for keeping the pool data we're aggregating accurate
> as your pools update. And explore whether there's a deeper integration
> down the road.
>
> Carlos
>
> [signature común]

---

# Reglas de follow-up

- **Email 1 → 0 respuesta**: esperar 5 días hábiles, enviar follow-up
  cortito ("Hi [name], just floating this back up — anything I can
  clarify?"). Máximo 2 follow-ups, después archivar.
- **Email con respuesta neutra ("interesting, let me think")**: agendar
  recordatorio para 2 semanas. No empujar.
- **Email con call agendado**: enviar deck completo + cap table + acceso
  a devnet **24 horas antes** del call, no después. Que llegue al call
  con preguntas, no leyendo el deck en vivo.
- **Tracking**: hoja de cálculo o Linear/Notion board con cada outreach:
  fecha enviado, respuesta sí/no, próximo paso, próxima fecha.

---

## Disclaimer final

Todos los emails de arriba son borradores. **Reemplazar todos los
`[VERIFICAR]` con datos reales antes de enviar.** Personalizar el primer
párrafo de cada uno con un dato específico del destinatario (su tweet,
su portfolio, una mención conjunta) — emails 100% genéricos tienen <1%
de respuesta. Demo/PoC banner mantener en cualquier follow-up con
material adjunto. No prometer yield garantizado en ningún canal,
público o privado.
