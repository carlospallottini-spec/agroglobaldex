# AgroGlobalDex — Web (frontend estático)

5 páginas HTML + 4 módulos JS que hablan directo con el programa Solana vía `@coral-xyz/anchor` cargado por CDN (esm.sh).

## Estructura

```
web 2.0/
├── index.html              Landing
├── about.html              Nosotros
├── marketplace.html        Listado on-chain de activos nativos + agregados
├── tokenize.html           Wizard 4 pasos para productores
├── aggregate.html          Admin: cura tokens RWA-agro de terceras plataformas
├── team.html               Equipo
├── contact.html            Contacto + alta KYC
└── js/
    ├── network-config.js   PROGRAM_ID, RPC, USDC mint, formatters
    ├── wallet-adapter.js   Phantom connect / persistencia / auto-rebind .nwb
    ├── kyc-gate.js         Chequeo on-chain de ComplianceRecord
    ├── agroglobaldex-client.js  Wrapper Anchor (PDA helpers + métodos)
    └── idl/
        └── agroglobaldex.json   Stub IDL — sobreescribir tras `anchor build`
```

## Correr local

No hay build step. Cualquier servidor estático sirve:

```bash
# desde la raíz del repo
python3 -m http.server 8000 --directory "web 2.0"
# luego abrir http://localhost:8000/tokenize.html
```

o:

```bash
npx serve "web 2.0"
```

## Flujo end-to-end de tokenización

1. Operador (authority) corre `solana/scripts/deploy-devnet.sh` → deploy + IDL copiado a `web 2.0/js/idl/`.
2. Operador inicializa el marketplace + carga el USDC mint devnet en el script `seed-devnet.ts`.
3. Productor abre `tokenize.html`, conecta Phantom (devnet), pide KYC en `contact.html?kyc=1`.
4. Operador aprueba el KYC corriendo `update_kyc` con la wallet del productor.
5. Productor llena el wizard de 4 pasos: tipo → detalles → certificado (SHA-256 calculado en el navegador) → firmar.
6. Tras la firma: el activo aparece en `marketplace.html` con badge "Nativo".

## Flujo de agregador

1. Operador (sólo authority) abre `aggregate.html`.
2. Agrega un SPL token Solana de terceros (ej. Agrotoken) o una referencia cross-chain (ej. Centrifuge en Ethereum).
3. Cross-chain → display-only, link al origen. SPL Solana → mismo trato visual que un nativo (trading on-chain queda como TODO, ver `solana/README.md`).

## Configuración

- `network-config.js` → `NETWORK = 'devnet'` por default. Cambiá a `'mainnet'` cuando haya audit + CASP MiCA.
- `PROGRAM_ID` ya seteado al keypair generado. Re-deploy → `anchor keys sync` regenera ambos.
- `IDL_URL` apunta a `./idl/agroglobaldex.json` (relativo a las páginas HTML).

## Troubleshooting

| Síntoma                                       | Causa probable                                       |
|-----------------------------------------------|------------------------------------------------------|
| "Phantom no detectado"                        | Instalá Phantom desde phantom.app                    |
| "No se pudo cargar el IDL"                    | Falta `js/idl/agroglobaldex.json` — copialo del build|
| "Wallet no conectada" al firmar               | Tocá "Conectar Wallet" en el nav antes               |
| "KYC requerido" al tokenizar                  | Pediile al operador que corra `update_kyc`           |
| Tx revierta con `JurisdictionNotAllowed`      | Tu wallet está marcada con KP/IR/SY/CU               |
| Tx revierta con `MissingWhitePaper`           | El campo "White paper URI" está vacío                |
| "Failed to fetch RPC"                         | Devnet rate-limited — usar un RPC propio (Helius)    |

## Disclaimer

Demo/PoC sobre Solana devnet. NO operativo en mainnet. Sin autorización CASP MiCA al día de hoy.
