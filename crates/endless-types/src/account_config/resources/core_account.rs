// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::transaction::authenticator::AuthenticationKey;
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

/// A Rust representation of an Account resource.
/// This is not how the Account is represented in the VM but it's a convenient representation.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct AccountResource {
    authentication_key: Vec<AuthenticationKey>,
    sequence_number: u64,
    guid_creation_num: u64,
    num_signatures_required: u64,
}

impl AccountResource {
    /// Constructs an Account resource.
    pub fn new(sequence_number: u64, authentication_key: Vec<AuthenticationKey>) -> Self {
        AccountResource {
            authentication_key,
            sequence_number,
            guid_creation_num: 0,
            num_signatures_required: 1,
        }
    }

    /// Return the sequence_number field for the given AccountResource
    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    /// Return the authentication_key field for the given AccountResource
    pub fn authentication_key(&self) -> &[AuthenticationKey] {
        &self.authentication_key
    }

    pub fn guid_creation_num(&self) -> u64 {
        self.guid_creation_num
    }

    pub fn num_signatures_required(&self) -> u64 {
        self.num_signatures_required
    }
}

impl MoveStructType for AccountResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("account");
    const STRUCT_NAME: &'static IdentStr = ident_str!("Account");
}

impl MoveResource for AccountResource {}
