// Copyright © Endless
// Copyright © Aptos Foundation

use endless_types::account_address::AccountAddress;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct VersionsResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    /// The list of transaction versions
    pub data: Vec<u64>,
}

#[derive(Serialize)]
pub struct FungibleAssetMetadataResponse {
    pub decimals: u8,
    pub icon_uri: String,
    pub name: String,
    pub project_uri: String,
    pub symbol: String,
    pub supply: String,
}

#[derive(Serialize)]
pub struct CoinResponse {
    pub token: AccountAddress,
    pub metadata: FungibleAssetMetadataResponse,
    pub balance: String,
    pub frozen: bool,
}

#[derive(Serialize)]
pub struct CoinsResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    /// The list of coins
    pub data: Vec<CoinResponse>,
}

#[derive(Serialize, Clone)]
pub struct Royalty {
    pub percent: f64,
    pub payee_address: AccountAddress,
}

#[derive(Serialize, Clone)]
pub struct CollectionResponse {
    pub id: AccountAddress,
    pub creator: AccountAddress,
    pub description: String,
    pub name: String,
    pub uri: String,
    pub current_supply: u64,
    pub total_minted: u64,
    pub max_supply: Option<u64>,
    pub royalty: Option<Royalty>,
}

#[derive(Serialize)]
pub struct CollectionsResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub data: Vec<CollectionResponse>,
}

#[derive(Serialize)]
pub struct NftTokenResponse {
    pub id: AccountAddress,
    pub name: String,
    pub index: u64,
    pub collection: CollectionResponse,
    pub description: String,
    pub uri: String,
    pub owner: AccountAddress,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct PropertyValue {
    #[serde(rename = "type")]
    pub typ: PropertyType,
    #[serde(deserialize_with = "deserialize_hex")]
    pub value: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum PropertyType {
    Bool = 0,
    U8 = 1,
    U16 = 2,
    U32 = 3,
    U64 = 4,
    U128 = 5,
    U256 = 6,
    Address = 7,
    Bytes = 8,
    String = 9,
}

#[derive(Serialize)]
pub struct NftTokenHistoryRecord {
    pub version: u64,
    #[serde(rename = "type")]
    pub typ: String,
    pub from: Option<AccountAddress>,
    pub to: Option<AccountAddress>,
}

#[derive(Serialize)]
pub struct NftTokenHistoryResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub data: Vec<NftTokenHistoryRecord>,
}

#[derive(Serialize)]
pub struct NftTokensResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub data: Vec<NftTokenResponse>,
}
