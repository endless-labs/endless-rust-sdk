// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{Address, HexEncodedBytes, PublicKey, Signature, U64};
use anyhow::bail;
use endless_types::{
    account_config::AccountResource,
    oidb::{OidbPublicKey, OidbSignature},
    transaction::authenticator::{AnyPublicKey, AnySignature, AuthenticationKey},
};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

/// Account data
///
/// A simplified version of the onchain Account resource
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object)]
pub struct AccountData {
    pub sequence_number: U64,
    pub authentication_key: Vec<HexEncodedBytes>,
    pub num_signatures_required: u64,
}

impl From<AccountResource> for AccountData {
    fn from(ar: AccountResource) -> Self {
        Self {
            sequence_number: ar.sequence_number().into(),
            authentication_key: ar
                .authentication_key()
                .iter()
                .map(|a| a.to_vec().into())
                .collect(),
            num_signatures_required: ar.num_signatures_required(),
        }
    }
}

/// Account signature validation request
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object)]
pub struct AccountVerifySignatureRequest {
    pub address: Address,
    pub signature: Signature,
    pub public_key: PublicKey,
    pub message: HexEncodedBytes,
}

impl AccountVerifySignatureRequest {
    pub fn verify(&self) -> anyhow::Result<()> {
        let sig: AnySignature = self.signature.clone().try_into()?;
        let pk: AnyPublicKey = self.public_key.clone().try_into()?;
        let address = match &pk {
            AnyPublicKey::Ed25519 { public_key } => {
                AuthenticationKey::ed25519(public_key).account_address()
            },
            AnyPublicKey::OIDB { .. } => AuthenticationKey::any_key(pk.clone()).account_address(),
            _ => bail!("Unsupported public key type"),
        };
        if &address != self.address.inner() {
            bail!("Invalid address");
        }
        sig.verify_arbitrary_msg(&pk, &self.message.0)
    }

    pub fn to_oidb(&self) -> Option<(OidbPublicKey, OidbSignature)> {
        let sig: AnySignature = self.signature.clone().try_into().ok()?;
        let pk: AnyPublicKey = self.public_key.clone().try_into().ok()?;
        match (sig, pk) {
            (AnySignature::OIDB { signature }, AnyPublicKey::OIDB { public_key }) => {
                Some((public_key, signature))
            },
            _ => None,
        }
    }
}
