// Copyright © Endless
// Copyright © Aptos Foundation

use crate::{
    move_types::{
        identifier::Identifier,
        language_storage::{ModuleId, TypeTag},
    },
    rest_client::{Client, PendingTransaction},
    transaction_builder::TransactionBuilder,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};
use anyhow::{anyhow, Context, Result};
use endless_crypto::ed25519::Ed25519PrivateKey;
use endless_ledger::{AuthenticationKey, Ed25519PublicKey};
use endless_rest_client::{
    endless_api_types::{UserTransaction, ViewFunction},
    error::RestError,
    Response,
};
use serde::de::DeserializeOwned;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub struct HelperClient<'a> {
    rest_client: &'a Client,
}

impl<'a> HelperClient<'a> {
    pub fn new(rest_client: &'a Client) -> Self {
        Self { rest_client }
    }

    pub async fn faucet_fund(
        &self,
        local_account: &LocalAccount,
        address: AccountAddress,
        overrides: Option<Overrides>,
    ) -> Result<PendingTransaction> {
        let txn_hash = self
            .entry_function(
                AccountAddress::ONE,
                "faucet",
                "fund",
                local_account,
                vec![bcs::to_bytes(&address).unwrap()],
                vec![],
                overrides,
            )
            .await
            .context("Failed to faucet_fund")?;

        Ok(txn_hash)
    }

    pub async fn simulate_transfer(
        &self,
        from_account: &LocalAccount,
        to_account: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> Result<Response<Vec<UserTransaction>>, RestError> {
        self.simulate_function(
            AccountAddress::ONE,
            "endless_account",
            "transfer",
            from_account,
            vec![
                bcs::to_bytes(&to_account).unwrap(),
                bcs::to_bytes(&amount).unwrap(),
            ],
            vec![],
            overrides,
        )
        .await
    }

    pub async fn transfer(
        &self,
        from_account: &LocalAccount,
        to_account: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> Result<PendingTransaction> {
        let txn_hash = self
            .entry_function(
                AccountAddress::ONE,
                "endless_account",
                "transfer",
                from_account,
                vec![
                    bcs::to_bytes(&to_account).unwrap(),
                    bcs::to_bytes(&amount).unwrap(),
                ],
                vec![],
                overrides,
            )
            .await
            .context("Failed to submit transaction to transfer coins")?;

        Ok(txn_hash)
    }

    pub async fn simulate_transfer_coins(
        &self,
        from_account: &LocalAccount,
        to_account: AccountAddress,
        amount: u128,
        metadata: AccountAddress,
        overrides: Option<Overrides>,
    ) -> Result<Response<Vec<UserTransaction>>, RestError> {
        self.simulate_function(
            AccountAddress::ONE,
            "endless_account",
            "transfer_coins",
            from_account,
            vec![
                bcs::to_bytes(&to_account).unwrap(),
                bcs::to_bytes(&amount).unwrap(),
                bcs::to_bytes(&metadata).unwrap(),
            ],
            vec!["0x1::fungible_asset::Metadata"],
            overrides,
        )
        .await
    }

    pub async fn transfer_coins(
        &self,
        from_account: &LocalAccount,
        to_account: AccountAddress,
        amount: u128,
        metadata: AccountAddress,
        overrides: Option<Overrides>,
    ) -> Result<PendingTransaction> {
        let txn_hash = self
            .entry_function(
                AccountAddress::ONE,
                "endless_account",
                "transfer_coins",
                from_account,
                vec![
                    bcs::to_bytes(&to_account).unwrap(),
                    bcs::to_bytes(&amount).unwrap(),
                    bcs::to_bytes(&metadata).unwrap(),
                ],
                vec!["0x1::fungible_asset::Metadata"],
                overrides,
            )
            .await
            .context("Failed to submit transaction to transfer coins")?;

        Ok(txn_hash)
    }

    pub async fn get_account_balance_by_metadata(
        &self,
        account: AccountAddress,
        metadata: AccountAddress,
    ) -> Result<u128, RestError> {
        let response: Response<Vec<u128>> = self
            .view_function(
                AccountAddress::ONE,
                "primary_fungible_store",
                "balance",
                vec![
                    bcs::to_bytes(&account).unwrap(),
                    bcs::to_bytes(&metadata).unwrap(),
                ],
                vec!["0x1::fungible_asset::Metadata"],
            )
            .await?;

        Ok(response.into_inner()[0])
    }

    pub async fn get_account_balance(&self, account: &AccountAddress) -> Result<u128> {
        let result: Vec<u128> = self
            .view_function(
                AccountAddress::ONE,
                "endless_coin",
                "balance",
                vec![bcs::to_bytes(&account).unwrap()],
                vec![],
            )
            .await
            .unwrap()
            .into_inner();

        Ok(result[0])
    }

    pub async fn simulate_function(
        &self,
        module_address: AccountAddress,
        module_name: &str,
        function_name: &str,
        signer: &LocalAccount,
        args: Vec<Vec<u8>>,
        type_args: Vec<&str>,
        overrides: Option<Overrides>,
    ) -> Result<Response<Vec<UserTransaction>>, RestError> {
        let overrides = overrides.unwrap_or_default();

        let chain_id = self
            .rest_client
            .get_index()
            .await
            .context("Failed to get chain ID")?
            .inner()
            .chain_id;
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(module_address, Identifier::new(module_name).unwrap()),
                Identifier::new(function_name).unwrap(),
                convert_type_args(type_args),
                args,
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + overrides.timeout_secs,
            ChainId::new(chain_id),
        )
        .sender(signer.address())
        .sequence_number(signer.sequence_number())
        .max_gas_amount(overrides.max_gas_amount)
        .gas_unit_price(overrides.gas_unit_price);
        let signed_txn = signer.sign_with_transaction_builder(transaction_builder);
        let result = self
            .rest_client
            .simulate_with_gas_estimation(&signed_txn, true, false)
            .await;

        signer.decrement_sequence_number();
        result
    }

    pub async fn entry_function(
        &self,
        module_address: AccountAddress,
        module_name: &str,
        function_name: &str,
        signer: &LocalAccount,
        args: Vec<Vec<u8>>,
        type_args: Vec<&str>,
        overrides: Option<Overrides>,
    ) -> Result<PendingTransaction> {
        let overrides = overrides.unwrap_or_default();

        let chain_id = self
            .rest_client
            .get_index()
            .await
            .context("Failed to get chain ID")?
            .inner()
            .chain_id;
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(module_address, Identifier::new(module_name).unwrap()),
                Identifier::new(function_name).unwrap(),
                convert_type_args(type_args),
                args,
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + overrides.timeout_secs,
            ChainId::new(chain_id),
        )
        .sender(signer.address())
        .max_gas_amount(overrides.max_gas_amount)
        .gas_unit_price(overrides.gas_unit_price);

        let signed_txn = signer.sign_with_transaction_builder(transaction_builder);
        Ok(self
            .rest_client
            .submit(&signed_txn)
            .await
            .context("Failed to submit transaction")?
            .into_inner())
    }

