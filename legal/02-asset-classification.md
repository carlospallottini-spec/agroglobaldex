# 02 — Clasificación regulatoria de los activos de AgroGlobalDex

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. La calificación final de cada token depende del diseño concreto del producto y debe ser confirmada por abogado matriculado UE.

---

## 1. Marco analítico: el "test de equivalencia funcional"

Bajo el derecho UE, la calificación regulatoria de un token **no depende de la etiqueta** que le ponga el emisor, sino de su **función económica** y **derechos asociados** (principio de "substance over form", reflejado en MiFID II art. 4(1)(44) y MiCA cdo. 14).

Las preguntas críticas:

1. ¿El token confiere **derecho a participar en beneficios o rendimientos** de un emisor/proyecto? → potencial **valor negociable** (MiFID II).
2. ¿Es **fungible y transferible** en mercado secundario? → potencial valor negociable.
3. ¿Representa un **activo subyacente físico** redimible 1:1? → potencial **commodity warrant / utility token**.
4. ¿Referencia el valor de **otro activo o cesta** para mantener estabilidad? → **ART** bajo MiCA.
5. ¿Referencia **una sola moneda fiat**? → **EMT** bajo MiCA (solo EMI/banco).
6. ¿Es **único y no fungible**? → fuera de MiCA (con matices).

---

## 2. Tokens de granos físicos

### 2.1. Caso típico
Token ERC-equivalente que representa **N toneladas de grano** (trigo, maíz, soja) depositadas en un warehouse certificado, con derecho del titular a **redención física o equivalente monetario**.

### 2.2. Análisis MiCA
- **No es ART**: no referencia una cesta para mantener valor estable, sino que **es** una representación 1:1 de un commodity físico específico.
- **No es EMT**: no referencia moneda fiat.
- **No es valor negociable bajo MiFID II** *en principio*: un warrant de commodity no es típicamente un instrumento financiero, sino un **título de propiedad sobre la mercancía** (similar a un warehouse receipt).
- **Probable categoría**: **OCA — utility token** bajo MiCA, con régimen art. 4–15 (white paper notificado, marketing communications, retracto).

### 2.3. Riesgos de recalificación
- **Si el token se usa como medio de pago o reserva de valor** en el marketplace → riesgo de ser tratado como ART.
- **Si hay un emisor que garantiza un precio fijo** o un mecanismo de estabilización → riesgo ART.
- **Si subyacente es derivado de commodity** (futuro, forward), no commodity físico → MiFID II (derivado).
- **Bajo MiFID II art. 4(1)(44) y Anexo I sección C(10)** los **derivados sobre commodities** son instrumentos financieros. Un "token de grano" estructurado como derivado entra en MiFID II.

### 2.4. Recomendación de diseño
- **Estructurar como warehouse receipt tokenizado**: representación digital de un título de depósito de la mercancía en almacén certificado (p. ej. bajo régimen de los **warrants** del derecho mercantil de cada Estado miembro).
- **Redención física garantizada**, sin promesa de retorno financiero.
- **Sin pool ni mecanismo de estabilización de precio**.
- Esto lo mantiene en territorio **OCA / fuera de MiFID II** con alta probabilidad. [VERIFICAR con abogado UE].

---

## 3. Tokens de créditos de carbono

### 3.1. Dos universos paralelos

**3.1.a. EU ETS — mercado regulado**
- Los **EU Allowances (EUA)** del **EU Emissions Trading System** (Directiva 2003/87/CE) son, desde MiFID II (Anexo I sección C(11)), **instrumentos financieros**.
- Tokenizarlos sin perder la calidad de EUA es **complejo**: requiere mantener trazabilidad en el Registro de la Unión.
- **Implicación**: si AgroGlobalDex toca EUAs tokenizados → **MiFID II en pleno**: investment firm autorización, prospecto, etc. **Recomendación: NO entrar en este mercado en fase PoC**.

**3.1.b. Mercados voluntarios de carbono**
- Créditos emitidos por estándares privados: **Verra (VCS), Gold Standard, Plan Vivo, Climate Action Reserve, ACR**.
- **No son instrumentos financieros per se** (no cubiertos por MiFID II Anexo I C(11)).
- **Probable categoría MiCA**: **OCA** si se tokenizan, salvo que el diseño los convierta en ART/EMT.

### 3.2. Marco adicional aplicable
- **Reglamento UE 2023/1115** (deforestación / EUDR): si el carbono se vincula a proyectos forestales, hay obligaciones de diligencia debida sobre la cadena de suministro.
- **Reglamento UE 2024/3012** (marco voluntario de certificación de absorciones de carbono / Carbon Removals Certification Framework) — pendiente de implementación, marcará el estándar UE para créditos de absorción. [VERIFICAR estado de aplicación].
- **CSRD (Directiva UE 2022/2464)**: reporting ESG corporativo — relevante si los compradores son grandes empresas que reportan bajo CSRD/ESRS.
- **Greenwashing**: Directiva UE 2024/825 (Empowering Consumers for the Green Transition) y propuesta de Green Claims Directive — los claims ambientales del marketplace deben ser **verificables, sustanciados y no engañosos**.

