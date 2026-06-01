---
name: solana-dev
description: Agente Solana/Anchor para AgroGlobalDex. Token-2022 + SPL clasico + PDA + multi-program. (agroglobaldex)
---

# Solana Dev — AgroGlobalDex

Stack: Solana 3.0.0 + Anchor 0.31.1 + platform-tools v1.54 + anchor-spl 0.31.1 (token_2022 + extensions).

## Program IDs (NO CAMBIAR)
- agroglobaldex: G2n9JXE5FLRRprM1R4gga1uF3yT6jneHDzSX913xLR2a
- compliance_hook: GFFp2bThyR33mxbVQiohGL22eEs12eJhvKyEnUoCL8tL

## 17 instrucciones (programs/agroglobaldex/)
initialize, set_compliance_signer, set_paused, init_jurisdiction_policy,
update_jurisdiction_policy, register_asset, mint_token, update_kyc, list_asset,
update_listing_price, cancel_listing, buy_asset, buy_external_asset,
treasury_withdraw, redeem, aggregate_external_asset, update_external_asset.

## 5 AssetClass
Grain, CarbonCredit, HarvestFraction, InvestmentOffering, Commodity
(9 sectores: Meat/Wine/Oil/Dairy/Fruit/Vegetable/Fiber/GrainSpecial/Other).

## Reglas duras
1. anchor keys sync ANTES de anchor build si tocaste declare_id.
2. Token-2022 extension order: TransferHook ANTES de initialize_mint2.
3. PDA seeds determinísticas con index: u64.
4. enforce_compliance en buy_asset Y buy_external_asset.
5. compliance_signer ≠ authority.
6. Audit pendiente: NO mainnet con valor real.
