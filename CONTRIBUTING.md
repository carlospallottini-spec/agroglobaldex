# Contributing — AgroGlobalDex

> Demo/PoC. Audit + CASP MiCA pendientes. NO mainnet.

Cualquier PR es bienvenido si va alineado con el roadmap (ver
[`MAINNET.md`](MAINNET.md)) y no rompe el toolchain validado.

## Setup local

### Solana

```bash
# Toolchain validado
sh -c "$(curl -sSfL https://release.anza.xyz/v3.0.0/install)"
cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked

cd solana/

# CRÍTICO: ulimit -n 65536 o el validator local muere en 60s
ulimit -n 65536

anchor build
nohup prlimit --nofile=65536 -- solana-test-validator --reset \
    --rpc-port 8899 --faucet-port 9900 --bind-address 127.0.0.1 \
    --limit-ledger-size 50000000 > /tmp/validator.log 2>&1 &

solana config set --url http://127.0.0.1:8899
solana airdrop 200

solana program deploy --program-id target/deploy/agroglobaldex-keypair.json target/deploy/agroglobaldex.so
solana program deploy --program-id target/deploy/compliance_hook-keypair.json target/deploy/compliance_hook.so

# Seed end-to-end (15 steps, exercitan TODAS las 21 ix)
npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts

# Tests (28 mocha)
anchor test --skip-local-validator
```

### Frontend

```bash
# Servir web 2.0/
python3 -m http.server 8000 --directory "web 2.0"
# http://localhost:8000/index.html (público) o /admin.html (operadores)
```

### Electron desktop

```bash
cd agrochain-electron/
npm install
npm run build  # genera dist/AgroGlobalDex-Setup-2.1.0.exe (Windows)
```

## Process — los 3 agentes IA del proyecto

Cada PR debería invocar el agente apropiado vía Claude Code:

| Tarea | Agente | SKILL.md |
|---|---|---|
| Smart contract / IDL / tests | **Solana Dev** | [`agents/solana-dev/SKILL.md`](agents/solana-dev/SKILL.md) |
| Pitch / outreach / grants | **Marketing & BD** | [`agents/marketing-bd/SKILL.md`](agents/marketing-bd/SKILL.md) |
| MiCA / T&Cs / KYC | **Legal & Compliance** | [`agents/legal-compliance/SKILL.md`](agents/legal-compliance/SKILL.md) |

Para tasks cross-dominio: usar [`gstack-skills/spec/`](gstack-skills/spec/)
para convertir intent → lista de cambios concretos antes de implementar.

## Reglas duras para PRs

### NO

- ❌ **Cambiar los program IDs**: `G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a`
  (agroglobaldex) y `GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL`
  (compliance_hook). Rompe TODO cliente que tenga el viejo embebido.
- ❌ **`anchor init` o `npm i` sin lock**: usar siempre el lock validado.
- ❌ **Invertir el orden de las Token-2022 extensions**: `TransferHook`
  va ANTES de `initialize_mint2`. Si lo invertís, falla en runtime.
- ❌ **PDA seeds basados en client input no-reconstructible**: siempre
  `index: u64` o algo derivable on-chain.
- ❌ **Eliminar `enforce_compliance`** de `buy_asset` / `buy_external_asset`.
- ❌ **Promesas de yield garantizado** en cualquier doc público
  (securities fraud).
- ❌ **Bypass de KYC** sin Reg D filing (US persons bloqueados on-chain).
- ❌ **Deploy a mainnet con valor real** sin audit profesional.
- ❌ **`git push --force` a branch productiva**.
- ❌ **`--no-verify`** en commits (pre-commit hooks son load-bearing).

### SÍ

- ✅ **Disclaimer Demo/PoC** en toda comunicación pública (`web 2.0/`,
  decks, blog posts).
- ✅ **Tests por cada feature**: happy + 1+ sad path mínimo.
- ✅ **`anchor build`** debe pasar OK antes del commit. CI rechaza warnings
  nuevos.
- ✅ **IDL copy** a `web 2.0/js/idl/` después de cada build que cambia
  la API on-chain.
- ✅ **Update del seed script** si agregás una ix nueva.
- ✅ **Update del CHANGELOG.md** en cada PR no-trivial.
- ✅ **Commit messages descriptivos**: imperativo en español, primera
  línea < 72 chars, body con WHY no WHAT.
- ✅ **Marcar `[VERIFICAR]`** si no estás seguro de un dato (en lugar de
  inventar).

## Estructura del repo

```
solana/                        Programs Anchor + tests + scripts
├── programs/agroglobaldex/    Programa principal (21 ix)
├── programs/compliance-hook/  Token-2022 TransferHook (2 ix)
├── tests/                     28 mocha tests
├── scripts/                   seed-localnet.ts + deploy-devnet.sh
├── Anchor.toml, Cargo.toml
└── RUNBOOK.md                 Operations playbook

web 2.0/                       Frontend (9 HTML + PWA)
├── *.html                     8 public + admin.html
├── js/                        Anchor client + multi-wallet + MWA
└── manifest.webmanifest, sw.js

agrochain-electron/            Desktop wrapper (.exe / .dmg / .AppImage)
mobile/                        Capacitor Android scaffold
marketing/                     Pitch deck, financials, FAQ, LOIs, grants
legal/                         MiCA + T&Cs + Privacy + white paper template
agents/                        SKILL.md de los 3 agentes del proyecto
gstack-skills/                 10 workflows reusados de gstack (MIT)
.github/workflows/             CI multi-platform releases
```

## Branching

- `main`: rama productiva (tags `v*` triggerean release).
- `claude/spanish-greeting-5qe8o`: rama de desarrollo activa.
- Feature branches: `<author>/<short-desc>`. Merge a `main` vía PR.

## License

MIT (código) · CC-BY-4.0 (docs).

`gstack-skills/` contiene archivos derivados de
[gstack](https://github.com/garrytan/gstack) (Garry Tan, MIT) — ver
[`gstack-skills/LICENSE.gstack`](gstack-skills/LICENSE.gstack).

---

## Pregunta antes de codear

Si tu PR toca:

- **Schema de cuenta** (e.g. agregar campo a `AssetRegistry`): pensá
  migración + si rompe accounts existentes en devnet.
- **Program ID**: NO. Discutilo primero.
- **MiCA / compliance**: invocá el agente Legal antes — los borradores
  son informativos, no jurídicos.
- **Tokenomics / cap table**: invocá el agente Marketing — todo cambio
  impacta el pitch.

Cualquier duda: abrí un issue antes del PR.
