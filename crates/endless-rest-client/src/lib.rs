// Copyright © Endless
// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

extern crate core;

pub mod endless;
pub mod error;
pub mod faucet;
pub use faucet::FaucetClient;
pub mod response;
pub use response::Response;
pub mod client_builder;
pub mod state;
pub mod types;

pub use crate::client_builder::{ClientBuilder, EndlessBaseUrl};
use crate::{
    endless::{Balance, EndlessCoin, EndlessVersion},
    error::RestError,
};
use anyhow::{anyhow, Result};
use core::fmt;
pub use endless_api_types::{
    self, transaction::VersionedEventV2, IndexResponseBcs, MoveModuleBytecode, PendingTransaction,
    Transaction,
};
use endless_api_types::{
    deserialize_from_string,
    mime_types::{BCS, BCS_SIGNED_TRANSACTION, BCS_VIEW_FUNCTION, JSON},
    BcsBlock, Block, EndlessError, EntryFunctionId, GasEstimation, HexEncodedBytes, IndexResponse,
    MoveModuleId, TransactionData, TransactionOnChainData, TransactionsBatchSubmissionResult,
    UserTransaction, VersionedEvent, ViewFunction, ViewRequest,
};
use endless_crypto::HashValue;
use endless_logger::{debug, info, sample, sample::SampleRate};
use endless_types::{
    account_address::AccountAddress,
    account_config::{AccountResource, NewBlockEvent, CORE_CODE_ADDRESS},
    contract_event::{EventWithVersion, EventWithVersionIdx},
    state_store::state_key::StateKey,
    transaction::SignedTransaction,
};
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, StructTag},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client as ReqwestClient, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
pub use state::State;
use std::{collections::BTreeMap, future::Future, ops::RangeBounds, str::FromStr, time::Duration};
use tokio::time::Instant;
pub use types::{deserialize_from_prefixed_hex_string, Account, QueryRange, Resource};
use url::Url;

pub const DEFAULT_VERSION_PATH_BASE: &str = "v1/";
const DEFAULT_MAX_WAIT_MS: u64 = 60000;
const DEFAULT_INTERVAL_MS: u64 = 1000;
static DEFAULT_MAX_WAIT_DURATION: Duration = Duration::from_millis(DEFAULT_MAX_WAIT_MS);
static DEFAULT_INTERVAL_DURATION: Duration = Duration::from_millis(DEFAULT_INTERVAL_MS);
const DEFAULT_MAX_SERVER_LAG_WAIT_DURATION: Duration = Duration::from_secs(60);
const RESOURCES_PER_CALL_PAGINATION: u64 = 9999;
const MODULES_PER_CALL_PAGINATION: u64 = 1000;
const X_ENDLESS_SDK_HEADER_VALUE: &str = concat!("endless-rust-sdk/", env!("CARGO_PKG_VERSION"));

pub type EndlessResult<T> = Result<T, RestError>;

#[derive(Clone, Debug)]
pub struct Client {
    inner: ReqwestClient,
    base_url: Url,
    version_path_base: String,
}

impl Client {
    pub fn builder(endless_base_url: EndlessBaseUrl) -> ClientBuilder {
        ClientBuilder::new(endless_base_url)
    }

    pub fn new(base_url: Url) -> Self {
        Self::builder(EndlessBaseUrl::Custom(base_url)).build()
    }

    pub fn new_local() -> Self {
        Self::builder(EndlessBaseUrl::Local).build()
    }

    pub fn new_testnet() -> Self {
        Self::builder(EndlessBaseUrl::Testnet).build()
    }

    pub fn path_prefix_string(&self) -> String {
        self.base_url
            .join(&self.version_path_base)
            .map(|path| path.to_string())
            .unwrap_or_else(|_| "<bad_base_url>".to_string())
    }

    /// Set a different version path base, e.g. "v1/" See
    /// DEFAULT_VERSION_PATH_BASE for the default value.
    pub fn version_path_base(mut self, version_path_base: String) -> EndlessResult<Self> {
        if !version_path_base.ends_with('/') {
            return Err(anyhow!("version_path_base must end with '/', e.g. 'v1/'").into());
        }
        self.version_path_base = version_path_base;
        Ok(self)
    }

    pub fn build_path(&self, path: &str) -> EndlessResult<Url> {
        Ok(self.base_url.join(&self.version_path_base)?.join(path)?)
    }

    pub async fn get_endless_version(&self) -> EndlessResult<Response<EndlessVersion>> {
        self.get_resource::<EndlessVersion>(CORE_CODE_ADDRESS, "0x1::version::Version")
            .await
    }

