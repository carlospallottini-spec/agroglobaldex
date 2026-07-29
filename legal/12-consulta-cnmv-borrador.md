# Borrador de consulta al Portal Fintech de la CNMV

**Destinatario:** fintech@cnmv.es
**Alternativa (recomendada por la CNMV):** formulario en
https://www.cnmv.es/Portal/Fintech/Formulario-Solicitud.aspx

> **Antes de enviarlo, sustituye todo lo que está entre corchetes.** No inventes
> datos: si aún no tienes sociedad constituida, dilo tal cual (el texto ya está
> redactado para ese caso). La CNMV atiende consultas de promotores individuales.

---

## Asunto del correo

```
Consulta Portal Fintech — calificación de tokens representativos de mercancía
agrícola y perímetro de actividad (proyecto AgroGlobalDex)
```

---

## Cuerpo del correo

```
A la atención del Portal Fintech
Comisión Nacional del Mercado de Valores

Estimados señores:

Me dirijo a ustedes al amparo del canal de consultas del Portal Fintech, con el
fin de solicitar su criterio sobre la calificación regulatoria de un proyecto en
fase de desarrollo, con carácter previo a cualquier actividad con usuarios
reales.

1. IDENTIFICACIÓN

Nombre y apellidos: [NOMBRE Y APELLIDOS]
NIF/NIE: [NÚMERO]
Domicilio: [DIRECCIÓN COMPLETA], Madrid
Teléfono: [TELÉFONO]  ·  Correo: [TU CORREO]
Proyecto: AgroGlobalDex  ·  Demostración técnica: https://agroglobaldex.com

Actúo como promotor individual. [Si no tienes sociedad, deja esta frase:] A día
de hoy no existe sociedad constituida; la constitución se plantea precisamente en
función del encaje regulatorio que resulte de esta consulta. [Si sí la tienes,
sustituye por: La sociedad promotora es [DENOMINACIÓN], con CIF [CIF].]

2. ESTADO ACTUAL DEL PROYECTO

Se trata de un desarrollo en fase de prueba de concepto. En este momento:

- No se ha realizado oferta al público de criptoactivos ni de instrumento alguno.
- No hay clientes, ni usuarios reales, ni fondos de terceros.
- El software opera únicamente en red de pruebas (devnet de Solana), sin valor
  económico real, y así se advierte expresamente en el sitio web.
- No se presta ningún servicio remunerado.

La consulta se formula, por tanto, con carácter preventivo: mi intención es
conocer el perímetro regulatorio aplicable antes de tomar decisiones de diseño
del producto, de estructura societaria y de solicitud de autorizaciones.

3. DESCRIPCIÓN FUNCIONAL

El proyecto consiste en un mercado digital de activos agropecuarios tokenizados
sobre la red pública Solana, con estas características:

a) Representación del activo. Cada token se emite bajo el estándar Token-2022 y
   representa una cantidad determinada de mercancía agrícola física (por ejemplo,
   N toneladas de grano) depositada en almacén. El diseño previsto contempla
   redención: el titular puede canjear el token por la mercancía o su equivalente
   monetario. No se promete rendimiento, ni participación en beneficios, ni
   revalorización; tampoco existe mecanismo alguno de estabilización de precio.

b) Cumplimiento incorporado al token. Cada transferencia invoca de forma
   obligatoria y en la misma transacción un programa de control ("transfer hook")
   que verifica identificación del usuario (KYC), jurisdicción y, cuando el activo
   así lo requiere, condición de inversor acreditado. Una transferencia que no
   supere esas comprobaciones no llega a ejecutarse. La verificación de identidad
   se realizaría off-chain por proveedor especializado, cuyo resultado se sella
   on-chain.

c) Ausencia de custodia. Los usuarios conservan sus propias claves en billeteras
   de autocustodia. La plataforma no custodia en ningún momento fondos ni
   criptoactivos de terceros, ni dispone de ellos.

d) Módulo de préstamo. Está previsto un servicio de préstamo con garantía de los
   propios tokens, cuyo valor se obtiene de un oráculo de precios externo.

e) Registro. Cada operación genera un asiento inmutable en la propia cadena.

4. CUESTIONES SOBRE LAS QUE SE SOLICITA CRITERIO

4.1. Calificación del token de mercancía. Un token que representa mercancía
     física depositada, fungible, transmisible y redimible, sin derecho a
     participación en resultados ni promesa de rendimiento, ¿debe considerarse
     "otro criptoactivo" a efectos del Reglamento (UE) 2023/1114 (MiCA), o podría
     quedar comprendido en el concepto de valor negociable e instrumento
     financiero conforme a MiFID II y al texto refundido de la Ley del Mercado de
     Valores?

4.2. Elementos determinantes de recalificación. ¿Qué características del diseño
     considerarían determinantes para desplazar la calificación hacia instrumento
     financiero? En particular, agradecería su criterio sobre el efecto de: (i)
     que el subyacente sea cosecha futura en lugar de mercancía ya depositada;
     (ii) la existencia de un mercado secundario dentro de la propia plataforma;
     (iii) la incorporación de cualquier derecho económico sobre resultados.

4.3. Perímetro de la actividad de la plataforma. Dado que el emparejamiento de
     órdenes se produce en la red pública y sin custodia por mi parte,
     ¿constituiría la operativa descrita alguno de los servicios de criptoactivos
     del artículo 3.1.16 de MiCA — señaladamente la explotación de una plataforma
     de negociación o la ejecución de órdenes — y quedaría por tanto sujeta a
     autorización como proveedor de servicios de criptoactivos?

4.4. Módulo de préstamo. ¿Cuál sería el tratamiento del servicio de préstamo con
     garantía de criptoactivos descrito, considerando que no figura entre los
     servicios enumerados en MiCA?

4.5. Régimen piloto DLT. En el supuesto de que la calificación fuese la de
     instrumento financiero, ¿resultaría de aplicación el Reglamento (UE)
     2022/858 sobre infraestructuras del mercado basadas en tecnología de
     registro descentralizado, y considerarían adecuado ese cauce para un
     proyecto de este tamaño?

4.6. Espacio controlado de pruebas. ¿Considerarían el proyecto susceptible de
     acogerse al espacio controlado de pruebas de la Ley 7/2020? De ser así,
     agradecería confirmación del plazo de la próxima cohorte y del cauce de
     presentación de la solicitud.

5. DOCUMENTACIÓN

Quedo a su disposición para aportar la documentación técnica que estimen
oportuna. Dispongo de descripción detallada de la arquitectura del sistema, del
código fuente y de un informe interno de revisión de seguridad, que puedo remitir
en el formato que indiquen.

Agradezco de antemano su atención y quedo a la espera de sus indicaciones.

Atentamente,

[NOMBRE Y APELLIDOS]
[TELÉFONO] · [CORREO]
Madrid, [FECHA]
```

