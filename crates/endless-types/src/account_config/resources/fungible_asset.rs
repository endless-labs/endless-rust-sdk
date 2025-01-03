// Copyright © Endless
// Copyright © Aptos Foundation

// Copyright © Endless
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct AggregatorU128 {
    pub value: u128,
    pub max_value: u128,
}

/// The balance resource held under an account.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct ConcurrentSupplyResource {
    pub current: AggregatorU128,
}

impl ConcurrentSupplyResource {
    pub fn supply(&self) -> u128 {
        self.current.value
    }

    pub fn max_supply(&self) -> Option<u128> {
        if self.current.max_value == u128::MAX {
            None
        } else {
            Some(self.current.max_value)
        }
    }
}

impl MoveStructType for ConcurrentSupplyResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("fungible_asset");
    const STRUCT_NAME: &'static IdentStr = ident_str!("ConcurrentSupply");
}

impl MoveResource for ConcurrentSupplyResource {}
