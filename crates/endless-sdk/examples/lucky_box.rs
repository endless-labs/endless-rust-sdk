use anyhow::{Ok, Result};
use bytes::Buf;
use ed25519_dalek_bip32::ed25519_dalek::Digest;
use endless_crypto::compat::Sha3_256;
use endless_sdk::{rest_client, transaction_builder::TransactionBuilder, types::LocalAccount};
use endless_types::{
    account_address::AccountAddress,
    chain_id::ChainId,
    transaction::{EntryFunction, TransactionPayload},
};
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use once_cell::sync::Lazy;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

static MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        AccountAddress::from_str("55vZofHKEZD3pC99z672xLb1gkVBLcRFpQ5ZPXUghHur").unwrap(),
        Identifier::from_str("lucky_box").unwrap(),
    )
});

static CLIENT: Lazy<rest_client::Client> =
    Lazy::new(|| rest_client::Client::new(Url::parse("https://rpc-test.endless.link").unwrap()));

static CHAIN_ID: Lazy<ChainId> = Lazy::new(ChainId::testnet);

#[tokio::main]
async fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let a = LocalAccount::generate(&mut rng);
    let b = LocalAccount::generate(&mut rng);
    let c = LocalAccount::generate(&mut rng);
    fund(&a).await?;
    let box_id: AccountAddress = pack(&a, &[&b, &c]).await;
    unpack(box_id, &[&b, &c], &b).await;
    Ok(())
}

/// `unpackers` is the list of accounts that can unpack the box
async fn pack(packer: &LocalAccount, unpackers: &[&LocalAccount]) -> AccountAddress {
    let root = calc_merkle_root(&unpackers.iter().map(|a| a.address()).collect::<Vec<_>>());
    let args = vec![
        // amount
        bcs::to_bytes(&10000000u64).unwrap(),
        // count
        bcs::to_bytes(&1u64).unwrap(),
        // true for random box
        bcs::to_bytes(&true).unwrap(),
        // box expire time
        bcs::to_bytes(&(now() + 30)).unwrap(),
        // merkle tree root
        bcs::to_bytes(&root).unwrap(),
    ];
    let tx_builder = TransactionBuilder::new(
        entry_fn("pack_with_merkle_tree", args),
        now() + 10,
        *CHAIN_ID,
    );
    let txn = packer.sign_with_transaction_builder(tx_builder);
    let resp = CLIENT.submit_and_wait(&txn).await.unwrap().into_inner();
    let pack_event = format!(
        "{}::{}::Packing",
        MODULE.address().to_bs58_string(),
        MODULE.name()
    );
    let packing_evt = resp
        .events()
        .unwrap()
        .iter()
        .find(|evt| evt.typ.to_string() == pack_event)
        .unwrap();
    let box_id = AccountAddress::from_str(
        packing_evt
            .data
            .as_object()
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();
    println!("Packed: {}", box_id.to_bs58_string());
    box_id
}

/// `unpackers` is the list of accounts that can unpack the box
/// `unpackers` must be the same as when packing this box
/// `unpacker` is the caller
async fn unpack(box_id: AccountAddress, unpackers: &[&LocalAccount], unpakcer: &LocalAccount) {
    let (path, bitmask) = calc_merkle_path(
        &unpackers.iter().map(|a| a.address()).collect::<Vec<_>>(),
        unpakcer.address(),
    );
    let args = vec![
        bcs::to_bytes(&box_id).unwrap(),
        bcs::to_bytes(&path).unwrap(),
        bcs::to_bytes(&bitmask).unwrap(),
    ];
    let txn_builder = TransactionBuilder::new(
        entry_fn("unpack_with_merkle_tree", args),
        now() + 10,
        *CHAIN_ID,
    );
    let txn = unpakcer.sign_with_transaction_builder(txn_builder);
    let resp = CLIENT.submit_and_wait(&txn).await.unwrap().into_inner();
    println!("Unpacked: {:?}", resp.version());
}

// Merkle Tree: https://en.wikipedia.org/wiki/Merkle_tree
fn calc_merkle_root(unpackers: &[AccountAddress]) -> u64 {
    assert!(!unpackers.is_empty());
    let mut nodes: Vec<u64> = unpackers.iter().map(|a| addr_to_u64(*a)).collect();
    while nodes.len() > 1 {
        let mut next = vec![];
        for i in (0..nodes.len()).step_by(2) {
            let lhs = nodes[i];
            let rhs = nodes.get(i + 1).cloned().unwrap_or(lhs);
            let hash = hash_to_u64(lhs, rhs);
            next.push(hash);
        }
        nodes = next;
    }
    nodes[0]
}

fn calc_merkle_path(unpackers: &[AccountAddress], unpacker: AccountAddress) -> (Vec<u64>, u64) {
    let mut index = unpackers.iter().position(|a| *a == unpacker).unwrap();
    let mut bitmask = 0u64;
    let mut depth = 0;
    let mut nodes: Vec<u64> = unpackers.iter().map(|a| addr_to_u64(*a)).collect();
    let mut path = vec![];
    while nodes.len() > 1 {
        let mut next = vec![];
        for i in (0..nodes.len()).step_by(2) {
            let lhs = nodes[i];
            let rhs = nodes.get(i + 1).cloned().unwrap_or(lhs);
            let hash = hash_to_u64(lhs, rhs);
            next.push(hash);
            if path.len() == depth {
                if index == i {
                    path.push(rhs);
                    index /= 2;
                } else if index == i + 1 {
                    path.push(lhs);
                    bitmask |= 1 << depth;
                    index /= 2;
                }
            }
        }
        depth += 1;
        nodes = next;
    }
    path.reverse();
    (path, bitmask)
}

fn hash_to_u64(lhs: u64, rhs: u64) -> u64 {
    let mut bytes = Vec::with_capacity(16);
    bytes.extend_from_slice(&lhs.to_be_bytes());
    bytes.extend_from_slice(&rhs.to_be_bytes());
    let hashed = Sha3_256::digest(&bytes).to_vec();
    (&hashed[..8]).get_u64()
}

fn addr_to_u64(addr: AccountAddress) -> u64 {
    (&addr.into_bytes()[..8]).get_u64()
}

fn entry_fn(func: &str, args: Vec<Vec<u8>>) -> TransactionPayload {
    TransactionPayload::EntryFunction(EntryFunction::new(
        MODULE.clone(),
        Identifier::new(func).unwrap(),
        vec![],
        args,
    ))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn fund(account: &LocalAccount) -> Result<()> {
    let txn_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("faucet").unwrap()),
            Identifier::new("fund").unwrap(),
            vec![],
            vec![account.address().into_bytes().to_vec()],
        )),
        now() + 30,
        *CHAIN_ID,
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
