// Copyright © Endless
// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use endless_openapi::{impl_poem_parameter, impl_poem_type};
use endless_types::account_address::AccountAddress;
use indoc::indoc;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// The address of an account
///
/// This is represented in a string as a 64 character hex string, sometimes
/// shortened by stripping leading 0s, and adding a 0x.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(AccountAddress);

impl Address {
    pub fn inner(&self) -> &AccountAddress {
        &self.0
    }

    /// Represent an account address in a way that is compliant with the v1 address
    /// standard. The standard is defined as part of AIP-40, read more here:
    /// https://github.com/endless-foundation/AIPs/blob/main/aips/aip-40.md
    ///
    /// In short, all special addresses MUST be represented in SHORT form, e.g.
    ///
    /// 0x1
    ///
    /// All other addresses MUST be represented in LONG form, e.g.
    ///
    /// 0x002098630cfad4734812fa37dc18d9b8d59242feabe49259e26318d468a99584
    ///
    /// For an explanation of what defines a special address, see `is_special`.
    ///
    /// All string representations of addresses MUST be prefixed with 0x.
    pub fn to_standard_string(&self) -> String {
        self.0.to_standard_string()
    }

    /// Web/CLI/Indexer DB: Bs58 encoded string
    ///
    /// System inner: BCS format
    ///
    /// output: BS58 encoded(Normal Account) or "0x1" alike
    pub fn to_bs58_string(&self) -> String {
        if self.0.is_special() {
            self.0.to_standard_string()
        } else {
            bs58::encode(self.0.as_slice()).into_string()
        }
    }

    /// (Account)Address from bs58_encoded or Short/Long address with or without leading 0x
    pub fn from_bs58_string(addr_str: &str) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Self(AccountAddress::from_str(addr_str).map_err(|e| {
            anyhow::format_err!("Invalid account address: {:#}", e)
        })?))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // While the inner type, AccountAddress, has a Display impl already, we don't
        // use it. As part of the AIP-40 migration, the Display impl of the inner
        // AccountAddress was changed to conform to AIP-40, but doing that for the API
        // would constitute a breaking change. So we keep an explicit display impl
        // here that maintains the existing address formatting behavior.
        write!(f, "{}", self.to_bs58_string())
    }
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Self(AccountAddress::from_str(s).map_err(|e| {
            anyhow::format_err!("Invalid account address: {:#}", e)
        })?))
    }
}

impl From<AccountAddress> for Address {
    fn from(address: AccountAddress) -> Self {
        Self(address)
    }
}

impl From<Address> for AccountAddress {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl From<&Address> for AccountAddress {
    fn from(address: &Address) -> Self {
        address.0
    }
}

impl From<Address> for move_core_types::value::MoveValue {
    fn from(d: Address) -> Self {
        move_core_types::value::MoveValue::Address(d.0)
    }
}

impl Serialize for Address {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let address = <String>::deserialize(deserializer)?;
        address.parse().map_err(D::Error::custom)
    }
}

impl_poem_type!(
    Address,
    "string",
    (
        example = Some(serde_json::Value::String(
            "0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1 ".to_string()
        )),
        format = Some("hex"),
        description = Some(indoc! {"
            A hex encoded 32 byte Endless account address.

            This is represented in a string as a 64 character hex string, sometimes
            shortened by stripping leading 0s, and adding a 0x.

            For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1.
        "})
    )
);

impl_poem_parameter!(Address);

#[cfg(test)]
mod tests {
    use crate::address::Address;
    use endless_types::account_address::AccountAddress;
    use serde_json::{json, Value};

    #[test]
    fn test_from_and_to_string() {
        let valid_addresses = vec!["0x1", "0x001", "0x00000000000000000000000000000001"];
        for address in valid_addresses {
            assert_eq!(address.parse::<Address>().unwrap().to_string(), "0x1");
        }

        let invalid_addresses = vec!["invalid", "00x1", "x1"];
        for address in invalid_addresses {
            assert!(address
                .parse::<Address>()
                .unwrap_err()
                .to_string()
                .starts_with(
                    "Invalid account address: Hex characters are invalid: Invalid character",
                ));
        }
    }

    #[test]
    fn test_from_and_to_json() {
        let address: Address = serde_json::from_value(json!("0x1")).unwrap();
        assert_eq!(address, "0x1".parse().unwrap());

        let val: Value = serde_json::to_value(address).unwrap();
        assert_eq!(val, json!("0x1"));
    }

    #[test]
    fn test_from_and_to_account_address() {
        let address: Address = serde_json::from_value(json!("0x1")).unwrap();

        let account_address: AccountAddress = address.into();
        assert_eq!(account_address, AccountAddress::ONE);

        let new_address: Address = account_address.into();
        assert_eq!(new_address, address);
    }

    #[test]
    fn test_from_and_to_account_address_reference() {
        let address: Address = serde_json::from_value(json!("0x1")).unwrap();

        let account_address: AccountAddress = (&address).into();
        assert_eq!(account_address, AccountAddress::ONE);

        let new_address: Address = account_address.into();
        assert_eq!(new_address, address);
    }
}
