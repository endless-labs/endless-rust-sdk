// Copyright © Endless
// Copyright © Aptos Foundation

use anyhow::{Context, Result};
use endless_rest_client::{Response, Transaction};
use endless_sdk::{
    helper_client::HelperClient,
    rest_client::{endless_api_types::Event, Client, FaucetClient},
    types::{account_address::AccountAddress, LocalAccount},
};
use serde::Deserialize;

const MODULE_NAME: &str = "nft";

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = Client::new_testnet();
    let faucet_client = FaucetClient::new_from_rest_client(rest_client.clone()).await;
    let helper_client = HelperClient::new(&rest_client);

    let alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    let bob = LocalAccount::generate(&mut rand::rngs::OsRng);
    println!("Alice: {}", alice.address().to_bs58_string_or_bcs());
    faucet_client
        .fund(alice.address())
        .await
        .context("Failed to fund Alice's account")?;

    println!(
        "Alice: {:?}",
        helper_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance")?
    );

    let response = create_collection(
        &alice,
        "description",
        18446744073709551615,
        "collection_name",
        "uri",
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        false,
        false,
        5,
        100,
    )
    .await
    .unwrap();
    let transaction = response.into_inner();
    println!("{}", serde_json::to_string(&transaction).unwrap());

    let response = mint(
        &alice,
        "collection_name",
        "description",
        "name",
        "uri",
        vec!["key".to_string()],
        vec!["vector<u8>".to_string()],
        vec!["value".to_string()],
    )
    .await
    .unwrap();

    let transaction = response.into_inner();
    println!("{}", serde_json::to_string(&transaction).unwrap());
    let (collection, token) =
        get_collection_and_token_by_events(transaction.events().unwrap()).unwrap();

    // let txn_hash = helper_client
    //     .transfer(&alice, bob.address(), 666, None)
    //     .await
    //     .unwrap();
    // let response = rest_client
    //     .wait_for_transaction(&txn_hash)
    //     .await
    //     .context("Failed when waiting for the transaction")
    //     .unwrap();

    // let transaction = response.into_inner();
    // println!("transfer {}", serde_json::to_string(&transaction).unwrap());

    // set
    set_collection_description(&alice, collection, "collection_description2")
        .await
        .unwrap();
    set_collection_uri(&alice, collection, "collection_uri2")
        .await
        .unwrap();
    set_description(&alice, token, "token_description2")
        .await
        .unwrap();
    set_name(&alice, token, "token_name2").await.unwrap();
    set_uri(&alice, token, "token_uri2").await.unwrap();

    // get
    let collection_description = get_collection_description(collection).await;
    println!("collection_description: {:?}", collection_description);
    let collection_uri = get_collection_uri(collection).await;
    println!("collection_uri: {:?}", collection_uri);
    let description = get_token_description(token).await;
    println!("description: {:?}", description);
    let name = get_token_name(token).await;
    println!("name: {:?}", name);
    let uri = get_token_uri(token).await;
    println!("uri: {:?}", uri);

    // object_transfer
    let response = object_transfer(&alice, token, bob.address()).await.unwrap();
    let transaction = response.into_inner();
    println!("{}", serde_json::to_string(&transaction).unwrap());

    println!(
        "Alice: {:?}",
        helper_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance")?
    );

    //indexer
    // let indexer_client = IndexerClient::new(INDEXER_URL.clone().to_string());

    // get_account_collections_with_owned_tokens
    // println!("\n\nget_account_collections_with_owned_tokens");
    // let v = indexer_client.get_account_collections_with_owned_tokens_variables(bob.address());
    // match indexer_client
    //     .get_account_collections_with_owned_tokens(v)
    //     .await
    // {
    //     Ok(response) => {
    //         println!("{:?}", response.data);
    //     },
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     },
    // }

    // get_account_owned_tokens
    // println!("\n\nget_account_owned_tokens");
    // let v = indexer_client.get_account_owned_tokens_variables(bob.address());
    // match indexer_client.get_account_owned_tokens(v).await {
    //     Ok(response) => {
    //         println!("{:?}", response.data);
    //     },
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     },
    // }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn create_collection(
    signer: &LocalAccount,
    description: &str,
    max_supply: u64,
    name: &str,
    uri: &str,
    mutable_description: bool,
    mutable_royalty: bool,
    mutable_uri: bool,
    mutable_token_description: bool,
    mutable_token_name: bool,
    mutable_token_properties: bool,
    mutable_token_uri: bool,
    tokens_burnable_by_creator: bool,
    tokens_freezable_by_creator: bool,
    royalty_numerator: u64,
    royalty_denominator: u64,
) -> Result<Response<Transaction>> {
    println!("create_collection...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&description).unwrap(),
        bcs::to_bytes(&max_supply).unwrap(),
        bcs::to_bytes(&name).unwrap(),
        bcs::to_bytes(&uri).unwrap(),
        bcs::to_bytes(&mutable_description).unwrap(),
        bcs::to_bytes(&mutable_royalty).unwrap(),
        bcs::to_bytes(&mutable_uri).unwrap(),
        bcs::to_bytes(&mutable_token_description).unwrap(),
        bcs::to_bytes(&mutable_token_name).unwrap(),
        bcs::to_bytes(&mutable_token_properties).unwrap(),
        bcs::to_bytes(&mutable_token_uri).unwrap(),
        bcs::to_bytes(&tokens_burnable_by_creator).unwrap(),
        bcs::to_bytes(&tokens_freezable_by_creator).unwrap(),
        bcs::to_bytes(&royalty_numerator).unwrap(),
        bcs::to_bytes(&royalty_denominator).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "create_collection",
            signer,
            args,
            vec![],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

async fn mint(
    signer: &LocalAccount,
    collection_name: &str,
    description: &str,
    name: &str,
    uri: &str,
    property_keys: Vec<String>,
    property_types: Vec<String>,
    property_values: Vec<String>,
) -> Result<Response<Transaction>> {
    println!("mint...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&collection_name).unwrap(),
        bcs::to_bytes(&description).unwrap(),
        bcs::to_bytes(&name).unwrap(),
        bcs::to_bytes(&uri).unwrap(),
        bcs::to_bytes(&property_keys).unwrap(),
        bcs::to_bytes(&property_types).unwrap(),
        bcs::to_bytes(&property_values).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "mint",
            signer,
            args,
            vec![],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

fn get_collection_and_token_by_events(
    events: &[Event],
) -> Result<(AccountAddress, AccountAddress), String> {
    for event in events {
        if event.typ.to_string() == "0x4::collection::Mint" {
            if let Ok(collection_mint_event) =
                serde_json::from_value::<CollectionMintEvent>(event.data.clone())
            {
                return Ok((
                    collection_mint_event.collection,
                    collection_mint_event.token,
                ));
            }
        }
    }

    Err("No '0x4::collection::Mint' event found".to_string())
}
async fn set_collection_description(
    signer: &LocalAccount,
    collection: AccountAddress,
    description: &str,
) -> Result<Response<Transaction>> {
    println!("set_collection_description...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&collection).unwrap(),
        bcs::to_bytes(&description).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "set_collection_description",
            signer,
            args,
            vec!["0x4::collection::Collection"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

async fn set_collection_uri(
    signer: &LocalAccount,
    collection: AccountAddress,
    uri: &str,
) -> Result<Response<Transaction>> {
    println!("set_collection_uri...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&collection).unwrap(),
        bcs::to_bytes(&uri).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "set_collection_uri",
            signer,
            args,
            vec!["0x4::collection::Collection"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

async fn set_description(
    signer: &LocalAccount,
    collection: AccountAddress,
    description: &str,
) -> Result<Response<Transaction>> {
    println!("set_description...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&collection).unwrap(),
        bcs::to_bytes(&description).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "set_description",
            signer,
            args,
            vec!["0x4::token::Token"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

async fn set_name(
    signer: &LocalAccount,
    token: AccountAddress,
    name: &str,
) -> Result<Response<Transaction>> {
    println!("set_name...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![
        bcs::to_bytes(&token).unwrap(),
        bcs::to_bytes(&name).unwrap(),
    ];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "set_name",
            signer,
            args,
            vec!["0x4::token::Token"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

async fn set_uri(
    signer: &LocalAccount,
    token: AccountAddress,
    uri: &str,
) -> Result<Response<Transaction>> {
    println!("set_uri...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![bcs::to_bytes(&token).unwrap(), bcs::to_bytes(&uri).unwrap()];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::FOUR,
            MODULE_NAME,
            "set_uri",
            signer,
            args,
            vec!["0x4::token::Token"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

pub async fn get_collection_description(collection: AccountAddress) -> String {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);
    let result: Vec<String> = helper_client
        .view_function(
            AccountAddress::FOUR,
            "collection",
            "description",
            vec![bcs::to_bytes(&collection).unwrap()],
            vec!["0x4::collection::Collection"],
        )
        .await
        .unwrap()
        .into_inner();
    result[0].clone()
}

pub async fn get_collection_uri(collection: AccountAddress) -> String {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);
    let result: Vec<String> = helper_client
        .view_function(
            AccountAddress::FOUR,
            "collection",
            "uri",
            vec![bcs::to_bytes(&collection).unwrap()],
            vec!["0x4::collection::Collection"],
        )
        .await
        .unwrap()
        .into_inner();
    result[0].clone()
}

pub async fn get_token_description(token: AccountAddress) -> String {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);
    let result: Vec<String> = helper_client
        .view_function(
            AccountAddress::FOUR,
            "token",
            "description",
            vec![bcs::to_bytes(&token).unwrap()],
            vec!["0x4::token::Token"],
        )
        .await
        .unwrap()
        .into_inner();
    result[0].clone()
}

pub async fn get_token_name(token: AccountAddress) -> String {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);
    let result: Vec<String> = helper_client
        .view_function(
            AccountAddress::FOUR,
            "token",
            "name",
            vec![bcs::to_bytes(&token).unwrap()],
            vec!["0x4::token::Token"],
        )
        .await
        .unwrap()
        .into_inner();
    result[0].clone()
}

pub async fn get_token_uri(token: AccountAddress) -> String {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);
    let result: Vec<String> = helper_client
        .view_function(
            AccountAddress::FOUR,
            "token",
            "uri",
            vec![bcs::to_bytes(&token).unwrap()],
            vec!["0x4::token::Token"],
        )
        .await
        .unwrap()
        .into_inner();
    result[0].clone()
}

async fn object_transfer(
    signer: &LocalAccount,
    token: AccountAddress,
    to: AccountAddress,
) -> Result<Response<Transaction>> {
    println!("object_transfer...");
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let args = vec![bcs::to_bytes(&token).unwrap(), bcs::to_bytes(&to).unwrap()];

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::ONE,
            "object",
            "transfer",
            signer,
            args,
            vec!["0x4::token::Token"],
            None,
        )
        .await
        .context("Failed to submit transaction")
        .unwrap();

    let response = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transaction")
        .unwrap();

    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct CollectionMintEvent {
    pub collection: AccountAddress,
    pub index: Index,
    pub token: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct Index {
    pub value: String,
}
