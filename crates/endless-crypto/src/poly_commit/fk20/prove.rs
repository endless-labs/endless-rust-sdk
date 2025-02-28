/// This modules contains code to create and verify opening proofs in a naive way.
/// It is also general, meaning the points we are creating opening proofs
/// for, do not need to have any special structure.
///
/// This generalized scheme can be seen in [BDFG21](https://eprint.iacr.org/2020/081.pdf)
///
/// This is in contrast to the scheme we will use in practice which dictates that the
/// points we open at, must be (cosets) of the roots of unity. This scheme is called FK20 and is orders
/// of magnitudes faster than the naive scheme.
///
/// We will use the naive scheme for testing purposes.
use crate::poly_commit::fk20::{commit_key::CommitKey, opening_key::OpeningKey};
use crate_crypto_internal_eth_kzg_bls12_381::{
    multi_pairings, G1Point, G1Projective, G2Point, G2Prepared, Scalar,
};
use crate_crypto_internal_eth_kzg_polynomial::monomial::{
    lagrange_interpolate, poly_eval, poly_sub, vanishing_poly, PolyCoeff,
};

/// Naively computes an opening proof that attests to the evaluation of
/// `polynomial` at `input_points`.
//
// Note: This method returns both the proof and the output points.
// This does not follow the convention of the other methods which
// produce proofs.
//
// This is done intentionally since that method
// has additional checks that require the evaluations and computing
// the output points, the naive way is quite expensive.
pub fn compute_multi_opening(
    commit_key: &CommitKey,
    polynomial: &PolyCoeff,
    input_points: &[Scalar],
) -> (G1Point, Vec<Scalar>) {
    _compute_multi_opening_naive(commit_key, polynomial, input_points)
}

/// Naively Verifies a multi-point opening proof.
pub fn verify_multi_opening(
    quotient_commitment: G1Point,
    opening_key: &OpeningKey,
    commitment: G1Point,
    input_points: &[Scalar],
    output_points: &[Scalar],
) -> bool {
    _verify_multi_opening_naive(
        opening_key,
        commitment,
        quotient_commitment,
        input_points,
        output_points,
    )
}

/// Computes a multi-point opening proof using the general formula.
///
/// This is done by committing to the following quotient polynomial:
///     Q(X) = f(X) - I(X) / Z(X)
/// Where:
///     - I(X) is the degree `k-1` polynomial that agrees with f(x) at all `k` points
///     - Z(X) is the degree `k` polynomial that evaluates to zero on all `k` points
///
/// We further note that since the degree of I(X) is less than the degree of Z(X),
/// the computation can be simplified in monomial form to Q(X) = f(X) / Z(X)
fn _compute_multi_opening_naive(
    commit_key: &CommitKey,
    polynomial: &PolyCoeff,
    points: &[Scalar],
) -> (G1Point, Vec<Scalar>) {
    // Divides `self` by x-z using Ruffinis rule
    fn divide_by_linear(poly: &[Scalar], z: Scalar) -> Vec<Scalar> {
        let mut quotient: Vec<Scalar> = Vec::with_capacity(poly.len());
        let mut k = Scalar::from(0u64);

        for coeff in poly.iter().rev() {
            let t = *coeff + k;
            quotient.push(t);
            k = z * t;
        }

        // Pop off the remainder term
        quotient.pop();

        // Reverse the results as monomial form stores coefficients starting with lowest degree
        quotient.reverse();
        quotient
    }

    let mut evaluations = Vec::new();
    for point in points {
        let evaluation = poly_eval(polynomial, point);
        evaluations.push(evaluation);
    }

    // Compute f(x) - I(x) / \prod (x - z_i)
    // Where I(x) is the polynomial such that r(z_i) = f(z_i) for all z_i
    //
    // We can speed up computation of I(x) by doing an IFFT, given the coset generator, since
    // we know all of the points are of the form k * \omega where \omega is a root of unity
    // and `k` is what is known as a coset generator.

    let coordinates: Vec<_> = points
        .iter()
        .zip(evaluations.iter())
        .map(|(p, e)| (*p, *e))
        .collect();
    let r_x = lagrange_interpolate(&coordinates).expect("lagrange interpolation failed");

    // Check that the r_x polynomial is correct, it should essentially be the polynomial that
    // evaluates to f(z_i) = r(z_i)
    for (point, evaluation) in points.iter().zip(evaluations.iter()) {
        assert_eq!(poly_eval(&r_x, point), *evaluation);
    }

    let poly_shifted = poly_sub(polynomial.to_vec().clone(), r_x.clone());

    let mut quotient_poly = poly_shifted.to_vec().clone();
    for point in points.iter() {
        quotient_poly = divide_by_linear(&quotient_poly, *point);
    }

    (commit_key.commit_g1(&quotient_poly).into(), evaluations)
}

