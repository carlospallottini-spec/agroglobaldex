# 09 — Risk Disclosures

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Estos disclosures deben revisarse por abogado matriculado UE y por el equipo de compliance antes de mostrarse a usuarios. Los textos deben adaptarse al idioma oficial del Estado miembro del Usuario.
>
> Base normativa: MiCA arts. 6, 7, 13, 14; Directiva 2014/65/UE (MiFID II) sobre risk warnings cuando aplique; Directiva 2005/29/CE sobre prácticas comerciales no engañosas; guía ESMA/EBA sobre marketing communications de criptoactivos.

---

## 1. Cuándo mostrar los disclosures

| Momento | Contenido |
|---|---|
| **Onboarding / Registro** | Advertencia general + cuestionario de apropiación (suitability básica) |
| **Antes de la primera operación** | Disclosure completo, requiere aceptación explícita (tick-box doble) |
| **Antes de cada operación con un nuevo tipo de token** | Disclosure específico del activo (resumen del white paper + riesgos del subyacente) |
| **En toda comunicación promocional** | Advertencia obligatoria art. 13 MiCA (ver §3) |
| **Periódicamente** (cada 12 meses) | Re-confirmación de comprensión de riesgos |

---

## 2. Advertencia general obligatoria (MiCA art. 13 / Anexo I)

> ⚠ **ADVERTENCIA SOBRE CRIPTOACTIVOS**
>
> Los criptoactivos pueden perder su valor en parte o en su totalidad, pueden no ser siempre transferibles y pueden no ser líquidos. Los criptoactivos no están cubiertos por sistemas de garantía de depósitos ni por sistemas de indemnización a inversores. La inversión en criptoactivos no es adecuada para todos los inversores.

**Esta advertencia debe**:
- Mostrarse en un lugar visible y destacado, **antes** de cualquier información promocional.
- Ser de tamaño tipográfico **no inferior** al texto promocional.
- Aparecer en **idioma oficial** del Estado miembro del usuario.
- Ser **inalterable** (sin disclaimers contradictorios que la suavicen).

---

## 3. Disclosure detallado de riesgos

### 3.1. Riesgo de pérdida total

> Usted puede **perder todo el capital invertido**. El precio de los criptoactivos es altamente volátil y puede caer rápidamente a cero. No existe garantía de recuperación. Solo debe invertir cantidades cuya pérdida total pueda asumir sin afectar a su situación financiera personal.

### 3.2. Riesgo de volatilidad

> El precio de los criptoactivos puede variar **drásticamente** en cortos periodos de tiempo. Movimientos de **±50% diarios** han ocurrido históricamente en este tipo de activos. La volatilidad puede ser amplificada por baja liquidez, eventos macroeconómicos, decisiones regulatorias, ciberataques, declaraciones públicas o rumores.

### 3.3. Riesgo de liquidez

> Es posible que no encuentre comprador o vendedor para su criptoactivo al precio deseado, o que no encuentre contraparte en absoluto. AgroGlobalDex **no garantiza la existencia de un mercado secundario** para ningún token. La liquidez puede **desaparecer súbitamente**.

### 3.4. Riesgo regulatorio

> El marco regulatorio aplicable a los criptoactivos evoluciona rápidamente. Cambios en MiCA, AMLR, regímenes fiscales, prohibiciones nacionales o sanciones pueden afectar a la legalidad, valor, transferibilidad o utilidad de los tokens. Algunos tokens pueden ser **recalificados** como instrumentos financieros o **retirados** del mercado.

### 3.5. Riesgo tecnológico — Smart contracts y blockchain

> Los criptoactivos dependen de tecnología compleja (blockchain, smart contracts, oracles, bridges). Pueden existir **vulnerabilidades** no descubiertas que provoquen pérdida o congelación de fondos. Los smart contracts son **inmutables** salvo mecanismos de upgrade y, una vez ejecutada una transacción, **no es reversible**. Eventos como hard forks, ataques 51%, ataques de gobernanza, fallos de oracle, o explotación de vulnerabilidades han causado pérdidas masivas en la industria.

### 3.6. Riesgo de custodia

> Aunque AgroGlobalDex mantiene los activos en custodia segregada conforme a MiCA art. 70, **ningún sistema de custodia es 100% seguro**. Pueden ocurrir ciberataques, errores operativos, insolvencia de proveedores. Para volúmenes elevados, considere autocustodia (con los riesgos propios de pérdida de claves).

### 3.7. Riesgo del activo subyacente (específico RWA agro)

#### 3.7.1. Tokens de granos físicos
> El valor del token depende del precio del **commodity subyacente** (grano), del que existe riesgo de:
> - Variación de precio por factores climáticos, geopolíticos, oferta/demanda global.
> - Daño, deterioro o pérdida en el almacén.
> - Insolvencia del operador del almacén o de la contraparte de origen.
> - Riesgo de calidad: rechazo del grano por no cumplir estándares de entrega.
> - Riesgo de **double counting** o emisión por encima del subyacente realmente disponible (riesgo de fraude en certificación).

