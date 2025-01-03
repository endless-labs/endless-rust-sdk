// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    event::{EventHandle, EventKey},
};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
    move_resource::{MoveResource, MoveStructType},
};
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A Rust representation of ObjectGroup.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct ObjectGroupResource {}

impl MoveStructType for ObjectGroupResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("object");
    const STRUCT_NAME: &'static IdentStr = ident_str!("ObjectGroup");
}

impl MoveResource for ObjectGroupResource {}

/// The balance resource held under an account.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct ObjectResource<T: MoveStructType> {
    inner: AccountAddress,
    phantom: PhantomData<T>,
}

impl<T: MoveStructType> ObjectResource<T> {
    pub fn new(inner: AccountAddress) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    pub fn inner(&self) -> AccountAddress {
        self.inner
    }
}

impl<T: MoveStructType> MoveStructType for ObjectResource<T> {
    const MODULE_NAME: &'static IdentStr = ident_str!("object");
    const STRUCT_NAME: &'static IdentStr = ident_str!("Object");

    fn type_params() -> Vec<TypeTag> {
        vec![TypeTag::Struct(Box::new(StructTag {
            address: AccountAddress::ONE,
            module: ident_str!("object").to_owned(),
            name: ident_str!("Object").to_owned(),
            type_params: T::type_params(),
        }))]
    }
}

impl<T: MoveStructType> MoveResource for ObjectResource<T> {}

/// The balance resource held under an account.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct ObjectCore {
    /// Used by guid to guarantee globally unique objects and create event streams
    guid_creation_num: u64,
    /// The address (object or account) that owns this object
    owner: AccountAddress,
    /// Object transferring is a common operation, this allows for disabling and enabling
    /// transfers bypassing the use of a TransferRef.
    allow_ungated_transfer: bool,
    /// Emitted events upon transferring of ownership.
    transfer_events: EventHandle,
}

impl ObjectCore {
    pub fn new(owner: AccountAddress, allow_ungated_transfer: bool) -> Self {
        let event_key = EventKey::new(0, owner);
        Self {
            guid_creation_num: 0,
            owner,
            allow_ungated_transfer,
            transfer_events: EventHandle::new(event_key, 0),
        }
    }

    pub fn owner(&self) -> AccountAddress {
        self.owner
    }
}

impl MoveStructType for ObjectCore {
    const MODULE_NAME: &'static IdentStr = ident_str!("object");
    const STRUCT_NAME: &'static IdentStr = ident_str!("ObjectCore");
}

impl MoveResource for ObjectCore {}
