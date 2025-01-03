use crate::poly_commit::{
    fk20::{commit_key::CommitKey, opening_key::OpeningKey},
    kzg::KZG,
};
use ark_bls12_381::Bls12_381;
use ark_serialize::CanonicalDeserialize;
use crate_crypto_internal_eth_kzg_bls12_381::{G1Point, G2Point};
use serde::Deserialize;

const TRUSTED_SETUP_JSON: &str = include_str!("trusted_setup_4096.json");
pub const FIELD_ELEMENTS_PER_CELL: usize = 64;

/// The number of field elements needed to represent a blob.
///
/// Note: This is originally specified in the 4844 specs.
pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum SubgroupCheck {
    Check,
    NoCheck,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TrustedSetup {
    /// G1 Monomial represents a list of uncompressed
    /// hex encoded group elements in the G1 group on the bls12-381 curve.
    ///
    /// Ethereum has multiple trusted setups, however the one being
    /// used currently contains 4096 G1 elements.
    pub g1_monomial: Vec<String>,

    /// G2 Monomial represents a list of uncompressed hex encoded
    /// group elements in the G2 group on the bls12-381 curve.
    ///
    /// The length of this vector is 65.
    pub g2_monomial: Vec<String>,
}

impl Default for TrustedSetup {
    fn default() -> Self {
        TrustedSetup::from_embed()
    }
}

impl TrustedSetup {
    /// Loads the official trusted setup file being used on mainnet from the embedded data folder.
    fn from_embed() -> TrustedSetup {
        Self::from_json_unchecked(TRUSTED_SETUP_JSON)
    }

    pub fn from_json(json: &str) -> TrustedSetup {
        Self::from_json_unchecked(json)
    }

    pub fn from_json_unchecked(json: &str) -> TrustedSetup {
        // Note: it is fine to panic here since this method is called on startup
        // and we want to fail fast if the trusted setup is malformed.
        serde_json::from_str(json)
            .expect("could not parse json string into a TrustedSetup structure")
    }

    pub fn to_commit_key(&self) -> CommitKey {
        let points = deserialize_g1_points_fk20(&self.g1_monomial, SubgroupCheck::Check);
        CommitKey::new(points)
    }

    pub fn to_opening_key(&self) -> OpeningKey {
        let g2_points = deserialize_g2_points_fk20(&self.g2_monomial, SubgroupCheck::Check);
        let num_g2_points = g2_points.len();
        // The setup needs as many g1 elements for the opening key as g2 elements, in order
        // to commit to the remainder/interpolation polynomial.
        let g1_points =
            deserialize_g1_points_fk20(&self.g1_monomial[..num_g2_points], SubgroupCheck::Check);

        OpeningKey::new(g1_points, g2_points)
    }

    pub fn to_kzg_bls12_381(&self, degree: usize) -> KZG<Bls12_381> {
        KZG {
            degree,
            crs_g1: deserialize_g1_points(&self.g1_monomial, SubgroupCheck::Check),
            crs_g2: deserialize_g2_points(&self.g2_monomial, SubgroupCheck::Check),
        }
    }
}

/// Deserialize G1 points from hex strings without checking that the element
/// is in the correct subgroup.
fn deserialize_g1_points_fk20<T: AsRef<str>>(
    g1_points_hex_str: &[T],
    check: SubgroupCheck,
) -> Vec<G1Point> {
    let mut g1_points = Vec::new();
    for g1_hex_str in g1_points_hex_str {
        let g1_hex_str = g1_hex_str.as_ref();

        let g1_hex_str_without_0x = g1_hex_str
            .strip_prefix("0x")
            .expect("expected hex points to be prefixed with `0x`");
        let g1_point_bytes: [u8; 48] = hex::decode(g1_hex_str_without_0x)
            .expect("trusted setup has malformed g1 points")
            .try_into()
            .expect("expected 48 bytes for G1 point");

        let point = match check {
            SubgroupCheck::Check => {
                G1Point::from_compressed(&g1_point_bytes).expect("invalid g1 point")
            },
            SubgroupCheck::NoCheck => {
                G1Point::from_compressed_unchecked(&g1_point_bytes).expect("invalid g1 point")
            },
        };

        g1_points.push(point)
    }

    g1_points
}

/// Deserialize G2 points from hex strings without checking that the element
/// is in the correct subgroup.
fn deserialize_g2_points_fk20<T: AsRef<str>>(
    g2_points_hex_str: &[T],
    subgroup_check: SubgroupCheck,
) -> Vec<G2Point> {
    let mut g2_points = Vec::new();
    for g2_hex_str in g2_points_hex_str {
        let g2_hex_str = g2_hex_str.as_ref();
        let g2_hex_str_without_0x = g2_hex_str
            .strip_prefix("0x")
            .expect("expected hex points to be prefixed with `0x`");
        let g2_point_bytes: [u8; 96] = hex::decode(g2_hex_str_without_0x)
            .expect("trusted setup has malformed g2 points")
            .try_into()
            .expect("expected 96 bytes for G2 point");

        let point = match subgroup_check {
            SubgroupCheck::Check => G2Point::from_compressed(&g2_point_bytes).unwrap(),
            SubgroupCheck::NoCheck => G2Point::from_compressed_unchecked(&g2_point_bytes).unwrap(),
        };
        g2_points.push(point)
    }

    g2_points
}

/// Deserialize G1 points from hex strings without checking that the element
/// is in the correct subgroup.
fn deserialize_g1_points<T: AsRef<str>>(
    g1_points_hex_str: &[T],
    check: SubgroupCheck,
) -> Vec<ark_bls12_381::G1Projective> {
    let mut g1_points = Vec::new();
    for g1_hex_str in g1_points_hex_str {
        let g1_hex_str = g1_hex_str.as_ref();

        let g1_hex_str_without_0x = g1_hex_str
            .strip_prefix("0x")
            .expect("expected hex points to be prefixed with `0x`");
        let g1_point_bytes: [u8; 48] = hex::decode(g1_hex_str_without_0x)
            .expect("trusted setup has malformed g1 points")
            .try_into()
            .expect("expected 48 bytes for G1 point");

        let point = match check {
            SubgroupCheck::Check => {
                ark_bls12_381::G1Projective::deserialize_compressed(&g1_point_bytes[..]).unwrap()
                //::from_compressed(&g1_point_bytes).expect("invalid g1 point")
            },
            SubgroupCheck::NoCheck => {
                ark_bls12_381::G1Projective::deserialize_compressed_unchecked(&g1_point_bytes[..])
                    .expect("invalid g1 point")
            },
        };

        g1_points.push(point)
    }

    g1_points
}

/// Deserialize G2 points from hex strings without checking that the element
/// is in the correct subgroup.
fn deserialize_g2_points<T: AsRef<str>>(
    g2_points_hex_str: &[T],
    subgroup_check: SubgroupCheck,
) -> Vec<ark_bls12_381::G2Projective> {
    let mut g2_points = Vec::new();
    for g2_hex_str in g2_points_hex_str {
        let g2_hex_str = g2_hex_str.as_ref();
        let g2_hex_str_without_0x = g2_hex_str
            .strip_prefix("0x")
            .expect("expected hex points to be prefixed with `0x`");
        let g2_point_bytes: [u8; 96] = hex::decode(g2_hex_str_without_0x)
            .expect("trusted setup has malformed g2 points")
            .try_into()
            .expect("expected 96 bytes for G2 point");

        let point = match subgroup_check {
            SubgroupCheck::Check => {
                ark_bls12_381::G2Projective::deserialize_compressed(&g2_point_bytes[..]).unwrap()
            },
            SubgroupCheck::NoCheck => {
                ark_bls12_381::G2Projective::deserialize_compressed_unchecked(&g2_point_bytes[..])
                    .unwrap()
            },
        };
        g2_points.push(point)
    }

    g2_points
}
