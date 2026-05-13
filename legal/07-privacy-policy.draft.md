# 07 — Política de Privacidad — BORRADOR

> **DISCLAIMER**: Borrador informativo generado por IA. No es asesoramiento legal. Debe revisarse por abogado matriculado UE / DPO antes de publicación.
>
> **Nota lingüística**: redactado en español. La versión legalmente vinculante para usuarios UE debe ser la **inglesa** y, si corresponde, traducida a la lengua oficial del Estado miembro de residencia del Usuario.

---

# Política de Privacidad de AgroGlobalDex

**Última actualización**: [FECHA]
**Versión**: 1.0 (borrador)

## 1. Responsable del tratamiento (Data Controller)

**[NOMBRE LEGAL DE LA SOCIEDAD]**
Domicilio: [DIRECCIÓN COMPLETA]
Número de registro mercantil: [Nº]
Email: privacy@[dominio]

**Delegado de Protección de Datos (DPO)**:
Nombre / Empresa: [DPO]
Email: dpo@[dominio]
Dirección: [DIRECCIÓN DPO]

## 2. Aplicabilidad

Esta Política describe cómo AgroGlobalDex trata los datos personales de Usuarios, visitantes del sitio web, candidatos a empleo y otras personas físicas, conforme al Reglamento (UE) 2016/679 (RGPD) y a la legislación nacional de protección de datos aplicable.

## 3. Categorías de datos personales tratados

### 3.1. Datos de identificación
Nombre, apellidos, fecha y lugar de nacimiento, nacionalidad, sexo, foto, copia de documento de identidad, firma.

### 3.2. Datos de contacto
Dirección postal, email, teléfono.

### 3.3. Datos de verificación (KYC)
Documento oficial (DNI / pasaporte / permiso de residencia), prueba de domicilio, datos biométricos (selfie / liveness para verificación facial), source of funds y source of wealth, datos de PEP, beneficial ownership.

### 3.4. Datos financieros y transaccionales
Cuenta bancaria, IBAN, historial de transacciones, balances, direcciones de wallet de criptoactivos, contrapartes.

### 3.5. Datos técnicos y de uso
Dirección IP, identificador de dispositivo, sistema operativo, navegador, geolocalización aproximada, logs de acceso, cookies, comportamiento de uso.

### 3.6. Datos de comunicación
Mensajes, tickets de soporte, grabaciones de videollamadas de verificación (cuando se realicen), correspondencia.

### 3.7. Datos sensibles (art. 9 RGPD)
**Biométricos** para verificación de identidad (con base en consentimiento explícito o necesidad de cumplimiento legal AML, [VERIFICAR base más adecuada con DPO]). No se tratan otras categorías sensibles salvo necesidad legal.

## 4. Finalidades del tratamiento y base legal

| Finalidad | Base legal (art. 6 RGPD) | Datos |
|---|---|---|
| Registro y gestión de cuenta | Ejecución de contrato (6.1.b) | Identificación, contacto |
| Verificación KYC / EDD | Obligación legal (6.1.c) — AMLR, MiCA | KYC, biométricos |
| Sanctions / PEP / Adverse Media screening | Obligación legal (6.1.c) | Identificación, listas |
| Travel Rule | Obligación legal (6.1.c) — Reg. UE 2023/1113 | Identidad, dirección wallet |
| Prestación de servicios (trading, custodia) | Ejecución de contrato (6.1.b) | Transaccionales, técnicos |
| Reporte fiscal | Obligación legal (6.1.c) — DAC8 y normativa nacional | Transaccionales |
| Detección y prevención de fraude | Interés legítimo (6.1.f) | Transaccionales, técnicos, de uso |
| Marketing y comunicaciones comerciales | Consentimiento (6.1.a) | Contacto, uso |
| Mejora del producto y analítica | Interés legítimo (6.1.f) | Técnicos, de uso (pseudonimizados) |
| Atención de derechos y consultas | Obligación legal (6.1.c) | Identificación, comunicación |
| Reporte de operaciones sospechosas (SAR) | Obligación legal (6.1.c) — AMLD/AMLR | Todos los necesarios |
| Datos biométricos para liveness | Consentimiento explícito (9.2.a) o necesidad para cumplir AML (9.2.g + base nacional) — [VERIFICAR con DPO] | Biométricos |