/// Verifies a multi-opening proof using the general formula.
///
/// This is done by checking if the following equation holds:
///     Q(x) Z(x) = f(X) - I(X)
/// Where:
///     f(X) is the polynomial that we want to verify opens at `k` points to `k` values
///     Q(X) is the quotient polynomial computed by the prover
///     I(X) is the degree k-1 polynomial that evaluates to `ys` at all `zs`` points
///     Z(X) is the polynomial that evaluates to zero on all `k` points
///
/// The verifier receives the commitments to Q(X) and f(X), so they check the equation
/// holds by using the following pairing equation:
///     e([Q(X)]_1, [Z(X)]_2) == e([f(X)]_1 - [I(X)]_1, [1]_2)
fn _verify_multi_opening_naive(
    opening_key: &OpeningKey,
    commitment: G1Point,
    proof: G1Point,
    input_points: &[Scalar],
    output_points: &[Scalar],
) -> bool {
    let coordinates: Vec<_> = input_points
        .iter()
        .zip(output_points.iter())
        .map(|(p, e)| (*p, *e))
        .collect();
    let r_x = lagrange_interpolate(&coordinates).unwrap();

    let vanishing_poly = vanishing_poly(input_points);
    let comm_vanishing_poly: G2Point = opening_key.commit_g2(&vanishing_poly).into();

    let comm_r_x = opening_key.commit_g1(&r_x);
    let comm_minus_r_x: G1Point = (G1Projective::from(commitment) - comm_r_x).into();
    multi_pairings(&[
        (&proof, &G2Prepared::from(comm_vanishing_poly)),
        (&comm_minus_r_x, &G2Prepared::from(-opening_key.g2_gen())),
    ])
}

#[cfg(test)]
mod tests {
    use crate::poly_commit::util::trusted_setup::TrustedSetup;
    use ark_std::iterable::Iterable;
    use crate_crypto_internal_eth_kzg_bls12_381::Scalar;

    #[test]
    fn smoke_test_naive_multi_opening() {
        let trust_setup = TrustedSetup::default();
        let (ck, opening_key) = (trust_setup.to_commit_key(), trust_setup.to_opening_key());

        let input_points: Vec<_> = [3, 9, 33, 999].iter().map(Scalar::from).collect();

        let polynomial: Vec<_> = (0..2048)
            .map(|_| -Scalar::from(rand::random::<u64>()))
            .collect();
        let commitment = ck.commit_g1(&polynomial).into();

        let open_tiemer = ark_std::start_timer!(|| "Open");
        let (quotient_commitment, output_points) =
            super::compute_multi_opening(&ck, &polynomial, &input_points);
        ark_std::end_timer!(open_tiemer);

        let verify_tiemer = ark_std::start_timer!(|| "Verify");
        let proof_valid = super::verify_multi_opening(
            quotient_commitment,
            &opening_key,
            commitment,
            &input_points,
            &output_points,
        );
        ark_std::end_timer!(verify_tiemer);
        assert!(proof_valid);
    }
}
