# AgroGlobalDex — App móvil (Android + iOS)

App nativa oficial de **AgroGlobalDex** para Google Play y App Store.

**Cómo funciona:** es un *wrapper* Capacitor — la app abre el sitio de
producción **https://agroglobaldex.com** dentro de una vista web nativa,
con ícono, splash screen y barra de estado con la marca. Ventaja enorme:
**cada vez que actualizás la web, la app se actualiza sola** — no hace
falta re-subir nada a las stores (salvo que cambies algo nativo: ícono,
nombre, permisos, versión de Capacitor).

Si no hay internet, la app muestra una pantalla propia "Sin conexión"
con la marca (está en `www/index.html`) y reintenta sola.

---

## Contenido de esta carpeta

| Archivo / carpeta | Qué es |
|---|---|
| `capacitor.config.ts` | Configuración de la app (id, nombre, URL, colores). **El `appId` (`io.agroglobaldex.app`) no se puede cambiar nunca después de publicar.** |
| `package.json` | Dependencias (Capacitor 8, versiones fijadas) y comandos. |
| `assets/` | Fuentes del ícono y splash (generadas desde el logo oficial 1024px). |
| `www/` | Página de fallback "Sin conexión". |
| `android/`, `ios/` | Proyectos nativos. **No existen todavía**: se crean con un comando (paso 3). |

---

## PARTE A — Publicar en Google Play (Android)

### Paso 0 — Instalar lo necesario (una sola vez)

1. **Node.js 20 o superior** — descargar el instalador "LTS" de
   https://nodejs.org e instalarlo con "siguiente, siguiente".
   Verificá abriendo una terminal y escribiendo: `node --version`
2. **Android Studio** — descargarlo de https://developer.android.com/studio
   e instalarlo con las opciones por defecto (incluye el SDK de Android y
   Java; no hace falta instalar nada más aparte).

### Paso 1 — Instalar dependencias

Abrí una terminal **dentro de esta carpeta** (`mobile/`) y ejecutá:

```bash
npm install
```

### Paso 2 — Generar íconos y splash (una sola vez, o si cambia el logo)

```bash
npm run assets
```

Esto toma `assets/icon.png` y `assets/splash.png` (creados desde el logo
oficial, fondo oscuro de marca `#05080A`) y genera automáticamente todos
los tamaños que Android e iOS exigen.
*Nota: si te dice que no existen las plataformas, corré primero el paso 3
y repetí este comando.*

### Paso 3 — Crear el proyecto Android (una sola vez)

```bash
npx cap add android
npx cap sync
npm run assets
```

Esto crea la carpeta `android/` con un proyecto nativo completo.

### Paso 4 — Probar la app

Con un teléfono Android conectado por USB (con "depuración USB" activada
en Ajustes → Opciones de desarrollador) o un emulador de Android Studio:

```bash
npx cap run android
```

O abrila en Android Studio y tocá el botón ▶:

```bash
npm run open:android
```

### Paso 5 — Crear la clave de firma (keystore) — ⚠️ LEER CON ATENCIÓN

Toda app de Android debe ir **firmada digitalmente** con una clave tuya.
Creala una única vez con este comando (en la terminal, dentro de `mobile/`):

```bash
keytool -genkey -v -keystore agroglobaldex-release.keystore \
  -alias agroglobaldex -keyalg RSA -keysize 2048 -validity 10000
```

Te va a pedir una contraseña y algunos datos (nombre, organización, país).

> ### 🚨 ADVERTENCIA CRÍTICA — NO PIERDAS ESTE ARCHIVO
> El archivo `agroglobaldex-release.keystore` y su contraseña son la
> **identidad de tu app**. Si los perdés, **NUNCA MÁS vas a poder
> publicar una actualización** de la app en Google Play: habría que
> publicar una app nueva desde cero, perdiendo usuarios, reseñas y
> posicionamiento.
>
> Guardá el archivo y la contraseña en **al menos dos lugares seguros**:
> por ejemplo un gestor de contraseñas (1Password/Bitwarden) + una copia
> del archivo en Google Drive/Dropbox privado + un pendrive en un cajón.
> **Nunca** lo subas al repositorio de código (git).

