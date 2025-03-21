// Copyright © Endless
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use serde::{Deserialize, Serialize};
use strum_macros::FromRepr;
/// The feature flags define in the Move source. This must stay aligned with the constants there.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[allow(non_camel_case_types)]
pub enum FeatureFlag {
    CODE_DEPENDENCY_CHECK = 1,
    TREAT_FRIEND_AS_PRIVATE = 2,
    SHA_512_AND_RIPEMD_160_NATIVES = 3,
    ENDLESS_STD_CHAIN_ID_NATIVES = 4,
    VM_BINARY_FORMAT_V6 = 5,
    COLLECT_AND_DISTRIBUTE_GAS_FEES = 6,
    MULTI_ED25519_PK_VALIDATE_V2_NATIVES = 7,
    BLAKE2B_256_NATIVE = 8,
    RESOURCE_GROUPS = 9,
    MULTISIG_ACCOUNTS = 10,
    DELEGATION_POOLS = 11,
    CRYPTOGRAPHY_ALGEBRA_NATIVES = 12,
    BLS12_381_STRUCTURES = 13,
    ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH = 14,
    STRUCT_CONSTRUCTORS = 15,
    PERIODICAL_REWARD_RATE_DECREASE = 16,
    PARTIAL_GOVERNANCE_VOTING = 17,
    SIGNATURE_CHECKER_V2 = 18,
    STORAGE_SLOT_METADATA = 19,
    CHARGE_INVARIANT_VIOLATION = 20,
    DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING = 21,
    GAS_PAYER_ENABLED = 22,
    ENDLESS_UNIQUE_IDENTIFIERS = 23,
    BULLETPROOFS_NATIVES = 24,
    SIGNER_NATIVE_FORMAT_FIX = 25,
    MODULE_EVENT = 26,
    EMIT_FEE_STATEMENT = 27,
    STORAGE_DELETION_REFUND = 28,
    SIGNATURE_CHECKER_V2_SCRIPT_FIX = 29,
    AGGREGATOR_V2_API = 30,
    SAFER_RESOURCE_GROUPS = 31,
    SAFER_METADATA = 32,
    SINGLE_SENDER_AUTHENTICATOR = 33,
    SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION = 34,
    FEE_PAYER_ACCOUNT_OPTIONAL = 35,
    AGGREGATOR_V2_DELAYED_FIELDS = 36,
    CONCURRENT_TOKEN_V2 = 37,
    LIMIT_MAX_IDENTIFIER_LENGTH = 38,
    OPERATOR_BENEFICIARY_CHANGE = 39,
    VM_BINARY_FORMAT_V7 = 40,
    RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET = 41,
    COMMISSION_CHANGE_DELEGATION_POOL = 42,
    BN254_STRUCTURES = 43,
    WEBAUTHN_SIGNATURE = 44,
    RECONFIGURE_WITH_DKG = 45,
    OIDB_SIGNATURE = 46,
    OIDB_ZKLESS_SIGNATURE = 47,
    REMOVE_DETAILED_ERROR_FROM_HASH = 48,
    JWK_CONSENSUS = 49,
    CONCURRENT_FUNGIBLE_ASSETS = 50,
    REFUNDABLE_BYTES = 51,
    OBJECT_CODE_DEPLOYMENT = 52,
    MAX_OBJECT_NESTING_CHECK = 53,
    KEYLESS_ACCOUNTS_WITH_PASSKEYS = 54,
    FAST_RANDOMNESS = 55,
    SKIP_COMPATIBILITY_CHECK = 56,
    REWARD_SPLIT = 57,
    EDS_SUPPLY_INFLATION = 58,
}

