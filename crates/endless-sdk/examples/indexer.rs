// Copyright © Endless
// Copyright © Aptos Foundation

use anyhow::Result;
use endless_indexer_client::query_v2::{
    get_coin_data, get_collection_data, get_nft_data, get_nft_history, get_transaction_version,
};
use endless_sdk::{
    constant::Network, indexer_client::IndexerClient, types::account_address::AccountAddress,
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let network = Network::Testnet;
    let indexer_client = IndexerClient::new((network.indexer_url()).to_string());

    let coin_id = AccountAddress::from_str("ENDLESSsssssssssssssssssssssssssssssssssssss").unwrap();
    let collection_id =
        AccountAddress::from_str("CTsYnA7PN5TJEMSSuRyYCipgYFgdFkY6v9cBvbrWCLm7").unwrap();
    let nft_id = AccountAddress::from_str("7E49bPbmbaWdkZgYTpfi7bdMwbGb3Y42EVeBd8mUgubo").unwrap();
    let account_address = AccountAddress::from_str(
        "0x2a35f554c042ac90b35d300764a244899c511a3c52286ce55c8d1feb66e3f039",
    )
    .unwrap();

    println!("\n get_coin_data_by_id");
    let response = indexer_client.get_coin_data_by_id(coin_id).await.unwrap();
    println!("{}", response.text().await?);

    println!("\n batch_get_coin_data_by_id");
    let response = indexer_client
        .batch_get_coin_data_by_id(vec![coin_id])
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_coin_data_by_owner");
    let conditions = get_coin_data::Conditions {
        coin: Some(coin_id),
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_coin_data_by_owner(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_collection_data_by_id");
    let conditions = get_coin_data::Conditions {
        coin: Some(collection_id),
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_coin_data_by_owner(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n batch_get_collection_data_by_ids");
    let response = indexer_client
        .batch_get_collection_data_by_ids(vec![collection_id])
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_collection_data_by_creator");
    let conditions = get_collection_data::Conditions {
        name: Some("collectionName".to_string()),
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_collection_data_by_creator(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_collection_data_by_nft_owner");
    let conditions = get_collection_data::Conditions {
        name: None,
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_collection_data_by_nft_owner(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_indexer_status");
    let response = indexer_client.get_indexer_status().await.unwrap();
    println!("{}", response.text().await?);

    println!("\n get_nft_data_by_id");
    let response = indexer_client.get_nft_data_by_id(nft_id).await.unwrap();
    println!("{}", response.text().await?);

    println!("\n get_nft_data_by_owner");
    let conditions = get_nft_data::Conditions {
        collection: Some(collection_id),
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_nft_data_by_owner(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_nft_history_by_id");
    let conditions = get_nft_history::Conditions {
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_nft_history_by_id(nft_id, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_transaction_version_by_account");
    let conditions = get_transaction_version::Conditions {
        page: Some(0),
        page_size: Some(100),
    };
    let response = indexer_client
        .get_transaction_version_by_account(account_address, Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    println!("\n get_latest_transaction_version");
    let conditions = get_transaction_version::Conditions {
        page: Some(0),
        page_size: Some(10),
    };
    let response = indexer_client
        .get_latest_transaction_version(Some(conditions))
        .await
        .unwrap();
    println!("{}", response.text().await?);

    Ok(())
}
