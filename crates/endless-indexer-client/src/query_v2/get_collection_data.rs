// Copyright © Endless
// Copyright © Aptos Foundation

use crate::IndexerClient;
use anyhow::Result;
use endless_types::account_address::AccountAddress;
use reqwest::{self, Response};
use std::str::FromStr;
use url::Url;

pub struct Conditions {
    pub name: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl IndexerClient {
    pub async fn get_collection_data_by_id(
        &self,
        collection_id: AccountAddress,
    ) -> Result<Response> {
        let url = Url::from_str(&format!(
            "{}/collections/{}",
            self.url,
            collection_id.to_bs58_string_or_bcs()
        ))?;

        let response = self.client.get(url.as_str()).send().await?;

        Ok(response)
    }

    pub async fn batch_get_collection_data_by_ids(
        &self,
        collection_ids: Vec<AccountAddress>,
    ) -> Result<Response> {
        let url = Url::from_str(&format!("{}/collections", self.url,))?;

        let response = self
            .client
            .post(url.as_str())
            .json(&collection_ids)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn get_collection_data_by_creator(
        &self,
        creator_address: AccountAddress,
        conditions: Option<Conditions>,
    ) -> Result<Response> {
        let mut url = Url::from_str(&format!(
            "{}/accounts/{}/collections",
            self.url,
            creator_address.to_bs58_string_or_bcs()
        ))?;

        if let Some(conditions) = conditions {
            if let Some(name) = conditions.name {
                url.query_pairs_mut().append_pair("name", &name);
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

    pub async fn get_collection_data_by_nft_owner(
        &self,
        nft_owner_address: AccountAddress,
        conditions: Option<Conditions>,
    ) -> Result<Response> {
        let mut url = Url::from_str(&format!(
            "{}/accounts/{}/nfts/collections",
            self.url,
            nft_owner_address.to_bs58_string_or_bcs()
        ))?;

        if let Some(conditions) = conditions {
            // if let Some(name) = conditions.name {
            //     url.query_pairs_mut().append_pair("name", &name);
            // }
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
