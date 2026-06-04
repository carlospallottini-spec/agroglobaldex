#!/usr/bin/env bash
# AgroGlobalDex — devnet deploy script (v2)
#
# Mejoras vs v1:
#   • Soporte para RPC privados (Helius/Triton) vía env var $HELIUS_API_KEY
#     o flag --rpc <url>. Helius gratis aguanta 100 req/s vs 10 del público.
#   • Pre-flight checks: toolchain, wallet, balance, ulimit -n.
#   • Deploy de AMBOS programas (agroglobaldex + compliance_hook).
#   • Copia automática de IDLs al frontend (web 2.0/js/idl/).
#   • Confirmación interactiva (skip con --yes).
#   • Logging con colores + timestamps.
#   • Resúmen final con program IDs + URLs Solscan.
#
# Uso:
#   ./deploy-devnet.sh                          # interactivo, RPC público
#   ./deploy-devnet.sh --yes                    # sin confirmación
#   HELIUS_API_KEY=xxx ./deploy-devnet.sh       # con Helius RPC
#   ./deploy-devnet.sh --rpc https://...        # RPC explícito

set -euo pipefail

# ─── Defaults & args ──────────────────────────────────────────────────
RPC_URL="${RPC_URL:-}"
YES=0
SKIP_AIRDROP=0
for arg in "$@"; do
  case $arg in
    --yes|-y) YES=1; shift ;;
    --rpc=*) RPC_URL="${arg#*=}"; shift ;;
    --rpc) shift; RPC_URL="$1"; shift ;;
    --no-airdrop) SKIP_AIRDROP=1; shift ;;
    --help|-h)
      sed -n '1,/^set -euo pipefail$/p' "$0" | head -n -2 | sed 's/^# \?//'
      exit 0 ;;
  esac
done

# Si no se pasó RPC explícito, usar Helius si la API key está disponible
if [ -z "$RPC_URL" ]; then
  if [ -n "${HELIUS_API_KEY:-}" ]; then
    RPC_URL="https://devnet.helius-rpc.com/?api-key=$HELIUS_API_KEY"
    RPC_NAME="Helius devnet"
  else
    RPC_URL="https://api.devnet.solana.com"
    RPC_NAME="Solana devnet público"
  fi
else
  RPC_NAME="custom"
fi

# ─── Colores ──────────────────────────────────────────────────────────
if [ -t 1 ]; then
  RED='\033[0;31m'; GRN='\033[0;32m'; YLW='\033[0;33m'; CYN='\033[0;36m'; BLD='\033[1m'; NC='\033[0m'
else
  RED=''; GRN=''; YLW=''; CYN=''; BLD=''; NC=''
fi
log()  { printf "${CYN}[%s]${NC} %s\n" "$(date +%H:%M:%S)" "$*"; }
ok()   { printf "${GRN}✓${NC} %s\n" "$*"; }
warn() { printf "${YLW}⚠${NC} %s\n" "$*"; }
err()  { printf "${RED}✗${NC} %s\n" "$*"; exit 1; }
hr()   { printf "${BLD}────────────────────────────────────────────${NC}\n"; }

# ─── Paths ────────────────────────────────────────────────────────────
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FRONTEND_IDL="$ROOT/../web 2.0/js/idl"

hr
printf "${BLD}AgroGlobalDex — devnet deploy${NC}\n"
printf "  RPC: ${CYN}%s${NC} (%s)\n" "$RPC_URL" "$RPC_NAME"
printf "  Repo: %s\n" "$ROOT"
hr

# ─── Pre-flight: toolchain ────────────────────────────────────────────
log "Pre-flight: toolchain"
command -v solana >/dev/null || err "solana CLI no instalado. Instalá con: sh -c \"\$(curl -sSfL https://release.anza.xyz/v3.0.0/install)\""
command -v anchor >/dev/null || err "anchor CLI no instalado. Instalá con: cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked"
command -v cargo >/dev/null || err "cargo no instalado"
SOL_VER=$(solana --version | awk '{print $2}')
ANCHOR_VER=$(anchor --version | awk '{print $2}')
ok "solana $SOL_VER · anchor $ANCHOR_VER"

