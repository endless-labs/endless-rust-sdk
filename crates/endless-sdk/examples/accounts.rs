// Copyright © Endless
// Copyright © Aptos Foundation

use anyhow::Result;
use endless_crypto::{
    hash::CryptoHash, test_utils::TestEndlessCrypto, SigningKey, ValidCryptoMaterialStringExt,
};
use endless_rest_client::error::RestError;
use endless_sdk::{helper_client::HelperClient, rest_client::Client};

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = Client::new_testnet();
    let helper_client = HelperClient::new(&rest_client);

    let local_account = helper_client
        .recover_account_from_private_key(
            "0xf6e009ceedbe3189681a982432e387010389b160e21ced1e402da2de82331961",
        )
        .unwrap();

    let sequence_number = match rest_client.get_account_bcs(local_account.address()).await {
        Ok(account) => account.inner().sequence_number(),
        Err(err) => match err {
            RestError::Api(_) => 0,
            _ => return Err(err.into()),
        },
    };
    local_account.set_sequence_number(sequence_number);

    println!(
        "local_account.sequence_number: {}",
        local_account.sequence_number()
    );
    println!(
        "local_account.private_key {:?}",
        local_account.private_key().to_encoded_string().unwrap()
    );

    println!(
        "local_account.public_key {:?}",
        local_account.public_key().to_encoded_string().unwrap()
    );

    println!(
        "local_account.authentication_key {:?}",
        local_account
            .authentication_key()
            .to_encoded_string()
            .unwrap()
    );

    println!("local_account address {}", local_account.address());
    println!(
        "local_account address bs58 {}",
        local_account.address().to_bs58_string_or_bcs()
    );

    let message = TestEndlessCrypto("A".to_string());
    println!("message hash {}", message.hash());
    let signature = local_account.private_key().sign(&message).unwrap();

    println!("signature {}", signature.to_encoded_string().unwrap());

    Ok(())
}
