#!/bin/bash

# import common functions
source "$(dirname "$0")"/bridges_common.sh

# Expected sovereign accounts.
#
# Generated by:
#
#	#[test]
#	fn generate_sovereign_accounts() {
#		use sp_core::crypto::Ss58Codec;
#		use polkadot_parachain_primitives::primitives::Sibling;
#
#		parameter_types! {
#			pub UniversalLocationAHR: InteriorMultiLocation = X2(GlobalConsensus(Rococo), Parachain(1000));
#			pub UniversalLocationAHW: InteriorMultiLocation = X2(GlobalConsensus(Wococo), Parachain(1000));
#		}
#
#		// SS58=42
#		println!("GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHW, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Rococo)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("GLOBAL_CONSENSUS_ROCOCO_ASSET_HUB_ROCOCO_1000_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusParachainConvertsFor::<UniversalLocationAHW, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X2(GlobalConsensus(Rococo), Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("ASSET_HUB_WOCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WOCOCO=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 SiblingParachainConvertsVia::<Sibling, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 1, interior: X1(Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#
#		// SS58=42
#		println!("GLOBAL_CONSENSUS_WOCOCO_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHR, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Wococo)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("GLOBAL_CONSENSUS_WOCOCO_ASSET_HUB_WOCOCO_1000_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusParachainConvertsFor::<UniversalLocationAHR, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X2(GlobalConsensus(Wococo), Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 SiblingParachainConvertsVia::<Sibling, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 1, interior: X1(Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#	}
GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT="5GxRGwT8bU1JeBPTUXc7LEjZMxNrK8MyL2NJnkWFQJTQ4sii"
GLOBAL_CONSENSUS_ROCOCO_ASSET_HUB_ROCOCO_1000_SOVEREIGN_ACCOUNT="5CfNu7eH3SJvqqPt3aJh38T8dcFvhGzEohp9tsd41ANhXDnQ"
ASSET_HUB_WOCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WOCOCO="5Eg2fntNprdN3FgH4sfEaaZhYtddZQSQUqvYJ1f2mLtinVhV"
GLOBAL_CONSENSUS_WOCOCO_SOVEREIGN_ACCOUNT="5EWw2NzfPr2DCahourp33cya6bGWEJViTnJN6Z2ruFevpJML"
GLOBAL_CONSENSUS_WOCOCO_ASSET_HUB_WOCOCO_1000_SOVEREIGN_ACCOUNT="5EJX8L4dwGyYnCsjZ91LfWAsm3rCN8vY2AYvT4mauMEjsrQz"
ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO="5Eg2fntNprdN3FgH4sfEaaZhYtddZQSQUqvYJ1f2mLtinVhV"

# Expected sovereign accounts for rewards on BridgeHubs.
#
# Generated by:
#[test]
#fn generate_sovereign_accounts_for_rewards() {
#	use sp_core::crypto::Ss58Codec;
#
#	// SS58=42
#	println!(
#		"ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_ThisChain=\"{}\"",
#		frame_support::sp_runtime::AccountId32::new(
#			PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#				LaneId([0, 0, 0, 1]),
#				*b"bhwo",
#				RewardsAccountOwner::ThisChain
#			))
#		)
#		.to_ss58check_with_version(42_u16.into())
#	);
#	// SS58=42
#	println!(
#		"ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_BridgedChain=\"{}\"",
#		frame_support::sp_runtime::AccountId32::new(
#			PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#				LaneId([0, 0, 0, 1]),
#				*b"bhwo",
#				RewardsAccountOwner::BridgedChain
#			))
#		)
#		.to_ss58check_with_version(42_u16.into())
#	);
#
#	// SS58=42
#	println!(
#		"ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_ThisChain=\"{}\"",
#		frame_support::sp_runtime::AccountId32::new(
#			PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#				LaneId([0, 0, 0, 1]),
#				*b"bhro",
#				RewardsAccountOwner::ThisChain
#			))
#		)
#		.to_ss58check_with_version(42_u16.into())
#	);
#	// SS58=42
#	println!(
#		"ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_BridgedChain=\"{}\"",
#		frame_support::sp_runtime::AccountId32::new(
#			PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#				LaneId([0, 0, 0, 1]),
#				*b"bhro",
#				RewardsAccountOwner::BridgedChain
#			))
#		)
#		.to_ss58check_with_version(42_u16.into())
#	);
#}
ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_ThisChain="5EHnXaT5BhiS8YRPMeHi97YHofTtNx4pLNb8wR8TwjVq1gzU"
ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_BridgedChain="5EHnXaT5BhiS8YRPMeHyt95svA95qWAh53XeVMpJQZNZHAzj"
ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_ThisChain="5EHnXaT5BhiS8YRNuCukWXTQdAqARjjXmpjehjx1YZNE5keZ"
ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_BridgedChain="5EHnXaT5BhiS8YRNuCv2FYzzjfWMtHqQWVgAFgdr1PExMN94"

