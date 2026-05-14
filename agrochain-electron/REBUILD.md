# AgroGlobalDex Electron — Cómo regenerar el .exe

> El `agrochain-electron/src/` es ahora un **mirror** de `web 2.0/`.
> Cada vez que actualices la web, sincronizá con:
>
> ```bash
> rm -rf agrochain-electron/src
> mkdir -p agrochain-electron/src
> cp -r "web 2.0/." agrochain-electron/src/
> ```

## Build del .exe (Windows portable + NSIS installer)

Requiere **máquina Windows o WSL con Wine + electron-builder**. Lo más fácil:
en una máquina Windows propia.

```bash
cd agrochain-electron
npm install                # primera vez
npm run build              # genera dist/AgroGlobalDex-2.0.0-portable.exe
                           #          dist/AgroGlobalDex Setup 2.0.0.exe (NSIS)
```

El `.exe` queda en `dist/`. Tamaño aproximado: 80-120 MB (incluye Chromium embebido).

## Build en Linux/Mac (cross-compile)

Linux puede generar `.exe` Windows con Wine + Mono instalados:

```bash
# Ubuntu
sudo apt-get install wine wine32 wine64 mono-complete

cd agrochain-electron
npm install
npm run build
```

## Build sin tu máquina (GitHub Actions)

Crear `.github/workflows/electron-build.yml` para que se buildee automático
al hacer push de un tag `v*`. Esto es lo más cómodo: ni tenés que tener
Windows, ni Wine. Push de tag y baja el `.exe` desde GitHub Releases.

Ejemplo de workflow (copiar a `.github/workflows/electron-build.yml`):

```yaml
name: Build Electron AgroGlobalDex
on:
  push:
    tags: [ 'v*' ]
  workflow_dispatch:

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 20 }
      - name: Sync web into electron src
        shell: pwsh
        run: |
          Remove-Item -Recurse -Force agrochain-electron\src -ErrorAction SilentlyContinue
          New-Item -ItemType Directory -Path agrochain-electron\src | Out-Null
          Copy-Item -Recurse "web 2.0\*" agrochain-electron\src\
      - name: Install
        run: cd agrochain-electron && npm install
      - name: Build
        run: cd agrochain-electron && npm run build
      - uses: softprops/action-gh-release@v2
        with:
          files: agrochain-electron/dist/*.exe
```

Después, en local:
```bash
git tag v2.0.0
git push origin v2.0.0
# Esperar ~5 min y descargar el .exe desde GitHub Releases
```

## Contenido del nuevo .exe (v2.0.0)

Comparado con la versión `1.1.0` original (AgroChain inicial), el `.exe`
ahora incluye **todo** lo desarrollado:

| Página | Estado | Funcionalidad |
|---|---|---|
| `index.html` | ✅ Actualizado | Hero + ticker + tracción |
| `marketplace.html` | ✅ Actualizado | Listings on-chain |
| `tokenize.html` | 🆕 Nuevo | Wizard 4 pasos para productores |
| `invest.html` | 🆕 Nuevo | Yield offerings (MiCA-compliant) |
| `aggregate.html` | 🆕 Nuevo | Admin: cura tokens externos |
| `investors.html` | 🆕 Nuevo | Fundraising deck visual |
| `about/contact/team.html` | ✅ Updated | Nav actualizado |

| JS modules | Funcionalidad |
|---|---|
| `agroglobaldex-client.js` | Cliente Anchor (16 ix + 5 AssetClass) |
| `wallet-adapter.js` | Multi-wallet (Phantom/Solflare/Backpack/Glow) |
| `mwa-helper.js` | Mobile Wallet Adapter para Saga/Capacitor |
| `kyc-gate.js` | KYC on-chain check |
| `network-config.js` | RPC + program ID + USDC mint config |
| `pwa-install.js` | Install prompt + bottom tab mobile (no relevant en Electron) |
| `idl/agroglobaldex.json` | IDL real del programa Solana |
| `idl/compliance_hook.json` | IDL del programa Transfer Hook |

| Otros assets | |
|---|---|
| `icons/` | Iconos 192/512/maskable |
| `manifest.webmanifest` | Manifesto PWA (web mode) |
| `sw.js` | Service worker (web mode) |

## Caveat para el Electron build

En la versión Electron desktop:
- ✅ El cliente Solana funciona (esm.sh CDN imports)
- ✅ Multi-wallet detection funciona si Phantom desktop está instalado
- ✅ Lectura on-chain funciona (RPC devnet/mainnet)
- ⚠️ PWA install button es ignorado (no aplica en Electron)
- ⚠️ Mobile bottom tab no se muestra (responsive a >780px solo)

## Tamaño del binario

- Portable .exe: ~85 MB
- NSIS installer: ~95 MB

Si quisieras reducirlo, podrías:
- Cambiar a **Tauri** (rust + webview nativo del SO) → ~5-15 MB
- Pero requiere reescribir parte del wrapper

Para PoC y demos a inversores, Electron está bien.
