// Copyright © Endless
// Copyright © Aptos Foundation

pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
}

impl Network {
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://rpc.endless.link/api/v1",
            Network::Testnet => "https://rpc-test.endless.link/v1",
            Network::Devnet => "https://rpc-testnet.endless.link/v1",
        }
    }

    pub fn indexer_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://idx.endless.link/api/v1",
            Network::Testnet => "https://idx-test.endless.link/api/v1",
            Network::Devnet => "https://idx-testnet.endless.link/api/v1",
        }
    }
}
