# AgroGlobalDex — Hoja de ruta de lanzamiento

> Estado a 2026-06-13. Documento vivo. Lo que importa: **qué falta, en qué orden,
> y por qué los videos van primero.**

## 1. Estado actual (qué SÍ está hecho)

| Capa | Estado | Evidencia |
|------|--------|-----------|
| Contrato Solana/Anchor | ✅ Funcional en devnet | 43 tests verdes en CI (anchor-test) |
| Seguridad (hardening interno) | ✅ LP shares, liquidación justa, oráculo Pyth no forjable, KYC hook | PR #11, SECURITY.md |
| CI/CD | ✅ Verde | anchor-test + clippy + fmt + cargo-audit + IDL-drift |
| Frontend web (PWA) | ✅ Desplegado, diseño inmersivo | agroglobaldex.com (Cloudflare) |
| App móvil | ✅ Wrapper Capacitor listo | mobile/ (falta `cap add android/ios` + prueba en device) |
| Deploy automático | ✅ main → Cloudflare | PR merge = deploy |

## 2. Qué FALTA para un lanzamiento real (con dinero real)

Estos son **bloqueantes duros**, no opcionales, antes de mover un euro real:

1. **Auditoría externa del contrato.** El hardening es interno (yo + tú). Mainnet
   con fondos exige una auditoría de un tercero (OtterSec, Neodyme, Sec3…).
   Coste típico: 15–60k USD, 2–6 semanas. **Sin esto no se toca mainnet.**
2. **Vía legal MiCA / CASP.** El banner DEMO es *legalmente obligatorio* hoy
   ("No autorizado como CASP MiCA"). Tokenizar RWA + dar crédito = actividad
   regulada (criptoactivos + posible folleto/EMI). Requiere abogado
   especializado y, probablemente, licencia o sandbox regulatorio.
3. **Limitación conocida del contrato:** acreditación bypasseable por
   transferencia P2P (el hook valida KYC+jurisdicción pero no `accredited`).
   Fix antes de mainnet (pasar `asset_class` al hook o PermanentDelegate).
4. **Deploy mainnet con ceremonia de llaves** (script listo:
   `deploy-mainnet.sh` → upgrade authority a multisig Squads).
5. **Liquidez y oráculo de producción** (precio de colateral por oráculo +
   multisig, no `authority` manual).

**Conclusión:** el lanzamiento de producto real está a **meses y a capital
(auditoría + legal)** de distancia. Eso NO es un fracaso — es la realidad de
cualquier RWA/DeFi serio.

## 3. Por eso: VIDEOS PRIMERO ✅

El "lanzamiento" que SÍ puedes hacer hoy, sin auditoría ni licencia, es el
**lanzamiento de la narrativa**: demo en video del producto funcionando en
devnet para captar inversores, partners y la comunidad. Es lo que destraba el
capital que paga la auditoría y el legal. Secuencia correcta:

```
HOY ──► Video demo + pitch  ──► Levantar capital/partners ──► Auditoría + Legal ──► Mainnet
       (narrativa, gratis)      (con la demo en mano)        (meses, capital)     (producto real)
```

## 4. Guion de video demo (90 segundos, devnet)

1. **0–10s — El problema.** "El productor no puede vender ni acceder a crédito.
   280B€ de mercado agro, cero liquidez." (hero de index.html).
2. **10–30s — Tokenizar.** Conectar wallet (Phantom devnet) → tokenizar una
   cosecha en `tokenize.html`. Mostrar la tx en Solscan.
3. **30–55s — Crédito instantáneo.** En `borrow.html`: depositar el token como
   colateral → recibir USDC al instante. Mostrar el cálculo LTV en vivo.
4. **55–75s — Marketplace + inversión.** Comprar una fracción en
   `marketplace.html`/`invest.html`. KYC gate visible.
5. **75–90s — Cierre.** "On-chain, en segundos, sobre Solana. Esto ya funciona
   en devnet hoy." Logo + Q4 2026.

**Tip:** graba con el banner DEMO visible (es honesto y legal), y muestra
Solscan en cada tx — la prueba on-chain es tu mayor diferenciador.

## 5. Pulido pre-grabación (1–2 días, opcional pero recomendado)

- [ ] Fondear una wallet devnet con SOL + USDC de faucet para la demo.
- [ ] Verificar que el `PROGRAM_ID` desplegado en devnet coincide con el del cliente.
- [ ] Pre-cargar 2–3 assets de ejemplo en el marketplace para que no se vea vacío.
- [ ] Probar el flujo completo en móvil real (Capacitor + Mobile Wallet Adapter).
- [ ] (Higiene, NO bloqueante) unificar tokens de diseño en `css/tokens.css`.

## 6. Riesgos a comunicar con honestidad en el pitch

Decir "es un PoC en devnet, auditoría y licencia en curso" **suma** credibilidad
ante inversores serios. Prometer "ya operamos con dinero real" sin auditoría ni
CASP es el error que mata la ronda (y es ilegal). La transparencia es el activo.