Después decile a Android dónde está la clave. Creá el archivo
`android/keystore.properties` con este contenido (con TU contraseña):

```properties
storeFile=../../agroglobaldex-release.keystore
storePassword=TU_CONTRASEÑA
keyAlias=agroglobaldex
keyPassword=TU_CONTRASEÑA
```

Y en `android/app/build.gradle`, dentro del bloque `android { ... }`,
agregá (Android Studio te ayuda; también podés pedirle a un desarrollador
o a Claude que lo haga — son ~10 líneas):

```gradle
def keystoreProperties = new Properties()
def keystorePropertiesFile = rootProject.file("keystore.properties")
if (keystorePropertiesFile.exists()) {
    keystoreProperties.load(new FileInputStream(keystorePropertiesFile))
}

android {
    // ... lo que ya está ...
    signingConfigs {
        release {
            storeFile file(keystoreProperties['storeFile'])
            storePassword keystoreProperties['storePassword']
            keyAlias keystoreProperties['keyAlias']
            keyPassword keystoreProperties['keyPassword']
        }
    }
    buildTypes {
        release {
            signingConfig signingConfigs.release
        }
    }
}
```

### Paso 6 — Generar el AAB (el archivo que se sube a Google Play)

```bash
npm run build:android
```

El archivo queda en:
`android/app/build/outputs/bundle/release/app-release.aab`

*(Google Play exige formato AAB, no APK. Si querés un APK para instalar
directo en un teléfono sin la store: `npm run build:apk`.)*

### Paso 7 — Subirlo a Google Play

1. Creá una cuenta de desarrollador en https://play.google.com/console
   — pago único de **US$ 25**. Google puede tardar 1–3 días en verificar
   tu identidad.
2. En la Console: **Crear app** → nombre "AgroGlobalDex", idioma, tipo
   "App", gratis.
3. Completá las fichas obligatorias (la Console te las lista):
   descripción, ícono 512×512 (usá `assets/icon.png` reducido — la
   Console lo acepta), capturas de pantalla, clasificación de contenido,
   público objetivo, **URL de política de privacidad** (ver checklist),
   declaración de seguridad de datos.
4. En **Producción → Crear versión** subí el `app-release.aab` y enviá a
   revisión. La primera revisión suele tardar de días a ~1 semana.
   Nota: Google exige para cuentas personales nuevas una prueba cerrada
   previa (12 testers durante 14 días) antes de poder publicar en
   producción — la Console te lo indica si aplica.

---

## PARTE B — Publicar en App Store (iOS) — resumen

Requiere **una Mac con Xcode** (no se puede compilar iOS desde Windows/Linux).

1. Instalá **Xcode** desde la Mac App Store (gratis, pesa mucho).
2. Inscribite en el **Apple Developer Program**: https://developer.apple.com
   — cuesta **US$ 99 por año** (si dejás de pagar, la app sale de la store).
3. En la terminal, dentro de `mobile/`:
   ```bash
   npm install
   npx cap add ios
   npx cap sync
   npm run assets
   npm run open:ios     # abre Xcode
   ```
4. En Xcode: seleccioná el proyecto **App** → pestaña *Signing &
   Capabilities* → marcá "Automatically manage signing" y elegí tu
   equipo (tu cuenta de developer). Xcode gestiona los certificados solo
   — acá no hay keystore manual como en Android, pero **no pierdas el
   acceso a tu Apple ID**.
5. Probala en un iPhone conectado o simulador (botón ▶).
6. Para publicar: menú **Product → Archive** → ventana Organizer →
   **Distribute App → App Store Connect**.
7. En https://appstoreconnect.apple.com creá la ficha de la app
   (screenshots de iPhone 6.7" y 6.5", descripción, política de
   privacidad, formulario "App Privacy") y enviá a revisión.

