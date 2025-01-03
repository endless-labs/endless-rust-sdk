// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use endless_api_types::deserialize_from_string;
use endless_api_types::{Address, U64};
use endless_types::transaction::authenticator::AuthenticationKey;
use move_core_types::{language_storage::StructTag, parser::parse_struct_tag};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    ops::{Bound, RangeBounds},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Resource {
    #[serde(rename = "type", deserialize_with = "deserialize_resource_type")]
    pub resource_type: StructTag,
    pub data: serde_json::Value,
}

pub fn deserialize_from_prefixed_hex_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    use serde::de::Error;

    let s = <String>::deserialize(deserializer)?;
    s.trim_start_matches("0x")
        .parse::<T>()
        .map_err(D::Error::custom)
}

pub fn deserialize_resource_type<'de, D>(deserializer: D) -> Result<StructTag, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s = <String>::deserialize(deserializer)?;
    parse_struct_tag(&s).map_err(D::Error::custom)
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Account {
    // #[serde(deserialize_with = "deserialize_from_prefixed_hex_string")]
    pub authentication_key: Vec<AuthenticationKey>,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub sequence_number: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventHandle {
    counter: U64,
    guid: EventHandleGUID,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventHandleGUID {
    len_bytes: u8,
    guid: GUID,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GUID {
    id: ID,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ID {
    creation_num: U64,
    addr: Address,
}

#[derive(Clone, Debug)]
pub enum QueryRange<R: RangeBounds<usize>> {
    ByBlock(R),
    ByVersion(R),
}

impl<R: RangeBounds<usize>> QueryRange<R> {
    /// Create a query range by block. The end block is inclusive.
    pub fn by_block(range: R) -> Self {
        Self::ByBlock(range)
    }

    /// Create a query range by version. The end version is inclusive.
    pub fn by_version(range: R) -> Self {
        Self::ByVersion(range)
    }

    pub(crate) fn into_query(self) -> String {
        match self {
            Self::ByBlock(range) => Self::format_range("block", range),
            Self::ByVersion(range) => Self::format_range("version", range),
        }
    }

    fn format_range(type_str: &str, range: R) -> String {
        let start = match range.start_bound() {
            Bound::Included(value) => format!("start_{type_str}={}", value),
            Bound::Excluded(value) => format!("start_{type_str}={}", value + 1),
            Bound::Unbounded => String::new(),
        };
        let end = match range.end_bound() {
            Bound::Included(value) => format!("end_{type_str}={}", value),
            Bound::Excluded(value) => format!("end_{type_str}={}", value - 1),
            Bound::Unbounded => return start,
        };
        if !start.is_empty() {
            format!("{}&{}", start, end)
        } else {
            end
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_query_range() {
        use super::QueryRange;
        use std::ops::Bound;
        let query_range = QueryRange::by_block(0..10);
        assert_eq!("start_block=0&end_block=9", query_range.into_query());
        let query_range = QueryRange::by_block(..=10);
        assert_eq!("end_block=10", query_range.into_query());
        let query_range = QueryRange::by_block(0..);
        assert_eq!("start_block=0", query_range.into_query());
        let query_range = QueryRange::by_block(..);
        assert_eq!("", query_range.into_query());
        let query_range = QueryRange::by_block((Bound::Excluded(0), Bound::Unbounded));
        assert_eq!("start_block=1", query_range.into_query());

        let query_range = QueryRange::by_version(0..10);
        assert_eq!("start_version=0&end_version=9", query_range.into_query());
    }
}
