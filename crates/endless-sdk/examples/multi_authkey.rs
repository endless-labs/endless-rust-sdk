use endless_crypto::SigningKey;
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
    let bob = LocalAccount::generate(&mut rng);
    let charlie = LocalAccount::generate(&mut rng);
    fund(&alice).await?;
    fund(&bob).await?;
    fund(&charlie).await?;
    update_account(&alice, &bob, 2).await?;
    send_coin_should_fail(&alice).await?;
    send_coin_should_success(&alice, &bob).await?;
    add_auth_key(&alice, &bob, &charlie).await?;
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
    let tx = CLIENT.submit_and_wait(&txn).await?.into_inner();
    println!(
        "[Fund] {}: {:?}",
        account.address().to_bs58_string_or_bcs(),
        tx.version()
    );
    Ok(())
}

async fn update_account(
    alice: &LocalAccount,
    bob: &LocalAccount,
    num_signatures_required: u64,
) -> anyhow::Result<()> {
    let txn_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("batch_add_authentication_key").unwrap(),
            vec![],
            vec![bcs::to_bytes(&num_signatures_required).unwrap()],
        )),
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let txn = alice.sign_multi_agent_with_transaction_builder(vec![bob], txn_builder);
    let tx = CLIENT.submit_and_wait(&txn).await?.into_inner();
    println!("[Update account] {:?}", tx.version());
    Ok(())
}

async fn add_auth_key(
    alice: &LocalAccount,
    bob: &LocalAccount,
    charlie: &LocalAccount,
) -> anyhow::Result<()> {
    let raw_txn = RawTransaction::new(
        alice.address(),
        alice.increment_sequence_number(),
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("add_authentication_key").unwrap(),
            vec![],
            vec![],
        )),
        5000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let message = RawTransactionWithData::new_multi_agent(raw_txn.clone(), vec![charlie.address()]);
    let alice_sig = alice.private_key().sign(&message)?;
    let bob_sig = bob.private_key().sign(&message)?;
    let charlie_sig = charlie.private_key().sign(&message)?;
    let auth = AccountAuthenticator::multi_auth_key(
        vec![
            AnyPublicKey::ed25519(alice.public_key().clone()),
            AnyPublicKey::ed25519(bob.public_key().clone()),
        ],
        vec![
            AnySignature::ed25519(alice_sig),
            AnySignature::ed25519(bob_sig),
        ],
    );
    let charlie_auth = AccountAuthenticator::ed25519(charlie.public_key().clone(), charlie_sig);
    let txn_auth =
        TransactionAuthenticator::multi_agent(auth, vec![charlie.address()], vec![charlie_auth]);
    let signed = SignedTransaction::new_with_authenticator(raw_txn, txn_auth);
    println!("tx hash: {:?}", signed.clone().committed_hash());
    let tx = CLIENT.submit_and_wait(&signed).await?.into_inner();
    println!("[Add Success] {:?}", tx.version());
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
    let err = CLIENT.submit_and_wait(&txn).await.expect_err("Should fail");
    match err {
        rest_client::error::RestError::Api(err) => {
            println!("[Fail]: {}", err.error.message);
        },
        _ => println!("[Fail] {:?}", err),
    }
    Ok(())
}

async fn send_coin_should_success(alice: &LocalAccount, bob: &LocalAccount) -> anyhow::Result<()> {
    let txn = RawTransaction::new(
        alice.address(),
        alice.increment_sequence_number(),
        transfer(AccountAddress::ONE, 1),
        5000,
        100,
        now() + 30,
        ChainId::new(CHAIN_ID),
    );
    let alice_signature = alice.private_key().sign(&txn)?;
    let bob_signature = bob.private_key().sign(&txn)?;
    let authenticator =
        TransactionAuthenticator::single_sender(AccountAuthenticator::multi_auth_key(
            vec![
                AnyPublicKey::ed25519(alice.public_key().clone()),
                AnyPublicKey::ed25519(bob.public_key().clone()),
            ],
            vec![
                AnySignature::ed25519(alice_signature),
                AnySignature::ed25519(bob_signature),
            ],
        ));
    let signed = SignedTransaction::new_with_authenticator(txn, authenticator);
    println!("tx hash: {:?}", signed.clone().committed_hash());
    let tx = CLIENT.submit_and_wait(&signed).await?.into_inner();
    println!("[Success] {:?}", tx.version());
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
