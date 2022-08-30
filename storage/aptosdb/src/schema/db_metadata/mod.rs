// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
//! This module defines the physical storage schema for indexes of min_readable_version of pruners.
//! For the state pruner, the metadata represents the key of the StaleNodeIndexSchema and for the
//! ledger pruner, the metadata represents the key of the TransactionSchema.
//!
//! ```text
//! |<------key---->|<------ value ----->|
//! | pruner tag    | pruned until values|
//! ```
//!

use crate::schema::DB_METADATA_CF_NAME;
use anyhow::{anyhow, Result};
use aptos_types::transaction::Version;
use byteorder::ReadBytesExt;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use schemadb::{
    define_schema,
    schema::{KeyCodec, ValueCodec},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
pub(crate) enum DbMetadata {
    LatestVersion(Version),
}

#[derive(Clone, Debug, Deserialize, FromPrimitive, PartialEq, ToPrimitive, Serialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum DbMetadataTag {
    LedgerPrunerProgress = 0,
    StateMerklePrunerProgress = 1,
    EpochEndingStateMerklePrunerProgress = 2,
}

define_schema!(
    DbMetadataSchema,
    DbMetadataTag,
    DbMetadata,
    DB_METADATA_CF_NAME
);

impl KeyCodec<DbMetadataSchema> for DbMetadataTag {
    fn encode_key(&self) -> Result<Vec<u8>> {
        Ok(vec![self.to_u8().unwrap()])
    }

    fn decode_key(mut data: &[u8]) -> Result<Self> {
        Self::from_u8(data.read_u8()?).ok_or_else(|| anyhow!("Failed to parse PrunerTag."))
    }
}

impl ValueCodec<DbMetadataSchema> for DbMetadata {
    fn encode_value(&self) -> Result<Vec<u8>> {
        Ok(bcs::to_bytes(self)?)
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        Ok(bcs::from_bytes(data)?)
    }
}

#[cfg(test)]
mod test;
