# 01 — Análisis MiCA aplicado a AgroGlobalDex

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Debe ser validado por abogado matriculado UE antes de cualquier uso. Las referencias a artículos son orientativas y pueden requerir actualización.

---

## 1. Qué es MiCA

El **Reglamento (UE) 2023/1114 del Parlamento Europeo y del Consejo, de 31 de mayo de 2023, relativo a los mercados de criptoactivos** (MiCA) establece un marco armonizado para la emisión, oferta al público, admisión a negociación y prestación de servicios sobre criptoactivos en la Unión Europea.

- **Entrada en vigor escalonada**: disposiciones sobre ART/EMT desde el **30 de junio de 2024**; régimen de CASPs (Crypto-Asset Service Providers) plenamente aplicable desde el **30 de diciembre de 2024**.
- **Pasaporte europeo**: una autorización CASP otorgada por una NCA permite operar en toda la UE bajo el principio de **single licence + passporting** (MiCA art. 65).

## 2. Qué regula MiCA

- **Emisión y oferta pública** de criptoactivos (Título II, III, IV).
- **Prestación de servicios** sobre criptoactivos por parte de CASPs (Título V).
- **Abuso de mercado** en mercados de criptoactivos (Título VI).
- **Obligaciones de transparencia, gobernanza y prudenciales** del emisor y CASP.

## 3. Qué NO regula MiCA

- **NFTs únicos y no fungibles** (cdo. 10–11; con matices: un "NFT" emitido en serie grande puede ser recalificado).
- **Instrumentos financieros** ya cubiertos por MiFID II (acciones, bonos, derivados, valores transferibles tokenizados) — siguen su régimen propio (Prospectus Regulation, MiFID II, CSDR).
- **Depósitos**, fondos del sistema financiero tradicional, productos de seguros, pensiones.
- **CBDCs** (monedas digitales de bancos centrales).
- **Criptoactivos puramente internos** a una entidad cerrada (closed-loop).

> **Crítico para AgroGlobalDex**: si un token representa **derechos económicos transferibles** sobre cosechas futuras (income/profit share), es probable que sea **valor negociable bajo MiFID II**, no criptoactivo bajo MiCA. Ver `02-asset-classification.md`.

---

## 4. Las tres categorías MiCA de criptoactivos

MiCA art. 3(1) define las categorías:

### 4.1. Asset-Referenced Tokens (ART) — art. 3(1)(6)
Tokens que **referencian el valor de otro activo, derechos o cesta** (commodity, otra divisa, otros criptoactivos) para mantener valor estable.
- Régimen estricto (Título III, art. 16–47): autorización previa, white paper aprobado por NCA, requisitos de capital, reservas, gobernanza.

### 4.2. Electronic Money Tokens (EMT) — art. 3(1)(7)
Tokens que mantienen valor estable **referenciando una única moneda fiat oficial**. Solo pueden emitirlos entidades de crédito o de dinero electrónico (EMI).
- Régimen Título IV (art. 48–58).

### 4.3. Other crypto-assets (OCA)
Categoría residual: cualquier criptoactivo que **no** sea ART ni EMT y no sea instrumento financiero.
- Régimen Título II (art. 4–15): obligación de **white paper notificado** (no aprobado) a la NCA, marketing communications, derecho de retracto.

---

## 5. Dónde caen los activos de AgroGlobalDex

Análisis preliminar (ampliado en `02-asset-classification.md`):

| Activo | Categoría más probable | Razonamiento |
|---|---|---|
| **Tokens de granos físicos en warehouse** (entrega física, redimible 1:1) | OCA (utility) o fuera de MiCA (commodity warrant) | Referencia a commodity → riesgo de recalificación como ART si actúa como reserva de valor estable. |
| **Tokens de créditos de carbono** (voluntarios) | OCA con probable régimen especial ESG | Depende si es token-representación de un EUA (EU ETS) o de crédito voluntario (Verra/Gold Standard). EUA = instrumento financiero. |
| **Fracciones de cosechas futuras** (revenue/profit share) | **MiFID II — valor negociable** | Derecho económico futuro transferible y agrupable → probable security. Sale de MiCA. |

> **Implicación práctica**: AgroGlobalDex probablemente requiere **doble régimen**: MiCA (CASP + OCA white paper) para granos y carbono; **MiFID II + Prospectus Regulation** para fracciones de cosechas. [VERIFICAR con abogado UE].

---

## 6. Régimen CASP — el marketplace probablemente es CASP

MiCA art. 3(1)(15) define **crypto-asset service provider** como toda persona jurídica que preste, profesionalmente y a terceros, uno o más de los **servicios sobre criptoactivos** del art. 3(1)(16).

Servicios relevantes para AgroGlobalDex (art. 3(1)(16)):

