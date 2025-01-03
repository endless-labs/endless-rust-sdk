// Copyright © Endless
// Copyright © Aptos Foundation

use reqwest::{Client, ClientBuilder};

pub mod query_v2;

#[derive(Clone, Debug)]
pub struct IndexerClient {
    pub client: Client,
    pub url: String,
}

impl IndexerClient {
    pub fn builder() -> ClientBuilder {
        reqwest::Client::builder()
    }

    pub fn new(url: String) -> Self {
        IndexerClient {
            client: Self::builder().build().unwrap(),
            url,
        }
    }
}
