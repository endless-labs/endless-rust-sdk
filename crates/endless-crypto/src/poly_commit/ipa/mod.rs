// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

//! This module provides the poly-commit
//!

use crate::poly_commit::ipa::{
    banderwagon::Element,
    ipa_multipoint::{
        crs::CRS,
        lagrange_basis::{LagrangeBasis, PrecomputedWeights},
        multiproof::{MultiPoint, MultiPointProof, ProverQuery, VerifierQuery},
        transcript::Transcript,
    },
};
use ark_ed_on_bls12_381_bandersnatch::Fr;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub mod banderwagon;
pub mod ipa_multipoint;

// 4096
pub const MAX_COMMIT_DATA_LENGTH_2_POW: usize = 12;

const PEDERSEN_SEED: &[u8] = b"endless_file_storage_202411";

const TRANSCRIPT_TAG: &[u8] = b"pofs";

pub static CRS: Lazy<crate::poly_commit::ipa::ipa_multipoint::crs::CRS> = Lazy::new(|| {
    CRS::new(
        num_traits::pow(2, MAX_COMMIT_DATA_LENGTH_2_POW),
        PEDERSEN_SEED,
    )
});

pub static PRECOMPUTED_WEIGHTS_MAP: Lazy<HashMap<usize, PrecomputedWeights>> = Lazy::new(|| {
    let mut ws = HashMap::new();
    for i in 0..(MAX_COMMIT_DATA_LENGTH_2_POW + 1) {
        let key = num_traits::pow(2, i);
        let w = PrecomputedWeights::new(key);
        ws.insert(key, w);
    }
    ws
});

pub fn generate_commitment(data: Vec<Fr>) -> Element {
    let domain_len = data.len();
    let poly = LagrangeBasis::new(data);

    CRS.get_first_n_elements(domain_len)
        .commit_lagrange_poly(&poly)
}

pub fn generate_proof(
    commitment: Element,
    datas: Vec<Fr>,
    points: Vec<u64>,
) -> (MultiPointProof, Vec<VerifierQuery>) {
    let mut prover_querys = vec![];
    let mut verifier_querys = vec![];

    for p in points {
        let y_i = datas.get(p as usize).unwrap();
        let prover_query = ProverQuery {
            commitment,
            poly: LagrangeBasis::new(datas.to_vec()),
            point: p as usize,
            result: *y_i,
        };

        prover_querys.push(prover_query.clone());
        verifier_querys.push(prover_query.into());
    }

    let mut transcript = Transcript::new(TRANSCRIPT_TAG);
    let proof = MultiPoint::open(
        &CRS.get_first_n_elements(datas.len()),
        PRECOMPUTED_WEIGHTS_MAP.get(&datas.len()).unwrap(),
        &mut transcript,
        prover_querys,
    );

    (proof, verifier_querys)
}

pub fn verify_proof(
    multiproof: MultiPointProof,
    verifier_querys: Vec<VerifierQuery>,
    domain_size: usize,
) -> bool {
    let mut transcript = Transcript::new(TRANSCRIPT_TAG);
    multiproof.check(
        &CRS.get_first_n_elements(domain_size),
        PRECOMPUTED_WEIGHTS_MAP.get(&domain_size).unwrap(),
        &verifier_querys,
        &mut transcript,
    )
}