## 5. Origen de los datos

- Directamente del **Usuario** al registrarse, completar KYC y usar los Servicios.
- De **proveedores de verificación** (Sumsub, Onfido, etc.) y bases de datos públicas / privadas (listas de sanciones, PEP, adverse media, registros mercantiles, registros de beneficial owners).
- De **redes blockchain públicas** (información transaccional on-chain).
- De **socios comerciales** (warehouses, registros de carbono) cuando interactúan con cuentas del Usuario.

## 6. Destinatarios — con quién compartimos los datos

- **Proveedores de servicios** (encargados del tratamiento, art. 28 RGPD): KYC, AML monitoring, hosting, email, atención al cliente, analítica, almacenamiento en la nube.
- **Bancos** y entidades de pago para liquidaciones fiat.
- **Autoridades competentes**: NCA, FIU, autoridades fiscales, jueces, autoridades de protección de datos. Se incluyen, sin limitarse: AMF/ACPR, BaFin, DNB/AFM, CNMV, AEPD, CNIL, BfDI, etc.
- **Asesores profesionales** (abogados, auditores, consultores) bajo deber de confidencialidad.
- **Adquirente potencial** en caso de operación corporativa (M&A), bajo NDA.

## 7. Transferencias internacionales

7.1. Cuando datos personales se transfieran fuera del EEE, AgroGlobalDex utilizará al menos uno de los siguientes mecanismos:
- **Decisión de adecuación** de la Comisión Europea (art. 45 RGPD).
- **Cláusulas Contractuales Tipo (SCCs)** aprobadas por la Comisión Europea (Decisión (UE) 2021/914).
- **Normas Corporativas Vinculantes (BCRs)** del proveedor.
- **Excepciones del art. 49 RGPD** en casos específicos.

7.2. Para transferencias a EEUU, AgroGlobalDex evaluará la inclusión del receptor en el **EU-US Data Privacy Framework** y, en su caso, complementará con SCCs y medidas técnicas suplementarias (cifrado, pseudonimización).

7.3. Lista de destinos habituales y mecanismo aplicado: disponible en [Anexo Transferencias].

## 8. Plazos de conservación

| Categoría | Plazo |
|---|---|
| Datos KYC y registros AML | **5 años** desde el fin de la relación de negocio (mínimo legal); ampliable a 10 años si exigido por derecho nacional |
| Registros transaccionales | **5–10 años** según normativa AML y fiscal |
| Datos de cuenta de Usuario activo | Mientras dure la relación + plazos AML |
| Datos de marketing | Hasta retirada del consentimiento |
| Logs técnicos | **6–24 meses** según finalidad |
| Datos biométricos | Solo durante el tiempo necesario para la verificación, luego se conserva un hash o token irreversible cuando sea posible |
| Grabaciones de videollamada KYC | Tiempo mínimo necesario, máximo 5 años |
| Datos de candidatos a empleo no contratados | Máx. 12 meses con consentimiento, si no, eliminación |

## 9. Seguridad

AgroGlobalDex aplica medidas técnicas y organizativas apropiadas conforme al art. 32 RGPD y al Reg. (UE) 2022/2554 (DORA), incluyendo: cifrado en reposo y en tránsito, control de accesos por rol, MFA, monitorización, pruebas de penetración, plan de continuidad, plan de gestión de incidentes, formación del personal.

## 10. Derechos del titular de los datos

