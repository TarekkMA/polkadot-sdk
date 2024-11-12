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

//! Implementation of the `archive_storage` method.

use std::{
	collections::{hash_map::Entry, HashMap},
	sync::Arc,
};

use itertools::Itertools;
use sc_client_api::{Backend, ChildInfo, StorageKey, StorageProvider};
use sp_runtime::traits::Block as BlockT;

use super::error::Error as ArchiveError;
use crate::{
	archive::archive::LOG_TARGET,
	common::{
		events::{
			ArchiveStorageDiffEvent, ArchiveStorageDiffItem, ArchiveStorageDiffOperationType,
			ArchiveStorageDiffResult, ArchiveStorageDiffType, ArchiveStorageResult,
			PaginatedStorageQuery, StorageQueryType, StorageResult,
		},
		storage::{IterQueryType, QueryIter, Storage},
	},
};
use tokio::sync::mpsc;

/// Generates the events of the `archive_storage` method.
pub struct ArchiveStorage<Client, Block, BE> {
	/// Storage client.
	client: Storage<Client, Block, BE>,
	/// The maximum number of responses the API can return for a descendant query at a time.
	storage_max_descendant_responses: usize,
	/// The maximum number of queried items allowed for the `archive_storage` at a time.
	storage_max_queried_items: usize,
}

impl<Client, Block, BE> ArchiveStorage<Client, Block, BE> {
	/// Constructs a new [`ArchiveStorage`].
	pub fn new(
		client: Arc<Client>,
		storage_max_descendant_responses: usize,
		storage_max_queried_items: usize,
	) -> Self {
		Self {
			client: Storage::new(client),
			storage_max_descendant_responses,
			storage_max_queried_items,
		}
	}
}

impl<Client, Block, BE> ArchiveStorage<Client, Block, BE>
where
	Block: BlockT + 'static,
	BE: Backend<Block> + 'static,
	Client: StorageProvider<Block, BE> + 'static,
{
	/// Generate the response of the `archive_storage` method.
	pub fn handle_query(
		&self,
		hash: Block::Hash,
		mut items: Vec<PaginatedStorageQuery<StorageKey>>,
		child_key: Option<ChildInfo>,
	) -> ArchiveStorageResult {
		let discarded_items = items.len().saturating_sub(self.storage_max_queried_items);
		items.truncate(self.storage_max_queried_items);

		let mut storage_results = Vec::with_capacity(items.len());
		for item in items {
			match item.query_type {
				StorageQueryType::Value => {
					match self.client.query_value(hash, &item.key, child_key.as_ref()) {
						Ok(Some(value)) => storage_results.push(value),
						Ok(None) => continue,
						Err(error) => return ArchiveStorageResult::err(error),
					}
				},
				StorageQueryType::Hash =>
					match self.client.query_hash(hash, &item.key, child_key.as_ref()) {
						Ok(Some(value)) => storage_results.push(value),
						Ok(None) => continue,
						Err(error) => return ArchiveStorageResult::err(error),
					},
				StorageQueryType::ClosestDescendantMerkleValue =>
					match self.client.query_merkle_value(hash, &item.key, child_key.as_ref()) {
						Ok(Some(value)) => storage_results.push(value),
						Ok(None) => continue,
						Err(error) => return ArchiveStorageResult::err(error),
					},
				StorageQueryType::DescendantsValues => {
					match self.client.query_iter_pagination(
						QueryIter {
							query_key: item.key,
							ty: IterQueryType::Value,
							pagination_start_key: item.pagination_start_key,
						},
						hash,
						child_key.as_ref(),
						self.storage_max_descendant_responses,
					) {
						Ok((results, _)) => storage_results.extend(results),
						Err(error) => return ArchiveStorageResult::err(error),
					}
				},
				StorageQueryType::DescendantsHashes => {
					match self.client.query_iter_pagination(
						QueryIter {
							query_key: item.key,
							ty: IterQueryType::Hash,
							pagination_start_key: item.pagination_start_key,
						},
						hash,
						child_key.as_ref(),
						self.storage_max_descendant_responses,
					) {
						Ok((results, _)) => storage_results.extend(results),
						Err(error) => return ArchiveStorageResult::err(error),
					}
				},
			};
		}

		ArchiveStorageResult::ok(storage_results, discarded_items)
	}
}