# ─── Pre-flight: wallet ───────────────────────────────────────────────
log "Pre-flight: wallet"
WALLET_PATH="$(solana config get keypair | awk '{print $NF}')"
[ -f "$WALLET_PATH" ] || err "Wallet no encontrada en $WALLET_PATH. Generala con: solana-keygen new"
WALLET_PUB="$(solana address)"
ok "Wallet: $WALLET_PUB"

# ─── Pre-flight: ulimit ──────────────────────────────────────────────
NOF="$(ulimit -n)"
if [ "$NOF" -lt 8192 ]; then
  warn "ulimit -n = $NOF (puede causar problemas con anchor build). Recomendado: 65536"
  warn "Subilo con: ulimit -n 65536"
fi

# ─── Set RPC + balance check ─────────────────────────────────────────
log "Configurando RPC"
solana config set --url "$RPC_URL" >/dev/null
ok "RPC: $RPC_URL"

log "Verificando balance"
BAL_SOL=$(solana balance 2>/dev/null | awk '{print $1}' || echo "0")
printf "  Balance: ${BLD}%s SOL${NC}\n" "$BAL_SOL"

# Necesitamos ~4 SOL para deployar 2 programas
MIN_SOL=4
NEEDED=$(awk -v b="$BAL_SOL" -v m="$MIN_SOL" 'BEGIN{print (b < m) ? 1 : 0}')
if [ "$NEEDED" -eq 1 ] && [ "$SKIP_AIRDROP" -eq 0 ]; then
  warn "Balance insuficiente (<$MIN_SOL SOL). Solicitando airdrop"
  for i in 1 2 3; do
    if solana airdrop 2 2>&1 | grep -qiE "(success|signature)"; then
      ok "Airdrop OK"
      break
    fi
    warn "Airdrop intento $i falló — el faucet devnet está rate-limited a menudo"
    sleep 5
  done
  BAL_SOL=$(solana balance 2>/dev/null | awk '{print $1}' || echo "0")
  printf "  Balance ahora: ${BLD}%s SOL${NC}\n" "$BAL_SOL"
fi

# ─── Confirm ─────────────────────────────────────────────────────────
if [ "$YES" -eq 0 ]; then
  printf "\n${YLW}Esto va a:${NC}\n"
  printf "  1. anchor keys sync (alinea declare_id! del lib.rs con el keypair)\n"
  printf "  2. anchor build (compila ambos programas)\n"
  printf "  3. solana program deploy agroglobaldex.so\n"
  printf "  4. solana program deploy compliance_hook.so\n"
  printf "  5. Copiar IDLs al frontend (web 2.0/js/idl/)\n\n"
  read -r -p "¿Continuar? [y/N] " ans
  [[ "$ans" =~ ^[yY] ]] || err "Cancelado"
fi

# ─── 1. keys sync ─────────────────────────────────────────────────────
log "anchor keys sync"
anchor keys sync >/dev/null
ok "Program IDs sincronizados"

# ─── 2. build ─────────────────────────────────────────────────────────
log "anchor build (esto puede tardar 1-2 minutos)"
anchor build 2>&1 | tail -3
[ -f target/deploy/agroglobaldex.so ] || err "agroglobaldex.so no se generó"
[ -f target/deploy/compliance_hook.so ] || err "compliance_hook.so no se generó"
ok "Build OK: $(ls -l target/deploy/*.so | awk '{print $9, "("$5/1024" KB)"}' | tr '\n' ' ')"

# ─── 3. deploy agroglobaldex ──────────────────────────────────────────
log "Deploy: agroglobaldex"
MAIN_PROGRAM_ID=$(solana address -k target/deploy/agroglobaldex-keypair.json)
solana program deploy --program-id target/deploy/agroglobaldex-keypair.json \
                     target/deploy/agroglobaldex.so 2>&1 | tail -2
ok "Deployed agroglobaldex: $MAIN_PROGRAM_ID"

# ─── 4. deploy compliance_hook ────────────────────────────────────────
log "Deploy: compliance_hook"
HOOK_PROGRAM_ID=$(solana address -k target/deploy/compliance_hook-keypair.json)
solana program deploy --program-id target/deploy/compliance_hook-keypair.json \
                     target/deploy/compliance_hook.so 2>&1 | tail -2
ok "Deployed compliance_hook: $HOOK_PROGRAM_ID"

