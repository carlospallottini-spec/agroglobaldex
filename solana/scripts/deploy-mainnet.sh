#!/usr/bin/env bash
# ============================================================================
# AgroGlobalDex — SECURE MAINNET DEPLOY / KEY CEREMONY
# ============================================================================
# The program keypairs committed in this repo are DEVNET-ONLY test identities
# (see .gitignore / SECURITY.md). For mainnet you MUST NOT reuse them. This
# script performs the secure ceremony:
#   1. Generate FRESH program keypairs OFFLINE (never committed).
#   2. Build + deploy with those keys.
#   3. Immediately transfer the *upgrade authority* to a Squads multisig, so
#      the program keypair secret becomes irrelevant going forward.
#
# Run this from an air-gapped / hardware-wallet machine. Requires: solana CLI,
# anchor, and a funded mainnet payer (hardware wallet recommended).
#
#   SQUADS_MULTISIG=<vault-pubkey> ./scripts/deploy-mainnet.sh
# ============================================================================
set -euo pipefail

: "${SQUADS_MULTISIG:?Set SQUADS_MULTISIG to the Squads vault pubkey that will own upgrades}"
CLUSTER="${CLUSTER:-mainnet-beta}"
KEYDIR="${KEYDIR:-$HOME/.agroglobaldex-keys}"   # OUTSIDE the repo. Never commit.

echo "==> Cluster: $CLUSTER   Upgrade authority (multisig): $SQUADS_MULTISIG"
mkdir -p "$KEYDIR"; chmod 700 "$KEYDIR"

# 1) Fresh keypairs (offline). These define the program addresses.
for prog in agroglobaldex compliance_hook; do
  if [ ! -f "$KEYDIR/$prog-keypair.json" ]; then
    solana-keygen new --no-bip39-passphrase --silent -o "$KEYDIR/$prog-keypair.json"
  fi
  echo "    $prog program id: $(solana-keygen pubkey "$KEYDIR/$prog-keypair.json")"
done

cat <<'NOTE'
==> ACTION REQUIRED before deploying:
    1. Put the new program ids into:
         - declare_id!(...) in programs/*/src/lib.rs
         - [programs.mainnet] in Anchor.toml
         - web 2.0/js/network-config.js (PROGRAM_ID / COMPLIANCE_HOOK_PROGRAM_ID)
       then `anchor build` so the .so embeds the right ids.
    2. Re-run this script with DEPLOY=1 to deploy.
NOTE
[ "${DEPLOY:-0}" = "1" ] || { echo "(dry run — set DEPLOY=1 to deploy)"; exit 0; }

# 2) Deploy each program with its fresh keypair as the (temporary) authority.
anchor build
for prog in agroglobaldex compliance_hook; do
  so="target/deploy/${prog}.so"
  solana program deploy "$so" \
    --program-id "$KEYDIR/$prog-keypair.json" \
    --url "$CLUSTER"
done

# 3) Hand the UPGRADE AUTHORITY to the Squads multisig. After this the program
#    keypair's secret no longer controls upgrades.
for prog in agroglobaldex compliance_hook; do
  pid="$(solana-keygen pubkey "$KEYDIR/$prog-keypair.json")"
  solana program set-upgrade-authority "$pid" \
    --new-upgrade-authority "$SQUADS_MULTISIG" \
    --url "$CLUSTER"
  echo "    $prog upgrade authority -> $SQUADS_MULTISIG"
done

echo "==> Done. Verify with: solana program show <PROGRAM_ID>  (Upgrade Authority == $SQUADS_MULTISIG)"
echo "==> Also: set the marketplace 'authority' to a Squads multisig and a SEPARATE compliance_signer."
