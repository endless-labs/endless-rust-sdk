// Copyright © Endless
// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    access_path::AccessPath,
    account_config::CORE_CODE_ADDRESS,
    event::{EventHandle, EventKey},
    state_store::{state_key::StateKey, StateView},
};
use anyhow::{format_err, Result};
use bytes::Bytes;
use move_core_types::{
    ident_str,
    identifier::{IdentStr, Identifier},
    language_storage::StructTag,
    move_resource::{MoveResource, MoveStructType},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt, fmt::Debug, sync::Arc};

mod approved_execution_hashes;
mod chain_id;
mod commit_history;
mod consensus_config;
mod endless_features;
mod endless_version;
mod execution_config;
mod gas_schedule;
mod jwk_consensus_config;
mod randomness_config;
mod timed_features;
mod timestamp;
mod transaction_fee;
mod validator_set;

pub use self::{
    approved_execution_hashes::ApprovedExecutionHashes,
    commit_history::CommitHistoryResource,
    consensus_config::{
        AnchorElectionMode, ConsensusAlgorithmConfig, ConsensusConfigV1, DagConsensusConfigV1,
        LeaderReputationType, OnChainConsensusConfig, ProposerAndVoterConfig, ProposerElectionType,
        ValidatorTxnConfig,
    },
    endless_features::*,
    endless_version::{
        Version, ENDLESS_MAX_KNOWN_VERSION, ENDLESS_VERSION_2, ENDLESS_VERSION_3, ENDLESS_VERSION_4,
    },
    execution_config::{
        BlockGasLimitType, ExecutionConfigV1, ExecutionConfigV2, ExecutionConfigV4,
        OnChainExecutionConfig, TransactionDeduperType, TransactionShufflerType,
    },
    gas_schedule::{GasSchedule, GasScheduleV2, StorageGasSchedule},
    jwk_consensus_config::{
        ConfigV1 as JWKConsensusConfigV1, OIDCProvider, OnChainJWKConsensusConfig,
    },
    randomness_config::{OnChainRandomnessConfig, RandomnessConfigMoveStruct},
    timed_features::{TimedFeatureFlag, TimedFeatureOverride, TimedFeatures, TimedFeaturesBuilder},
    timestamp::CurrentTimeMicroseconds,
    transaction_fee::TransactionFeeBurnCap,
    validator_set::{ConsensusScheme, ValidatorSet},
};

/// To register an on-chain config in Rust:
/// 1. Implement the `OnChainConfig` trait for the Rust representation of the config
/// 2. Add the config's `ConfigID` to `ON_CHAIN_CONFIG_REGISTRY`

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConfigID(&'static str, &'static str, &'static str);

impl ConfigID {
    pub fn name(&self) -> String {
        self.2.to_string()
    }
}

impl fmt::Display for ConfigID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OnChain config ID [address: {}, identifier: {}]",
            self.0, self.1
        )
    }
}

pub trait OnChainConfigProvider: Debug + Clone + Send + Sync + 'static {
    fn get<T: OnChainConfig>(&self) -> Result<T>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InMemoryOnChainConfig {
    configs: HashMap<ConfigID, Vec<u8>>,
}

impl InMemoryOnChainConfig {
    pub fn new(configs: HashMap<ConfigID, Vec<u8>>) -> Self {
        Self { configs }
    }
}

impl OnChainConfigProvider for InMemoryOnChainConfig {
    fn get<T: OnChainConfig>(&self) -> Result<T> {
        let bytes = self
            .configs
            .get(&T::CONFIG_ID)
            .ok_or_else(|| format_err!("[on-chain cfg] config not in payload"))?;
        T::deserialize_into_config(bytes)
    }
}

#[derive(Clone, Debug)]
pub struct OnChainConfigPayload<P: OnChainConfigProvider> {
    epoch: u64,
    provider: Arc<P>,
}

