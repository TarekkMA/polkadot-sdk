// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-yaoqqom-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_bridge_messages
// --chain=bridge-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658`
		//  Estimated: `52645`
		// Minimum execution time: 39_412_000 picoseconds.
		Weight::from_parts(41_134_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4076]`.
	/// The range of component `n` is `[1, 4076]`.
	fn receive_n_messages_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658`
		//  Estimated: `52645`
		// Minimum execution time: 39_723_000 picoseconds.
		Weight::from_parts(40_576_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 6_229
			.saturating_add(Weight::from_parts(9_728_848, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658`
		//  Estimated: `52645`
		// Minimum execution time: 47_676_000 picoseconds.
		Weight::from_parts(50_641_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658`
		//  Estimated: `52645`
		// Minimum execution time: 38_966_000 picoseconds.
		Weight::from_parts(41_584_685, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 16
			.saturating_add(Weight::from_parts(2_150, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501`
		//  Estimated: `3966`
		// Minimum execution time: 31_678_000 picoseconds.
		Weight::from_parts(32_448_000, 0)
			.saturating_add(Weight::from_parts(0, 3966))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501`
		//  Estimated: `3966`
		// Minimum execution time: 31_454_000 picoseconds.
		Weight::from_parts(32_435_000, 0)
			.saturating_add(Weight::from_parts(0, 3966))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501`
		//  Estimated: `6086`
		// Minimum execution time: 35_656_000 picoseconds.
		Weight::from_parts(36_745_000, 0)
			.saturating_add(Weight::from_parts(0, 6086))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: Some(105506), added: 107981, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof_with_dispatch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `52645`
		// Minimum execution time: 56_644_000 picoseconds.
		Weight::from_parts(61_155_733, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 14
			.saturating_add(Weight::from_parts(7_525, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
