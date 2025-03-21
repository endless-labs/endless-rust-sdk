// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::{
    abstract_write_op::{AbstractResourceWriteOp, GroupWrite},
    change_set::VMChangeSet,
    check_change_set::CheckChangeSet,
};
use endless_aggregator::{
    delayed_change::DelayedChange,
    delta_change_set::{delta_add, DeltaOp},
};
use endless_types::{
    account_address::AccountAddress,
    contract_event::ContractEvent,
    on_chain_config::CurrentTimeMicroseconds,
    state_store::{state_key::StateKey, state_value::StateValueMetadata},
    write_set::WriteOp,
};
use move_core_types::{
    identifier::Identifier,
    language_storage::{StructTag, TypeTag},
    value::MoveTypeLayout,
};
use move_vm_types::delayed_values::delayed_field_id::DelayedFieldID;
use std::{collections::BTreeMap, sync::Arc};

pub(crate) struct MockChangeSetChecker;

impl CheckChangeSet for MockChangeSetChecker {
    fn check_change_set(&self, _change_set: &VMChangeSet) -> PartialVMResult<()> {
        Ok(())
    }
}

macro_rules! as_state_key {
    ($k:ident) => {
        StateKey::raw($k.to_string().into_bytes())
    };
    ($k:expr) => {
        StateKey::raw($k.to_string().into_bytes())
    };
}
pub(crate) use as_state_key;

macro_rules! as_bytes {
    ($v:ident) => {
        bcs::to_bytes(&$v).expect("unexpected serialization error")
    };
    ($v:expr) => {
        bcs::to_bytes(&$v).expect("unexpected serialization error")
    };
}

pub(crate) use as_bytes;
use move_binary_format::errors::PartialVMResult;

pub(crate) fn raw_metadata(v: u64) -> StateValueMetadata {
    StateValueMetadata::legacy(v, &CurrentTimeMicroseconds { microseconds: v })
}

pub(crate) fn mock_create(k: impl ToString, v: u128) -> (StateKey, WriteOp) {
    (
        as_state_key!(k),
        WriteOp::legacy_creation(as_bytes!(v).into()),
    )
}

pub(crate) fn mock_modify(k: impl ToString, v: u128) -> (StateKey, WriteOp) {
    (
        as_state_key!(k),
        WriteOp::legacy_modification(as_bytes!(v).into()),
    )
}

pub(crate) fn mock_delete(k: impl ToString) -> (StateKey, WriteOp) {
    (as_state_key!(k), WriteOp::legacy_deletion())
}

pub(crate) fn mock_create_with_layout(
    k: impl ToString,
    v: u128,
    layout: Option<Arc<MoveTypeLayout>>,
) -> (StateKey, AbstractResourceWriteOp) {
    (
        as_state_key!(k),
        AbstractResourceWriteOp::from_resource_write_with_maybe_layout(
            WriteOp::legacy_creation(as_bytes!(v).into()),
            layout,
        ),
    )
}

pub(crate) fn mock_modify_with_layout(
    k: impl ToString,
    v: u128,
    layout: Option<Arc<MoveTypeLayout>>,
) -> (StateKey, AbstractResourceWriteOp) {
    (
        as_state_key!(k),
        AbstractResourceWriteOp::from_resource_write_with_maybe_layout(
            WriteOp::legacy_modification(as_bytes!(v).into()),
            layout,
        ),
    )
}

pub(crate) fn mock_delete_with_layout(k: impl ToString) -> (StateKey, AbstractResourceWriteOp) {
    (
        as_state_key!(k),
        AbstractResourceWriteOp::from_resource_write_with_maybe_layout(
            WriteOp::legacy_deletion(),
            None,
        ),
    )
}

pub(crate) fn mock_add(k: impl ToString, v: u128) -> (StateKey, DeltaOp) {
    const DUMMY_LIMIT: u128 = 1000;
    (as_state_key!(k), delta_add(v, DUMMY_LIMIT))
}

pub(crate) fn mock_tag_0() -> StructTag {
    StructTag {
        address: AccountAddress::ONE,
        module: Identifier::new("a").unwrap(),
        name: Identifier::new("a").unwrap(),
        type_params: vec![TypeTag::U8],
    }
}

pub(crate) fn mock_tag_1() -> StructTag {
    StructTag {
        address: AccountAddress::ONE,
        module: Identifier::new("abcde").unwrap(),
        name: Identifier::new("fgh").unwrap(),
        type_params: vec![TypeTag::U64],
    }
}