impl<P: OnChainConfigProvider> OnChainConfigPayload<P> {
    pub fn new(epoch: u64, provider: P) -> Self {
        Self {
            epoch,
            provider: Arc::new(provider),
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn get<T: OnChainConfig>(&self) -> Result<T> {
        self.provider.get()
    }
}

/// Trait to be implemented by a storage type from which to read on-chain configs
pub trait ConfigStorage {
    fn fetch_config(&self, access_path: AccessPath) -> Option<Bytes>;
}

/// Trait to be implemented by a Rust struct representation of an on-chain config
/// that is stored in storage as a serialized byte array
pub trait OnChainConfig: Send + Sync + DeserializeOwned {
    const ADDRESS: &'static str = "0x1";
    const MODULE_IDENTIFIER: &'static str;
    const TYPE_IDENTIFIER: &'static str;
    const CONFIG_ID: ConfigID = ConfigID(
        Self::ADDRESS,
        Self::MODULE_IDENTIFIER,
        Self::TYPE_IDENTIFIER,
    );

    // Single-round BCS deserialization from bytes to `Self`
    // This is the expected deserialization pattern if the Rust representation lives natively in Move.
    // but sometimes `deserialize_into_config` may need an extra customized round of deserialization
    // when the data is represented as opaque vec<u8> in Move.
    // In the override, we can reuse this default logic via this function
    // Note: we cannot directly call the default `deserialize_into_config` implementation
    // in its override - this will just refer to the override implementation itself
    fn deserialize_default_impl(bytes: &[u8]) -> Result<Self> {
        bcs::from_bytes::<Self>(bytes)
            .map_err(|e| format_err!("[on-chain config] Failed to deserialize into config: {}", e))
    }

    // Function for deserializing bytes to `Self`
    // It will by default try one round of BCS deserialization directly to `Self`
    // The implementation for the concrete type should override this function if this
    // logic needs to be customized
    fn deserialize_into_config(bytes: &[u8]) -> Result<Self> {
        Self::deserialize_default_impl(bytes)
    }

    /// TODO: This does not work if `T`'s reflection on the Move side is using resource groups.
    fn fetch_config<T>(storage: &T) -> Option<Self>
    where
        T: ConfigStorage + ?Sized,
    {
        let access_path = Self::access_path().ok()?;
        match storage.fetch_config(access_path) {
            Some(bytes) => Self::deserialize_into_config(&bytes).ok(),
            None => None,
        }
    }

    fn access_path() -> anyhow::Result<AccessPath> {
        access_path_for_config(Self::CONFIG_ID)
    }

    fn struct_tag() -> StructTag {
        struct_tag_for_config(Self::CONFIG_ID)
    }
}

impl<S: StateView> ConfigStorage for S {
    fn fetch_config(&self, access_path: AccessPath) -> Option<Bytes> {
        let state_key = StateKey::access_path(access_path);
        self.get_state_value(&state_key)
            .ok()?
            .map(|s| s.bytes().clone())
    }
}

pub fn new_epoch_event_key() -> EventKey {
    EventKey::new(0, CORE_CODE_ADDRESS)
}

pub fn access_path_for_config(config_id: ConfigID) -> anyhow::Result<AccessPath> {
    let struct_tag = struct_tag_for_config(config_id);
    Ok(AccessPath::new(
        CORE_CODE_ADDRESS,
        AccessPath::resource_path_vec(struct_tag)?,
    ))
}

pub fn struct_tag_for_config(config_id: ConfigID) -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new(config_id.1).expect("fail to make identifier"),
        name: Identifier::new(config_id.2).expect("fail to make identifier"),
        type_params: vec![],
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationResource {
    epoch: u64,
    last_reconfiguration_time: u64,
    events: EventHandle,
}

impl ConfigurationResource {
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn last_reconfiguration_time(&self) -> u64 {
        self.last_reconfiguration_time
    }

    pub fn events(&self) -> &EventHandle {
        &self.events
    }

    #[cfg(feature = "fuzzing")]
    pub fn bump_epoch_for_test(&self) -> Self {
        let epoch = self.epoch + 1;
        let last_reconfiguration_time = self.last_reconfiguration_time + 1;
        let mut events = self.events.clone();
        *events.count_mut() += 1;

        Self {
            epoch,
            last_reconfiguration_time,
            events,
        }
    }
}

#[cfg(feature = "fuzzing")]
impl Default for ConfigurationResource {
    fn default() -> Self {
        Self {
            epoch: 0,
            last_reconfiguration_time: 0,
            events: EventHandle::new(EventKey::new(16, CORE_CODE_ADDRESS), 0),
        }
    }
}

impl MoveStructType for ConfigurationResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("reconfiguration");
    const STRUCT_NAME: &'static IdentStr = ident_str!("Configuration");
}

impl MoveResource for ConfigurationResource {}

impl OnChainConfig for ConfigurationResource {
    const MODULE_IDENTIFIER: &'static str = "reconfiguration";
    const TYPE_IDENTIFIER: &'static str = "Configuration";
}
