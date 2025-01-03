use anyhow::bail;
use endless_crypto::{
    bls12381::{PrivateKey as Bls12381PrivateKey, Signature as Bls12381Signature},
    PrivateKey, SigningKey, Uniform,
};
use endless_ledger::AuthenticationKey;
use endless_sdk::{rest_client, transaction_builder::TransactionBuilder, types::LocalAccount};
use endless_types::{
    account_address::AccountAddress,
    chain_id::{ChainId, NamedChain},
    transaction::{
        authenticator::{
            AccountAuthenticator, AnyPublicKey, AnySignature, TransactionAuthenticator,
        },
        EntryFunction, RawTransaction, RawTransactionWithData, SignedTransaction,
        TransactionPayload,
    },
};
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use once_cell::sync::Lazy;
use std::time::{SystemTime, UNIX_EPOCH};

const CHAIN_ID: u8 = NamedChain::TESTNET as u8;
static CLIENT: Lazy<rest_client::Client> = Lazy::new(rest_client::Client::new_testnet);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let alice = LocalAccount::generate(&mut rng);
    let priv1 = Bls12381PrivateKey::generate(&mut rng);
    let priv2 = Bls12381PrivateKey::generate(&mut rng);
    fund(&alice).await?;
    fund_bls(&priv1).await?;
    fund_bls(&priv2).await?;
    add_auth_key(&alice, &priv1).await?;
    add_auth_key(&alice, &priv2).await?;
    update_signatures_required(&alice, 3).await?;
    send_coin_should_fail(&alice).await?;
    send_coin_should_fail2(&alice, &priv1).await?;
    send_coin_should_success(&alice, &priv1, &priv2).await?;
    Ok(())
}

async fn fund(account: &LocalAccount) -> anyhow::Result<()> {
    let txn_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("faucet").unwrap()),
            Identifier::new("fund").unwrap(),
            vec![],
            vec![account.address().into_bytes().to_vec()],
        )),
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let txn = account.sign_with_transaction_builder(txn_builder);
    let tx = CLIENT.submit_and_wait_bcs(&txn).await?.into_inner();
    println!(
        "[Fund] {}: {:?}",
        account.address().to_bs58_string_or_bcs(),
        tx.version
    );
    Ok(())
}

async fn fund_bls(priv_key: &Bls12381PrivateKey) -> anyhow::Result<()> {
    let addr = AuthenticationKey::bls12381(&priv_key.public_key()).account_address();
    let txn = RawTransaction::new(
        addr,
        0,
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("faucet").unwrap()),
            Identifier::new("fund").unwrap(),
            vec![],
            vec![addr.into_bytes().to_vec()],
        )),
        10000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let signature = priv_key.sign(&txn)?;
    let auth = TransactionAuthenticator::single_sender(AccountAuthenticator::bls12381(
        priv_key.public_key(),
        signature,
    ));
    let signed = SignedTransaction::new_with_authenticator(txn, auth);
    let tx = CLIENT.submit_and_wait_bcs(&signed).await?.into_inner();
    println!("[Fund] {}: {:?}", addr.to_bs58_string_or_bcs(), tx.version);
    Ok(())
}

async fn add_auth_key(alice: &LocalAccount, priv_key: &Bls12381PrivateKey) -> anyhow::Result<()> {
    let txn = RawTransaction::new(
        alice.address(),
        alice.increment_sequence_number(),
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("add_authentication_key").unwrap(),
            vec![],
            vec![],
        )),
        10000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let new_addr = AuthenticationKey::bls12381(&priv_key.public_key()).account_address();
    let message = RawTransactionWithData::new_multi_agent(txn.clone(), vec![new_addr]);
    let alice_signature = alice.private_key().sign(&message)?;
    let b_signature = priv_key.sign(&message)?;
    let auth = TransactionAuthenticator::multi_agent(
        AccountAuthenticator::ed25519(alice.public_key().clone(), alice_signature),
        vec![new_addr],
        vec![AccountAuthenticator::bls12381(
            priv_key.public_key(),
            b_signature,
        )],
    );
    let txn = SignedTransaction::new_with_authenticator(txn, auth);
    let tx = CLIENT.submit_and_wait_bcs(&txn).await?.into_inner();
    println!("[Add auth key] {:?}", tx.version);
    Ok(())
}

