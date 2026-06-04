# AgroGlobalDex Mobile

Dos opciones de "app móvil" para AgroGlobalDex, ambas reutilizan el mismo
código de `web 2.0/` sin reescribir la UI:

## 1. PWA (más rápido, ya funciona)

La web ya es **Progressive Web App** instalable. Desde Chrome / Edge /
Safari (iOS 16+) en el móvil:

1. Abrí `https://agroglobaldex.io` (o tu URL local).
2. Tocá "Añadir a pantalla de inicio" / "Install app".
3. Listo: ícono en el home, splash screen, fullscreen, offline shell,
   bottom tab bar nativa estilo app.

Implementación:
- `web 2.0/manifest.webmanifest` — nombre, iconos 192/512/maskable, theme,
  5 shortcuts (Marketplace · Tokenizar · **Crédito** · Invertir · Comprobantes).
- `web 2.0/sw.js` v6 — service worker (cache-first para assets estáticos,
  network-first para HTML/JSON). Precachea las 11 pages + IDLs.
- `web 2.0/js/pwa-install.js` — registra el SW, muestra botón "Instalar
  app", dibuja la bottom tab bar mobile (5 secciones: Inicio · Mercado ·
  Tokenizar · **Crédito** · Invertir). Inyecta CSS mobile-only:
  - `min-height:44px` en todos los botones (Apple HIG tap target).
  - `font-size:16px!important` en inputs (evita auto-zoom de iOS al focus).
  - `table{overflow-x:auto}` para tablas anchas (receipts.html, borrow).

Las 11 páginas accesibles (todas responsive a 360px+):

| Public | Admin |
|---|---|
| index.html | aggregate.html |
| marketplace.html | admin.html |
| tokenize.html | |
| **borrow.html** (crédito al productor, lending market) | |
| invest.html | |
| **receipts.html** (ledger TradeReceipt) | |
| investors.html | |
| about.html, contact.html, team.html | |

Meta tags PWA inyectados en cada página (theme-color, apple-touch-icon,
apple-mobile-web-app-*).

## 2. Capacitor → APK Android nativo

Si querés un .apk firmado (publicable en Play Store o sideload):

```bash
cd mobile/
npm install
npm run sync          # copia web 2.0/ a android/app/src/main/assets/
npm run open:android  # abre Android Studio
# O directo a línea de comandos:
npx cap run android
# O build apk debug:
npx cap copy android
cd android && ./gradlew assembleDebug
# El APK queda en android/app/build/outputs/apk/debug/
```

### Setup local one-time

Necesitás:
- Node 20+
- JDK 17+
- Android SDK (Android Studio o `commandlinetools`)
- `ANDROID_HOME` apuntando al SDK

`mobile/capacitor.config.ts` apunta a `../web 2.0` como `webDir`. Cualquier
cambio en la web se sincroniza con `npx cap sync`.

### Wallet móvil

- En PWA, **Phantom Mobile** y **Solflare Mobile** funcionan como deep
  links desde el navegador del móvil (el browser-in-app de la wallet).
- En la versión Capacitor, podés instalar
  `@solana-mobile/mobile-wallet-adapter-protocol` para integración nativa
  con Seed Vault y wallets compatibles con MWA. Documentado como TODO al
  final del README — la integración requiere unas 2 horas de tweaks.

## Disclaimer

Es una PoC. Antes de publicar en stores: proper signing, ProGuard rules,
revisión de permisos (la app sólo necesita Internet), y compliance de
contenido (banner Demo/PoC visible).
