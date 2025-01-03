// Copyright © Endless
// Copyright © Aptos Foundation

use crate::IndexerClient;
use anyhow::Result;
use reqwest::{self, Response};

impl IndexerClient {
    pub async fn get_indexer_status(&self) -> Result<Response> {
        let response = self.client.get(self.url.as_str()).send().await?;

        Ok(response)
    }
}
