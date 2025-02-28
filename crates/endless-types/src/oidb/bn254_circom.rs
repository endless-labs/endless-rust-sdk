// Copyright © Endless
// Copyright © Aptos Foundation

use super::circuit_constants;
use crate::{
    jwks::rsa::RSA_JWK,
    oidb::{Configuration, IdCommitment, OidbPublicKey, OidbSignature, ZkpOrOpenIdSig},
    serialize,
};
use anyhow::bail;
use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use endless_crypto::{poseidon_bn254, CryptoMaterialError};
use num_traits::{One, Zero};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

// TODO(oidb): Some of this stuff, if not all, belongs to the endless-crypto crate

pub const G1_PROJECTIVE_COMPRESSED_NUM_BYTES: usize = 32;
pub const G2_PROJECTIVE_COMPRESSED_NUM_BYTES: usize = 64;

static EMPTY_EXTRA_HASH: Lazy<Fr> = Lazy::new(|| {
    poseidon_bn254::pad_and_hash_string("{", circuit_constants::MAX_EXTRA_FIELD_BYTES as usize)
        .unwrap()
});

/// This will do the proper subgroup membership checks.
pub(crate) fn g1_projective_str_to_affine(x: &str, y: &str) -> anyhow::Result<G1Affine> {
    let g1_affine = G1Bytes::new_unchecked(x, y)?.deserialize_into_affine()?;
    Ok(g1_affine)
}

/// This will do the proper subgroup membership checks.
pub(crate) fn g2_projective_str_to_affine(x: [&str; 2], y: [&str; 2]) -> anyhow::Result<G2Affine> {
    let g2_affine = G2Bytes::new_unchecked(x, y)?.as_affine()?;
    Ok(g2_affine)
}

/// Converts a decimal string to an Fq
fn parse_fq_element(s: &str) -> Result<Fq, CryptoMaterialError> {
    s.parse::<Fq>()
        .map_err(|_e| CryptoMaterialError::DeserializationError)
}

