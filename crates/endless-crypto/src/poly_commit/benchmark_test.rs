use crate::poly_commit::{
    self,
    ipa::{generate_commitment, generate_proof, verify_proof},
    kzg::KZG,
    util::trusted_setup::TrustedSetup,
};
use ark_bls12_381::Bls12_381;
use ark_std::{self, end_timer, iterable::Iterable, start_timer, UniformRand};
use crate_crypto_internal_eth_kzg_bls12_381::Scalar;

#[test]
fn test_ipa_kzg() {
    for i in [5, 12] {
        let data_size = 1 << i;
        println!("====================================Start verifying proof data size {data_size}");
        //test_ipa(data_size);

        //test_ipa(data_size);
        //test_kzg((data_size / 2) as usize);
        multi_opening_fk20((data_size / 2) as usize);
    }
}

#[allow(dead_code)]
fn test_ipa(degree: u32) {
    let sig_nums = vec![4];

    for sig_nums in sig_nums {
        let datas = (0..degree)
            .map(ark_ed_on_bls12_381_bandersnatch::Fr::from)
            .collect::<Vec<ark_ed_on_bls12_381_bandersnatch::Fr>>();

        let commitment = generate_commitment(datas.clone());
        //println!("commitment: {:?}",  hex::encode(&commitment.to_bytes()));

        let prove_time = start_timer!(|| format!(
            "IPA Open Proof {} signature at {} degree ",
            sig_nums,
            degree / 4
        ));
        let points = (0..sig_nums * 4).collect();
        let (multiproof, verifier_querys) = generate_proof(commitment, datas.clone(), points);
        end_timer!(prove_time);

        //println!("multiproof.open_proof: L_vec {:?} L_vec {:?}", multiproof.open_proof.L_vec.len(), multiproof.open_proof.R_vec.len());
        //println!("multiproof.open_proof: {:?}\n", multiproof.open_proof);

        let check_time = start_timer!(|| format!(
            "IPA Verify {} signature at {} degree ",
            sig_nums,
            degree / 4
        ));
        assert!(verify_proof(multiproof, verifier_querys, degree as usize));
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

#[allow(dead_code)]
pub fn test_single_evaluation(
    kzg_instance: &KZG<Bls12_381>,
    poly: &[ark_bls12_381::Fr],
    commitment: ark_bls12_381::G1Projective,
) {
    let mut rng = ark_std::test_rng();

    // generate a random point and open the polynomial at that point
    let point = ark_bls12_381::Fr::rand(&mut rng);
    let pi = kzg_instance.open(poly, point);

    // verify the proof
    let value = poly_commit::kzg::utils::evaluate(poly, point);

    let verify_timer = start_timer!(|| format!(
        "KZG Signle Verify 1 point at {} degree",
        kzg_instance.degree / 2
    ));
    assert!(kzg_instance.verify(point, value, commitment, pi));
    end_timer!(verify_timer);
}

#[allow(dead_code)]
pub fn test_multi_evaluation(
    kzg_instance: &KZG<Bls12_381>,
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
        values.push(poly_commit::kzg::utils::evaluate(poly, *point));
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

#[allow(dead_code)]
pub fn multi_opening_fk20(degree: usize) {
    let trust_setup = TrustedSetup::default();
    let (ck, opening_key) = (trust_setup.to_commit_key(), trust_setup.to_opening_key());

    let input_usize = [3, 9, 11, 12];

    let polynomial: Vec<_> = (0..degree)
        .map(|_| -Scalar::from(rand::random::<u64>()))
        .collect();
    let values: Vec<_> = input_usize
        .iter()
        .map(|p| polynomial.get(p).unwrap())
        .collect();
    println!("values {:?}", values);

    let input_points: Vec<_> = input_usize.iter().map(|i| Scalar::from(i as u64)).collect();

    let commitment = ck.commit_g1(&polynomial).into();

    let open_tiemer = start_timer!(|| "Open");
    let (quotient_proof, output_points) =
        poly_commit::fk20::prove::compute_multi_opening(&ck, &polynomial, &input_points);
    end_timer!(open_tiemer);

    println!("output_points {:?}", output_points);

    let verify_tiemer = start_timer!(|| "Verify");
    let proof_valid = poly_commit::fk20::prove::verify_multi_opening(
        quotient_proof,
        &opening_key,
        commitment,
        &input_points,
        &output_points,
    );
    end_timer!(verify_tiemer);
    assert!(proof_valid);
}
