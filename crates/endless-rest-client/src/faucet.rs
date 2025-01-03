// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Result};
use endless_api_types::TransactionOnChainData;
use endless_cached_packages::endless_stdlib;
use endless_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    PrivateKey, Uniform,
};
use endless_types::{
    chain_id::ChainId,
    transaction::{
        authenticator::AuthenticationKey, RawTransaction, SignedTransaction, TransactionPayload,
    },
};
use move_core_types::account_address::AccountAddress;
use reqwest::Url;
use std::{
    sync::atomic::{AtomicU64, AtomicU8, Ordering},
    time::SystemTime,
};

struct Account {
    private_key: Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    address: AccountAddress,
}

impl Account {
    fn generate() -> Self {
        let private_key = Ed25519PrivateKey::generate(&mut rand::rngs::OsRng);
        let public_key = private_key.public_key();
        let address = AuthenticationKey::ed25519(&public_key).account_address();
        Self {
            private_key,
            public_key,
            address,
        }
    }

    fn sign_transaction(&self, txn: RawTransaction) -> SignedTransaction {
        txn.sign(&self.private_key, self.public_key.clone())
            .expect("Signing a txn can't fail")
            .into_inner()
    }
}

pub struct FaucetClient {
    rest_client: Client,
    signer: Account,
    seq: AtomicU64,
    chain_id: AtomicU8,
}

impl FaucetClient {
    pub async fn new(rest_url: Url) -> Self {
        Self::new_from_rest_client(Client::new(rest_url)).await
    }

    pub async fn new_for_testing(rest_url: Url) -> Self {
        Self::new_from_rest_client(
            Client::new(rest_url)
                // By default the path is prefixed with the version, e.g. `v1`.
                // The fake API used in the faucet tests doesn't have a
                // versioned API however, so we just set it to `/`.
                .version_path_base("/".to_string())
                .unwrap(),
        )
        .await
    }

    pub async fn new_from_rest_client(rest_client: Client) -> Self {
        let signer = Account::generate();
        let signer_addr = signer.address;
        let client = Self {
            rest_client,
            signer,
            seq: AtomicU64::new(0),
            chain_id: AtomicU8::new(0),
        };
        client.fund(signer_addr).await.unwrap();
        client
    }

    /// Create an account with zero balance.
    pub async fn create_account(&self, address: AccountAddress) -> Result<()> {
        self.submit_txn(endless_stdlib::endless_account_create_account(address))
            .await?;
        Ok(())
    }

    /// Fund an account with the given amount.
    pub async fn fund(&self, address: AccountAddress) -> Result<()> {
        self.submit_txn(endless_stdlib::faucet_fund(address))
            .await?;
        Ok(())
    }

    async fn submit_txn(&self, payload: TransactionPayload) -> Result<TransactionOnChainData> {
        if self.chain_id.load(Ordering::Relaxed) == 0 {
            let chain_id = self
                .rest_client
                .get_ledger_information()
                .await?
                .into_inner()
                .chain_id;
            let _ =
                self.chain_id
                    .compare_exchange(0, chain_id, Ordering::Relaxed, Ordering::Relaxed);
        }

        let txn = RawTransaction::new(
            self.signer.address,
            self.seq.load(Ordering::Relaxed),
            payload,
            100000,
            100,
            now() + 60,
            ChainId::new(self.chain_id.load(Ordering::Relaxed)),
        );
        let signed = self.signer.sign_transaction(txn);
        let resp = self.rest_client.submit_and_wait_bcs(&signed).await?;
        self.seq.fetch_add(1, Ordering::Relaxed);
        Ok(resp.into_inner())
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
