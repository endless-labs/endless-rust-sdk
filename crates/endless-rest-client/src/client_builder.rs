// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::{
    get_version_path_with_base, Client, DEFAULT_VERSION_PATH_BASE, X_ENDLESS_SDK_HEADER_VALUE,
};
use anyhow::Result;
use endless_api_types::X_ENDLESS_CLIENT;
use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder,
};
use std::{env, str::FromStr, time::Duration};
use url::Url;

pub enum EndlessBaseUrl {
    Mainnet,
    Devnet,
    Testnet,
    Local,
    Custom(Url),
}

impl EndlessBaseUrl {
    pub fn to_url(&self) -> Url {
        match self {
            EndlessBaseUrl::Mainnet => {
                unimplemented!("Mainnet is not yet supported")
            },
            EndlessBaseUrl::Devnet => {
                unimplemented!("Devnet is not yet supported")
            },
            EndlessBaseUrl::Testnet => Url::from_str("https://rpc-test.endless.link").unwrap(),
            EndlessBaseUrl::Local => Url::from_str("http://localhost:8080").unwrap(),
            EndlessBaseUrl::Custom(url) => url.to_owned(),
        }
    }
}

pub struct ClientBuilder {
    reqwest_builder: ReqwestClientBuilder,
    version_path_base: String,
    base_url: Url,
    timeout: Duration,
    headers: HeaderMap,
}

impl ClientBuilder {
    pub fn new(endless_base_url: EndlessBaseUrl) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            X_ENDLESS_CLIENT,
            HeaderValue::from_static(X_ENDLESS_SDK_HEADER_VALUE),
        );

        let mut client_builder = Self {
            reqwest_builder: ReqwestClient::builder(),
            base_url: endless_base_url.to_url(),
            version_path_base: DEFAULT_VERSION_PATH_BASE.to_string(),
            timeout: Duration::from_secs(10), // Default to 10 seconds
            headers,
        };

        if let Ok(key) = env::var("X_API_KEY") {
            client_builder = client_builder.api_key(&key).unwrap();
        }
        client_builder
    }

    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn header(mut self, header_key: &str, header_val: &str) -> Result<Self> {
        self.headers.insert(
            HeaderName::from_str(header_key)?,
            HeaderValue::from_str(header_val)?,
        );
        Ok(self)
    }

    pub fn api_key(mut self, api_key: &str) -> Result<Self> {
        self.headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        Ok(self)
    }

    pub fn version_path_base(mut self, version_path_base: String) -> Self {
        self.version_path_base = version_path_base;
        self
    }

    pub fn build(self) -> Client {
        let version_path_base = get_version_path_with_base(self.base_url.clone());

        Client {
            inner: self
                .reqwest_builder
                .default_headers(self.headers)
                .timeout(self.timeout)
                .cookie_store(true)
                .build()
                .unwrap(),
            base_url: self.base_url,
            version_path_base,
        }
    }
}
