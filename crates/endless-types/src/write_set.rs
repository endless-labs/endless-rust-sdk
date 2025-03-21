// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! For each transaction the VM executes, the VM will output a `WriteSet` that contains each access
//! path it updates. For each access path, the VM can either give its new value or delete it.

use crate::{
    state_store::{
        state_key::StateKey,
        state_value::{PersistedStateValueMetadata, StateValue, StateValueMetadata},
    },
    write_set::WriteOp::{Creation, Deletion, Modification},
};
use anyhow::{bail, ensure, Result};
use bytes::Bytes;
use endless_crypto_derive::{BCSCryptoHash, CryptoHasher};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{btree_map, BTreeMap},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

// Note: in case this changes in the future, it doesn't have to be a constant, and can be read from
// genesis directly if necessary.
pub static TOTAL_SUPPLY_STATE_KEY: Lazy<StateKey> = Lazy::new(|| {
    StateKey::table_item(
        "1b854694ae746cdbd8d44186ca4929b2b337df21d1c74633be19b2710552fdca"
            .parse()
            .unwrap(),
        vec![
            6, 25, 220, 41, 160, 170, 200, 250, 20, 103, 20, 5, 142, 141, 214, 210, 208, 243, 189,
            245, 246, 51, 25, 7, 191, 145, 243, 172, 216, 30, 105, 53,
        ],
    )
});

#[derive(Eq, Clone, Debug, PartialEq)]
pub enum WriteOpKind {
    Creation,
    Modification,
    Deletion,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "WriteOp")]
enum PersistedWriteOp {
    Creation(Bytes),
    Modification(Bytes),
    Deletion,
    CreationWithMetadata {
        data: Bytes,
        metadata: PersistedStateValueMetadata,
    },
    ModificationWithMetadata {
        data: Bytes,
        metadata: PersistedStateValueMetadata,
    },
    DeletionWithMetadata {
        metadata: PersistedStateValueMetadata,
    },
}