### 3.3. Recomendación de diseño
- **Comenzar solo con créditos voluntarios** de estándares reconocidos (Verra/Gold Standard).
- **Token = representación 1:1 de un crédito retirado del registro oficial y custodiado en cuenta dedicada del marketplace**, con mecanismo de retiro on-chain auditable.
- **No prometer rendimientos financieros** sobre el token (eso lo convertiría en security).
- **White paper OCA** (MiCA art. 6) + claims ambientales verificados por tercero independiente.

---

## 4. Fracciones de cosechas futuras

### 4.1. Naturaleza económica
Un token que da derecho al titular a recibir un **porcentaje de la producción/ingresos de una cosecha futura** de un productor identificado.

### 4.2. Análisis bajo MiFID II y Prospectus Regulation

- **MiFID II art. 4(1)(44)** define "valor negociable" (transferable securities): valores negociables en mercado de capitales con exclusión de instrumentos de pago, entre los que se incluyen, en particular:
  - acciones,
  - bonos,
  - **cualquier otro valor que dé derecho a adquirir o vender tales valores o que dé lugar a una liquidación en efectivo determinada por referencia a valores, divisas, tipos de interés o rendimientos, materias primas u otros índices o medidas**.

- Las **fracciones de cosechas futuras** encajan en esta categoría porque:
  1. Confieren **derechos económicos** (porción de ingresos / producción).
  2. Son **fungibles y agrupables** (token estandarizado).
  3. Son **transferibles** en mercado secundario.
  4. La liquidación se determina **por referencia a un commodity / índice** (precio del grano).

- **Conclusión preliminar**: las fracciones de cosechas son, con alta probabilidad, **valores negociables bajo MiFID II** → salen del ámbito de MiCA (MiCA art. 2(4)(a)) y entran en:
  - **Prospectus Regulation (UE) 2017/1129**: requiere prospecto aprobado por la NCA antes de oferta pública (con excepciones por umbral).
  - **MiFID II**: la plataforma de negociación necesita ser **investment firm autorizada** (no basta con CASP).
  - **CSDR (Reg. UE 909/2014)**: si se emite y registra como security tokenizado, posible aplicación.
  - **DLT Pilot Regime (Reg. UE 2022/858)**: régimen especial para mercados secundarios de securities tokenizados — opt-in, con sandbox.

### 4.3. Implicaciones operativas

| Requisito | Coste / plazo |
|---|---|
| Prospecto Prospectus Regulation aprobado por NCA | 3–9 meses, €50k–€500k coste legal/auditoría [VERIFICAR] |
| Investment firm authorisation MiFID II | 12–18 meses, capital inicial mínimo €750k para multilateral trading facility (MTF) [VERIFICAR Reg. (UE) 2019/2033 IFR/IFD] |
| Doble licencia CASP + investment firm | Posible mediante art. 60 MiCA (notificación simplificada para ESI) |

### 4.4. Excepciones al prospecto (Prospectus Regulation art. 1(4) y art. 3(2))
- Oferta solo a **inversores cualificados**.
- Oferta a **<150 personas no cualificadas por Estado miembro**.
- Denominación unitaria mínima **≥€100.000** por inversor.
- Contraprestación total **<€1.000.000 en 12 meses** (no requiere prospecto pero sí cumplir derecho nacional). [VERIFICAR umbral, los Estados miembros pueden elevarlo hasta €8.000.000].

### 4.5. Recomendación estratégica

Tres opciones:

**Opción A — Evitar MiFID II (recomendada para PoC)**
- Rediseñar las "fracciones de cosechas" como **contratos de pre-compra de commodity físico** (forward físico bilateral), no tokens fungibles con derecho a flujo de efectivo.
- O bien: **tokens de utility** que dan derecho a **N kg de grano físico** de una cosecha futura, sin componente financiero.
- **Resultado probable**: salen de MiFID II, vuelven a OCA bajo MiCA.

**Opción B — Asumir MiFID II y operar bajo prospecto + investment firm**
- Coste, plazo y complejidad significativos.
- Solo viable post-PoC con financiación Serie A+.

**Opción C — Restringir a inversores cualificados / minimum ticket €100k**
- Evita el prospecto pero sigue requiriendo investment firm para operar el mercado secundario.
- Limita drásticamente el mercado retail.

> **Recomendación del agente**: **Opción A** para la versión PoC/MVP. Reevaluar B/C cuando haya recursos para asumir el coste regulatorio. [VERIFICAR con abogado UE].

---

## 5. Tabla resumen

| Activo | Régimen probable | Norma principal | Esfuerzo regulatorio |
|---|---|---|---|
| Tokens granos (warehouse receipt) | OCA — utility | MiCA Título II | Bajo–Medio (white paper notificado + CASP) |
| Tokens carbono voluntario | OCA — utility | MiCA Título II + EUDR + Green Claims | Medio (verificación ESG + white paper) |
| Tokens EU Allowances (ETS) | Instrumento financiero | MiFID II + ETS Directive | **Muy alto — evitar en PoC** |
| Fracciones cosechas (revenue share) | Valor negociable | MiFID II + Prospectus Reg. | **Muy alto — rediseñar a utility** |

---

*Fin del documento 02.*
