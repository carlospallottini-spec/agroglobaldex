# AgroGlobalDex — Investor FAQ

> Las 20 preguntas que más probablemente te haga un VC.
> Respuestas honestas, sin hype. Si una respuesta es "no", dilo.

---

## 1. ¿Por qué Solana y no Ethereum / Polygon / Avalanche?

- **Throughput**: 50k+ TPS, finalidad sub-segundo. Las plataformas RWA en Ethereum (Centrifuge) ejecutan al ritmo de los bloques L1, no del usuario.
- **Costos**: <$0.001 por tx. Crítico para que un productor argentino tokenizando 100 kg de carne no pierda margen en fees.
- **Token-2022**: las extensions (Transfer Hook, Metadata Pointer, Permanent Delegate) son requisitos no negociables para compliance regulatoria. Solo Solana las tiene production-ready.
- **Ecosistema agro**: Agrotoken (líder LATAM) y Topaz ya en Solana.

## 2. ¿Por qué no usar Agrotoken / RIPE / Centrifuge directamente?

Cada uno cubre un slice. Agrotoken = granos AR/BR. RIPE = USA. Centrifuge = RWA agnostic en Ethereum. **Ningún player ataca cross-sector + cross-país + MiCA**. Los agregamos en el Caso 2 — no compitamos, los distribuimos.

## 3. El TAM de USD 12T es agro total. ¿Qué porción es realmente tokenizable?

Empezamos por **commodities con certificado verificable**: granos (warehouse receipts), carnes (origen + USDA/Senasa), vinos (DOC/DOP), aceites (DOC), lácteos. Eso es ~30% del TAM = USD 3.6T. **SOM realista Y5: 0.01% = USD 360M.** Plan financiero usa 0.008% = USD 280M (conservador).

## 4. ¿Cómo defienden la cuota de mercado? ¿Cuál es el moat?

- **Compliance MiCA + Transfer Hook** = altísima barrera técnica + regulatoria de re-implementación.
- **Aggregator effect**: cada plataforma que agregamos vuelve más valioso AgroGlobalDex para el inversor. Network effect bilateral.
- **Trust layer**: ser el primero en tener audit + CASP authorization en Europa.
- **Brand**: "el marketplace agro on-chain MiCA-grade" — first-mover en una categoría legítima.

## 5. ¿Cuál es el path a profitabilidad?

Break-even Y5 con USD 280M GMV. Si el plan se atrasa 12 meses (escenario downside), break-even Y6. Si acelera (200 pilotos Y5), profitable Y4.

## 6. ¿Por qué SAFE y no equity priced?

Pre-seed standard. Más rápido cerrar (no due diligence completa, no negociación de term sheet largo). Convertible al priced seed. Cap razonable (USD 5M) protege a ambos lados.

## 7. ¿Quién es el founding team? ¿Por qué este equipo?

Carlos Pallottini — founder técnico actual. **Honestidad**: solo founder hoy. **Plan con el round**: contratar 2 ingenieros + buscar co-founder agro/regulatorio. **No mentir sobre advisors** que aún no firmaron.

## 8. ¿Quién es el competidor más serio?

**Agrotoken**. Ya tiene tracción real (~USD 30M TVL), equipo grande, ronda Series A cerrada. NO peleamos head-to-head: les ofrecemos distribución global vía aggregator (Caso 2). Esa es la pitch a Agrotoken también.

## 9. ¿Qué pasa si MiCA se relaja / endurece?

- **Relaja**: nos beneficia (menor compliance overhead).
- **Endurece**: nos beneficia (mayor barrera para nuevos entrantes).
- **Cambia drásticamente**: tenemos que adaptar el `JurisdictionPolicy` on-chain (ya es mutable) + el `compliance-hook` program (upgradeable via Squads multisig).

## 10. ¿Cómo manejan el riesgo de fraude del productor?

Tres capas:
1. **KYC on-chain**: solo wallets verificadas pueden mintear o comprar.
2. **Certificado off-chain con SHA-256 on-chain**: el productor sube el warehouse receipt / DOC / etc. y su hash queda inmutable. Si falsifica, es fraud documental clásico, denunciable.
3. **Curator network**: para tokens externos agregados, la authority del marketplace los verifica antes de listar.

**No prometemos zero-fraud**. Prometemos trazabilidad y accountability.

## 11. ¿Cómo se hace cumplir el "principal + yield" en InvestmentOffering?

**Off-chain.** Contrato legal vinculante entre productor y holders. El token on-chain es **certificado de derecho**, no instrument auto-ejecutable. Esto es deliberado: yield offering = security bajo MiFID II, no puede liquidar yield on-chain sin licencia investment firm.

## 12. ¿Qué pasa si Solana cae / forks / etc.?

- **Auditamos por OOM/CU/edge cases** antes de mainnet.
- **Multisig Squads como upgrade authority** mitiga compromise de keys.
- **Circuit breaker `set_paused`** ya implementado.
- **Plan de migración**: si Solana se vuelve no viable (improbable a corto plazo), podemos rebuild en SVM-compatible chains.

## 13. ¿Cuánto cuesta llegar a mainnet?

USD 306k–792k (rango realista). Detalle en `marketing/financials-5y.md` y en el deck slide 12.

## 14. ¿Por qué pre-seed USD 500k y no seed USD 2M ahora?

Necesitamos **product-market fit con cooperativas reales** antes de levantar más. Pre-seed da 12-15 meses de runway para:
- Hacer audit
- Cerrar 3-5 pilotos
- Obtener CASP application en curso
Después levantamos seed con tracción medible.

## 15. ¿Cuál es la estrategia de ir a mainnet vs. quedarse en devnet?

Mainnet **post-audit**, **post-CASP filing**, **post-3 pilotos firmados**. No antes. Cualquier sugerencia de "lanzar a mainnet rápido" sin esos hitos es irresponsable.

## 16. Token? ICO? Airdrop?

**No por ahora.** AgroGlobalDex es **infrastructure**, no un token de capa de aplicación. Si en el futuro emitimos token de governance, sería **post-Y2** con utility clara (fee discount, governance del JurisdictionPolicy, etc.). Pre-seed no incluye tokens — equity-only (vía SAFE).

## 17. Customer concentration risk

Y1-Y2 muy concentrado (3-12 cooperativas). Plan de diversificación:
- Sector mix (no solo granos)
- País mix (AR + BR + ES + UY)
- Aggregator partnerships diversifican el revenue stream

## 18. Exit landscape

Comparables M&A en RWA / agritech:
- **Sygnum** (Swiss tokenization) — bank acquisition, USD 200M+ valuation.
- **Securitize** (USA RWA) — multiple raises USD 100M+ each, IPO talk.
- **Carbonplace** — Bank-backed carbon, Series B USD 45M.

Path probable: estratégico (banco con mesa agro, exchange, large fintech) o IPO post-Series B si EU markets se reabren.

## 19. ¿Cuánto del round va al founder?

**Cero.** Founder cobra salario standard (~USD 70k/año en pre-seed UE) y vive de su track record / equity. Inversor verifica esto en el use-of-funds.

## 20. ¿Hay deal-breakers?

Sí. **No** podemos ofrecer:
- Yield garantizado (securities fraud).
- Ofertas a US persons sin Reg D filing.
- Tokenización de instrumentos que no podamos custodial-verify off-chain.

Si un inversor pide alguno de estos, pasamos.

---

**¿Hay una pregunta que NO está acá y que tu inversor te hizo? Mandala a `carlos@agroglobaldex.io` y la agrego a la próxima versión.**