#### 3.7.2. Tokens de créditos de carbono
> Los créditos de carbono voluntarios:
> - **No están garantizados por ningún gobierno** ni por el sistema EU ETS (salvo que se trate de EUAs).
> - Pueden ser **invalidados** por el estándar de certificación (Verra, Gold Standard) si se detectan irregularidades en el proyecto.
> - Su valor depende de la demanda voluntaria, que es volátil y políticamente sensible.
> - Existe **riesgo reputacional**: medios y reguladores han cuestionado la integridad de proyectos de offset.
> - Puede haber **double counting** entre mercados voluntarios y compromisos nacionales (NDCs).

#### 3.7.3. Fracciones de cosechas futuras (si aplica)
> - **Riesgo de cosecha**: clima adverso, plagas, enfermedades, decisiones del productor.
> - **Riesgo del productor**: solvencia, cumplimiento contractual, fraude.
> - **Riesgo de mercado del commodity** en el momento de la liquidación.
> - **Riesgo legal**: estos tokens **podrían recalificarse como valores negociables** bajo MiFID II y quedar sujetos a régimen distinto.

### 3.8. Riesgo de contraparte

> Operaciones con otros usuarios o intermediarios pueden fallar si la contraparte incumple. AgroGlobalDex puede actuar como contraparte o como facilitador; en ningún caso garantiza el cumplimiento por parte de terceros.

### 3.9. Riesgo operativo y de servicio

> Los Servicios pueden experimentar **interrupciones, retrasos o fallos** por mantenimiento, fallos técnicos, ataques de denegación de servicio o decisiones de AgroGlobalDex. Durante tales interrupciones puede ser imposible operar o retirar activos.

### 3.10. Riesgo AML y de bloqueo de cuenta

> AgroGlobalDex está obligada a **bloquear, congelar o reportar** cuentas y fondos en cumplimiento de la normativa AML, sanciones internacionales y requerimientos de autoridades, **sin previo aviso al Usuario** cuando la ley lo exija. Esto puede impedir el acceso a los fondos durante tiempo prolongado.

### 3.11. Riesgo fiscal

> El régimen fiscal de los criptoactivos es **complejo y variable** entre jurisdicciones. El Usuario es el único responsable de cumplir sus obligaciones fiscales (declaración, retención, etc.). AgroGlobalDex puede estar obligada a reportar a autoridades fiscales (DAC8 y normativa nacional).

### 3.12. Ausencia de garantía de depósito o de inversor

> Los criptoactivos y los fondos asociados **no están cubiertos** por el Fondo de Garantía de Depósitos (Directiva 2014/49/UE) ni por sistemas de indemnización de inversores (Directiva 97/9/CE).

### 3.13. Riesgo de conflicto de interés

> AgroGlobalDex puede tener relaciones con emisores, operadores de almacenes, oráculos o validadores. Estos conflictos se gestionan conforme a MiCA art. 72 y políticas internas, pero no se eliminan completamente.

### 3.14. Riesgo de información y manipulación de mercado

> Los mercados de criptoactivos pueden ser objeto de **manipulación** (pump & dump, wash trading, insider trading). Aunque AgroGlobalDex monitoriza y reporta abusos conforme a MiCA Título VI, el Usuario debe operar con prudencia.

### 3.15. Riesgo ESG / Greenwashing

> Específico para tokens de carbono: la **integridad ambiental** de los proyectos puede ser cuestionada. Reguladores (UE Green Claims Directive) y medios pueden invalidar afirmaciones de impacto. AgroGlobalDex usará estándares reconocidos y verificación independiente, pero no garantiza la perfección de los proyectos.

---

## 4. Cuestionario de apropiación (suitability) — propuesta básica

Antes de permitir operar, el Usuario debe completar un cuestionario que confirme:

1. **Comprensión** de los riesgos descritos (preguntas de comprensión, no solo aceptación).
2. **Tolerancia financiera**: ingresos, patrimonio, capacidad de pérdida.
3. **Experiencia previa** en crypto y/o commodities.
4. **Horizonte de inversión** y objetivos.

Si el resultado indica que el producto **no es apropiado** para el Usuario, debe mostrarse una advertencia adicional clara y requerir confirmación explícita para continuar. Para usuarios retail, **considerar limitar exposición** automáticamente.

---

## 5. Marketing communications — reglas (MiCA art. 7 y 13)

Toda comunicación promocional debe:
- Ser **identificable como promocional**.
- Ser **fair, clear, not misleading**.
- Ser **consistente** con la información del white paper.
- Incluir la **advertencia obligatoria** (§2 de este documento) de forma prominente.
- **No prometer rentabilidades** específicas ni minimizar riesgos.
- **No usar urgencia falsa** ("últimas plazas", "oferta limitada").
- **No comparar** ventajosamente vs. instrumentos financieros tradicionales sin matización.
- Conservar copia y trazabilidad de toda comunicación promocional emitida.
- Reportar a la NCA si así lo exige (algunas NCAs requieren registro previo o notificación).

---

## 6. Idiomas

Mostrar disclosures en el idioma oficial del Estado miembro de residencia del Usuario, o como mínimo en **inglés + idioma del EM sede**. Para Usuarios consumidores, el derecho UE impone idioma comprensible.

---

*Fin del documento 09.*