pub(crate) fn mock_tag_2() -> StructTag {
    StructTag {
        address: AccountAddress::ONE,
        module: Identifier::new("abcdex").unwrap(),
        name: Identifier::new("fghx").unwrap(),
        type_params: vec![TypeTag::U128],
    }
}

pub(crate) struct VMChangeSetBuilder {
    resource_write_set: BTreeMap<StateKey, AbstractResourceWriteOp>,
    module_write_set: BTreeMap<StateKey, WriteOp>,
    events: Vec<(ContractEvent, Option<MoveTypeLayout>)>,
    delayed_field_change_set: BTreeMap<DelayedFieldID, DelayedChange<DelayedFieldID>>,
    aggregator_v1_write_set: BTreeMap<StateKey, WriteOp>,
    aggregator_v1_delta_set: BTreeMap<StateKey, DeltaOp>,
}

impl VMChangeSetBuilder {
    pub(crate) fn new() -> Self {
        Self {
            resource_write_set: BTreeMap::new(),
            module_write_set: BTreeMap::new(),
            events: vec![],
            delayed_field_change_set: BTreeMap::new(),
            aggregator_v1_write_set: BTreeMap::new(),
            aggregator_v1_delta_set: BTreeMap::new(),
        }
    }

    pub(crate) fn with_resource_write_set(
        mut self,
        resource_write_set: impl IntoIterator<Item = (StateKey, AbstractResourceWriteOp)>,
    ) -> Self {
        assert!(self.resource_write_set.is_empty());
        self.resource_write_set.extend(resource_write_set);
        self
    }

    pub(crate) fn with_module_write_set(
        mut self,
        module_write_set: impl IntoIterator<Item = (StateKey, WriteOp)>,
    ) -> Self {
        assert!(self.module_write_set.is_empty());
        self.module_write_set.extend(module_write_set);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_events(
        mut self,
        events: impl IntoIterator<Item = (ContractEvent, Option<MoveTypeLayout>)>,
    ) -> Self {
        assert!(self.events.is_empty());
        self.events.extend(events);
        self
    }

    pub(crate) fn with_delayed_field_change_set(
        mut self,
        delayed_field_change_set: impl IntoIterator<
            Item = (DelayedFieldID, DelayedChange<DelayedFieldID>),
        >,
    ) -> Self {
        assert!(self.delayed_field_change_set.is_empty());
        self.delayed_field_change_set
            .extend(delayed_field_change_set);
        self
    }

    pub(crate) fn with_aggregator_v1_write_set(
        mut self,
        aggregator_v1_write_set: impl IntoIterator<Item = (StateKey, WriteOp)>,
    ) -> Self {
        assert!(self.aggregator_v1_write_set.is_empty());
        self.aggregator_v1_write_set.extend(aggregator_v1_write_set);
        self
    }

    pub(crate) fn with_aggregator_v1_delta_set(
        mut self,
        aggregator_v1_delta_set: impl IntoIterator<Item = (StateKey, DeltaOp)>,
    ) -> Self {
        assert!(self.aggregator_v1_delta_set.is_empty());
        self.aggregator_v1_delta_set.extend(aggregator_v1_delta_set);
        self
    }

    pub(crate) fn build(self) -> VMChangeSet {
        VMChangeSet::new(
            self.resource_write_set,
            self.module_write_set,
            self.events,
            self.delayed_field_change_set,
            self.aggregator_v1_write_set,
            self.aggregator_v1_delta_set,
            &MockChangeSetChecker,
        )
        .unwrap()
    }
}

pub(crate) struct ExpandedVMChangeSetBuilder {
    resource_write_set: BTreeMap<StateKey, (WriteOp, Option<Arc<MoveTypeLayout>>)>,
    resource_group_write_set: BTreeMap<StateKey, GroupWrite>,
    module_write_set: BTreeMap<StateKey, WriteOp>,
    aggregator_v1_write_set: BTreeMap<StateKey, WriteOp>,
    aggregator_v1_delta_set: BTreeMap<StateKey, DeltaOp>,
    delayed_field_change_set: BTreeMap<DelayedFieldID, DelayedChange<DelayedFieldID>>,
    reads_needing_delayed_field_exchange:
        BTreeMap<StateKey, (StateValueMetadata, u64, Arc<MoveTypeLayout>)>,
    group_reads_needing_delayed_field_exchange: BTreeMap<StateKey, (StateValueMetadata, u64)>,
    events: Vec<(ContractEvent, Option<MoveTypeLayout>)>,
}

#[allow(dead_code)]
impl ExpandedVMChangeSetBuilder {
    pub(crate) fn new() -> Self {
        Self {
            resource_write_set: BTreeMap::new(),
            resource_group_write_set: BTreeMap::new(),
            module_write_set: BTreeMap::new(),
            aggregator_v1_write_set: BTreeMap::new(),
            aggregator_v1_delta_set: BTreeMap::new(),
            delayed_field_change_set: BTreeMap::new(),
            reads_needing_delayed_field_exchange: BTreeMap::new(),
            group_reads_needing_delayed_field_exchange: BTreeMap::new(),
            events: vec![],
        }
    }

