// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use endless_api_types::{U128, U64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EndlessCoin {
    pub value: U128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: EndlessCoin,
}

impl Balance {
    pub fn get(&self) -> u128 {
        *self.coin.value.inner()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndlessVersion {
    pub major: U64,
}