El Usuario puede ejercer los siguientes derechos (arts. 15–22 RGPD):
- **Acceso** a sus datos.
- **Rectificación** de datos inexactos.
- **Supresión** ("derecho al olvido") con las limitaciones legales (los datos KYC/AML no pueden suprimirse durante el plazo legal de conservación).
- **Limitación** del tratamiento.
- **Portabilidad** de los datos (cuando aplique).
- **Oposición** al tratamiento basado en interés legítimo.
- **No ser objeto de decisiones automatizadas** con efectos jurídicos (incluido perfilado) salvo excepciones (art. 22).
- **Retirar el consentimiento** en cualquier momento (sin afectar la licitud previa).

### 10.1. Cómo ejercerlos
Email: **privacy@[dominio]** o **dpo@[dominio]**.
Se requerirá verificación de identidad del solicitante.
Plazo de respuesta: **1 mes** (prorrogable 2 meses adicionales por complejidad, con aviso).

### 10.2. Autoridad de control
Si el Usuario considera vulnerados sus derechos, puede presentar reclamación ante la **autoridad de control competente**:
- **España**: AEPD — www.aepd.es
- **Francia**: CNIL — www.cnil.fr
- **Alemania**: BfDI / autoridad estatal — www.bfdi.bund.de
- **Países Bajos**: Autoriteit Persoonsgegevens — autoriteitpersoonsgegevens.nl
- **Irlanda**: DPC — www.dataprotection.ie
- O ante la autoridad del Estado miembro de residencia habitual o lugar de la presunta infracción.

## 11. Decisiones automatizadas y perfilado

AgroGlobalDex utiliza sistemas automatizados para:
- **Sanctions / PEP screening**: con revisión humana en caso de hit.
- **Detección de fraude y transaction monitoring**: las alertas son revisadas por analistas antes de decisiones que afecten al Usuario.
- **Risk scoring AML**: revisión humana antes de bloqueo o terminación.

El Usuario tiene derecho a obtener intervención humana, expresar su punto de vista e impugnar la decisión.

## 12. Cookies

AgroGlobalDex utiliza cookies y tecnologías similares conforme a la Directiva ePrivacy (2002/58/CE) y al RGPD. Las categorías y el procedimiento de consentimiento se detallan en la **Política de Cookies** ([URL]). Las cookies no esenciales requieren consentimiento previo y pueden retirarse en cualquier momento.

## 13. Menores

Los Servicios no están dirigidos a menores de 18 años. AgroGlobalDex no recolecta datos de menores conscientemente. Si se detecta, los datos se eliminan.

## 14. Datos on-chain

Las transacciones en blockchain son **públicas y, en general, irreversibles**. AgroGlobalDex no controla las copias del registro distribuido en nodos de terceros. El Usuario reconoce que, una vez registrada una transacción on-chain, el "derecho al olvido" puede ser técnicamente imposible respecto de los datos registrados públicamente (típicamente, direcciones de wallet y datos de transacción; datos de identificación off-chain del Usuario sí se gestionan conforme a esta Política). Se aplican técnicas de **minimización** y **pseudonimización** (no se vinculan datos personales con direcciones on-chain salvo en sistemas internos protegidos).

## 15. Notificación de brechas

En caso de brecha de seguridad con riesgo para los derechos del Usuario, AgroGlobalDex notificará a la autoridad de control competente dentro de las **72 horas** (art. 33 RGPD) y comunicará al Usuario afectado sin demora indebida cuando el riesgo sea alto (art. 34).

## 16. Modificaciones

Esta Política puede actualizarse. Las modificaciones materiales se notificarán al Usuario con preaviso razonable. La versión vigente está siempre disponible en [URL].

---

## Anexos referenciados

- **Anexo Transferencias**: lista de proveedores fuera del EEE y mecanismos aplicables.
- **Política de Cookies**: detalle de cookies usadas y consentimiento.
- **Registro de Actividades de Tratamiento (RoPA)**: documento interno, disponible bajo solicitud justificada a autoridades.

---

*Fin del borrador de Política de Privacidad. Pendiente de revisión por DPO/abogado.*
