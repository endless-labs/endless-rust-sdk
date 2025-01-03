// Copyright © Endless
// Copyright © Aptos Foundation

use anyhow::{Context, Result};
use endless_sdk::{
    helper_client::HelperClient,
    rest_client::{Client, FaucetClient},
    types::{account_address::AccountAddress, LocalAccount},
};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub icon_uri: String,
    pub project_uri: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = Client::new_testnet();
    let faucet_client = FaucetClient::new_from_rest_client(rest_client.clone()).await;
    let helper_client = HelperClient::new(&rest_client);

    let alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    let bob = LocalAccount::generate(&mut rand::rngs::OsRng);

    println!("\n=== Addresses ===");
    println!("Alice: {}", alice.address().to_bs58_string_or_bcs());
    println!("Bob: {}", bob.address().to_bs58_string_or_bcs());

    faucet_client
        .fund(alice.address())
        .await
        .context("Failed to fund Alice's account")?;
    faucet_client
        .fund(bob.address())
        .await
        .context("Failed to fund Bob's account")?;

    println!("\n=== Initial Balances ===");
    println!(
        "Alice: {:?}",
        helper_client
            .get_account_balance(&alice.address())
            .await
            .context("Failed to get Alice's account balance")?
    );
    println!(
        "Bob: {:?}",
        helper_client
            .get_account_balance(&bob.address())
            .await
            .context("Failed to get Bob's account balance")?
    );

    let eds_metadata =
        AccountAddress::from_str("ENDLESSsssssssssssssssssssssssssssssssssssss").unwrap();

    let response: Metadata = rest_client
        .get_account_resource_bcs::<Metadata>(eds_metadata, "0x1::fungible_asset::Metadata")
        .await
        .unwrap()
        .into_inner();
    println!("EDS metadata: {:?}", response);

    helper_client
        .simulate_transfer_coins(&alice, bob.address(), 6u128, eds_metadata, None)
        .await
        .unwrap();

    let txn_hash = helper_client
        .transfer_coins(&alice, bob.address(), 6u128, eds_metadata, None)
        .await
        .context("Failed to submit transaction to transfer coins")?;
    rest_client
        .wait_for_transaction_bcs(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?;

    let balance = helper_client
        .get_account_balance_by_metadata(bob.address(), eds_metadata)
        .await
        .unwrap();

    println!("Bob's balance: {}", balance);
    Ok(())
}