    pub(crate) fn with_resource_write_set(
        mut self,
        resource_write_set: impl IntoIterator<Item = (StateKey, (WriteOp, Option<Arc<MoveTypeLayout>>))>,
    ) -> Self {
        assert!(self.resource_write_set.is_empty());
        self.resource_write_set.extend(resource_write_set);
        self
    }

    pub(crate) fn with_resource_group_write_set(
        mut self,
        resource_group_write_set: impl IntoIterator<Item = (StateKey, GroupWrite)>,
    ) -> Self {
        assert!(self.resource_group_write_set.is_empty());
        self.resource_group_write_set
            .extend(resource_group_write_set);
        self
    }

    pub(crate) fn with_module_write_set(
        mut self,
        module_write_set: impl IntoIterator<Item = (StateKey, WriteOp)>,
    ) -> Self {
        assert!(self.module_write_set.is_empty());
        self.module_write_set.extend(module_write_set);
        self
    }

    pub(crate) fn with_aggregator_v1_write_set(
        mut self,
        aggregator_v1_write_set: impl IntoIterator<Item = (StateKey, WriteOp)>,
    ) -> Self {
        assert!(self.aggregator_v1_write_set.is_empty());
        self.aggregator_v1_write_set.extend(aggregator_v1_write_set);
        self
    }

    pub(crate) fn with_aggregator_v1_delta_set(
        mut self,
        aggregator_v1_delta_set: impl IntoIterator<Item = (StateKey, DeltaOp)>,
    ) -> Self {
        assert!(self.aggregator_v1_delta_set.is_empty());
        self.aggregator_v1_delta_set.extend(aggregator_v1_delta_set);
        self
    }

    pub(crate) fn with_delayed_field_change_set(
        mut self,
        delayed_field_change_set: impl IntoIterator<
            Item = (DelayedFieldID, DelayedChange<DelayedFieldID>),
        >,
    ) -> Self {
        assert!(self.delayed_field_change_set.is_empty());
        self.delayed_field_change_set
            .extend(delayed_field_change_set);
        self
    }

    pub(crate) fn with_reads_needing_delayed_field_exchange(
        mut self,
        reads_needing_delayed_field_exchange: impl IntoIterator<
            Item = (StateKey, (StateValueMetadata, u64, Arc<MoveTypeLayout>)),
        >,
    ) -> Self {
        assert!(self.reads_needing_delayed_field_exchange.is_empty());
        self.reads_needing_delayed_field_exchange
            .extend(reads_needing_delayed_field_exchange);
        self
    }

    pub(crate) fn with_group_reads_needing_delayed_field_exchange(
        mut self,
        group_reads_needing_delayed_field_exchange: impl IntoIterator<
            Item = (StateKey, (StateValueMetadata, u64)),
        >,
    ) -> Self {
        assert!(self.group_reads_needing_delayed_field_exchange.is_empty());
        self.group_reads_needing_delayed_field_exchange
            .extend(group_reads_needing_delayed_field_exchange);
        self
    }

    pub(crate) fn with_events(
        mut self,
        events: impl IntoIterator<Item = (ContractEvent, Option<MoveTypeLayout>)>,
    ) -> Self {
        assert!(self.events.is_empty());
        self.events.extend(events);
        self
    }

    pub(crate) fn try_build(self) -> PartialVMResult<VMChangeSet> {
        VMChangeSet::new_expanded(
            self.resource_write_set,
            self.resource_group_write_set,
            self.module_write_set,
            self.aggregator_v1_write_set,
            self.aggregator_v1_delta_set,
            self.delayed_field_change_set,
            self.reads_needing_delayed_field_exchange,
            self.group_reads_needing_delayed_field_exchange,
            self.events,
            &MockChangeSetChecker,
        )
    }

    pub(crate) fn build(self) -> VMChangeSet {
        self.try_build().unwrap()
    }
}
