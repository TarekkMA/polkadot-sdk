// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! API implementation for `archive`.

use crate::{
	archive::{
		archive_storage::{deduplicate_storage_diff_items, ArchiveStorage, ArchiveStorageDiff},
		error::Error as ArchiveError,
		ArchiveApiServer,
	},
	common::events::{
		ArchiveStorageDiffEvent, ArchiveStorageDiffItem, ArchiveStorageResult,
		PaginatedStorageQuery,
	},
	hex_string, MethodResult, SubscriptionTaskExecutor,
};

use codec::Encode;
use futures::FutureExt;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	PendingSubscriptionSink,
};
use sc_client_api::{
	Backend, BlockBackend, BlockchainEvents, CallExecutor, ChildInfo, ExecutorProvider, StorageKey,
	StorageProvider,
};
use sc_rpc::utils::Subscription;
use sp_api::{CallApiAt, CallContext};
use sp_blockchain::{
	Backend as BlockChainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_core::{Bytes, U256};
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT, NumberFor},
	SaturatedConversion,
};
use std::{collections::HashSet, marker::PhantomData, sync::Arc};

use tokio::sync::mpsc;

/// The configuration of [`Archive`].
pub struct ArchiveConfig {
	/// The maximum number of items the `archive_storage` can return for a descendant query before
	/// pagination is required.
	pub max_descendant_responses: usize,
	/// The maximum number of queried items allowed for the `archive_storage` at a time.
	pub max_queried_items: usize,
}

/// The maximum number of items the `archive_storage` can return for a descendant query before
/// pagination is required.
///
/// Note: this is identical to the `chainHead` value.
const MAX_DESCENDANT_RESPONSES: usize = 5;

/// The maximum number of queried items allowed for the `archive_storage` at a time.
///
/// Note: A queried item can also be a descendant query which can return up to
/// `MAX_DESCENDANT_RESPONSES`.
const MAX_QUERIED_ITEMS: usize = 8;

/// The buffer capacity for each storage query.
///
/// This is small because the underlying JSON-RPC server has
/// its down buffer capacity per connection as well.
const STORAGE_QUERY_BUF: usize = 16;

impl Default for ArchiveConfig {
	fn default() -> Self {
		Self {
			max_descendant_responses: MAX_DESCENDANT_RESPONSES,
			max_queried_items: MAX_QUERIED_ITEMS,
		}
	}
}

/// An API for archive RPC calls.
pub struct Archive<BE: Backend<Block>, Block: BlockT, Client> {
	/// Substrate client.
	client: Arc<Client>,
	/// Backend of the chain.
	backend: Arc<BE>,
	/// Executor to spawn subscriptions.
	executor: SubscriptionTaskExecutor,
	/// The hexadecimal encoded hash of the genesis block.
	genesis_hash: String,
	/// The maximum number of items the `archive_storage` can return for a descendant query before
	/// pagination is required.
	storage_max_descendant_responses: usize,
	/// The maximum number of queried items allowed for the `archive_storage` at a time.
	storage_max_queried_items: usize,
	/// Phantom member to pin the block type.
	_phantom: PhantomData<Block>,
}

impl<BE: Backend<Block>, Block: BlockT, Client> Archive<BE, Block, Client> {
	/// Create a new [`Archive`].
	pub fn new<GenesisHash: AsRef<[u8]>>(
		client: Arc<Client>,
		backend: Arc<BE>,
		genesis_hash: GenesisHash,
		executor: SubscriptionTaskExecutor,
		config: ArchiveConfig,
	) -> Self {
		let genesis_hash = hex_string(&genesis_hash.as_ref());
		Self {
			client,
			backend,
			executor,
			genesis_hash,
			storage_max_descendant_responses: config.max_descendant_responses,
			storage_max_queried_items: config.max_queried_items,
			_phantom: PhantomData,
		}
	}
}

/// Parse hex-encoded string parameter as raw bytes.
///
/// If the parsing fails, returns an error propagated to the RPC method.
fn parse_hex_param(param: String) -> Result<Vec<u8>, ArchiveError> {
	// Methods can accept empty parameters.
	if param.is_empty() {
		return Ok(Default::default())
	}

	array_bytes::hex2bytes(&param).map_err(|_| ArchiveError::InvalidParam(param))
}