    pub async fn get_block_by_height(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> EndlessResult<Response<Block>> {
        self.get(self.build_path(&format!(
            "blocks/by_height/{}?with_transactions={}",
            height, with_transactions
        ))?)
        .await
    }

    pub async fn get_block_by_height_bcs(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> EndlessResult<Response<BcsBlock>> {
        let url = self.build_path(&format!(
            "blocks/by_height/{}?with_transactions={}",
            height, with_transactions
        ))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    /// This will get all the transactions from the block in successive calls
    /// and will handle the successive calls
    ///
    /// Note: This could take a long time to run
    pub async fn get_full_block_by_height_bcs(
        &self,
        height: u64,
        page_size: u16,
    ) -> EndlessResult<Response<BcsBlock>> {
        let (mut block, state) = self
            .get_block_by_height_bcs(height, true)
            .await?
            .into_parts();

        let mut current_version = block.first_version;

        // Set the current version to the last known transaction
        if let Some(ref txns) = block.transactions {
            if let Some(txn) = txns.last() {
                current_version = txn.version + 1;
            }
        } else {
            return Err(RestError::Unknown(anyhow!(
                "No transactions were returned in the block"
            )));
        }

        // Add in all transactions by paging through the other transactions
        while current_version <= block.last_version {
            let page_end_version =
                std::cmp::min(block.last_version, current_version + page_size as u64 - 1);

            let transactions = self
                .get_transactions_bcs(
                    Some(current_version),
                    Some((page_end_version - current_version + 1) as u16),
                )
                .await?
                .into_inner();
            if let Some(txn) = transactions.last() {
                current_version = txn.version + 1;
            };
            block.transactions.as_mut().unwrap().extend(transactions);
        }

        Ok(Response::new(block, state))
    }

    pub async fn get_block_by_version(
        &self,
        version: u64,
        with_transactions: bool,
    ) -> EndlessResult<Response<Block>> {
        self.get(self.build_path(&format!(
            "blocks/by_version/{}?with_transactions={}",
            version, with_transactions
        ))?)
        .await
    }

    pub async fn get_block_by_version_bcs(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> EndlessResult<Response<BcsBlock>> {
        let url = self.build_path(&format!(
            "blocks/by_version/{}?with_transactions={}",
            height, with_transactions
        ))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_balance(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<Balance>> {
        self.get_account_balance_inner(address, None).await
    }

    // invoke "endless_coin::balance(acc_addr)" to get Native Coin(FA) balance
    async fn get_account_balance_inner(
        &self,
        address: AccountAddress,
        version: Option<u64>,
    ) -> EndlessResult<Response<Balance>> {
        let view_req = ViewRequest {
            function: EntryFunctionId::from_str("0x1::endless_coin::balance").unwrap(),
            type_arguments: vec![],
            arguments: vec![serde_json::Value::String(address.to_standard_string())],
        };
        let resp = self.view(&view_req, version).await?;
        resp.and_then(|info| {
            let value = info.first().unwrap();
            let balance = value.as_str().unwrap().parse::<u128>().unwrap();
            Ok(Balance {
                coin: EndlessCoin {
                    value: endless_api_types::U128(balance),
                },
            })
        })
    }

    // only used in faucet to query and transfer native coin
    // so here omit coin type param
    pub async fn get_account_balance_bcs(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<u64>> {
        // let resp = self
        //     .get_account_resource_bcs::<CoinStoreResource>(
        //         address,
        //         &format!("0x1::coin::CoinStore<{}>", coin_type),
        //     )
        //     .await?;
        // resp.and_then(|resource| Ok(resource.coin()))
        self.get_account_balance_bcs_inner(address).await
    }

    // TODO: fetch user custom FA amount
    async fn get_account_balance_bcs_inner(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<u64>> {
        let view_fun = ViewFunction {
            module: ModuleId::new(
                AccountAddress::from_str("0x1").unwrap(),
                Identifier::from_str("endless_coin").unwrap(),
            ),
            function: Identifier::from_str("balance").unwrap(),
            ty_args: vec![],
            args: vec![address.to_vec()],
        };
        let resp = self.view_bcs(&view_fun, None).await?;
        println!("get_account_balance_bcs_inner, resp:{:?}", resp);
        resp.and_then(|balance: u64| {
            // let value = info.get(0).unwrap();
            // let balance = value.as_str().unwrap().parse::<u64>().unwrap();
            Ok(balance)
        })
    }

    pub async fn get_account_balance_at_version(
        &self,
        address: AccountAddress,
        version: u64,
    ) -> EndlessResult<Response<Balance>> {
        self.get_account_balance_inner(address, Some(version)).await
    }

    pub async fn get_index(&self) -> EndlessResult<Response<IndexResponse>> {
        self.get(self.build_path("")?).await
    }

    pub async fn get_index_bcs(&self) -> EndlessResult<Response<IndexResponseBcs>> {
        let url = self.build_path("")?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    // TODO: Remove this, just use `get_index`: https://github.com/endless-labs/endless-core/issues/5597.
    pub async fn get_ledger_information(&self) -> EndlessResult<Response<State>> {
        let response = self.get_index_bcs().await?.map(|r| State {
            chain_id: r.chain_id,
            epoch: r.epoch.into(),
            version: r.ledger_version.into(),
            timestamp_usecs: r.ledger_timestamp.into(),
            oldest_ledger_version: r.oldest_ledger_version.into(),
            oldest_block_height: r.oldest_block_height.into(),
            block_height: r.block_height.into(),
            cursor: None,
        });
        assert_eq!(response.inner().chain_id, response.state().chain_id);
        assert_eq!(response.inner().epoch, response.state().epoch);
        assert_eq!(response.inner().version, response.state().version);
        assert_eq!(response.inner().block_height, response.state().block_height);

        Ok(response)
    }

    pub async fn view(
        &self,
        request: &ViewRequest,
        version: Option<u64>,
    ) -> EndlessResult<Response<Vec<serde_json::Value>>> {
        let request = serde_json::to_string(request)?;
        let mut url = self.build_path("view")?;
        if let Some(version) = version {
            url.set_query(Some(format!("ledger_version={}", version).as_str()));
        }

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, JSON)
            .body(request)
            .send()
            .await?;

        self.json(response).await
    }

    pub async fn view_bcs<T: DeserializeOwned>(
        &self,
        request: &ViewFunction,
        version: Option<u64>,
    ) -> EndlessResult<Response<T>> {
        let txn_payload = bcs::to_bytes(request)?;
        let mut url = self.build_path("view")?;
        if let Some(version) = version {
            url.set_query(Some(format!("ledger_version={}", version).as_str()));
        }

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_VIEW_FUNCTION)
            .header(ACCEPT, BCS)
            .body(txn_payload)
            .send()
            .await?;

        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|bytes| bcs::from_bytes(&bytes))?)
    }

    pub async fn simulate(
        &self,
        txn: &SignedTransaction,
    ) -> EndlessResult<Response<Vec<UserTransaction>>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions/simulate")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;

        self.json(response).await
    }

    pub async fn simulate_with_gas_estimation(
        &self,
        txn: &SignedTransaction,
        estimate_max_gas_amount: bool,
        estimate_max_gas_unit_price: bool,
    ) -> EndlessResult<Response<Vec<UserTransaction>>> {
        let txn_payload = bcs::to_bytes(txn)?;

        let url = self.build_path(&format!(
            "transactions/simulate?estimate_max_gas_amount={}&estimate_gas_unit_price={}",
            estimate_max_gas_amount, estimate_max_gas_unit_price
        ))?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;

        self.json(response).await
    }

    pub async fn simulate_bcs(
        &self,
        txn: &SignedTransaction,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions/simulate")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .header(ACCEPT, BCS)
            .body(txn_payload)
            .send()
            .await?;

        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|bytes| bcs::from_bytes(&bytes))?)
    }

    pub async fn simulate_bcs_with_gas_estimation(
        &self,
        txn: &SignedTransaction,
        estimate_max_gas_amount: bool,
        estimate_max_gas_unit_price: bool,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path(&format!(
            "transactions/simulate?estimate_max_gas_amount={}&estimate_gas_unit_price={}",
            estimate_max_gas_amount, estimate_max_gas_unit_price
        ))?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .header(ACCEPT, BCS)
            .body(txn_payload)
            .send()
            .await?;

        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|bytes| bcs::from_bytes(&bytes))?)
    }