/// Parse hex-encoded string parameter as raw bytes.
///
/// If the parsing fails, returns an error propagated to the RPC method.
pub fn parse_hex_param(param: String) -> Result<Vec<u8>, ArchiveError> {
	// Methods can accept empty parameters.
	if param.is_empty() {
		return Ok(Default::default())
	}

	array_bytes::hex2bytes(&param).map_err(|_| ArchiveError::InvalidParam(param))
}

#[derive(Debug, PartialEq, Clone)]
pub struct DiffDetails {
	key: StorageKey,
	return_type: ArchiveStorageDiffType,
	child_trie_key: Option<ChildInfo>,
	child_trie_key_string: Option<String>,
}

/// The type of storage query.
#[derive(Debug, PartialEq, Clone, Copy)]
enum FetchStorageType {
	/// Only fetch the value.
	Value,
	/// Only fetch the hash.
	Hash,
	/// Fetch both the value and the hash.
	Both,
}

/// The return value of the `fetch_storage` method.
#[derive(Debug, PartialEq, Clone)]
enum FetchedStorage {
	/// Storage value under a key.
	Value(StorageResult),
	/// Storage hash under a key.
	Hash(StorageResult),
	/// Both storage value and hash under a key.
	Both { value: StorageResult, hash: StorageResult },
}

pub struct ArchiveStorageDiff<Client, Block, BE> {
	client: Storage<Client, Block, BE>,
}

impl<Client, Block, BE> ArchiveStorageDiff<Client, Block, BE> {
	pub fn new(client: Arc<Client>) -> Self {
		Self { client: Storage::new(client) }
	}
}

