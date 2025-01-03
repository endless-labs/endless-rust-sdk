// Copyright © Endless
// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The official Rust SDK for Endless.
//!
//! ## Modules
//!
//! This SDK provides all the necessary components for building on top of the Endless Blockchain. Some of the important modules are:
//!
//! * `crypto` - Types used for signing and verifying
//! * `move_types` - Includes types used when interacting with the Move VM
//! * `rest_client` - The Endless API Client, used for sending requests to the Endless Blockchain.
//! * `transaction_builder` - Includes helpers for constructing transactions
//! * `types` - Includes types for Endless on-chain data structures
//!
//! ## Example
//!
//! Here is a simple example to show how to create two accounts and do a P2p transfer on testnet:
//! todo(davidiw) bring back example using rest
//!

pub use bcs;

pub mod coin_client;
pub mod helper_client;

pub mod crypto {
    pub use endless_crypto::*;
}

pub mod move_types {
    pub use move_core_types::*;
}

pub mod rest_client {
    pub use endless_rest_client::*;
}

pub mod indexer_client {
    pub use endless_indexer_client::*;
}

pub mod transaction_builder;

pub mod types;

pub mod constant;