    pub async fn submit(
        &self,
        txn: &SignedTransaction,
    ) -> EndlessResult<Response<PendingTransaction>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;

        self.json::<PendingTransaction>(response).await
    }

    pub async fn submit_without_serializing_response(&self, txn: &SignedTransaction) -> Result<()> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;

        self.check_response(response).await?;
        Ok(())
    }

    pub async fn submit_bcs(&self, txn: &SignedTransaction) -> EndlessResult<Response<()>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .header(ACCEPT, BCS)
            .body(txn_payload)
            .send()
            .await?;

        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|bytes| bcs::from_bytes(&bytes))?)
    }

    pub async fn submit_batch(
        &self,
        txns: &[SignedTransaction],
    ) -> EndlessResult<Response<TransactionsBatchSubmissionResult>> {
        let txn_payload = bcs::to_bytes(&txns.to_vec())?;
        let url = self.build_path("transactions/batch")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;
        self.json(response).await
    }

    pub async fn submit_batch_bcs(
        &self,
        txns: &[SignedTransaction],
    ) -> EndlessResult<Response<TransactionsBatchSubmissionResult>> {
        let txn_payload = bcs::to_bytes(&txns.to_vec())?;
        let url = self.build_path("transactions/batch")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .header(ACCEPT, BCS)
            .body(txn_payload)
            .send()
            .await?;

        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|bytes| bcs::from_bytes(&bytes))?)
    }

    pub async fn submit_and_wait(
        &self,
        txn: &SignedTransaction,
    ) -> EndlessResult<Response<Transaction>> {
        self.submit(txn).await?;
        self.wait_for_signed_transaction(txn).await
    }

    pub async fn submit_and_wait_bcs(
        &self,
        txn: &SignedTransaction,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        self.submit_bcs(txn).await?;
        self.wait_for_signed_transaction_bcs(txn).await
    }

    pub async fn wait_for_transaction(
        &self,
        pending_transaction: &PendingTransaction,
    ) -> EndlessResult<Response<Transaction>> {
        self.wait_for_transaction_by_hash(
            pending_transaction.hash.into(),
            *pending_transaction
                .request
                .expiration_timestamp_secs
                .inner(),
            Some(DEFAULT_MAX_SERVER_LAG_WAIT_DURATION),
            None,
        )
        .await
    }

    pub async fn wait_for_transaction_bcs(
        &self,
        pending_transaction: &PendingTransaction,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        self.wait_for_transaction_by_hash_bcs(
            pending_transaction.hash.into(),
            *pending_transaction
                .request
                .expiration_timestamp_secs
                .inner(),
            Some(DEFAULT_MAX_SERVER_LAG_WAIT_DURATION),
            None,
        )
        .await
    }

    pub async fn wait_for_signed_transaction(
        &self,
        transaction: &SignedTransaction,
    ) -> EndlessResult<Response<Transaction>> {
        let expiration_timestamp = transaction.expiration_timestamp_secs();
        self.wait_for_transaction_by_hash(
            transaction.clone().committed_hash(),
            expiration_timestamp,
            Some(DEFAULT_MAX_SERVER_LAG_WAIT_DURATION),
            None,
        )
        .await
    }

    pub async fn wait_for_signed_transaction_bcs(
        &self,
        transaction: &SignedTransaction,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        let expiration_timestamp = transaction.expiration_timestamp_secs();
        self.wait_for_transaction_by_hash_bcs(
            transaction.clone().committed_hash(),
            expiration_timestamp,
            Some(DEFAULT_MAX_SERVER_LAG_WAIT_DURATION),
            None,
        )
        .await
    }

    /// Implementation of waiting for a transaction
    /// * `hash`: hash of the submitted transaction
    /// * `expiration_timestamp_secs`: expiration time of the submitted transaction
    /// * `max_server_lag_wait`:
    ///     Fullnodes generally lag some amount behind the authoritative blockchain ledger state.
    ///     This field gives the node some time to update its ledger state to the point
    ///     where your transaction might have expired.
    ///     We recommend setting this value to at least 60 seconds.
    /// * `timeout_from_call`:
    ///     When an absolute timeout for this function is needed,
    ///     irrespective of whether expiry time is reached.
    async fn wait_for_transaction_by_hash_inner<F, Fut, T>(
        &self,
        hash: HashValue,
        expiration_timestamp_secs: u64,
        max_server_lag_wait: Option<Duration>,

        timeout_from_call: Option<Duration>,
        fetch: F,
    ) -> EndlessResult<Response<T>>
    where
        F: Fn(HashValue) -> Fut,
        Fut: Future<Output = EndlessResult<WaitForTransactionResult<T>>>,
    {
        // TODO: make this configurable
        const DEFAULT_DELAY: Duration = Duration::from_millis(500);
        let mut reached_mempool = false;
        let start = std::time::Instant::now();
        loop {
            let mut chain_timestamp_usecs = None;
            match fetch(hash).await {
                Ok(WaitForTransactionResult::Success(result)) => {
                    return Ok(result);
                },
                Ok(WaitForTransactionResult::FailedExecution(vm_status)) => {
                    return Err(anyhow!(
                        "Transaction committed on chain, but failed execution: {}",
                        vm_status
                    ))?;
                },
                Ok(WaitForTransactionResult::Pending(state)) => {
                    reached_mempool = true;
                    if expiration_timestamp_secs <= state.timestamp_usecs / 1_000_000 {
                        return Err(anyhow!("Transaction expired. It is guaranteed it will not be committed on chain.").into());
                    }
                    chain_timestamp_usecs = Some(state.timestamp_usecs);
                },
                Ok(WaitForTransactionResult::NotFound(error)) => {
                    if let RestError::Api(endless_error_response) = error {
                        if let Some(state) = endless_error_response.state {
                            if expiration_timestamp_secs <= state.timestamp_usecs / 1_000_000 {
                                if reached_mempool {
                                    return Err(anyhow!("Transaction expired. It is guaranteed it will not be committed on chain.").into());
                                } else {
                                    // We want to know whether we ever got Pending state from the mempool,
                                    // to warn in case we didn't.
                                    // Unless we are calling endpoint that is a very large load-balanced pool of nodes,
                                    // we should always see pending after submitting a transaction.
                                    // (i.e. if we hit the node we submitted a transaction to,
                                    // it shouldn't return NotFound on the first call)
                                    //
                                    // At the end, when the expiration happens, we might get NotFound or Pending
                                    // based on whether GC run on the full node to remove expired transaction,
                                    // so that information is not useful. So we need to keep this variable as state.
                                    return Err(anyhow!("Transaction expired, without being seen in mempool. It is guaranteed it will not be committed on chain.").into());
                                }
                            }
                            chain_timestamp_usecs = Some(state.timestamp_usecs);
                        }
                    } else {
                        return Err(error);
                    }
                    sample!(
                        SampleRate::Duration(Duration::from_secs(30)),
                        debug!(
                            "Cannot yet find transaction in mempool on {:?}, continuing to wait.",
                            self.path_prefix_string(),
                        )
                    );
                },
                Err(err) => {
                    debug!("Fetching error, will retry: {}", err);
                },
            }

            if let Some(max_server_lag_wait_duration) = max_server_lag_wait {
                if endless_infallible::duration_since_epoch().as_secs()
                    > expiration_timestamp_secs + max_server_lag_wait_duration.as_secs()
                {
                    return Err(anyhow!(
                        "Ledger on endpoint ({}) is more than {}s behind current time, timing out waiting for the transaction. Warning, transaction ({}) might still succeed.",
                        self.path_prefix_string(),
                        max_server_lag_wait_duration.as_secs(),
                        hash,
                    ).into());
                }
            }

            let elapsed = start.elapsed();
            if let Some(timeout_duration) = timeout_from_call {
                if elapsed > timeout_duration {
                    return Err(anyhow!(
                        "Timeout of {}s after calling wait_for_transaction reached. Warning, transaction ({}) might still succeed.",
                        timeout_duration.as_secs(),
                        hash,
                    ).into());
                }
            }

            if elapsed.as_secs() > 30 {
                sample!(
                    SampleRate::Duration(Duration::from_secs(30)),
                    debug!(
                        "Continuing to wait for transaction {}, ledger on endpoint ({}) is {}",
                        hash,
                        self.path_prefix_string(),
                        if let Some(timestamp_usecs) = chain_timestamp_usecs {
                            format!(
                                "{}s behind current time",
                                endless_infallible::duration_since_epoch()
                                    .saturating_sub(Duration::from_micros(timestamp_usecs))
                                    .as_secs()
                            )
                        } else {
                            "unreachable".to_string()
                        },
                    )
                );
            }

            tokio::time::sleep(DEFAULT_DELAY).await;
        }
    }

    pub async fn wait_for_transaction_by_hash(
        &self,
        hash: HashValue,
        expiration_timestamp_secs: u64,
        max_server_lag_wait: Option<Duration>,
        timeout_from_call: Option<Duration>,
    ) -> EndlessResult<Response<Transaction>> {
        self.wait_for_transaction_by_hash_inner(
            hash,
            expiration_timestamp_secs,
            max_server_lag_wait,
            timeout_from_call,
            |hash| async move {
                let resp = self.get_transaction_by_hash_inner(hash).await?;
                if resp.status() != StatusCode::NOT_FOUND {
                    let txn_resp: Response<Transaction> = self.json(resp).await?;
                    let (transaction, state) = txn_resp.into_parts();

                    if !transaction.is_pending() {
                        if !transaction.success() {
                            Ok(WaitForTransactionResult::FailedExecution(
                                transaction.vm_status(),
                            ))
                        } else {
                            Ok(WaitForTransactionResult::Success(Response::new(
                                transaction,
                                state,
                            )))
                        }
                    } else {
                        Ok(WaitForTransactionResult::Pending(state))
                    }
                } else {
                    let error_response = parse_error(resp).await;
                    Ok(WaitForTransactionResult::NotFound(error_response))
                }
            },
        )
        .await
    }

    pub async fn wait_for_transaction_by_hash_bcs(
        &self,
        hash: HashValue,
        expiration_timestamp_secs: u64,
        max_server_lag_wait: Option<Duration>,
        timeout_from_call: Option<Duration>,
    ) -> EndlessResult<Response<TransactionOnChainData>> {
        self.wait_for_transaction_by_hash_inner(
            hash,
            expiration_timestamp_secs,
            max_server_lag_wait,
            timeout_from_call,
            |hash| async move {
                let resp = self.get_transaction_by_hash_bcs_inner(hash).await?;
                if resp.status() != StatusCode::NOT_FOUND {
                    let resp = self.check_and_parse_bcs_response(resp).await?;
                    let resp = resp.and_then(|bytes| bcs::from_bytes(&bytes))?;
                    let (maybe_pending_txn, state) = resp.into_parts();

                    // If we have a committed transaction, determine if it failed or not
                    if let TransactionData::OnChain(txn) = maybe_pending_txn {
                        let status = txn.info.status();

                        if status.is_success() {
                            Ok(WaitForTransactionResult::Success(Response::new(txn, state)))
                        } else {
                            Ok(WaitForTransactionResult::FailedExecution(format!(
                                "{:?}",
                                status
                            )))
                        }
                    } else {
                        Ok(WaitForTransactionResult::Pending(state))
                    }
                } else {
                    let error_response = parse_error(resp).await;
                    Ok(WaitForTransactionResult::NotFound(error_response))
                }
            },
        )
        .await
    }

    pub async fn wait_for_version(&self, version: u64) -> Result<State> {
        const DEFAULT_TIMEOUT: Duration = Duration::from_secs(240);
        const DEFAULT_DELAY: Duration = Duration::from_millis(500);

        let start = std::time::Instant::now();
        loop {
            let state = self.get_ledger_information().await?.into_inner();
            if state.version >= version {
                return Ok(state);
            }

            if start.elapsed() >= DEFAULT_TIMEOUT {
                return Err(anyhow!(
                    "timeout when waiting for version {}, only got to {}",
                    version,
                    state.version
                ));
            }

            tokio::time::sleep(DEFAULT_DELAY).await;
        }
    }

    pub async fn get_transactions(
        &self,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<Vec<Transaction>>> {
        let url = self.build_path("transactions")?;

        let mut request = self.inner.get(url);
        if let Some(start) = start {
            request = request.query(&[("start", start)])
        }

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)])
        }

        let response = request.send().await?;

        self.json(response).await
    }

    pub async fn get_transactions_bcs(
        &self,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<Vec<TransactionOnChainData>>> {
        let url = self.build_path("transactions")?;
        let response = self.get_bcs_with_page(url, start, limit).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_transaction_by_hash(
        &self,
        hash: HashValue,
    ) -> EndlessResult<Response<Transaction>> {
        self.json(self.get_transaction_by_hash_inner(hash).await?)
            .await
    }

    pub async fn get_transaction_by_hash_bcs(
        &self,
        hash: HashValue,
    ) -> EndlessResult<Response<TransactionData>> {
        let response = self.get_transaction_by_hash_bcs_inner(hash).await?;
        let response = self.check_and_parse_bcs_response(response).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_transaction_by_hash_bcs_inner(
        &self,
        hash: HashValue,
    ) -> EndlessResult<reqwest::Response> {
        let url = self.build_path(&format!("transactions/by_hash/{}", hash.to_hex_literal()))?;
        let response = self.inner.get(url).header(ACCEPT, BCS).send().await?;
        Ok(response)
    }

    async fn get_transaction_by_hash_inner(
        &self,
        hash: HashValue,
    ) -> EndlessResult<reqwest::Response> {
        let url = self.build_path(&format!("transactions/by_hash/{}", hash.to_hex_literal()))?;
        Ok(self.inner.get(url).send().await?)
    }

    pub async fn get_transaction_by_version(
        &self,
        version: u64,
    ) -> EndlessResult<Response<Transaction>> {
        self.json(self.get_transaction_by_version_inner(version).await?)
            .await
    }

    pub async fn get_transaction_by_version_bcs(
        &self,
        version: u64,
    ) -> EndlessResult<Response<TransactionData>> {
        let url = self.build_path(&format!("transactions/by_version/{}", version))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    async fn get_transaction_by_version_inner(
        &self,
        version: u64,
    ) -> EndlessResult<reqwest::Response> {
        let url = self.build_path(&format!("transactions/by_version/{}", version))?;
        Ok(self.inner.get(url).send().await?)
    }

    pub async fn get_transactions_by_version(
        &self,
        versions: Vec<u64>,
    ) -> EndlessResult<Response<Vec<Transaction>>> {
        let url = self.build_path("transactions/by_version")?;
        self.json(self.inner.post(url).json(&versions).send().await?)
            .await
    }

    pub async fn get_transactions_by_version_bcs(
        &self,
        versions: Vec<u64>,
    ) -> EndlessResult<Response<Vec<TransactionData>>> {
        let url = self.build_path("transactions/by_version")?;
        let response = self.post_bcs(url, serde_json::to_value(&versions)?).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_transactions(
        &self,
        address: AccountAddress,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> EndlessResult<Response<Vec<Transaction>>> {
        let url = self.build_path(&format!("accounts/{}/transactions", address.to_hex()))?;

        let mut request = self.inner.get(url);
        if let Some(start) = start {
            request = request.query(&[("start", start)])
        }

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)])
        }

        let response = request.send().await?;

        self.json(response).await
    }

    pub async fn get_account_transactions_bcs(
        &self,
        address: AccountAddress,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<Vec<TransactionOnChainData>>> {
        let url = self.build_path(&format!("accounts/{}/transactions", address.to_hex()))?;
        let response = self.get_bcs_with_page(url, start, limit).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_resources(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<Vec<Resource>>> {
        self.paginate_with_cursor(
            &format!("accounts/{}/resources", address.to_hex()),
            RESOURCES_PER_CALL_PAGINATION,
            None,
        )
        .await
    }

    pub async fn get_account_resources_bcs(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<BTreeMap<StructTag, Vec<u8>>>> {
        self.paginate_with_cursor_bcs(
            &format!("accounts/{}/resources", address.to_hex()),
            RESOURCES_PER_CALL_PAGINATION,
            None,
        )
        .await
    }

    pub async fn get_account_resources_at_version(
        &self,
        address: AccountAddress,
        version: u64,
    ) -> EndlessResult<Response<Vec<Resource>>> {
        self.paginate_with_cursor(
            &format!("accounts/{}/resources", address.to_hex()),
            RESOURCES_PER_CALL_PAGINATION,
            Some(version),
        )
        .await
    }

    pub async fn get_account_resources_at_version_bcs(
        &self,
        address: AccountAddress,
        version: u64,
    ) -> EndlessResult<Response<BTreeMap<StructTag, Vec<u8>>>> {
        self.paginate_with_cursor_bcs(
            &format!("accounts/{}/resources", address.to_hex()),
            RESOURCES_PER_CALL_PAGINATION,
            Some(version),
        )
        .await
    }

    pub async fn get_resource<T: DeserializeOwned>(
        &self,
        address: AccountAddress,
        resource_type: &str,
    ) -> EndlessResult<Response<T>> {
        let resp = self.get_account_resource(address, resource_type).await?;
        resp.and_then(|conf| {
            if let Some(res) = conf {
                serde_json::from_value(res.data)
                    .map_err(|e| anyhow!("deserialize {} failed: {}", resource_type, e).into())
            } else {
                Err(anyhow!(
                    "could not find resource {} in account {}",
                    resource_type,
                    address
                )
                .into())
            }
        })
    }

    pub async fn get_account_resource(
        &self,
        address: AccountAddress,
        resource_type: &str,
    ) -> EndlessResult<Response<Option<Resource>>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}",
            address.to_hex(),
            resource_type
        ))?;

        let response = self
            .inner
            .get(url)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
        self.json(response).await
    }

    pub async fn get_account_resource_bcs<T: DeserializeOwned>(
        &self,
        address: AccountAddress,
        resource_type: &str,
    ) -> EndlessResult<Response<T>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}",
            address.to_hex(),
            resource_type
        ))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_resource_at_version_bcs<T: DeserializeOwned>(
        &self,
        address: AccountAddress,
        resource_type: &str,
        version: u64,
    ) -> EndlessResult<Response<T>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}?ledger_version={}",
            address.to_hex(),
            resource_type,
            version
        ))?;

        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_resource_at_version_bytes(
        &self,
        address: AccountAddress,
        resource_type: &str,
        version: u64,
    ) -> EndlessResult<Response<Vec<u8>>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}?ledger_version={}",
            address.to_hex(),
            resource_type,
            version
        ))?;

        let response = self.get_bcs(url).await?;
        Ok(response.map(|inner| inner.to_vec()))
    }

    pub async fn get_account_resource_bytes(
        &self,
        address: AccountAddress,
        resource_type: &str,
    ) -> EndlessResult<Response<Vec<u8>>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}",
            address.to_hex(),
            resource_type
        ))?;

        let response = self.get_bcs(url).await?;
        Ok(response.map(|inner| inner.to_vec()))
    }

    pub async fn get_account_resource_at_version(
        &self,
        address: AccountAddress,
        resource_type: &str,
        version: u64,
    ) -> EndlessResult<Response<Option<Resource>>> {
        let url = self.build_path(&format!(
            "accounts/{}/resource/{}?ledger_version={}",
            address.to_hex(),
            resource_type,
            version
        ))?;

        let response = self.inner.get(url).send().await?;
        self.json(response).await
    }

    pub async fn get_account_modules(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<Vec<MoveModuleBytecode>>> {
        self.paginate_with_cursor(
            &format!("accounts/{}/modules", address.to_hex()),
            MODULES_PER_CALL_PAGINATION,
            None,
        )
        .await
    }

    pub async fn get_account_modules_bcs(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<BTreeMap<MoveModuleId, Vec<u8>>>> {
        self.paginate_with_cursor_bcs(
            &format!("accounts/{}/modules", address.to_hex()),
            MODULES_PER_CALL_PAGINATION,
            None,
        )
        .await
    }

    pub async fn get_account_module(
        &self,
        address: AccountAddress,
        module_name: &str,
    ) -> EndlessResult<Response<MoveModuleBytecode>> {
        let url = self.build_path(&format!(
            "accounts/{}/module/{}",
            address.to_hex(),
            module_name
        ))?;
        self.get(url).await
    }

    pub async fn get_account_module_bcs(
        &self,
        address: AccountAddress,
        module_name: &str,
    ) -> EndlessResult<Response<bytes::Bytes>> {
        let url = self.build_path(&format!(
            "accounts/{}/module/{}",
            address.to_hex(),
            module_name
        ))?;
        self.get_bcs(url).await
    }

    pub async fn get_account_module_bcs_at_version(
        &self,
        address: AccountAddress,
        module_name: &str,
        version: u64,
    ) -> EndlessResult<Response<bytes::Bytes>> {
        let url = self.build_path(&format!(
            "accounts/{}/module/{}?ledger_version={}",
            address.to_hex(),
            module_name,
            version
        ))?;
        self.get_bcs(url).await
    }

    pub async fn get_events_by_type<R: RangeBounds<usize>>(
        &self,
        type_tag: &str,
        range: QueryRange<R>,
    ) -> EndlessResult<Response<Vec<VersionedEventV2>>> {
        let url = self.build_path(&format!("events/{}?{}", type_tag, range.into_query(),))?;
        self.get(url).await
    }

    pub async fn get_events_by_type_bcs<R: RangeBounds<usize>>(
        &self,
        type_tag: &str,
        range: QueryRange<R>,
    ) -> EndlessResult<Response<Vec<EventWithVersionIdx>>> {
        let url = self.build_path(&format!("events/{}?{}", type_tag, range.into_query(),))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_account_events(
        &self,
        address: AccountAddress,
        struct_tag: &str,
        field_name: &str,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<Vec<VersionedEvent>>> {
        let url = self.build_path(&format!(
            "accounts/{}/events/{}/{}",
            address.to_hex_literal(),
            struct_tag,
            field_name
        ))?;
        let mut request = self.inner.get(url);
        if let Some(start) = start {
            request = request.query(&[("start", start)])
        }

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)])
        }

        let response = request.send().await?;
        self.json(response).await
    }

    pub async fn get_account_events_bcs(
        &self,
        address: AccountAddress,
        struct_tag: &str,
        field_name: &str,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<Vec<EventWithVersion>>> {
        let url = self.build_path(&format!(
            "accounts/{}/events/{}/{}",
            address.to_hex_literal(),
            struct_tag,
            field_name
        ))?;

        let response = self.get_bcs_with_page(url, start, limit).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_new_block_events_bcs(
        &self,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<VersionedNewBlockEvent>>> {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct NewBlockEventResponse {
            hash: String,
            #[serde(deserialize_with = "deserialize_from_string")]
            epoch: u64,
            #[serde(deserialize_with = "deserialize_from_string")]
            round: u64,
            #[serde(deserialize_with = "deserialize_from_string")]
            height: u64,
            #[serde(deserialize_with = "deserialize_from_prefixed_hex_string")]
            previous_block_votes_bitvec: HexEncodedBytes,
            proposer: String,
            failed_proposer_indices: Vec<String>,
            #[serde(deserialize_with = "deserialize_from_string")]
            time_microseconds: u64,
        }

        let url = self.build_path("accounts/0x1/events/1")?;

        let response = self.get_bcs_with_page(url, start, limit).await?;
        let response: Response<Vec<EventWithVersion>> =
            response.and_then(|inner| bcs::from_bytes(&inner))?;

        response.and_then(|events| {
            let new_events: Result<Vec<_>> = events
                .into_iter()
                .map(|event| {
                    let version = event.transaction_version;
                    let event = event.event.v1()?;
                    let sequence_number = event.sequence_number();

                    Ok(VersionedNewBlockEvent {
                        event: bcs::from_bytes(event.event_data())?,
                        version,
                        sequence_number,
                    })
                })
                .collect();
            new_events
        })
    }

    pub async fn get_table_item<K: Serialize>(
        &self,
        table_handle: AccountAddress,
        key_type: &str,
        value_type: &str,
        key: K,
    ) -> EndlessResult<Response<Value>> {
        let url = self.build_path(&format!("tables/{}/item", table_handle))?;
        let data = json!({
            "key_type": key_type,
            "value_type": value_type,
            "key": json!(key),
        });

        let response = self.inner.post(url).json(&data).send().await?;
        self.json(response).await
    }

    pub async fn get_table_item_at_version<K: Serialize>(
        &self,
        table_handle: AccountAddress,
        key_type: &str,
        value_type: &str,
        key: K,
        version: u64,
    ) -> EndlessResult<Response<Value>> {
        let url = self.build_path(&format!(
            "tables/{}/item?ledger_version={}",
            table_handle, version
        ))?;
        let data = json!({
            "key_type": key_type,
            "value_type": value_type,
            "key": json!(key),
        });

        let response = self.inner.post(url).json(&data).send().await?;
        self.json(response).await
    }

    pub async fn get_table_item_bcs<K: Serialize + fmt::Display, T: DeserializeOwned>(
        &self,
        table_handle: AccountAddress,
        key_type: &str,
        value_type: &str,
        key: K,
    ) -> EndlessResult<Response<T>> {
        let url = self.build_path(&format!(
            "tables/{}/item",
            table_handle.to_standard_string()
        ))?;
        let data = json!({
            "key_type": key_type,
            "value_type": value_type,
            "key": json!(key),
        });

        let response = self.post_bcs(url, data).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_table_item_bcs_at_version<K: Serialize, T: DeserializeOwned>(
        &self,
        table_handle: AccountAddress,
        key_type: &str,
        value_type: &str,
        key: K,
        version: u64,
    ) -> EndlessResult<Response<T>> {
        let url = self.build_path(&format!(
            "tables/{}/item?ledger_version={}",
            table_handle, version
        ))?;
        let data = json!({
            "key_type": key_type,
            "value_type": value_type,
            "key": json!(key),
        });

        let response = self.post_bcs(url, data).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn get_raw_table_item(
        &self,
        table_handle: AccountAddress,
        key: &[u8],
        version: u64,
    ) -> EndlessResult<Response<Vec<u8>>> {
        let url = self.build_path(&format!(
            "tables/{}/raw_item?ledger_version={}",
            table_handle, version
        ))?;
        let data = json!({
            "key": hex::encode(key),
        });

        let response = self.post_bcs(url, data).await?;
        Ok(response.map(|inner| inner.to_vec()))
    }

    pub async fn get_raw_state_value(
        &self,
        state_key: &StateKey,
        version: u64,
    ) -> EndlessResult<Response<Vec<u8>>> {
        let url = self.build_path(&format!(
            "experimental/state_values/raw?ledger_version={}",
            version
        ))?;
        let data = json!({
            "key": hex::encode(bcs::to_bytes(state_key)?),
        });

        let response = self.post_bcs(url, data).await?;
        Ok(response.map(|inner| inner.to_vec()))
    }

    pub async fn get_account(&self, address: AccountAddress) -> EndlessResult<Response<Account>> {
        let url = self.build_path(&format!("accounts/{}", address.to_hex()))?;
        let response = self.inner.get(url).send().await?;
        self.json(response).await
    }

    pub async fn get_account_bcs(
        &self,
        address: AccountAddress,
    ) -> EndlessResult<Response<AccountResource>> {
        let url = self.build_path(&format!("accounts/{}", address.to_hex()))?;
        let response = self.get_bcs(url).await?;
        Ok(response.and_then(|inner| bcs::from_bytes(&inner))?)
    }

    pub async fn estimate_gas_price(&self) -> EndlessResult<Response<GasEstimation>> {
        let url = self.build_path("estimate_gas_price")?;
        let response = self.inner.get(url).send().await?;
        self.json(response).await
    }

    pub async fn set_failpoint(&self, name: String, actions: String) -> EndlessResult<String> {
        let mut base = self.build_path("set_failpoint")?;
        let url = base
            .query_pairs_mut()
            .append_pair("name", &name)
            .append_pair("actions", &actions)
            .finish();
        let response = self.inner.get(url.clone()).send().await?;

        if !response.status().is_success() {
            Err(parse_error(response).await)
        } else {
            Ok(response
                .text()
                .await
                .map_err(|e| anyhow::anyhow!("To text failed: {:?}", e))?)
        }
    }

    async fn check_response(
        &self,
        response: reqwest::Response,
    ) -> EndlessResult<(reqwest::Response, State)> {
        if !response.status().is_success() {
            Err(parse_error(response).await)
        } else {
            let state = parse_state(&response)?;

            Ok((response, state))
        }
    }

    async fn json<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> EndlessResult<Response<T>> {
        let (response, state) = self.check_response(response).await?;
        let json = response.json().await.map_err(anyhow::Error::from)?;
        Ok(Response::new(json, state))
    }

    pub async fn health_check(&self, seconds: u64) -> EndlessResult<()> {
        let url = self.build_path("-/healthy")?;
        let response = self
            .inner
            .get(url)
            .query(&[("duration_secs", seconds)])
            .send()
            .await?;

        if !response.status().is_success() {
            Err(parse_error(response).await)
        } else {
            Ok(())
        }
    }

    async fn get<T: DeserializeOwned>(&self, url: Url) -> EndlessResult<Response<T>> {
        self.json(self.inner.get(url).send().await?).await
    }

    async fn get_bcs(&self, url: Url) -> EndlessResult<Response<bytes::Bytes>> {
        let response = self.inner.get(url).header(ACCEPT, BCS).send().await?;
        self.check_and_parse_bcs_response(response).await
    }

    async fn post_bcs(
        &self,
        url: Url,
        data: serde_json::Value,
    ) -> EndlessResult<Response<bytes::Bytes>> {
        let response = self
            .inner
            .post(url)
            .header(ACCEPT, BCS)
            .json(&data)
            .send()
            .await?;
        self.check_and_parse_bcs_response(response).await
    }

    async fn get_bcs_with_page(
        &self,
        url: Url,
        start: Option<u64>,
        limit: Option<u16>,
    ) -> EndlessResult<Response<bytes::Bytes>> {
        let mut request = self.inner.get(url).header(ACCEPT, BCS);
        if let Some(start) = start {
            request = request.query(&[("start", start)])
        }

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)])
        }

        let response = request.send().await?;
        self.check_and_parse_bcs_response(response).await
    }

    async fn check_and_parse_bcs_response(
        &self,
        response: reqwest::Response,
    ) -> EndlessResult<Response<bytes::Bytes>> {
        let (response, state) = self.check_response(response).await?;
        Ok(Response::new(response.bytes().await?, state))
    }

    pub async fn try_until_ok<F, Fut, RetryFun, T>(
        total_wait: Option<Duration>,
        initial_interval: Option<Duration>,
        should_retry: RetryFun,
        function: F,
    ) -> EndlessResult<T>
    where
        F: Fn() -> Fut,
        RetryFun: Fn(StatusCode, Option<EndlessError>) -> bool,
        Fut: Future<Output = EndlessResult<T>>,
    {
        let total_wait = total_wait.unwrap_or(DEFAULT_MAX_WAIT_DURATION);
        let mut backoff = initial_interval.unwrap_or(DEFAULT_INTERVAL_DURATION);
        let mut result = Err(RestError::Unknown(anyhow!("Failed to run function")));
        let start = Instant::now();

        // TODO: Add jitter
        while start.elapsed() < total_wait {
            result = function().await;

            let retry = match &result {
                Ok(_) => break,
                Err(err) => match err {
                    RestError::Api(inner) => {
                        should_retry(inner.status_code, Some(inner.error.clone()))
                    },
                    RestError::Http(status_code, _e) => should_retry(*status_code, None),
                    RestError::Bcs(_)
                    | RestError::Json(_)
                    | RestError::Timeout(_)
                    | RestError::Unknown(_) => true,
                    RestError::UrlParse(_) => false,
                },
            };

            if !retry {
                break;
            }

            info!(
                "Failed to call API, retrying in {}ms: {:?}",
                backoff.as_millis(),
                result.as_ref().err().unwrap()
            );

            tokio::time::sleep(backoff).await;
            backoff = backoff.saturating_mul(2);
        }

        result
    }

    /// This function builds a URL for use in pagination. It handles setting a limit,
    /// adding the cursor, and adding a ledger version if given.
    pub fn build_url_for_pagination(
        &self,
        base: &str,
        limit_per_request: u64,
        ledger_version: Option<u64>,
        cursor: Option<String>,
    ) -> EndlessResult<Url> {
        let mut path = format!("{}?limit={}", base, limit_per_request);
        if let Some(ledger_version) = ledger_version {
            path = format!("{}&ledger_version={}", path, ledger_version);
        }
        if let Some(cursor) = cursor {
            path = format!("{}&start={}", path, cursor);
        }
        self.build_path(&path)
    }

    /// This function calls an endpoint that has pagination support and paginates
    /// using the cursor the API returns. It keeps paginating until the API doesn't
    /// return a cursor anymore. Since the functions calling this function are
    /// expected to return the data wrapped in a Response (exactly one), we return
    /// the full results merged together wrapped in the Response we received from
    /// the final call.
    pub async fn paginate_with_cursor<T: for<'a> Deserialize<'a>>(
        &self,
        base_path: &str,
        limit_per_request: u64,
        ledger_version: Option<u64>,
    ) -> EndlessResult<Response<Vec<T>>> {
        let mut result = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let url = self.build_url_for_pagination(
                base_path,
                limit_per_request,
                ledger_version,
                cursor,
            )?;
            let raw_response = self.inner.get(url).send().await?;
            let response: Response<Vec<T>> = self.json(raw_response).await?;
            cursor = response.state().cursor.clone();
            if cursor.is_none() {
                break Ok(response.map(|mut v| {
                    result.append(&mut v);
                    result
                }));
            } else {
                result.extend(response.into_inner());
            }
        }
    }

    /// This function works just like `paginate_with_cursor`, but it calls the internal
    /// helper functions for dealing with BCS data and collects data in the format we
    /// use for BCS endpoint functions.
    pub async fn paginate_with_cursor_bcs<T: for<'a> Deserialize<'a> + Ord>(
        &self,
        base_path: &str,
        limit_per_request: u64,
        ledger_version: Option<u64>,
    ) -> EndlessResult<Response<BTreeMap<T, Vec<u8>>>> {
        let mut result = BTreeMap::new();
        let mut cursor: Option<String> = None;

        loop {
            let url = self.build_url_for_pagination(
                base_path,
                limit_per_request,
                ledger_version,
                cursor,
            )?;
            let response: Response<BTreeMap<T, Vec<u8>>> = self
                .get_bcs(url)
                .await?
                .and_then(|inner| bcs::from_bytes(&inner))?;
            cursor = response.state().cursor.clone();
            if cursor.is_none() {
                break Ok(response.map(|mut v| {
                    result.append(&mut v);
                    result
                }));
            } else {
                result.extend(response.into_inner());
            }
        }
    }
}