#[allow(unused)]
/// Converts a decimal string to an Fr
pub fn parse_fr_element(s: &str) -> Result<Fr, CryptoMaterialError> {
    s.parse::<Fr>()
        .map_err(|_e| CryptoMaterialError::DeserializationError)
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct G1Bytes(pub(crate) [u8; G1_PROJECTIVE_COMPRESSED_NUM_BYTES]);

impl G1Bytes {
    pub fn new_unchecked(x: &str, y: &str) -> anyhow::Result<Self> {
        let g1 = G1Projective::new_unchecked(
            parse_fq_element(x)?,
            parse_fq_element(y)?,
            parse_fq_element("1")?,
        );

        let bytes: Vec<u8> = serialize!(g1);
        Self::new_from_vec(bytes)
    }

    /// Used internally or for testing.
    pub fn new_from_vec(vec: Vec<u8>) -> anyhow::Result<Self> {
        if vec.len() == G1_PROJECTIVE_COMPRESSED_NUM_BYTES {
            let mut bytes = [0; G1_PROJECTIVE_COMPRESSED_NUM_BYTES];
            bytes.copy_from_slice(&vec);
            Ok(Self(bytes))
        } else {
            bail!(
                "Serialized BN254 G1 must have exactly {} bytes",
                G1_PROJECTIVE_COMPRESSED_NUM_BYTES
            )
        }
    }

    pub fn deserialize_into_affine(&self) -> Result<G1Affine, CryptoMaterialError> {
        self.try_into()
    }
}

impl TryInto<G1Projective> for &G1Bytes {
    type Error = CryptoMaterialError;

    fn try_into(self) -> Result<G1Projective, CryptoMaterialError> {
        G1Projective::deserialize_compressed(self.0.as_slice())
            .map_err(|_| CryptoMaterialError::DeserializationError)
    }
}

impl TryInto<G1Affine> for &G1Bytes {
    type Error = CryptoMaterialError;

    fn try_into(self) -> Result<G1Affine, CryptoMaterialError> {
        let g1_projective: G1Projective = self.try_into()?;
        Ok(g1_projective.into())
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct G2Bytes(#[serde(with = "BigArray")] pub(crate) [u8; G2_PROJECTIVE_COMPRESSED_NUM_BYTES]);

impl G2Bytes {
    pub fn new_unchecked(x: [&str; 2], y: [&str; 2]) -> anyhow::Result<Self> {
        let g2 = G2Projective::new_unchecked(
            Fq2::new(parse_fq_element(x[0])?, parse_fq_element(x[1])?),
            Fq2::new(parse_fq_element(y[0])?, parse_fq_element(y[1])?),
            Fq2::new(parse_fq_element("1")?, parse_fq_element("0")?),
        );

        let bytes: Vec<u8> = serialize!(g2);
        Self::new_from_vec(bytes)
    }

    pub fn new_from_vec(vec: Vec<u8>) -> anyhow::Result<Self> {
        if vec.len() == G2_PROJECTIVE_COMPRESSED_NUM_BYTES {
            let mut bytes = [0; G2_PROJECTIVE_COMPRESSED_NUM_BYTES];
            bytes.copy_from_slice(&vec);
            Ok(Self(bytes))
        } else {
            bail!(
                "Serialized BN254 G2 must have exactly {} bytes",
                G2_PROJECTIVE_COMPRESSED_NUM_BYTES
            )
        }
    }

    pub fn as_affine(&self) -> Result<G2Affine, CryptoMaterialError> {
        self.try_into()
    }
}

impl TryInto<G2Projective> for &G2Bytes {
    type Error = CryptoMaterialError;

    fn try_into(self) -> Result<G2Projective, CryptoMaterialError> {
        G2Projective::deserialize_compressed(self.0.as_slice())
            .map_err(|_| CryptoMaterialError::DeserializationError)
    }
}

impl TryInto<G2Affine> for &G2Bytes {
    type Error = CryptoMaterialError;

    fn try_into(self) -> Result<G2Affine, CryptoMaterialError> {
        let g2_projective: G2Projective = self.try_into()?;
        Ok(g2_projective.into())
    }
}

pub fn get_public_inputs_hash(
    sig: &OidbSignature,
    pk: &OidbPublicKey,
    jwk: &RSA_JWK,
    config: &Configuration,
) -> anyhow::Result<Fr> {
    if let ZkpOrOpenIdSig::Groth16Zkp(proof) = &sig.sig {
        let (has_extra_field, extra_field_hash) = match &proof.extra_field {
            None => (Fr::zero(), *EMPTY_EXTRA_HASH),
            Some(extra_field) => (
                Fr::one(),
                poseidon_bn254::pad_and_hash_string(
                    extra_field,
                    config.max_extra_field_bytes as usize,
                )?,
            ),
        };

        let (override_aud_val_hash, use_override_aud) = match &proof.override_aud_val {
            Some(override_aud_val) => (
                poseidon_bn254::pad_and_hash_string(
                    override_aud_val,
                    IdCommitment::MAX_AUD_VAL_BYTES,
                )?,
                ark_bn254::Fr::from(1),
            ),
            None => (
                poseidon_bn254::pad_and_hash_string("", IdCommitment::MAX_AUD_VAL_BYTES)?,
                ark_bn254::Fr::from(0),
            ),
        };

        // Add the hash of the jwt_header with the "." separator appended
        let jwt_header_with_separator = format!("{}.", sig.jwt_header_b64);
        let jwt_header_hash = poseidon_bn254::pad_and_hash_string(
            &jwt_header_with_separator,
            config.max_jwt_header_b64_bytes as usize,
        )?;

        let jwk_hash = jwk.to_poseidon_scalar()?;

        // Add the hash of the value of the `iss` field
        let iss_field_hash = poseidon_bn254::pad_and_hash_string(
            pk.iss_val.as_str(),
            config.max_iss_val_bytes as usize,
        )?;

        // Add the id_commitment as a scalar
        let idc = Fr::from_le_bytes_mod_order(&pk.idc.0);

        // Add the exp_timestamp_secs as a scalar
        let exp_timestamp_secs = Fr::from(sig.exp_timestamp_secs);

        // Add the epk lifespan as a scalar
        let exp_horizon_secs = Fr::from(proof.exp_horizon_secs);

        // Add the epk as padded and packed scalars
        let mut epk_frs = poseidon_bn254::pad_and_pack_bytes_to_scalars_with_len(
            sig.ephemeral_pubkey.to_bytes().as_slice(),
            config.max_commited_epk_bytes as usize,
        )?;

        // println!("Num EPK scalars:    {}", epk_frs.len());
        // for (i, e) in epk_frs.iter().enumerate() {
        //     println!("EPK Fr[{}]:          {}", i, e.to_string())
        // }
        // println!("IDC:                {}", idc);
        // println!("exp_timestamp_secs: {}", exp_timestamp_secs);
        // println!("exp_horizon_secs:   {}", exp_horizon_secs);
        // println!("iss field hash:     {}", pk.iss_val);
        // println!("Has extra field:    {}", has_extra_field);
        // println!("Extra field val:    {:?}", proof.extra_field);
        // println!("Extra field hash:   {}", extra_field_hash);
        // println!("JWT header val:     {}", jwt_header_with_separator);
        // println!("JWT header hash:    {}", jwt_header_hash);
        // println!("JWK hash:           {}", jwk_hash);
        // println!("Override aud hash:  {}", override_aud_val_hash);
        // println!("Use override aud:   {}", use_override_aud.to_string());

        let mut frs = vec![];
        frs.append(&mut epk_frs);
        frs.push(idc);
        frs.push(exp_timestamp_secs);
        frs.push(exp_horizon_secs);
        frs.push(iss_field_hash);
        frs.push(has_extra_field);
        frs.push(extra_field_hash);
        frs.push(jwt_header_hash);
        frs.push(jwk_hash);
        frs.push(override_aud_val_hash);
        frs.push(use_override_aud);
        poseidon_bn254::hash_scalars(frs)
    } else {
        bail!("Cannot get_public_inputs_hash for OidbSignature")
    }
}

#[cfg(test)]
mod test {
    use crate::oidb::{
        bn254_circom::{
            G1Bytes, G2Bytes, G1_PROJECTIVE_COMPRESSED_NUM_BYTES,
            G2_PROJECTIVE_COMPRESSED_NUM_BYTES,
        },
        circuit_constants::devnet_prepared_vk,
        Groth16VerificationKey,
    };
    use ark_bn254::Bn254;
    use ark_groth16::PreparedVerifyingKey;

    #[test]
    pub fn test_bn254_serialized_sizes() {
        let g1 = G1Bytes::new_unchecked(
            "16672231080302629756836614130913173861541009360974119524782950408048375831661",
            "1076145001163048025135533382088266750240489485046298539187659509488738517245",
        )
        .unwrap();

        let g2 = G2Bytes::new_unchecked(
            [
                "1125365732643211423779651913319958385653115422366520671538751860820509133538",
                "10055196097002324305342942912758079446356594743098794928675544207400347950287",
            ],
            [
                "10879716754714953827605171295191459580695363989155343984818520267224463075503",
                "440220374146936557739765173414663598678359360031905981547938788314460390904",
            ],
        )
        .unwrap();

        let g1_bytes = bcs::to_bytes(&g1).unwrap();
        assert_eq!(g1_bytes.len(), G1_PROJECTIVE_COMPRESSED_NUM_BYTES);

        let g2_bytes = bcs::to_bytes(&g2).unwrap();
        assert_eq!(g2_bytes.len(), G2_PROJECTIVE_COMPRESSED_NUM_BYTES);
    }

    #[test]
    // Tests conversion between the devnet ark_groth16::PreparedVerificationKey and our Move
    // representation of it.
    fn print_groth16_pvk() {
        let groth16_vk: Groth16VerificationKey = devnet_prepared_vk().into();
        let same_pvk: PreparedVerifyingKey<Bn254> = groth16_vk.try_into().unwrap();

        assert_eq!(same_pvk, devnet_prepared_vk());
    }
}