impl<Client, Block, BE> ArchiveStorageDiff<Client, Block, BE>
where
	Block: BlockT + 'static,
	BE: Backend<Block> + 'static,
	Client: StorageProvider<Block, BE> + Send + Sync + 'static,
{
	/// Fetch the storage from the given key.
	fn fetch_storage(
		&self,
		hash: Block::Hash,
		key: StorageKey,
		maybe_child_trie: Option<ChildInfo>,
		ty: FetchStorageType,
	) -> Result<Option<FetchedStorage>, String> {
		match ty {
			FetchStorageType::Value => {
				let result = self.client.query_value(hash, &key, maybe_child_trie.as_ref())?;

				Ok(result.map(FetchedStorage::Value))
			},

			FetchStorageType::Hash => {
				let result = self.client.query_hash(hash, &key, maybe_child_trie.as_ref())?;

				Ok(result.map(FetchedStorage::Hash))
			},

			FetchStorageType::Both => {
				let value = self.client.query_value(hash, &key, maybe_child_trie.as_ref())?;
				let Some(value) = value else {
					return Ok(None);
				};

				let hash = self.client.query_hash(hash, &key, maybe_child_trie.as_ref())?;
				let Some(hash) = hash else {
					return Ok(None);
				};

				Ok(Some(FetchedStorage::Both { value, hash }))
			},
		}
	}

	/// Check if the key belongs to the provided query items.
	///
	/// A key belongs to the query items when:
	/// - the provided key is a prefix of the key in the query items.
	/// - the query items are empty.
	///
	/// Returns an optional `FetchStorageType` based on the query items.
	/// If the key does not belong to the query items, returns `None`.
	fn belongs_to_query(key: &StorageKey, items: &[DiffDetails]) -> Option<FetchStorageType> {
		// User has requested all keys, by default this fallbacks to fetching the value.
		if items.is_empty() {
			return Some(FetchStorageType::Value)
		}

		let mut value = false;
		let mut hash = false;

		for item in items {
			if key.as_ref().starts_with(&item.key.as_ref()) {
				match item.return_type {
					ArchiveStorageDiffType::Value => value = true,
					ArchiveStorageDiffType::Hash => hash = true,
				}
			}
		}

		match (value, hash) {
			(true, true) => Some(FetchStorageType::Both),
			(true, false) => Some(FetchStorageType::Value),
			(false, true) => Some(FetchStorageType::Hash),
			(false, false) => None,
		}
	}

	/// Send the provided result to the `tx` sender.
	///
	/// Returns `false` if the sender has been closed.
	fn send_result(
		tx: &mpsc::Sender<ArchiveStorageDiffEvent>,
		result: FetchedStorage,
		operation_type: ArchiveStorageDiffOperationType,
		child_trie_key: Option<String>,
	) -> bool {
		let items = match result {
			FetchedStorage::Value(storage_result) | FetchedStorage::Hash(storage_result) =>
				vec![storage_result],
			FetchedStorage::Both { value, hash } => vec![value, hash],
		};

		for item in items {
			let res = ArchiveStorageDiffEvent::StorageDiff(ArchiveStorageDiffResult {
				key: item.key,
				result: item.result,
				operation_type,
				child_trie_key: child_trie_key.clone(),
			});
			if tx.blocking_send(res).is_err() {
				return false
			}
		}

		true
	}

	fn handle_trie_queries_inner(
		&self,
		hash: Block::Hash,
		previous_hash: Block::Hash,
		items: Vec<DiffDetails>,
		tx: &mpsc::Sender<ArchiveStorageDiffEvent>,
	) -> Result<(), String> {
		// Parse the child trie key as `ChildInfo` and `String`.
		let maybe_child_trie = items.first().and_then(|item| item.child_trie_key.clone());
		let maybe_child_trie_str =
			items.first().and_then(|item| item.child_trie_key_string.clone());

		// Iterator over the current block and previous block
		// at the same time to compare the keys. This approach effectively
		// leverages backpressure to avoid memory consumption.
		let mut keys_iter = self.client.raw_keys_iter(hash, maybe_child_trie.clone())?;
		let mut previous_keys_iter =
			self.client.raw_keys_iter(previous_hash, maybe_child_trie.clone())?;

		let mut lhs = keys_iter.next();
		let mut rhs = previous_keys_iter.next();

		loop {
			// Check if the key was added or deleted or modified based on the
			// lexicographical order of the keys.
			let (operation_type, key) = match (&lhs, &rhs) {
				(Some(lhs_key), Some(rhs_key)) =>
					if lhs_key < rhs_key {
						let key = lhs_key.clone();

						lhs = keys_iter.next();

						(ArchiveStorageDiffOperationType::Added, key)
					} else if lhs_key > rhs_key {
						let key = rhs_key.clone();

						rhs = previous_keys_iter.next();

						(ArchiveStorageDiffOperationType::Deleted, key)
					} else {
						let key = lhs_key.clone();

						lhs = keys_iter.next();
						rhs = previous_keys_iter.next();

						(ArchiveStorageDiffOperationType::Modified, key)
					},
				(Some(lhs_key), None) => {
					let key = lhs_key.clone();

					lhs = keys_iter.next();

					(ArchiveStorageDiffOperationType::Added, key)
				},
				(None, Some(rhs_key)) => {
					let key = rhs_key.clone();

					rhs = previous_keys_iter.next();

					(ArchiveStorageDiffOperationType::Deleted, key)
				},
				(None, None) => break,
			};

			let Some(fetch_type) = Self::belongs_to_query(&key, &items) else {
				// The key does not belong the the query items.
				continue;
			};

			let maybe_result = match operation_type {
				ArchiveStorageDiffOperationType::Added =>
					self.fetch_storage(hash, key.clone(), maybe_child_trie.clone(), fetch_type)?,
				ArchiveStorageDiffOperationType::Deleted => self.fetch_storage(
					previous_hash,
					key.clone(),
					maybe_child_trie.clone(),
					fetch_type,
				)?,
				ArchiveStorageDiffOperationType::Modified => {
					let Some(storage_result) = self.fetch_storage(
						hash,
						key.clone(),
						maybe_child_trie.clone(),
						fetch_type,
					)?
					else {
						continue
					};

					let Some(previous_storage_result) = self.fetch_storage(
						previous_hash,
						key.clone(),
						maybe_child_trie.clone(),
						fetch_type,
					)?
					else {
						continue
					};

					// For modified records we need to check the actual storage values.
					if storage_result == previous_storage_result {
						continue
					}

					Some(storage_result)
				},
			};

			if let Some(storage_result) = maybe_result {
				if !Self::send_result(
					&tx,
					storage_result,
					operation_type,
					maybe_child_trie_str.clone(),
				) {
					return Ok(())
				}
			}
		}

		Ok(())
	}

	/// The items provided to this method are obtained by calling `deduplicate_storage_diff_items`.
	/// The deduplication method ensures that all items `Vec<DiffDetails>` correspond to the same
	/// `child_trie_key`.
	///
	/// This method will iterate over the keys of the main trie or a child trie and fetch the
	/// given keys. The fetched keys will be sent to the provided `tx` sender to leverage
	/// the backpressure mechanism.
	pub async fn handle_trie_queries(
		&self,
		hash: Block::Hash,
		previous_hash: Block::Hash,
		trie_queries: Vec<Vec<DiffDetails>>,
		tx: mpsc::Sender<ArchiveStorageDiffEvent>,
	) -> Result<(), tokio::task::JoinError> {
		let this = ArchiveStorageDiff { client: self.client.clone() };

		tokio::task::spawn_blocking(move || {
			for items in trie_queries {
				log::trace!(
					target: LOG_TARGET,
					"handle_trie_queries: hash={:?}, previous_hash={:?}, items={:?}",
					hash,
					previous_hash,
					items
				);

				let result = this.handle_trie_queries_inner(hash, previous_hash, items, &tx);

				if let Err(error) = result {
					log::trace!(
						target: LOG_TARGET,
						"handle_trie_queries: sending error={:?}",
						error,
					);

					let _ = tx.blocking_send(ArchiveStorageDiffEvent::err(error));

					return
				} else {
					log::trace!(
						target: LOG_TARGET,
						"handle_trie_queries: sending storage diff done",
					);
				}
			}

			let _ = tx.blocking_send(ArchiveStorageDiffEvent::StorageDiffDone);
		})
		.await?;

		Ok(())
	}
}