// If the user provided no version in the path, use the default. If the
// provided version has no trailing slash, add it, otherwise url.join
// will ignore the version path base.
pub fn get_version_path_with_base(base_url: Url) -> String {
    match base_url.path() {
        "/" => DEFAULT_VERSION_PATH_BASE.to_string(),
        path => {
            if !path.ends_with('/') {
                format!("{}/", path)
            } else {
                path.to_string()
            }
        },
    }
}

pub fn retriable_with_404(status_code: StatusCode, endless_error: Option<EndlessError>) -> bool {
    retriable(status_code, endless_error) | matches!(status_code, StatusCode::NOT_FOUND)
}

pub fn retriable(status_code: StatusCode, _endless_error: Option<EndlessError>) -> bool {
    matches!(
        status_code,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::GATEWAY_TIMEOUT
            | StatusCode::BAD_GATEWAY
            | StatusCode::INSUFFICIENT_STORAGE
    )
}

impl From<(ReqwestClient, Url)> for Client {
    fn from((inner, base_url): (ReqwestClient, Url)) -> Self {
        Client {
            inner,
            base_url,
            version_path_base: DEFAULT_VERSION_PATH_BASE.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionedNewBlockEvent {
    /// event
    pub event: NewBlockEvent,
    /// version
    pub version: u64,
    /// sequence number
    pub sequence_number: u64,
}

fn parse_state(response: &reqwest::Response) -> EndlessResult<State> {
    Ok(State::from_headers(response.headers())?)
}

fn parse_state_optional(response: &reqwest::Response) -> Option<State> {
    State::from_headers(response.headers())
        .map(Some)
        .unwrap_or(None)
}

async fn parse_error(response: reqwest::Response) -> RestError {
    let status_code = response.status();
    let maybe_state = parse_state_optional(&response);
    match response.json::<EndlessError>().await {
        Ok(error) => (error, maybe_state, status_code).into(),
        Err(e) => RestError::Http(status_code, e),
    }
}

pub struct GasEstimationParams {
    pub estimated_gas_used: u64,
    pub estimated_gas_price: u64,
}

enum WaitForTransactionResult<T> {
    NotFound(RestError),
    FailedExecution(String),
    Pending(State),
    Success(Response<T>),
}
