// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate criterion;

use ark_bls12_381::Bls12_381;
use ark_std::{end_timer, iterable::Iterable, start_timer, UniformRand};
use crate_crypto_internal_eth_kzg_bls12_381::Scalar;
use criterion::{measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion};
use endless_crypto::poly_commit::{fk20, ipa, kzg, util::trusted_setup::TrustedSetup};
use itertools::Itertools;

fn test_ipa<M: Measurement>(g: &mut BenchmarkGroup<M>, degree: usize) {
    let sig_nums = vec![4];

    for sig_nums in sig_nums {
        let datas = (0..degree)
            .map(|i| ark_ed_on_bls12_381_bandersnatch::Fr::from(i as u64))
            .collect::<Vec<ark_ed_on_bls12_381_bandersnatch::Fr>>();

        let commitment = ipa::generate_commitment(datas.clone());
        //println!("commitment: {:?}",  hex::encode(&commitment.to_bytes()));

        let prove_time = start_timer!(|| format!(
            "IPA Open Proof {} signature at {} degree ",
            sig_nums,
            degree / 4
        ));

        g.bench_function(BenchmarkId::new("ipa::prove", degree), |b| {
            b.iter(|| {
                let points = (0..sig_nums * 4).collect();
                ipa::generate_proof(commitment, datas.clone(), points);
            })
        });

        let points = (0..sig_nums * 4).collect();
        let (multiproof, verifier_querys) = ipa::generate_proof(commitment, datas.clone(), points);
        end_timer!(prove_time);

        //println!("multiproof.open_proof: L_vec {:?} L_vec {:?}", multiproof.open_proof.L_vec.len(), multiproof.open_proof.R_vec.len());
        //println!("multiproof.open_proof: {:?}\n", multiproof.open_proof);

        let check_time = start_timer!(|| format!(
            "IPA Verify {} signature at {} degree ",
            sig_nums,
            degree / 4
        ));

        g.bench_function(BenchmarkId::new("ipa::verify", degree), |b| {
            b.iter(|| {
                let query_clone = verifier_querys.iter().cloned().collect_vec();

                assert!(ipa::verify_proof(multiproof.clone(), query_clone, degree));
            })
        });

        assert!(ipa::verify_proof(multiproof, verifier_querys, degree));
        end_timer!(check_time);
    }
}

#[allow(dead_code)]
fn test_kzg(degree: usize) {
    // initialize kzg instance
    let mut rng = ark_std::test_rng();
    /*    // trusted setup ceremony
        let secret = ark_bls12_381::Fr::rand(&mut rng);

        let mut kzg_instance = KZG::<Bls12_381>::new(G1::rand(&mut rng), G2::rand(&mut rng), degree, secret);

        for g1 in kzg_instance.crs_g1 {
        let mut bytes = [0u8; 48];
            g1.serialize_compressed(&mut bytes[..]).expect("TODO: panic message");
            println!("0x{:x?}", hex::encode(bytes));
        };

        for g2 in &kzg_instance.crs_g2[..=64] {
            let mut bytes = [0u8; 96];
            g2.serialize_compressed(&mut bytes[..]).expect("TODO: panic message");
            println!("0x{:x?}", hex::encode(bytes));
        };
    */

    let kzg_instance = TrustedSetup::default().to_kzg_bls12_381(degree);

    //println!("g1 len {}, g2 len {}", kzg_instance.crs_g1.len(), kzg_instance.crs_g2.len());

    //println!("Setting up KZG {:?}", kzg_instance);

    // generate a random polynomial and commit it
    let poly = vec![ark_bls12_381::Fr::rand(&mut rng); degree + 1];
    let commitment = kzg_instance.commit(&poly);

    // test single point evaluation
    //test_single_evaluation(&kzg_instance, &poly, commitment);

    // test multi point evaluation
    test_multi_evaluation(&kzg_instance, &poly, commitment);
}

