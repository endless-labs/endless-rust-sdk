// Copyright © Endless
// Copyright © Aptos Foundation

use anyhow::{Context, Result};
use endless_sdk::{
    helper_client::{HelperClient, Overrides},
    rest_client::Client,
    types::{account_address::AccountAddress, LocalAccount},
};

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    let bob = LocalAccount::generate(&mut rand::rngs::OsRng);

    println!("\n=== Addresses ===");
    println!("Alice: {}", alice.address().to_bs58_string_or_bcs());
    println!("Bob: {}", bob.address().to_bs58_string_or_bcs());

    helper_client
        .faucet_fund(
            &LocalAccount::generate(&mut rand::rngs::OsRng),
            alice.address(),
            None,
        )
        .await
        .unwrap();
    helper_client
        .faucet_fund(
            &LocalAccount::generate(&mut rand::rngs::OsRng),
            bob.address(),
            None,
        )
        .await
        .unwrap();

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

    // transfer
    let txn_hash = helper_client
        .transfer(&alice, bob.address(), 6u128, None)
        .await
        .context("Failed to submit transaction to transfer coins")?;
    rest_client
        .wait_for_transaction_bcs(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?;

    helper_client
        .simulate_transfer(&alice, bob.address(), 6u128, None)
        .await
        .unwrap();

    // entry_function
    let arg1 = bob.address();
    let arg2 = 600u128;
    let overrides = Overrides {
        max_gas_amount: 5_000,
        gas_unit_price: 100,
        timeout_secs: 10,
    };

    let transaction_on_chain_data = helper_client
        .simulate_function(
            AccountAddress::ONE,
            "endless_account",
            "transfer",
            &alice,
            vec![bcs::to_bytes(&arg1).unwrap(), bcs::to_bytes(&arg2).unwrap()],
            vec![],
            Option::Some(overrides.clone()),
        )
        .await
        .unwrap()
        .into_inner();

    println!(
        "transaction_on_chain_data: {}",
        serde_json::to_string(&transaction_on_chain_data).unwrap()
    );

    let txn_hash = helper_client
        .entry_function(
            AccountAddress::ONE,
            "endless_account",
            "transfer",
            &alice,
            vec![bcs::to_bytes(&arg1).unwrap(), bcs::to_bytes(&arg2).unwrap()],
            vec![],
            Option::Some(overrides),
        )
        .await
        .context("Failed to submit transaction to transfer coins")?;
    rest_client
        .wait_for_transaction_bcs(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?;

    println!("\n=== Transfer Success ===");

    // view_function
    let result: Vec<u128> = helper_client
        .view_function(
            AccountAddress::ONE,
            "endless_coin",
            "balance",
            vec![bcs::to_bytes(&arg1).unwrap()],
            vec![],
        )
        .await
        .unwrap()
        .into_inner();

    println!("Bob: {:?}", result[0]);

    println!("Done");

    Ok(())
}