---

## Notas de uso

**Qué esperar.** La respuesta del Portal Fintech es orientativa: expresa criterios
de la CNMV, no constituye autorización ni resolución administrativa, y no
sustituye el asesoramiento de un abogado. Aun así tiene dos valores enormes:
orienta el diseño del producto antes de gastar dinero, y acredita ante inversores
una actitud de cumplimiento proactivo.

**Plazos.** No hay plazo legal tasado para este canal. Es habitual que la
respuesta tarde varias semanas. No lo interpretes como desinterés.

**Qué NO hacer.**
- No afirmes estar autorizado ni sugieras que la CNMV supervisa el proyecto.
- No lances oferta al público ni captes fondos mientras esperas respuesta.
- No adornes el estado del proyecto: la honestidad sobre la fase de prueba de
  concepto es lo que hace que la consulta sea admisible y útil.

**Después de enviarlo.** Guarda copia del envío y de la respuesta en el data room
(`docs/DATA_ROOM.md`): es material de primer orden en una ronda de financiación.

**Si el correo rebota o no obtienes respuesta**, usa el formulario oficial, que
entra por registro:
https://www.cnmv.es/Portal/Fintech/Formulario-Solicitud.aspx

---

*Documento preparatorio. No constituye asesoramiento legal. La calificación
definitiva del producto debe confirmarla un abogado especializado en mercados de
valores y criptoactivos.*