| Letra | Servicio | Aplica a AgroGlobalDex? |
|---|---|---|
| (a) | Custodia y administración de criptoactivos por cuenta de clientes | **Sí** — wallets de usuarios |
| (b) | Operación de una plataforma de negociación de criptoactivos | **Sí** — marketplace / DEX |
| (c) | Canje de criptoactivos por fondos | Probable |
| (d) | Canje de criptoactivos por otros criptoactivos | Probable |
| (e) | Ejecución de órdenes por cuenta de clientes | Probable |
| (f) | Colocación de criptoactivos | Si emite primario |
| (g) | Recepción y transmisión de órdenes | Probable |
| (h) | Asesoramiento sobre criptoactivos | No previsto |
| (i) | Gestión de cartera | No previsto |
| (j) | Servicios de transferencia | Probable |

**Conclusión**: AgroGlobalDex requerirá autorización CASP cubriendo, como mínimo, los servicios (a), (b) y según diseño (c)–(g).

---

## 7. Artículos clave aplicables

### Para oferta de OCA (granos, carbono)
- **Art. 4** — Oferta pública de criptoactivos: requisitos generales.
- **Art. 5** — Excepciones a la oferta pública (oferta a <150 personas por Estado miembro, contraprestación total <€1.000.000 en 12 meses, solo inversores cualificados, etc.). [VERIFICAR umbrales en versión vigente].
- **Art. 6** — Contenido del white paper (ver `08-white-paper-template.md`).
- **Art. 7** — Marketing communications.
- **Art. 8** — Notificación a la NCA (no aprobación previa para OCA).
- **Art. 9** — Publicación del white paper.
- **Art. 12** — Modificación del white paper.
- **Art. 13** — Marketing communications: requisitos y advertencias.
- **Art. 14** — Derecho de retracto del consumidor retail (14 días, [VERIFICAR plazo exacto]).
- **Art. 15** — Responsabilidad del emisor por información en el white paper.

### Para autorización CASP
- **Art. 59** — Requisito de autorización.
- **Art. 60** — Régimen específico para entidades ya autorizadas (entidades de crédito, ESI MiFID, EMI) que pueden prestar servicios mediante **notificación simplificada** en vez de autorización plena. **Relevante**: si nos asociamos con una ESI/EMI, podemos acelerar.
- **Art. 62** — Contenido de la solicitud de autorización.
- **Art. 63** — Evaluación por la NCA (plazo objetivo: 25 días hábiles para completitud + 40 días hábiles para decisión, total ~3 meses; en la práctica 6–12 meses) [VERIFICAR].
- **Art. 65** — Pasaporte UE (notificación a otras NCAs).
- **Art. 67** — Capital mínimo (Anexo IV): clase 1 (€50.000), clase 2 (€125.000), clase 3 (€150.000) según servicios prestados. [VERIFICAR cifras en Anexo IV vigente].
- **Art. 68** — Gobernanza interna.
- **Art. 70** — Requisitos prudenciales.
- **Art. 71** — Información al cliente.
- **Art. 72** — Mejor ejecución.
- **Art. 73** — Custodia: segregación de fondos y criptoactivos de clientes.
- **Art. 74** — Quejas y resolución de disputas.

### Gobernanza y conducta
- **Art. 75–83** — Obligaciones de gobernanza, externalización, abuso de mercado, registros.

---

## 8. Régimen transitorio

**MiCA art. 143** establece régimen transitorio para entidades que **ya prestaban servicios sobre criptoactivos antes del 30 de diciembre de 2024** bajo regímenes nacionales (p. ej. PSAN francés, registro VASP español, BaFin alemán):

- Pueden continuar operando hasta el **1 de julio de 2026** mientras tramitan autorización MiCA, [VERIFICAR fecha en la transposición de cada Estado miembro] (algunos lo han acortado: Francia hasta 1 julio 2026, Alemania hasta 31 diciembre 2025, etc.).
- **AgroGlobalDex no califica para este régimen transitorio** si lanza después de diciembre 2024, salvo que adquiera una entidad pre-existente.

## 9. Implicaciones prácticas para AgroGlobalDex

1. **Necesita autorización CASP** plena bajo MiCA antes de operar productivamente en la UE.
2. **Probable doble régimen** MiCA + MiFID II por las fracciones de cosechas → considerar **reestructurar el producto** para evitar la calificación de security (ej. token de utility con redención física, no profit share).
3. **White paper notificado** a la NCA para granos y carbono (OCA) antes de oferta pública.
4. **Régimen de marketing communications** muy estricto (MiCA art. 7, 13): toda comunicación promocional debe estar identificada como tal, ser fair/clear/not misleading, y consistente con el white paper.
5. **Capital mínimo y gobernanza**: el founder debe constituir entidad jurídica UE con capital, directivos fit & proper, sistemas de control interno.
6. **AML/CFT**: la entidad CASP es **obliged entity** bajo AMLD6 / AMLR — ver `04-kyc-aml-framework.md`.

---

## 10. Próximos pasos sugeridos

1. **Decision producto**: ¿se elimina o reestructura las fracciones de cosechas para evitar MiFID II?
2. **Elegir Estado miembro** sede (ver `05-jurisdictional-strategy.md`).
3. **Constituir entidad** UE con capital social suficiente.
4. **Contratar abogado especializado MiCA** local.
5. **Iniciar borrador de solicitud CASP**: ~6 meses de preparación + ~6 meses de tramitación.
6. **Diseñar white paper** para los OCAs (granos, carbono).

---

*Fin del documento 01.*
