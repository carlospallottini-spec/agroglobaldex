#!/usr/bin/env bash
# AgroGlobalDex — devnet deploy script
# Requisitos: solana ≥ 2.1, anchor ≥ 0.31, node ≥ 20, una wallet con SOL devnet.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> Configurando Solana CLI a devnet"
solana config set --url https://api.devnet.solana.com >/dev/null

echo "==> Pidiendo SOL devnet (si la wallet está vacía)"
BAL=$(solana balance --lamports | awk '{print $1}')
if [ "${BAL%% *}" -lt 1000000000 ]; then
  solana airdrop 2 || true
fi

echo "==> anchor keys sync (sincroniza program id en lib.rs y Anchor.toml)"
anchor keys sync

echo "==> anchor build"
anchor build

echo "==> anchor deploy --provider.cluster devnet"
anchor deploy --provider.cluster devnet

echo "==> Copiando IDL a la web"
mkdir -p "../web 2.0/js/idl"
cp target/idl/agroglobaldex.json "../web 2.0/js/idl/agroglobaldex.json"

echo
echo "✓ Deploy listo."
echo "  Program ID: $(solana address -k target/deploy/agroglobaldex-keypair.json)"
echo
echo "  Próximo paso: corré 'ts-node scripts/seed-devnet.ts' para inicializar"
echo "  el marketplace + cargar datos demo."
