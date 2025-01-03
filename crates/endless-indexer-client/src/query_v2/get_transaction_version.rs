// Copyright © Endless
// Copyright © Aptos Foundation

use crate::IndexerClient;
use anyhow::Result;
use endless_types::account_address::AccountAddress;
use reqwest::{self, Response};
use std::str::FromStr;
use url::Url;

pub struct Conditions {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl IndexerClient {
    pub async fn get_transaction_version_by_account(
        &self,
        account: AccountAddress,
        conditions: Option<Conditions>,
    ) -> Result<Response> {
        let mut url = Url::from_str(&format!(
            "{}/accounts/{}/transactions",
            self.url,
            account.to_bs58_string_or_bcs()
        ))?;

        if let Some(conditions) = conditions {
            if let Some(page) = conditions.page {
                url.query_pairs_mut().append_pair("page", &page.to_string());
            }
            if let Some(page_size) = conditions.page_size {
                url.query_pairs_mut()
                    .append_pair("pageSize", &page_size.to_string());
            }
        }

        let response = self.client.get(url.as_str()).send().await?;

        Ok(response)
    }

    pub async fn get_latest_transaction_version(
        &self,
        conditions: Option<Conditions>,
    ) -> Result<Response> {
        let mut url = Url::from_str(&format!("{}/transactions/user", self.url))?;

        if let Some(conditions) = conditions {
            if let Some(page) = conditions.page {
                url.query_pairs_mut().append_pair("page", &page.to_string());
            }
            if let Some(page_size) = conditions.page_size {
                url.query_pairs_mut()
                    .append_pair("pageSize", &page_size.to_string());
            }
        }

        let response = self.client.get(url.as_str()).send().await?;

        Ok(response)
    }
}