async fn update_signatures_required(
    account: &LocalAccount,
    num_signatures_required: u64,
) -> anyhow::Result<()> {
    let txn_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("set_num_signatures_required").unwrap(),
            vec![],
            vec![bcs::to_bytes(&num_signatures_required).unwrap()],
        )),
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let txn = account.sign_with_transaction_builder(txn_builder);
    let tx = CLIENT.submit_and_wait_bcs(&txn).await?.into_inner();
    println!("[Update signatures required] {:?}", tx.version);
    Ok(())
}

async fn send_coin_should_fail(account: &LocalAccount) -> anyhow::Result<()> {
    let txn = RawTransaction::new(
        account.address(),
        // Do not increment sequence number, because it will fail
        account.sequence_number(),
        transfer(AccountAddress::ONE, 1),
        5000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let txn = account.sign_transaction(txn);
    let err = CLIENT
        .submit_and_wait_bcs(&txn)
        .await
        .expect_err("Should fail");
    match err {
        rest_client::error::RestError::Api(err) => {
            let msg = &err.error.message;
            if !msg.contains("INVALID_AUTH_KEY") {
                bail!(rest_client::error::RestError::Api(err));
            }
            println!("[Fail] {}", msg);
        },
        err => bail!(err),
    }
    Ok(())
}

async fn send_coin_should_fail2(
    alice: &LocalAccount,
    priv1: &Bls12381PrivateKey,
) -> anyhow::Result<()> {
    let txn = RawTransaction::new(
        alice.address(),
        // Do not increment sequence number, because it will fail
        alice.sequence_number(),
        transfer(AccountAddress::ONE, 1),
        5000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let alice_signature = alice.private_key().sign(&txn)?;
    let priv1_signature = priv1.sign(&txn)?;
    let pub1 = priv1.public_key();
    let authenticator =
        TransactionAuthenticator::single_sender(AccountAuthenticator::multi_auth_key(
            vec![
                AnyPublicKey::ed25519(alice.public_key().clone()),
                AnyPublicKey::bls12381(vec![pub1]),
            ],
            vec![
                AnySignature::ed25519(alice_signature),
                AnySignature::bls12381(priv1_signature),
            ],
        ));
    let signed = SignedTransaction::new_with_authenticator(txn, authenticator);
    let err = CLIENT
        .submit_and_wait_bcs(&signed)
        .await
        .expect_err("Should fail");
    match err {
        rest_client::error::RestError::Api(err) => {
            let msg = &err.error.message;
            if !msg.contains("INVALID_AUTH_KEY") {
                bail!(rest_client::error::RestError::Api(err));
            }
            println!("[Fail] {}", msg);
        },
        err => bail!(err),
    }
    Ok(())
}

async fn send_coin_should_success(
    account: &LocalAccount,
    priv1: &Bls12381PrivateKey,
    priv2: &Bls12381PrivateKey,
) -> anyhow::Result<()> {
    let txn = RawTransaction::new(
        account.address(),
        account.increment_sequence_number(),
        transfer(AccountAddress::ONE, 1),
        5000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let alice_signature = account.private_key().sign(&txn)?;
    let priv1_signature = priv1.sign(&txn)?;
    let pub1 = priv1.public_key();
    let priv2_signature = priv2.sign(&txn)?;
    let pub2 = priv2.public_key();
    let agg_signature = Bls12381Signature::aggregate(vec![priv1_signature, priv2_signature])?;
    let authenticator =
        TransactionAuthenticator::single_sender(AccountAuthenticator::multi_auth_key(
            vec![
                AnyPublicKey::ed25519(account.public_key().clone()),
                AnyPublicKey::bls12381(vec![pub1, pub2]),
            ],
            vec![
                AnySignature::ed25519(alice_signature),
                AnySignature::bls12381(agg_signature),
            ],
        ));
    let signed = SignedTransaction::new_with_authenticator(txn, authenticator);
    let tx = CLIENT.submit_and_wait_bcs(&signed).await?.into_inner();
    println!("[Success] {:?}", tx.version);
    Ok(())
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn transfer(to: AccountAddress, amount: u128) -> TransactionPayload {
    TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            AccountAddress::ONE,
            Identifier::new("endless_coin").unwrap(),
        ),
        Identifier::new("transfer").unwrap(),
        vec![],
        vec![bcs::to_bytes(&to).unwrap(), bcs::to_bytes(&amount).unwrap()],
    ))
}