> ⚠️ **Riesgo iOS (guideline 4.2 "Minimum Functionality"):** Apple
> rechaza con frecuencia apps que son "solo un sitio web empaquetado".
> Mitigaciones: la app ya usa splash/status bar nativos y fallback
> offline; sumar notificaciones push nativas o integración con wallet
> nativa (Solana Mobile Wallet Adapter) aumenta mucho la probabilidad de
> aprobación. Alternativa si rechazan: distribuir iOS como PWA
> (el sitio ya es instalable desde Safari).

---

## Checklist de publicación

- [ ] **Cuenta Google Play Console** — US$ 25 (pago único).
- [ ] **Apple Developer Program** — US$ 99/año (solo si querés iOS).
- [ ] **Keystore Android creado y respaldado en 2+ lugares** (paso 5 — crítico).
- [ ] **Política de privacidad publicada en una URL pública.**
  ⚠️ Hoy el sitio **NO** tiene página de privacidad publicada; existen
  borradores en el repo: `legal/07-privacy-policy.draft.md` y
  `legal/06-terms-of-service.draft.md`. Hay que convertirlos en páginas
  del sitio (p.ej. `https://agroglobaldex.com/privacy.html`) **antes** de
  enviar a las stores — ambas lo exigen sin excepción.
- [ ] **Screenshots**: mínimo 2 por store (teléfono; Google además pide
  ícono 512×512 y "feature graphic" 1024×500; Apple pide capturas de
  iPhone 6.7").
- [ ] **Descripción corta y larga** de la app (ES/EN).
- [ ] **Clasificación de contenido** (cuestionario en cada console —
  declarar que hay funcionalidad financiera/cripto).
- [ ] **Formulario de seguridad de datos / App Privacy** (qué datos
  recolecta la web: wallet address, analytics, etc.).
- [ ] Probar la app en un dispositivo real antes de enviar.

## ⚠️ Compliance cripto/finanzas en las stores (importante)

AgroGlobalDex opera con tokens sobre Solana (tokenización, crédito,
inversión). Ambas stores tienen reglas específicas — conviene revisarlas
antes de enviar, porque el rechazo por esto es común:

- **Google Play**: las apps que ofrecen productos financieros o cripto
  deben declararlo en la Console (sección "Servicios financieros"); en
  varios países exige acreditar licencias/registro local para servicios
  de intercambio o billeteras. Prohíbe la minería en el dispositivo.
- **Apple App Store** (guideline 3.1.5): los exchanges/servicios cripto
  deben estar ofrecidos por la entidad debidamente licenciada en las
  jurisdicciones donde la app está disponible; las apps de "inversión"
  deben venir de la entidad regulada correspondiente.
- Mientras el producto sea **demo/PoC**, dejarlo claro dentro de la app
  y en la ficha reduce riesgo, pero no lo elimina: si la app permite
  operaciones reales con dinero, las stores pueden pedir documentación.
- El análisis regulatorio del proyecto está en `legal/` (MiCA, KYC/AML,
  licencias) — usarlo como base para lo que pidan las stores.

## Mantenimiento

- **Cambios en la web** (`web 2.0/` → deploy a agroglobaldex.com): la app
  los muestra automáticamente. No hay que tocar nada acá.
- **Cambiar ícono/splash**: reemplazar `assets/icon.png` / `assets/splash.png`
  y correr `npm run assets && npx cap sync`.
- **Subir una actualización a las stores**: aumentá `versionCode` y
  `versionName` en `android/app/build.gradle` (y el número de versión en
  Xcode para iOS), regenerá el AAB (paso 6) y subilo.
- **Actualizar Capacitor** (1–2 veces al año, para cumplir los requisitos
  de API level de Google): `npm install @capacitor/core@latest @capacitor/cli@latest @capacitor/android@latest @capacitor/ios@latest`
  y después `npx cap sync`.

## Alternativa sin stores: PWA

El sitio ya es una Progressive Web App instalable: en el teléfono, abrir
https://agroglobaldex.com y tocar "Añadir a pantalla de inicio". Ícono,
splash, offline shell y tab bar ya funcionan (ver `web 2.0/manifest.webmanifest`,
`web 2.0/sw.js`, `web 2.0/js/pwa-install.js`). Es la vía más rápida de
distribución mientras las cuentas de developer se tramitan.