/// Deduplicate the provided items and return a list of `DiffDetails`.
///
/// Each list corresponds to a single child trie or the main trie.
pub fn deduplicate_storage_diff_items(
	items: Vec<ArchiveStorageDiffItem<String>>,
) -> Result<Vec<Vec<DiffDetails>>, ArchiveError> {
	let mut deduplicated: HashMap<Option<ChildInfo>, Vec<DiffDetails>> = HashMap::new();

	for diff_item in items {
		// Ensure the provided hex keys are valid before deduplication.
		let key = StorageKey(parse_hex_param(diff_item.key)?);
		let child_trie_key_string = diff_item.child_trie_key.clone();
		let child_trie_key = diff_item
			.child_trie_key
			.map(|child_trie_key| parse_hex_param(child_trie_key))
			.transpose()?
			.map(ChildInfo::new_default_from_vec);

		let diff_item = DiffDetails {
			key,
			return_type: diff_item.return_type,
			child_trie_key: child_trie_key.clone(),
			child_trie_key_string,
		};

		match deduplicated.entry(child_trie_key.clone()) {
			Entry::Occupied(mut entry) => {
				let mut should_insert = true;

				for existing in entry.get() {
					// This points to a different return type.
					if existing.return_type != diff_item.return_type {
						continue
					}
					// Keys and return types are identical.
					if existing.key == diff_item.key {
						should_insert = false;
						break
					}

					// The following two conditions ensure that we keep the shortest key.

					// The current key is a longer prefix of the existing key.
					if diff_item.key.as_ref().starts_with(&existing.key.as_ref()) {
						should_insert = false;
						break
					}

					// The existing key is a longer prefix of the current key.
					// We need to keep the current key and remove the existing one.
					if existing.key.as_ref().starts_with(&diff_item.key.as_ref()) {
						let to_remove = existing.clone();
						entry.get_mut().retain(|item| item != &to_remove);
						break;
					}
				}

				if should_insert {
					entry.get_mut().push(diff_item);
				}
			},
			Entry::Vacant(entry) => {
				entry.insert(vec![diff_item]);
			},
		}
	}

	Ok(deduplicated
		.into_iter()
		.sorted_by_key(|(child_trie_key, _)| child_trie_key.clone())
		.map(|(_, values)| values)
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dedup_empty() {
		let items = vec![];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert!(result.is_empty());
	}

	#[test]
	fn dedup_single() {
		let items = vec![ArchiveStorageDiffItem {
			key: "0x01".into(),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: None,
		}];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 1);

		let expected = DiffDetails {
			key: StorageKey(vec![1]),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: None,
			child_trie_key_string: None,
		};
		assert_eq!(result[0][0], expected);
	}

	#[test]
	fn dedup_with_different_keys() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x02".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 2);

		let expected = vec![
			DiffDetails {
				key: StorageKey(vec![1]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
				child_trie_key_string: None,
			},
			DiffDetails {
				key: StorageKey(vec![2]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
				child_trie_key_string: None,
			},
		];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_with_same_keys() {
		// Identical keys.
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 1);

		let expected = vec![DiffDetails {
			key: StorageKey(vec![1]),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: None,
			child_trie_key_string: None,
		}];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_with_same_prefix() {
		// Identical keys.
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x01ff".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 1);

		let expected = vec![DiffDetails {
			key: StorageKey(vec![1]),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: None,
			child_trie_key_string: None,
		}];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_with_different_return_types() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Hash,
				child_trie_key: None,
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 2);

		let expected = vec![
			DiffDetails {
				key: StorageKey(vec![1]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
				child_trie_key_string: None,
			},
			DiffDetails {
				key: StorageKey(vec![1]),
				return_type: ArchiveStorageDiffType::Hash,
				child_trie_key: None,
				child_trie_key_string: None,
			},
		];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_with_different_child_tries() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x01".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x02".into()),
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].len(), 1);
		assert_eq!(result[1].len(), 1);

		let expected = vec![
			vec![DiffDetails {
				key: StorageKey(vec![1]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some(ChildInfo::new_default_from_vec(vec![1])),
				child_trie_key_string: Some("0x01".into()),
			}],
			vec![DiffDetails {
				key: StorageKey(vec![1]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some(ChildInfo::new_default_from_vec(vec![2])),
				child_trie_key_string: Some("0x02".into()),
			}],
		];
		assert_eq!(result, expected);
	}

	#[test]
	fn dedup_with_same_child_tries() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x01".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x01".into()),
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 1);

		let expected = vec![DiffDetails {
			key: StorageKey(vec![1]),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: Some(ChildInfo::new_default_from_vec(vec![1])),
			child_trie_key_string: Some("0x01".into()),
		}];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_with_shorter_key_reverse_order() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x01ff".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
		];
		let result = deduplicate_storage_diff_items(items).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].len(), 1);

		let expected = vec![DiffDetails {
			key: StorageKey(vec![1]),
			return_type: ArchiveStorageDiffType::Value,
			child_trie_key: None,
			child_trie_key_string: None,
		}];
		assert_eq!(result[0], expected);
	}

	#[test]
	fn dedup_multiple_child_tries() {
		let items = vec![
			ArchiveStorageDiffItem {
				key: "0x02".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x01".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x02".into(),
				return_type: ArchiveStorageDiffType::Hash,
				child_trie_key: Some("0x01".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x02".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x01".into(),
				return_type: ArchiveStorageDiffType::Hash,
				child_trie_key: Some("0x02".into()),
			},
			ArchiveStorageDiffItem {
				key: "0x01ff".into(),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: Some("0x02".into()),
			},
		];

		let result = deduplicate_storage_diff_items(items).unwrap();

		let expected = vec![
			vec![DiffDetails {
				key: StorageKey(vec![2]),
				return_type: ArchiveStorageDiffType::Value,
				child_trie_key: None,
				child_trie_key_string: None,
			}],
			vec![
				DiffDetails {
					key: StorageKey(vec![1]),
					return_type: ArchiveStorageDiffType::Value,
					child_trie_key: Some(ChildInfo::new_default_from_vec(vec![1])),
					child_trie_key_string: Some("0x01".into()),
				},
				DiffDetails {
					key: StorageKey(vec![2]),
					return_type: ArchiveStorageDiffType::Hash,
					child_trie_key: Some(ChildInfo::new_default_from_vec(vec![1])),
					child_trie_key_string: Some("0x01".into()),
				},
			],
			vec![
				DiffDetails {
					key: StorageKey(vec![1]),
					return_type: ArchiveStorageDiffType::Value,
					child_trie_key: Some(ChildInfo::new_default_from_vec(vec![2])),
					child_trie_key_string: Some("0x02".into()),
				},
				DiffDetails {
					key: StorageKey(vec![1]),
					return_type: ArchiveStorageDiffType::Hash,
					child_trie_key: Some(ChildInfo::new_default_from_vec(vec![2])),
					child_trie_key_string: Some("0x02".into()),
				},
			],
		];

		assert_eq!(result, expected);
	}
}