impl PersistedWriteOp {
    fn into_in_mem_form(self) -> WriteOp {
        use PersistedWriteOp::*;

        match self {
            Creation(data) => WriteOp::Creation {
                data,
                metadata: StateValueMetadata::none(),
            },
            Modification(data) => WriteOp::Modification {
                data,
                metadata: StateValueMetadata::none(),
            },
            Deletion => WriteOp::Deletion {
                metadata: StateValueMetadata::none(),
            },
            CreationWithMetadata { data, metadata } => WriteOp::Creation {
                data,
                metadata: metadata.into_in_mem_form(),
            },
            ModificationWithMetadata { data, metadata } => WriteOp::Modification {
                data,
                metadata: metadata.into_in_mem_form(),
            },
            DeletionWithMetadata { metadata } => WriteOp::Deletion {
                metadata: metadata.into_in_mem_form(),
            },
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum WriteOp {
    Creation {
        data: Bytes,
        metadata: StateValueMetadata,
    },
    Modification {
        data: Bytes,
        metadata: StateValueMetadata,
    },
    Deletion {
        metadata: StateValueMetadata,
    },
}

impl WriteOp {
    fn to_persistable(&self) -> PersistedWriteOp {
        use PersistedWriteOp::*;

        let metadata = self.metadata().clone().into_persistable();
        match metadata {
            None => match self {
                WriteOp::Creation { data, .. } => Creation(data.clone()),
                WriteOp::Modification { data, .. } => Modification(data.clone()),
                WriteOp::Deletion { .. } => Deletion,
            },
            Some(metadata) => match self {
                WriteOp::Creation { data, .. } => CreationWithMetadata {
                    data: data.clone(),
                    metadata,
                },
                WriteOp::Modification { data, .. } => ModificationWithMetadata {
                    data: data.clone(),
                    metadata,
                },
                WriteOp::Deletion { .. } => DeletionWithMetadata { metadata },
            },
        }
    }

    /// Merges two write ops on the same state item.
    ///
    /// returns `false` if the result indicates no op has happened -- that's when the first op
    ///   creates the item and the second deletes it.
    pub fn squash(op: &mut Self, other: Self) -> Result<bool> {
        use WriteOp::*;

        match (&op, other) {
            (Modification { .. } | Creation { .. }, Creation { .. }) // create existing
            | (Deletion { .. }, Modification { .. } | Deletion { .. }) // delete or modify already deleted
            => {
                bail!("The given change sets cannot be squashed")
            },
            (Creation {metadata: old_meta, .. } , Modification {data, metadata}) => {
                Self::ensure_metadata_compatible(old_meta, &metadata)?;

                *op = Creation {
                    data,
                    metadata,
                };
            },
            (Modification{metadata: old_meta, .. } , Modification {data, metadata}) => {
                Self::ensure_metadata_compatible(old_meta, &metadata)?;

                *op = Modification {
                    data,
                    metadata,
                };
            },
            (Modification {metadata: old_meta, ..}, Deletion {metadata}) => {
                Self::ensure_metadata_compatible(old_meta, &metadata)?;

                *op = Deletion {
                    metadata,
                }
            },
            (Deletion {metadata}, Creation {data, ..}) => {
                // n.b. With write sets from multiple sessions being squashed together, it's possible
                //   to see two ops carrying different metadata (or one with it the other without)
                //   due to deleting in one session and recreating in another. The original metadata
                //   shouldn't change due to the squash.
                // And because the deposit or refund happens after all squashing is finished, it's
                // not a concern of fairness.
                *op = Modification {
                    data,
                    metadata: metadata.clone(),
                }
            },
            (Creation { metadata: old_meta, .. }, Deletion { metadata }) => {
                Self::ensure_metadata_compatible(old_meta, &metadata)?;

                return Ok(false)
            },
        }
        Ok(true)
    }

    fn ensure_metadata_compatible(
        old: &StateValueMetadata,
        new: &StateValueMetadata,
    ) -> Result<()> {
        // Write ops shouldn't be squashed after the second one is charged for fees, which might
        // result in metadata change (bytes_deposit increase, for example).
        ensure!(
            old == new,
            "Squashing incompatible metadata: old:{old:?}, new:{new:?}",
        );
        Ok(())
    }

    pub fn bytes(&self) -> Option<&Bytes> {
        use WriteOp::*;

        match self {
            Creation { data, .. } | Modification { data, .. } => Some(data),
            Deletion { .. } => None,
        }
    }

    pub fn size(&self) -> usize {
        use WriteOp::*;

        match self {
            Creation { data, .. } | Modification { data, .. } => data.len(),
            Deletion { .. } => 0,
        }
    }

    pub fn metadata(&self) -> &StateValueMetadata {
        use WriteOp::*;

        match self {
            Creation { metadata, .. } | Modification { metadata, .. } | Deletion { metadata } => {
                metadata
            },
        }
    }

    pub fn get_metadata_mut(&mut self) -> &mut StateValueMetadata {
        use WriteOp::*;

        match self {
            Creation { metadata, .. } | Modification { metadata, .. } | Deletion { metadata } => {
                metadata
            },
        }
    }

    pub fn into_metadata(self) -> StateValueMetadata {
        match self {
            Creation { metadata, .. } | Modification { metadata, .. } | Deletion { metadata } => {
                metadata
            },
        }
    }

    pub fn legacy_creation(data: Bytes) -> Self {
        Self::Creation {
            data,
            metadata: StateValueMetadata::none(),
        }
    }

    pub fn legacy_modification(data: Bytes) -> Self {
        Self::Modification {
            data,
            metadata: StateValueMetadata::none(),
        }
    }

    pub fn legacy_deletion() -> Self {
        Self::Deletion {
            metadata: StateValueMetadata::none(),
        }
    }
}

impl<'de> Deserialize<'de> for WriteOp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        PersistedWriteOp::deserialize(deserializer).map(|persisted| persisted.into_in_mem_form())
    }
}

impl Serialize for WriteOp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_persistable().serialize(serializer)
    }
}

pub enum WriteOpSize {
    Creation { write_len: u64 },
    Modification { write_len: u64 },
    Deletion,
}

impl WriteOpSize {
    pub fn write_len(&self) -> Option<u64> {
        match self {
            WriteOpSize::Creation { write_len } | WriteOpSize::Modification { write_len } => {
                Some(*write_len)
            },
            WriteOpSize::Deletion => None,
        }
    }
}

pub trait TransactionWrite: Debug {
    fn bytes(&self) -> Option<&Bytes>;

    // Returns state value that would be observed by a read following the 'self' write.
    fn as_state_value(&self) -> Option<StateValue>;

