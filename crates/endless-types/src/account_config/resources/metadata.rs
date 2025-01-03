// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

/// The balance resource held under an account.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct MetadataResource {
    /// Name of the fungible metadata, i.e., "USDT".
    name: String,
    /// Symbol of the fungible metadata, usually a shorter version of the name.
    /// For example, Singapore Dollar is SGD.
    symbol: String,
    /// Number of decimals used for display purposes.
    /// For example, if `decimals` equals `2`, a balance of `505` coins should
    /// be displayed to a user as `5.05` (`505 / 10 ** 2`).
    decimals: u8,
    /// The Uniform Resource Identifier (uri) pointing to an image that can be used as the icon for this fungible
    /// asset.
    icon_uri: String,
    /// The Uniform Resource Identifier (uri) pointing to the website for the fungible asset.
    project_uri: String,
}

impl MetadataResource {
    pub fn new(
        name: String,
        symbol: String,
        decimals: u8,
        icon_uri: String,
        project_uri: String,
    ) -> Self {
        Self {
            name,
            symbol,
            decimals,
            icon_uri,
            project_uri,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn icon_uri(&self) -> String {
        self.icon_uri.clone()
    }

    pub fn project_uri(&self) -> String {
        self.project_uri.clone()
    }
}

impl MoveStructType for MetadataResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("fungible_asset");
    const STRUCT_NAME: &'static IdentStr = ident_str!("Metadata");
}

impl MoveResource for MetadataResource {}
