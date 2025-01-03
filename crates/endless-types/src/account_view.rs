// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::{
    access_path::AccessPath,
    account_config::{
        eds_metadata, primary_store, AccountResource, ChainIdResource, FungibleStoreResource,
        ObjectGroupResource,
    },
    on_chain_config::{ConfigurationResource, OnChainConfig, ValidatorSet, Version},
    state_store::state_key::StateKey,
    validator_config::{ValidatorConfig, ValidatorOperatorConfigResource},
};
use anyhow::anyhow;
use bytes::Bytes;
use move_core_types::{
    account_address::AccountAddress,
    language_storage::StructTag,
    move_resource::{MoveResource, MoveStructType},
};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
pub trait AccountView {
    fn get_state_value(&self, state_key: &StateKey) -> anyhow::Result<Option<Bytes>>;

    fn get_account_address(&self) -> anyhow::Result<Option<AccountAddress>>;

    fn get_validator_set(&self) -> anyhow::Result<Option<ValidatorSet>> {
        self.get_on_chain_config::<ValidatorSet>()
    }

    fn get_configuration_resource(&self) -> anyhow::Result<Option<ConfigurationResource>> {
        self.get_move_resource::<ConfigurationResource>()
    }

    fn get_move_resource<T: MoveResource>(&self) -> anyhow::Result<Option<T>> {
        self.get_resource_impl(T::struct_tag().access_vector())
    }

    fn get_validator_config_resource(&self) -> anyhow::Result<Option<ValidatorConfig>> {
        self.get_resource::<ValidatorConfig>()
    }

    fn get_validator_operator_config_resource(
        &self,
    ) -> anyhow::Result<Option<ValidatorOperatorConfigResource>> {
        self.get_resource::<ValidatorOperatorConfigResource>()
    }

    fn get_on_chain_config<T: OnChainConfig>(&self) -> anyhow::Result<Option<T>> {
        self.get_resource_impl(T::access_path()?.path)
    }

    fn get_version(&self) -> anyhow::Result<Option<Version>> {
        self.get_on_chain_config::<Version>()
    }

    fn get_resource<T: MoveResource>(&self) -> anyhow::Result<Option<T>> {
        self.get_resource_impl(T::struct_tag().access_vector())
    }

    fn get_resource_from_group<T: MoveResource>(
        &self,
        address: AccountAddress,
    ) -> anyhow::Result<Option<T>> {
        let ap = AccessPath::resource_group_access_path(address, ObjectGroupResource::struct_tag());
        let Some(group_blob) = self.get_state_value(&StateKey::access_path(ap))? else {
            return Ok(None);
        };
        let group: BTreeMap<StructTag, Vec<u8>> = bcs::from_bytes(&group_blob)?;
        if let Some(data) = group.get(&T::struct_tag()) {
            Ok(Some(bcs::from_bytes(data)?))
        } else {
            Ok(None)
        }
    }

    fn get_chain_id_resource(&self) -> anyhow::Result<Option<ChainIdResource>> {
        self.get_resource::<ChainIdResource>()
    }

    fn get_coin_store_resource(&self) -> anyhow::Result<Option<FungibleStoreResource>> {
        let account_address = self
            .get_account_address()?
            .ok_or_else(|| anyhow!("Could not fetch account address"))?;
        let store_addr = primary_store(&account_address, &eds_metadata());
        self.get_resource_from_group(store_addr)
    }

    fn get_state_key_for_path(&self, path: Vec<u8>) -> anyhow::Result<StateKey> {
        let account_address = self
            .get_account_address()?
            .ok_or_else(|| anyhow!("Could not fetch account address"))?;
        Ok(StateKey::access_path(AccessPath::new(
            account_address,
            path,
        )))
    }

    fn get_account_resource(&self) -> anyhow::Result<Option<AccountResource>> {
        self.get_resource::<AccountResource>()
    }

    fn get_config<T: OnChainConfig>(&self) -> anyhow::Result<Option<T>> {
        self.get_resource_impl(T::access_path()?.path)
    }

    fn get_resource_impl<T: DeserializeOwned>(&self, path: Vec<u8>) -> anyhow::Result<Option<T>> {
        self.get_state_value(&self.get_state_key_for_path(path)?)?
            .map(|bytes| bcs::from_bytes(&bytes))
            .transpose()
            .map_err(Into::into)
    }
}