impl FeatureFlag {
    pub fn default_features() -> Vec<Self> {
        vec![
            FeatureFlag::CODE_DEPENDENCY_CHECK,
            FeatureFlag::TREAT_FRIEND_AS_PRIVATE,
            FeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES,
            FeatureFlag::ENDLESS_STD_CHAIN_ID_NATIVES,
            FeatureFlag::VM_BINARY_FORMAT_V6,
            FeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES,
            FeatureFlag::BLAKE2B_256_NATIVE,
            FeatureFlag::RESOURCE_GROUPS,
            FeatureFlag::MULTISIG_ACCOUNTS,
            FeatureFlag::DELEGATION_POOLS,
            FeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES,
            FeatureFlag::BLS12_381_STRUCTURES,
            FeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH,
            FeatureFlag::STRUCT_CONSTRUCTORS,
            FeatureFlag::SIGNATURE_CHECKER_V2,
            FeatureFlag::STORAGE_SLOT_METADATA,
            FeatureFlag::CHARGE_INVARIANT_VIOLATION,
            FeatureFlag::ENDLESS_UNIQUE_IDENTIFIERS,
            FeatureFlag::GAS_PAYER_ENABLED,
            FeatureFlag::BULLETPROOFS_NATIVES,
            FeatureFlag::SIGNER_NATIVE_FORMAT_FIX,
            FeatureFlag::MODULE_EVENT,
            FeatureFlag::EMIT_FEE_STATEMENT,
            FeatureFlag::STORAGE_DELETION_REFUND,
            FeatureFlag::SIGNATURE_CHECKER_V2_SCRIPT_FIX,
            FeatureFlag::AGGREGATOR_V2_API,
            FeatureFlag::SAFER_RESOURCE_GROUPS,
            FeatureFlag::SAFER_METADATA,
            FeatureFlag::SINGLE_SENDER_AUTHENTICATOR,
            FeatureFlag::SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION,
            FeatureFlag::FEE_PAYER_ACCOUNT_OPTIONAL,
            FeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS,
            FeatureFlag::CONCURRENT_TOKEN_V2,
            FeatureFlag::LIMIT_MAX_IDENTIFIER_LENGTH,
            FeatureFlag::OPERATOR_BENEFICIARY_CHANGE,
            FeatureFlag::BN254_STRUCTURES,
            FeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET,
            FeatureFlag::COMMISSION_CHANGE_DELEGATION_POOL,
            FeatureFlag::WEBAUTHN_SIGNATURE,
            FeatureFlag::RECONFIGURE_WITH_DKG, //TODO: re-enable once randomness is ready.
            FeatureFlag::JWK_CONSENSUS,
            FeatureFlag::REFUNDABLE_BYTES,
            FeatureFlag::OBJECT_CODE_DEPLOYMENT,
            FeatureFlag::MAX_OBJECT_NESTING_CHECK,
            FeatureFlag::KEYLESS_ACCOUNTS_WITH_PASSKEYS,
            FeatureFlag::PERIODICAL_REWARD_RATE_DECREASE,
            FeatureFlag::COLLECT_AND_DISTRIBUTE_GAS_FEES,
            FeatureFlag::REWARD_SPLIT,
            FeatureFlag::EDS_SUPPLY_INFLATION,
        ]
    }
}

/// Representation of features on chain as a bitset.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Features {
    #[serde(with = "serde_bytes")]
    pub features: Vec<u8>,
}

impl Default for Features {
    fn default() -> Self {
        let mut features = Features {
            features: vec![0; 5],
        };

        for feature in FeatureFlag::default_features() {
            features.enable(feature);
        }

        features
    }
}

impl OnChainConfig for Features {
    const MODULE_IDENTIFIER: &'static str = "features";
    const TYPE_IDENTIFIER: &'static str = "Features";
}

impl Features {
    fn resize_for_flag(&mut self, flag: FeatureFlag) -> (usize, u8) {
        let byte_index = (flag as u64 / 8) as usize;
        let bit_mask = 1 << (flag as u64 % 8);
        while self.features.len() <= byte_index {
            self.features.push(0);
        }
        (byte_index, bit_mask)
    }

    pub fn enable(&mut self, flag: FeatureFlag) {
        let (byte_index, bit_mask) = self.resize_for_flag(flag);
        self.features[byte_index] |= bit_mask;
    }