    // Returns metadata that would be observed by a read following the 'self' write.
    // Provided as a separate method to avoid the clone in as_state_value method
    // (although default implementation below does just that).
    fn as_state_value_metadata(&self) -> Option<StateValueMetadata> {
        self.as_state_value()
            .map(|state_value| state_value.into_metadata())
    }

    // Often, the contents of W:TransactionWrite are converted to Option<StateValue>, e.g.
    // to emulate reading from storage after W has been applied. However, in some contexts,
    // it is also helpful to convert a StateValue to a potential instance of W that would
    // have the desired effect. This allows e.g. storing sentinel elements of type W in
    // data-structures (notably in MVHashMap). The kind of W will be Modification and not
    // Creation, but o.w. if there are several instances of W that correspond to the
    // provided maybe_state_value, an arbitrary one may be provided.
    fn from_state_value(maybe_state_value: Option<StateValue>) -> Self;

    fn extract_raw_bytes(&self) -> Option<Bytes> {
        self.bytes().cloned()
    }

    fn as_u128(&self) -> anyhow::Result<Option<u128>> {
        match self.bytes() {
            Some(bytes) => Ok(Some(bcs::from_bytes(bytes)?)),
            None => Ok(None),
        }
    }

    fn write_op_kind(&self) -> WriteOpKind;

    fn is_deletion(&self) -> bool {
        self.write_op_kind() == WriteOpKind::Deletion
    }

    fn is_creation(&self) -> bool {
        self.write_op_kind() == WriteOpKind::Creation
    }

    fn is_modification(&self) -> bool {
        self.write_op_kind() == WriteOpKind::Modification
    }

    fn set_bytes(&mut self, bytes: Bytes);

    fn write_op_size(&self) -> WriteOpSize {
        use WriteOpKind::*;
        match self.write_op_kind() {
            Creation => WriteOpSize::Creation {
                write_len: self.bytes().unwrap().len() as u64,
            },
            Modification => WriteOpSize::Modification {
                write_len: self.bytes().unwrap().len() as u64,
            },
            Deletion { .. } => WriteOpSize::Deletion,
        }
    }
}

impl TransactionWrite for WriteOp {
    fn bytes(&self) -> Option<&Bytes> {
        self.bytes()
    }

    fn as_state_value(&self) -> Option<StateValue> {
        self.bytes()
            .map(|bytes| StateValue::new_with_metadata(bytes.clone(), self.metadata().clone()))
    }

    // Note that even if WriteOp is DeletionWithMetadata, the method returns None, as a later
    // read would not read the metadata of the deletion op.
    fn as_state_value_metadata(&self) -> Option<StateValueMetadata> {
        self.bytes().map(|_| self.metadata().clone())
    }

    fn from_state_value(maybe_state_value: Option<StateValue>) -> Self {
        match maybe_state_value {
            None => Self::legacy_deletion(),
            Some(state_value) => {
                let (metadata, data) = state_value.unpack();
                Self::Modification { data, metadata }
            },
        }
    }

    fn write_op_kind(&self) -> WriteOpKind {
        use WriteOpKind::*;
        match self {
            WriteOp::Creation { .. } => Creation,
            WriteOp::Modification { .. } => Modification,
            WriteOp::Deletion { .. } => Deletion,
        }
    }

    fn set_bytes(&mut self, bytes: Bytes) {
        use WriteOp::*;

        match self {
            Creation { data, .. } | Modification { data, .. } => *data = bytes,
            Deletion { .. } => (),
        }
    }
}

#[allow(clippy::format_collect)]
impl std::fmt::Debug for WriteOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Creation { data, metadata } => write!(
                f,
                "Creation({}, metadata:{:?})",
                data.iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>(),
                metadata,
            ),
            Modification { data, metadata } => write!(
                f,
                "Modification({}, metadata:{:?})",
                data.iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>(),
                metadata,
            ),
            Deletion { metadata } => {
                write!(f, "Deletion(metadata:{:?})", metadata,)
            },
        }
    }
}

#[derive(
    BCSCryptoHash, Clone, CryptoHasher, Debug, Eq, Hash, PartialEq, Serialize, Deserialize,
)]
pub enum WriteSet {
    V0(WriteSetV0),
}

impl Default for WriteSet {
    fn default() -> Self {
        Self::V0(WriteSetV0::default())
    }
}

impl WriteSet {
    pub fn into_mut(self) -> WriteSetMut {
        match self {
            Self::V0(write_set) => write_set.0,
        }
    }
}