LANE_ID="00000001"

function init_ro_wo() {
    ensure_relayer

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay init-bridge rococo-to-bridge-hub-wococo \
	--source-host localhost \
	--source-port 9942 \
	--source-version-mode Auto \
	--target-host localhost \
	--target-port 8945 \
	--target-version-mode Auto \
	--target-signer //Bob
}

function init_wo_ro() {
    ensure_relayer

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay init-bridge wococo-to-bridge-hub-rococo \
        --source-host localhost \
        --source-port 9945 \
        --source-version-mode Auto \
        --target-host localhost \
        --target-port 8943 \
        --target-version-mode Auto \
        --target-signer //Bob
}

function run_relay() {
    ensure_relayer

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay relay-headers-and-messages bridge-hub-rococo-bridge-hub-wococo \
        --rococo-host localhost \
        --rococo-port 9942 \
        --rococo-version-mode Auto \
        --bridge-hub-rococo-host localhost \
        --bridge-hub-rococo-port 8943 \
        --bridge-hub-rococo-version-mode Auto \
        --bridge-hub-rococo-signer //Charlie \
        --wococo-headers-to-bridge-hub-rococo-signer //Bob \
        --wococo-parachains-to-bridge-hub-rococo-signer //Bob \
        --bridge-hub-rococo-transactions-mortality 4 \
        --wococo-host localhost \
        --wococo-port 9945 \
        --wococo-version-mode Auto \
        --bridge-hub-wococo-host localhost \
        --bridge-hub-wococo-port 8945 \
        --bridge-hub-wococo-version-mode Auto \
        --bridge-hub-wococo-signer //Charlie \
        --rococo-headers-to-bridge-hub-wococo-signer //Bob \
        --rococo-parachains-to-bridge-hub-wococo-signer //Bob \
        --bridge-hub-wococo-transactions-mortality 4 \
        --lane "${LANE_ID}"
}

