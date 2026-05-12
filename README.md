# AgroGlobalDex

> **Prototipo de marketplace para tokenización de producción agropecuaria europea sobre Solana.**
> Conectar productores españoles (cooperativas agroganaderas, agricultores) con inversores particulares mediante contratos vinculantes (SPV propietaria del activo) y representación on-chain en Solana.

⚠️ **Estado actual: Demo / Proof of Concept.** Este repositorio contiene un prototipo en fase de diseño y desarrollo. **No es una plataforma operativa**, **no constituye oferta de valores ni servicios sobre criptoactivos**, y **no está autorizada como CASP MiCA** ni inscrita en CNMV / Banco de España. Las cifras y operaciones que aparecen en el frontend son **ilustrativas** (mock data).

---

## Por qué este proyecto

El productor agropecuario español **cobra a 90–180 días** de la gran distribución, paga hasta **18 % en adelantos de factoring**, y a menudo no accede a circulante sin aval personal. En paralelo, el inversor minorista europeo no tiene acceso fácil a producción agrícola real.

AgroGlobalDex pretende **cerrar ese hueco** con:
- **SPV por activo** (sociedad española propietaria de la producción tokenizada — esto da exigibilidad jurídica al token).
- **Representación on-chain** en Solana (Token-2022 con Transfer Hook para KYC, oráculos Switchboard para precio de commodities verificables).
- **Cumplimiento regulatorio cumplidor** vía Folleto simplificado Art. 1.4 Reg. UE 2017/1129 o licencia CASP MiCA.

## Qué hay en este repo

| Carpeta / archivo | Descripción |
|---|---|
| [`web 2.0/`](web%202.0/) | **Aplicación web principal.** 5 páginas (Inicio, Nosotros, Marketplace, Equipo, Contacto). HTML5 + CSS3 + JavaScript vanilla. Sin frameworks. |
| [`web agrochaindex/`](web%20agrochaindex/) | Versión alternativa de la web (misma idea, espejo). |
| [`agrochain-electron/`](agrochain-electron/) | Wrapper Electron que empaqueta el sitio como `.exe` portable para Windows. |
| [`AgroChain_App_Final.html`](AgroChain_App_Final.html) | Prototipo single-file inicial. |
| `AgroChain-Logo.*` | Logo del producto en SVG, PNG y ICO. |

> **Nota de naming.** El repositorio mantiene el prefijo histórico `AgroChain` en algunos archivos del PoC. La marca oficial del producto es **AgroGlobalDex**.

## Descarga la app de escritorio

👉 **[AgroGlobalDex.exe — Releases](../../releases/latest)** — Doble clic, sin instalación. Windows 10/11 x64.

> Windows mostrará un aviso de SmartScreen ("Editor desconocido") la primera vez porque el `.exe` no está firmado digitalmente. Pulsa **Más información → Ejecutar de todos modos**. Esto es normal en software no comercial.

## Cómo ejecutarlo localmente

### Opción A — Doble clic al `.exe` (recomendado)
Descarga desde Releases.

### Opción B — Navegador
Abre cualquier HTML del directorio `web 2.0/` directamente. Sin dependencias.

### Opción C — Compilar el `.exe` tú mismo
Requiere Node.js 20+.
```bash
cd agrochain-electron
npm install
npm run build
# El .exe queda en agrochain-electron/dist/
```

## Stack técnico actual y previsto

**Hoy (PoC, este repo):**
- HTML5 + CSS3 + JavaScript vanilla
- Leaflet (mapas)
- Electron 33 + electron-builder 25 (empaquetado a `.exe`)
- Iconos SVG → ICO multi-resolución vía `sharp` + `png-to-ico`

**Roadmap técnico (para llegar a MVP regulado):**
- **Anchor 0.30+ / Rust** para los programas Solana
- **Token-2022** con `TransferHook` (enforcement KYC), `PermanentDelegate` (acción judicial), `MetadataPointer` (certificado del productor)
- **Metaplex Bubblegum (cNFTs)** para representar lotes/unidades físicas
- **Switchboard On-Demand** como oráculo de precios (feeds custom con 3 fuentes validadas)
- **Streamflow** para distribución periódica de yield (lácteo) + program Anchor custom para liquidaciones de cosecha
- **Meteora DLMM** para mercado secundario
- **Civic Pass / Sumsub** para KYC/AML6
- **Fireblocks o Anchorage** custodia institucional
- **Helius** RPC + indexing (DAS API + Webhooks)
- **Arweave (Irys)** para storage permanente de certificados de proveniencia
- **Privy/Dynamic** para wallet onboarding con embedded wallet (email/social login)

## Roadmap regulatorio

| Fase | Hito | Plazo estimado |
|---|---|---|
| 0 | Limpieza copy + constitución HoldCo S.L. | T+1 mes |
| 1 | Decisión clasificación token con abogado colegiado · OpCo + 1ª SPV · alta SEPBLAC | T+2 meses |
| 2 | Contrato con primer productor · auditor físico · KYC integrado (Sumsub) | T+3 meses |
| 3 | Consulta previa CNMV · borrador Folleto Art. 1.4 / Whitepaper MiCA | T+4 meses |
| 4 | Custodia técnica · auditoría smart contracts (OtterSec o Halborn) | T+5 meses |
| 5 | Piloto privado con inversores acreditados (Art. 39 RD 1310/2005) | T+6 meses |
| 6+ | Apertura a minoristas en España | T+9–12 meses |

## Captación de capital

Vías de financiación previstas para llegar a MVP:
- **Solana Foundation Grant** — $50–75K en 3 milestones técnicos (devnet → oracle → mainnet beta).
- **Hyperdrive (Colosseum)** — aceleradora ex-Solana Foundation; $25K preseed + demo day.
- **CDTI Neotec** — subvención hasta €325K a fondo perdido (70 % del proyecto). Convocatoria anual.
- **ENISA Jóvenes Emprendedores** — préstamo participativo €25–300K sin garantías personales.
- **Ecosystem funds Solana**: RockawayX (EU), Helius Ventures, Hashed, Big Brain Holdings.
- **Angels cripto-ES**: una vez con devnet funcional + grant aprobado.

Runway mínimo estimado hasta MVP en mainnet con primer asset real: **€220–280K** (ingeniería + auditoría + infra + legal técnico).

## Marca visual

- Verde neón principal: `#00FF6A`
- Fondo oscuro: `#05080A`
- Tipografía display: DM Serif Display + Outfit
- Logo: hexágono blockchain + semilla en el centro (SVG/PNG/ICO incluidos en el repo)

## Aviso legal

Este repositorio contiene un **prototipo informativo**. Su lectura y uso no genera relación contractual ni de inversión con ninguna entidad. AgroGlobalDex **no está autorizado** actualmente como prestador de servicios sobre criptoactivos (CASP) ni para la oferta pública de valores. **Para cualquier paso operativo se exigirá previamente la estructura legal, regulatoria y de cumplimiento descrita en el roadmap.**

## Licencia

MIT — el código fuente del prototipo se publica con fines de transparencia y revisión técnica para hackathon.
