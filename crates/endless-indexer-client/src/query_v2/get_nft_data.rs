// Copyright © Endless
// Copyright © Aptos Foundation

use crate::IndexerClient;
use anyhow::{Ok, Result};
use endless_types::account_address::AccountAddress;
use reqwest::{self, Response};
use std::str::FromStr;
use url::Url;

pub struct Conditions {
    pub collection: Option<AccountAddress>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl IndexerClient {
    pub async fn get_nft_data_by_id(&self, nft_id: AccountAddress) -> Result<Response> {
        let url = Url::from_str(&format!(
            "{}/nfts/{}",
            self.url,
            nft_id.to_bs58_string_or_bcs()
        ))?;

        let response = self.client.get(url.as_str()).send().await?;

        Ok(response)
    }

    pub async fn get_nft_data_by_owner(
        &self,
        owner_address: AccountAddress,
        conditions: Option<Conditions>,
    ) -> Result<Response> {
        let mut url = Url::from_str(&format!(
            "{}/accounts/{}/nfts",
            self.url,
            owner_address.to_bs58_string_or_bcs()
        ))?;

        if let Some(conditions) = conditions {
            if let Some(collection) = conditions.collection {
                url.query_pairs_mut()
                    .append_pair("collection", &collection.to_bs58_string_or_bcs());
            }
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
