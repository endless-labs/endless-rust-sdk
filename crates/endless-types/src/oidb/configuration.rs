// Copyright © Endless
// Copyright © Aptos Foundation

use crate::{
    invalid_signature,
    move_utils::as_move_value::AsMoveValue,
    oidb::{circuit_constants, circuit_testcases::SAMPLE_EXP_HORIZON_SECS},
};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::MoveStructType,
    value::{MoveStruct, MoveValue},
    vm_status::{StatusCode, VMStatus},
};
use serde::{Deserialize, Serialize};

/// Reflection of endless_framework::openid_account::Configs
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub override_aud_vals: Vec<String>,
    pub max_oidb_signatures_per_txn: u16,
    pub max_exp_horizon_secs: u64,
    pub training_wheels_pubkey: Option<Vec<u8>>,
    pub max_commited_epk_bytes: u16,
    pub max_iss_val_bytes: u16,
    pub max_extra_field_bytes: u16,
    pub max_jwt_header_b64_bytes: u32,
}

impl AsMoveValue for Configuration {
    fn as_move_value(&self) -> MoveValue {
        MoveValue::Struct(MoveStruct::Runtime(vec![
            self.override_aud_vals.as_move_value(),
            self.max_oidb_signatures_per_txn.as_move_value(),
            self.max_exp_horizon_secs.as_move_value(),
            self.training_wheels_pubkey.as_move_value(),
            self.max_commited_epk_bytes.as_move_value(),
            self.max_iss_val_bytes.as_move_value(),
            self.max_extra_field_bytes.as_move_value(),
            self.max_jwt_header_b64_bytes.as_move_value(),
        ]))
    }
}

/// WARNING: This struct uses resource groups on the Move side. Do NOT implement OnChainConfig
/// for it, since `OnChainConfig::fetch_config` does not work with resource groups (yet).
impl MoveStructType for Configuration {
    const MODULE_NAME: &'static IdentStr = ident_str!("openid_account");
    const STRUCT_NAME: &'static IdentStr = ident_str!("Configuration");
}

impl Configuration {
    /// Should only be used for testing.
    pub const OVERRIDE_AUD_FOR_TESTING: &'static str = "some_override_aud";

    pub fn new_for_devnet() -> Configuration {
        Configuration {
            override_aud_vals: vec![],
            max_oidb_signatures_per_txn: 3,
            max_exp_horizon_secs: 10_000_000, // ~115.74 days
            training_wheels_pubkey: None,
            max_commited_epk_bytes: circuit_constants::MAX_COMMITED_EPK_BYTES,
            max_iss_val_bytes: circuit_constants::MAX_ISS_VAL_BYTES,
            max_extra_field_bytes: circuit_constants::MAX_EXTRA_FIELD_BYTES,
            max_jwt_header_b64_bytes: circuit_constants::MAX_JWT_HEADER_B64_BYTES,
        }
    }

    pub fn new_for_testing() -> Configuration {
        let mut config = Self::new_for_devnet();
        config.max_exp_horizon_secs = SAMPLE_EXP_HORIZON_SECS + 1; // ~31,689 years
        config
    }

    pub fn is_allowed_override_aud(&self, override_aud_val: &String) -> Result<(), VMStatus> {
        let matches = self
            .override_aud_vals
            .iter()
            .filter(|&e| e.eq(override_aud_val))
            .count();

        if matches == 0 {
            Err(invalid_signature!(
                "override aud is not allow-listed in 0x1::openid_account"
            ))
        } else {
            Ok(())
        }
    }
}