case "$1" in
  run-relay)
    init_ro_wo
    init_wo_ro
    run_relay
    ;;
  init-asset-hub-rococo-local)
      ensure_polkadot_js_api
      # create foreign assets for native Wococo token (governance call on Rococo)
      force_create_foreign_asset \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9910" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X1": { "GlobalConsensus": "Wococo" } } }')" \
          "$GLOBAL_CONSENSUS_WOCOCO_SOVEREIGN_ACCOUNT" \
          10000000000 \
          true
      # drip SA which holds reserves
      transfer_balance \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$GLOBAL_CONSENSUS_WOCOCO_ASSET_HUB_WOCOCO_1000_SOVEREIGN_ACCOUNT" \
          $((1000000000 + 50000000000 * 20))
      # HRMP
      open_hrmp_channels \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 1013 4 524288
      open_hrmp_channels \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1013 1000 4 524288
      ;;
  init-bridge-hub-rococo-local)
      ensure_polkadot_js_api
      # SA of sibling asset hub pays for the execution
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO" \
          $((1000000000 + 50000000000 * 20))
      # drip SA of lane dedicated to asset hub for paying rewards for delivery
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_ThisChain" \
          $((1000000000 + 2000000000000))
      # drip SA of lane dedicated to asset hub for paying rewards for delivery confirmation
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhwo_BridgedChain" \
          $((1000000000 + 2000000000000))
      ;;
  init-asset-hub-wococo-local)
      ensure_polkadot_js_api
      # set Wococo flavor - set_storage with:
      # - `key` is `HexDisplay::from(&asset_hub_rococo_runtime::xcm_config::Flavor::key())`
      # - `value` is `HexDisplay::from(&asset_hub_rococo_runtime::RuntimeFlavor::Wococo.encode())`
      set_storage \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9010" \
          "$(jq --null-input '[["0x48297505634037ef48c848c99c0b1f1b", "0x01"]]')"
      # create foreign assets for native Rococo token (governance call on Wococo)
      force_create_foreign_asset \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9010" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X1": { "GlobalConsensus": "Rococo" } } }')" \
          "$GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT" \
          10000000000 \
          true
      # drip SA which holds reserves
      transfer_balance \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$GLOBAL_CONSENSUS_ROCOCO_ASSET_HUB_ROCOCO_1000_SOVEREIGN_ACCOUNT" \
          $((1000000000 + 50000000000 * 20))
      # HRMP
      open_hrmp_channels \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 1014 4 524288
      open_hrmp_channels \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1014 1000 4 524288
      ;;
  init-bridge-hub-wococo-local)
      # set Wococo flavor - set_storage with:
      # - `key` is `HexDisplay::from(&bridge_hub_rococo_runtime::xcm_config::Flavor::key())`
      # - `value` is `HexDisplay::from(&bridge_hub_rococo_runtime::RuntimeFlavor::Wococo.encode())`
      set_storage \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1014 \
          "ws://127.0.0.1:8945" \
          "$(jq --null-input '[["0x48297505634037ef48c848c99c0b1f1b", "0x01"]]')"
      # SA of sibling asset hub pays for the execution
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ASSET_HUB_WOCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WOCOCO" \
          $((1000000000 + 50000000000 * 20))
      # drip SA of lane dedicated to asset hub for paying rewards for delivery
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_ThisChain" \
          $((1000000000 + 2000000000000))
      # drip SA of lane dedicated to asset hub for paying rewards for delivery confirmation
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ON_BRIDGE_HUB_WOCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000001_bhro_BridgedChain" \
          $((1000000000 + 2000000000000))
      ;;
  reserve-transfer-assets-from-asset-hub-rococo-local)
      ensure_polkadot_js_api
      # send ROCs to Alice account on AHW
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Wococo" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": 200000000000 } } ] }')" \
          0 \
          "Unlimited"
      ;;
  reserve-transfer-assets-from-asset-hub-wococo-local)
      ensure_polkadot_js_api
      # send WOCs to Alice account on AHR
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Rococo" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": 150000000000 } } ] }')" \
          0 \
          "Unlimited"
      ;;
  claim-rewards-bridge-hub-rococo-local)
      ensure_polkadot_js_api
      # bhwo -> [62, 68, 77, 6f] -> 0x6268776f
      claim_rewards \
          "ws://127.0.0.1:8943" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268776f" \
          "ThisChain"
      claim_rewards \
          "ws://127.0.0.1:8943" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268776f" \
          "BridgedChain"
      ;;
  claim-rewards-bridge-hub-wococo-local)
      # bhro -> [62, 68, 72, 6f] -> 0x6268726f
      claim_rewards \
          "ws://127.0.0.1:8945" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268726f" \
          "ThisChain"
      claim_rewards \
          "ws://127.0.0.1:8945" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268726f" \
          "BridgedChain"
      ;;
  stop)
    pkill -f polkadot
    pkill -f parachain
    ;;
  import)
    # to avoid trigger anything here
    ;;
  *)
    echo "A command is require. Supported commands for:
    Local (zombienet) run:
          - run-relay
          - init-asset-hub-rococo-local
          - init-bridge-hub-rococo-local
          - init-asset-hub-wococo-local
          - init-bridge-hub-wococo-local
          - reserve-transfer-assets-from-asset-hub-rococo-local
          - reserve-transfer-assets-from-asset-hub-wococo-local
          - claim-rewards-bridge-hub-rococo-local
          - claim-rewards-bridge-hub-wococo-local";
    exit 1
    ;;
esac
