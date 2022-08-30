// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use aptos_types::transaction::Version;
use std::collections::HashMap;

/// Key-Value batch that will be written into db atomically with other batches.
pub type StateValueBatch<K, V> = HashMap<(K, Version), V>;

pub trait StateValueWriter<K, V>: Send + Sync {
    /// Writes a kv batch into storage.
    fn write_kv_batch(&self, kv_batch: &StateValueBatch<K, Option<V>>) -> Result<()>;

    fn write_usage(&self, version: Version, items: usize, total_bytes: usize) -> Result<()>;
}