    pub fn disable(&mut self, flag: FeatureFlag) {
        let (byte_index, bit_mask) = self.resize_for_flag(flag);
        self.features[byte_index] &= !bit_mask;
    }

    pub fn into_flag_vec(self) -> Vec<FeatureFlag> {
        let Self { features } = self;
        features
            .into_iter()
            .flat_map(|byte| (0..8).map(move |bit_idx| byte & (1 << bit_idx) != 0))
            .enumerate()
            .filter(|(_feature_idx, enabled)| *enabled)
            .map(|(feature_idx, _)| FeatureFlag::from_repr(feature_idx).unwrap())
            .collect()
    }

    pub fn is_enabled(&self, flag: FeatureFlag) -> bool {
        let val = flag as u64;
        let byte_index = (val / 8) as usize;
        let bit_mask = 1 << (val % 8);
        byte_index < self.features.len() && (self.features[byte_index] & bit_mask != 0)
    }

    pub fn are_resource_groups_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::RESOURCE_GROUPS)
    }

    pub fn is_storage_slot_metadata_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::STORAGE_SLOT_METADATA)
    }

    pub fn is_module_event_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::MODULE_EVENT)
    }

    pub fn is_emit_fee_statement_enabled(&self) -> bool {
        // requires module events
        self.is_module_event_enabled() && self.is_enabled(FeatureFlag::EMIT_FEE_STATEMENT)
    }

    pub fn is_storage_deletion_refund_enabled(&self) -> bool {
        // requires emit fee statement
        self.is_emit_fee_statement_enabled()
            && self.is_enabled(FeatureFlag::STORAGE_DELETION_REFUND)
    }

    /// Whether the Aggregator V2 API feature is enabled.
    /// Once enabled, the functions from aggregator_v2.move will be available for use.
    pub fn is_aggregator_v2_api_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::AGGREGATOR_V2_API)
    }

    /// Whether the Aggregator V2 delayed fields feature is enabled.
    /// Once enabled, Aggregator V2 functions become parallel.
    pub fn is_aggregator_v2_delayed_fields_enabled(&self) -> bool {
        // This feature depends on resource groups being split inside VMChange set,
        // which is gated by RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET feature, so
        // require that feature to be enabled as well.
        self.is_enabled(FeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS)
            && self.is_resource_groups_split_in_vm_change_set_enabled()
    }

    pub fn is_resource_groups_split_in_vm_change_set_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET)
    }

    /// Whether the OIDB feature is enabled, specifically the ZK path with ZKP-based signatures.
    /// The ZK-less path is controlled via a different `FeatureFlag::OIDB_ZKLESS_SIGNATURE` flag.
    pub fn is_oidb_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::OIDB_SIGNATURE)
    }

    /// If `FeatureFlag::OIDB_SIGNATURE` is enabled, this feature additionally allows for a "ZK-less
    /// path" where the blockchain can verify OpenID signatures directly. This ZK-less mode exists
    /// for two reasons. First, it gives as a simpler way to test the feature. Second, it acts as a
    /// safety precaution in case of emergency (e.g., if the ZK-based signatures must be temporarily
    /// turned off due to a zeroday exploit, the ZK-less path will still allow users to transact,
    /// but without privacy).
    pub fn is_oidb_zkless_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::OIDB_ZKLESS_SIGNATURE)
    }

    pub fn is_reconfigure_with_dkg_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::RECONFIGURE_WITH_DKG)
    }

    pub fn is_refundable_bytes_enabled(&self) -> bool {
        self.is_enabled(FeatureFlag::REFUNDABLE_BYTES)
    }
}

#[test]
fn test_features_into_flag_vec() {
    let mut features = Features { features: vec![] };
    features.enable(FeatureFlag::BLS12_381_STRUCTURES);
    features.enable(FeatureFlag::BN254_STRUCTURES);
    let flag_vec = features.into_flag_vec();
    assert_eq!(
        vec![
            FeatureFlag::BLS12_381_STRUCTURES,
            FeatureFlag::BN254_STRUCTURES
        ],
        flag_vec
    );
}