    pub async fn view_function<T: DeserializeOwned>(
        &self,
        module_address: AccountAddress,
        module_name: &str,
        function_name: &str,
        args: Vec<Vec<u8>>,
        type_args: Vec<&str>,
    ) -> Result<Response<T>, RestError> {
        // BCS
        let bcs_view_request = ViewFunction {
            module: ModuleId::new(module_address, Identifier::new(module_name).unwrap()),
            function: Identifier::new(function_name).unwrap(),
            ty_args: convert_type_args(type_args),
            args,
        };

        self.rest_client.view_bcs(&bcs_view_request, None).await
    }

    pub fn recover_account_from_private_key(&self, private_key_str: &str) -> Result<LocalAccount> {
        let private_key = if let Some(stripped) = private_key_str.strip_prefix("0x") {
            stripped
        } else {
            private_key_str
        };

        if private_key.len() != 64 {
            return Err(anyhow!("private_key length err"));
        }
        let key_bytes = hex::decode(private_key)?;
        let private_key = (&key_bytes[..]).try_into()?;
        let public_key = Ed25519PublicKey::from(&private_key);
        let address = AuthenticationKey::ed25519(&public_key).account_address();
        let local_account = LocalAccount::new(address, private_key, 0);

        Ok(local_account)
    }

    pub fn recover_account_from_ed25519_key(&self, key: Ed25519PrivateKey) -> LocalAccount {
        let public_key = Ed25519PublicKey::from(&key);
        let address = AuthenticationKey::ed25519(&public_key).account_address();
        LocalAccount::new(address, key, 0)
    }
}

fn convert_type_args(vec: Vec<&str>) -> Vec<TypeTag> {
    vec.into_iter()
        .map(|s| TypeTag::from_str(s).unwrap())
        .collect()
}

#[derive(Clone)]
pub struct Overrides {
    pub max_gas_amount: u64,

    pub gas_unit_price: u64,

    /// This is the number of seconds from now you're willing to wait for the
    /// transaction to be committed.
    pub timeout_secs: u64,
}

impl Default for Overrides {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 100,
            timeout_secs: 60,
        }
    }
}