#[async_trait]
impl<BE, Block, Client> ArchiveApiServer<Block::Hash> for Archive<BE, Block, Client>
where
	Block: BlockT + 'static,
	Block::Header: Unpin,
	BE: Backend<Block> + 'static,
	Client: BlockBackend<Block>
		+ ExecutorProvider<Block>
		+ HeaderBackend<Block>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ CallApiAt<Block>
		+ StorageProvider<Block, BE>
		+ 'static,
{
	fn archive_unstable_body(&self, hash: Block::Hash) -> RpcResult<Option<Vec<String>>> {
		let Ok(Some(signed_block)) = self.client.block(hash) else { return Ok(None) };

		let extrinsics = signed_block
			.block
			.extrinsics()
			.iter()
			.map(|extrinsic| hex_string(&extrinsic.encode()))
			.collect();

		Ok(Some(extrinsics))
	}

	fn archive_unstable_genesis_hash(&self) -> RpcResult<String> {
		Ok(self.genesis_hash.clone())
	}

	fn archive_unstable_header(&self, hash: Block::Hash) -> RpcResult<Option<String>> {
		let Ok(Some(header)) = self.client.header(hash) else { return Ok(None) };

		Ok(Some(hex_string(&header.encode())))
	}

	fn archive_unstable_finalized_height(&self) -> RpcResult<u64> {
		Ok(self.client.info().finalized_number.saturated_into())
	}

	fn archive_unstable_hash_by_height(&self, height: u64) -> RpcResult<Vec<String>> {
		let height: NumberFor<Block> = U256::from(height)
			.try_into()
			.map_err(|_| ArchiveError::InvalidParam(format!("Invalid block height: {}", height)))?;

		let finalized_num = self.client.info().finalized_number;

		if finalized_num >= height {
			let Ok(Some(hash)) = self.client.block_hash(height) else { return Ok(vec![]) };
			return Ok(vec![hex_string(&hash.as_ref())])
		}

		let blockchain = self.backend.blockchain();
		// Fetch all the leaves of the blockchain that are on a higher or equal height.
		let mut headers: Vec<_> = blockchain
			.leaves()
			.map_err(|error| ArchiveError::FetchLeaves(error.to_string()))?
			.into_iter()
			.filter_map(|hash| {
				let Ok(Some(header)) = self.client.header(hash) else { return None };

				if header.number() < &height {
					return None
				}

				Some(header)
			})
			.collect();

		let mut result = Vec::new();
		let mut visited = HashSet::new();

		while let Some(header) = headers.pop() {
			if header.number() == &height {
				result.push(hex_string(&header.hash().as_ref()));
				continue
			}

			let parent_hash = *header.parent_hash();

			// Continue the iteration for unique hashes.
			// Forks might intersect on a common chain that is not yet finalized.
			if visited.insert(parent_hash) {
				let Ok(Some(next_header)) = self.client.header(parent_hash) else { continue };
				headers.push(next_header);
			}
		}

		Ok(result)
	}

	fn archive_unstable_call(
		&self,
		hash: Block::Hash,
		function: String,
		call_parameters: String,
	) -> RpcResult<MethodResult> {
		let call_parameters = Bytes::from(parse_hex_param(call_parameters)?);

		let result =
			self.client
				.executor()
				.call(hash, &function, &call_parameters, CallContext::Offchain);

		Ok(match result {
			Ok(result) => MethodResult::ok(hex_string(&result)),
			Err(error) => MethodResult::err(error.to_string()),
		})
	}

	fn archive_unstable_storage(
		&self,
		hash: Block::Hash,
		items: Vec<PaginatedStorageQuery<String>>,
		child_trie: Option<String>,
	) -> RpcResult<ArchiveStorageResult> {
		let items = items
			.into_iter()
			.map(|query| {
				let key = StorageKey(parse_hex_param(query.key)?);
				let pagination_start_key = query
					.pagination_start_key
					.map(|key| parse_hex_param(key).map(|key| StorageKey(key)))
					.transpose()?;

				// Paginated start key is only supported
				if pagination_start_key.is_some() && !query.query_type.is_descendant_query() {
					return Err(ArchiveError::InvalidParam(
						"Pagination start key is only supported for descendants queries"
							.to_string(),
					))
				}

				Ok(PaginatedStorageQuery {
					key,
					query_type: query.query_type,
					pagination_start_key,
				})
			})
			.collect::<Result<Vec<_>, ArchiveError>>()?;

		let child_trie = child_trie
			.map(|child_trie| parse_hex_param(child_trie))
			.transpose()?
			.map(ChildInfo::new_default_from_vec);

		let storage_client = ArchiveStorage::new(
			self.client.clone(),
			self.storage_max_descendant_responses,
			self.storage_max_queried_items,
		);

		Ok(storage_client.handle_query(hash, items, child_trie))
	}

	fn archive_unstable_storage_diff(
		&self,
		pending: PendingSubscriptionSink,
		hash: Block::Hash,
		previous_hash: Option<Block::Hash>,
		items: Vec<ArchiveStorageDiffItem<String>>,
	) {
		let storage_client = ArchiveStorageDiff::new(self.client.clone());
		let client = self.client.clone();

		let fut = async move {
			// Deduplicate the items.
			let mut trie_items = match deduplicate_storage_diff_items(items) {
				Ok(items) => items,
				Err(error) => {
					pending.reject(error).await;
					return
				},
			};
			// Default to using the main storage trie if no items are provided.
			if trie_items.is_empty() {
				trie_items.push(Vec::new());
			}

			let previous_hash = if let Some(previous_hash) = previous_hash {
				previous_hash
			} else {
				let Ok(Some(current_header)) = client.header(hash) else {
					pending
						.reject(ArchiveError::InvalidParam(format!(
							"Block header is not present: {}",
							hash
						)))
						.await;

					return
				};
				*current_header.parent_hash()
			};

			let Ok(mut sink) = pending.accept().await.map(Subscription::from) else { return };
			let (tx, mut rx) = tokio::sync::mpsc::channel(STORAGE_QUERY_BUF);
			for trie_queries in trie_items {
				let storage_fut = storage_client.handle_trie_queries(
					hash,
					previous_hash,
					trie_queries,
					tx.clone(),
				);
				let result =
					futures::future::join(storage_fut, process_events(&mut rx, &mut sink)).await;
				if !result.1 {
					return;
				}
			}

			let _ = sink.send(&ArchiveStorageDiffEvent::StorageDiffDone).await;
		};

		self.executor.spawn("substrate-rpc-subscription", Some("rpc"), fut.boxed());
	}
}

/// Returns true if the events where processed successfully, false otherwise.
async fn process_events(
	rx: &mut mpsc::Receiver<ArchiveStorageDiffEvent>,
	sink: &mut Subscription,
) -> bool {
	while let Some(event) = rx.recv().await {
		let is_error_event = std::matches!(event, ArchiveStorageDiffEvent::StorageDiffError(_));

		if let Err(_) = sink.send(&event).await {
			return false
		}

		if is_error_event {
			// Stop further processing if an error event is received.
			return false
		}
	}

	true
}