# ─── 5. copy IDLs + integrity check ─────────────────────────────────
# Hash both IDLs antes y después de copiar. Diferencia = corrupción durante
# el cp (improbable pero defensa en profundidad). Imprimimos los hashes para
# que queden en el log del deploy y se puedan comparar contra el commit.
log "Copiando IDLs al frontend"
mkdir -p "$FRONTEND_IDL"

if command -v sha256sum >/dev/null 2>&1; then
  HASH_TOOL=sha256sum
elif command -v shasum >/dev/null 2>&1; then
  HASH_TOOL="shasum -a 256"
else
  HASH_TOOL=""
  warn "ni sha256sum ni shasum disponibles — skip integrity check"
fi

cp target/idl/agroglobaldex.json "$FRONTEND_IDL/agroglobaldex.json"
cp target/idl/compliance_hook.json "$FRONTEND_IDL/compliance_hook.json"

if [ -n "$HASH_TOOL" ]; then
  H1=$($HASH_TOOL target/idl/agroglobaldex.json | awk '{print $1}')
  H2=$($HASH_TOOL "$FRONTEND_IDL/agroglobaldex.json" | awk '{print $1}')
  H3=$($HASH_TOOL target/idl/compliance_hook.json | awk '{print $1}')
  H4=$($HASH_TOOL "$FRONTEND_IDL/compliance_hook.json" | awk '{print $1}')
  [ "$H1" = "$H2" ] || err "IDL agroglobaldex corrupto al copiar (hash mismatch)"
  [ "$H3" = "$H4" ] || err "IDL compliance_hook corrupto al copiar (hash mismatch)"
  printf "  agroglobaldex.json   sha256 = %s\n" "$H1"
  printf "  compliance_hook.json sha256 = %s\n" "$H3"
fi
ok "IDLs copiados a $FRONTEND_IDL"

# ─── Resumen final ───────────────────────────────────────────────────
hr
printf "${GRN}${BLD}✓ Deploy completo${NC}\n"
hr
printf "  ${BLD}Marketplace program:${NC} %s\n" "$MAIN_PROGRAM_ID"
printf "    Solscan: ${CYN}https://solscan.io/account/%s?cluster=devnet${NC}\n" "$MAIN_PROGRAM_ID"
printf "  ${BLD}Compliance-hook program:${NC} %s\n" "$HOOK_PROGRAM_ID"
printf "    Solscan: ${CYN}https://solscan.io/account/%s?cluster=devnet${NC}\n" "$HOOK_PROGRAM_ID"
printf "\n  ${BLD}Wallet authority:${NC} %s\n" "$WALLET_PUB"
printf "  ${BLD}RPC:${NC} %s\n" "$RPC_URL"
hr

# ─── Auto-bootstrap del marketplace (idempotente) ─────────────────────
hr
if [ "$YES" = "1" ]; then
  RUN_INIT="y"
else
  printf "\n${BLD}¿Inicializar marketplace + jurisdiction_policy + lending_market ahora?${NC} [Y/n] "
  read -r RUN_INIT
  RUN_INIT=${RUN_INIT:-y}
fi

if [ "$RUN_INIT" = "y" ] || [ "$RUN_INIT" = "Y" ]; then
  log "Corriendo initialize-devnet.ts (idempotente)…"
  export ANCHOR_PROVIDER_URL="$RPC_URL"
  export ANCHOR_WALLET="$HOME/.config/solana/id.json"
  cd "$ROOT"
  if npx ts-node --project tsconfig.seed.json scripts/initialize-devnet.ts; then
    ok "Bootstrap on-chain completo. Marketplace listo en devnet."
  else
    err "El bootstrap falló — revisar logs arriba. Las programs siguen deployadas."
  fi
fi

printf "\n${BLD}Próximos pasos:${NC}\n"
printf "  1. (opcional) Cargar datos demo end-to-end:\n"
printf "     ${CYN}npx ts-node --project tsconfig.seed.json scripts/seed-localnet.ts${NC}\n"
printf "     (necesita validator local; para devnet usar el flow web)\n\n"
printf "  2. Servir la web local apuntando a devnet (la default):\n"
printf "     ${CYN}cd .. && python3 -m http.server 8000 --directory \"web 2.0\"${NC}\n"
printf "     Abrir: http://localhost:8000/index.html\n\n"
printf "  3. Compartir las URLs Solscan de arriba con tu inversor/auditor.\n\n"