pub fn test_single_evaluation(
    kzg_instance: &kzg::KZG<Bls12_381>,
    poly: &[ark_bls12_381::Fr],
    commitment: ark_bls12_381::G1Projective,
) {
    let mut rng = ark_std::test_rng();

    // generate a random point and open the polynomial at that point
    let point = ark_bls12_381::Fr::rand(&mut rng);
    let pi = kzg_instance.open(poly, point);

    // verify the proof
    let value = kzg::utils::evaluate(poly, point);

    let verify_timer = start_timer!(|| format!(
        "KZG Signle Verify 1 point at {} degree",
        kzg_instance.degree / 2
    ));
    assert!(kzg_instance.verify(point, value, commitment, pi));
    end_timer!(verify_timer);
}

pub fn test_multi_evaluation(
    kzg_instance: &kzg::KZG<Bls12_381>,
    poly: &[ark_bls12_381::Fr],
    commitment: ark_bls12_381::G1Projective,
) {
    let mut rng = ark_std::test_rng();

    // generate three random points and open the polynomial at those points
    let points: Vec<ark_bls12_381::Fr> =
        (0..4).map(|_| ark_bls12_381::Fr::rand(&mut rng)).collect();

    let open_timer = start_timer!(|| format!(
        "KZG Multi Open {} signature at {} degree",
        points.len() / 2,
        kzg_instance.degree / 2
    ));
    let pi = kzg_instance.multi_open(poly, &points);
    end_timer!(open_timer);

    // evaluate the polynomial at those points
    let mut values = vec![];
    for point in &points {
        values.push(kzg::utils::evaluate(poly, *point));
    }

    // verify the proof
    let verify_timer = start_timer!(|| format!(
        "KZG Multi Verify {} signature at {} degree",
        points.len() / 2,
        kzg_instance.degree / 2
    ));
    assert!(kzg_instance.verify_multi(&points, &values, commitment, pi));
    end_timer!(verify_timer);
}

pub fn test_fk20<M: Measurement>(g: &mut BenchmarkGroup<M>, degree: usize) {
    let trust_setup = TrustedSetup::default();
    let (ck, opening_key) = (trust_setup.to_commit_key(), trust_setup.to_opening_key());

    let input_usize = [3, 9, 11, 12];

    let polynomial: Vec<_> = (0..degree)
        .map(|_| -Scalar::from(rand::random::<u64>()))
        .collect();

    let input_points: Vec<_> = input_usize.iter().map(|i| Scalar::from(i as u64)).collect();

    let commitment = ck.commit_g1(&polynomial).into();

    let open_tiemer = start_timer!(|| "Open");

    g.bench_function(BenchmarkId::new("fk20::prove", degree), |b| {
        b.iter(|| {
            fk20::prove::compute_multi_opening(&ck, &polynomial, &input_points);
        })
    });

    let (quotient_proof, output_points) =
        fk20::prove::compute_multi_opening(&ck, &polynomial, &input_points);
    end_timer!(open_tiemer);

    let verify_tiemer = start_timer!(|| "Verify");

    g.bench_function(BenchmarkId::new("fk20::verify", degree), |b| {
        b.iter(|| {
            fk20::prove::verify_multi_opening(
                quotient_proof,
                &opening_key,
                commitment,
                &input_points,
                &output_points,
            );
        })
    });

    let proof_valid = fk20::prove::verify_multi_opening(
        quotient_proof,
        &opening_key,
        commitment,
        &input_points,
        &output_points,
    );
    end_timer!(verify_tiemer);
    assert!(proof_valid);
}

/// Runs all the benchmarks.
fn bench_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("poly_commit");

    group.sample_size(50);

    for n in [16, 256, 4096] {
        test_ipa(&mut group, n as usize);
        test_fk20(&mut group, n as usize);
    }

    group.finish();
}

criterion_group!(
    name = poly_commit;
    config = Criterion::default(); //.measurement_time(Duration::from_secs(100));
    targets = bench_group
);

criterion_main!(poly_commit);
