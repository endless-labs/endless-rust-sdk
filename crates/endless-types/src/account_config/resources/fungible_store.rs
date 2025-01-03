// Copyright © Endless
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::AggregatorU128;
use move_core_types::{
    account_address::AccountAddress,
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

pub const EDS_METADATA_ADDR: &str =
    "0xc69712057e634bebc9ab02745d2d69ee738e3eb4f5d30189a9acbf8e08fb823e";

pub fn primary_store(address: &AccountAddress, metadata: &AccountAddress) -> AccountAddress {
    let mut bytes = address.to_vec();
    bytes.append(&mut metadata.to_vec());
    bytes.push(0xFC);
    AccountAddress::from_bytes(endless_crypto::hash::HashValue::sha3_256_of(&bytes).to_vec())
        .unwrap()
}

pub fn eds_metadata() -> AccountAddress {
    AccountAddress::from_hex_literal(EDS_METADATA_ADDR).unwrap()
}

/// The balance resource held under an account.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct FungibleStoreResource {
    pub metadata: AccountAddress,
    pub balance: AggregatorU128,
    pub frozen: bool,
}

impl FungibleStoreResource {
    pub fn new(metadata: AccountAddress, balance: u128, frozen: bool) -> Self {
        Self {
            metadata,
            balance: AggregatorU128 {
                value: balance,
                max_value: u128::MAX,
            },
            frozen,
        }
    }

    pub fn metadata(&self) -> AccountAddress {
        self.metadata
    }

    pub fn balance(&self) -> u128 {
        self.balance.value
    }

    pub fn frozen(&self) -> bool {
        self.frozen
    }
}

impl MoveStructType for FungibleStoreResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("fungible_asset");
    const STRUCT_NAME: &'static IdentStr = ident_str!("FungibleStore");
}

impl MoveResource for FungibleStoreResource {}