impl Deref for WriteSet {
    type Target = WriteSetV0;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::V0(write_set) => write_set,
        }
    }
}

impl DerefMut for WriteSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::V0(write_set) => write_set,
        }
    }
}

/// `WriteSet` contains all access paths that one transaction modifies. Each of them is a `WriteOp`
/// where `Value(val)` means that serialized representation should be updated to `val`, and
/// `Deletion` means that we are going to delete this access path.
#[derive(
    BCSCryptoHash, Clone, CryptoHasher, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize,
)]
pub struct WriteSetV0(WriteSetMut);

impl WriteSetV0 {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> btree_map::Iter<'_, StateKey, WriteOp> {
        self.0.write_set.iter()
    }

    pub fn get(&self, key: &StateKey) -> Option<&WriteOp> {
        self.0.get(key)
    }

    pub fn get_total_supply(&self) -> Option<u128> {
        let value = self
            .0
            .get(&TOTAL_SUPPLY_STATE_KEY)
            .and_then(|op| op.bytes())
            .map(|bytes| bcs::from_bytes::<u128>(bytes));
        value.transpose().map_err(anyhow::Error::msg).unwrap()
    }

    // This is a temporary method to update the total supply in the write set.
    // TODO: get rid of this func() and use WriteSetMut instead; for that we need to change
    //       VM execution such that to 'TransactionOutput' is materialized after updating
    //       total_supply.
    pub fn update_total_supply(&mut self, value: u128) {
        assert!(self
            .0
            .write_set
            .insert(
                TOTAL_SUPPLY_STATE_KEY.clone(),
                WriteOp::legacy_modification(bcs::to_bytes(&value).unwrap().into())
            )
            .is_some());
    }
}

/// A mutable version of `WriteSet`.
///
/// This is separate because it goes through validation before becoming an immutable `WriteSet`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteSetMut {
    // TODO: Change to HashMap with a stable iterator for serialization.
    write_set: BTreeMap<StateKey, WriteOp>,
}

impl WriteSetMut {
    pub fn new(write_ops: impl IntoIterator<Item = (StateKey, WriteOp)>) -> Self {
        Self {
            write_set: write_ops.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, item: (StateKey, WriteOp)) {
        self.write_set.insert(item.0, item.1);
    }

    pub fn extend(&mut self, write_ops: impl IntoIterator<Item = (StateKey, WriteOp)>) {
        self.write_set.extend(write_ops);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.write_set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.write_set.len()
    }

    pub fn freeze(self) -> Result<WriteSet> {
        // TODO: add structural validation
        Ok(WriteSet::V0(WriteSetV0(self)))
    }

    pub fn get(&self, key: &StateKey) -> Option<&WriteOp> {
        self.write_set.get(key)
    }

    pub fn as_inner_mut(&mut self) -> &mut BTreeMap<StateKey, WriteOp> {
        &mut self.write_set
    }

    pub fn squash(mut self, other: Self) -> Result<Self> {
        use btree_map::Entry::*;

        for (key, op) in other.write_set.into_iter() {
            match self.write_set.entry(key) {
                Occupied(mut entry) => {
                    if !WriteOp::squash(entry.get_mut(), op)? {
                        entry.remove();
                    }
                },
                Vacant(entry) => {
                    entry.insert(op);
                },
            }
        }

        Ok(self)
    }
}

impl FromIterator<(StateKey, WriteOp)> for WriteSetMut {
    fn from_iter<I: IntoIterator<Item = (StateKey, WriteOp)>>(iter: I) -> Self {
        let mut ws = WriteSetMut::default();
        for write in iter {
            ws.insert((write.0, write.1));
        }
        ws
    }
}

impl<'a> IntoIterator for &'a WriteSet {
    type IntoIter = btree_map::Iter<'a, StateKey, WriteOp>;
    type Item = (&'a StateKey, &'a WriteOp);

    fn into_iter(self) -> Self::IntoIter {
        match self {
            WriteSet::V0(write_set) => write_set.0.write_set.iter(),
        }
    }
}

impl IntoIterator for WriteSet {
    type IntoIter = btree_map::IntoIter<StateKey, WriteOp>;
    type Item = (StateKey, WriteOp);

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::V0(write_set) => write_set.0.write_set.into_iter(),
        }
    }
}
