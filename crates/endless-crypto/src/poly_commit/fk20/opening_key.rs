use crate_crypto_internal_eth_kzg_bls12_381::{
    lincomb::{g1_lincomb, g2_lincomb},
    G1Point, G1Projective, G2Point, G2Projective, Scalar,
};

/// Opening Key is used to verify opening proofs made about a committed polynomial.
#[derive(Clone, Debug)]
pub struct OpeningKey {
    /// The powers of tau G1 used in the setup
    ///
    /// ie group elements of the form `{ \tau^i G }`
    pub g1s: Vec<G1Point>,
    /// The powers of tau G2 used in the setup
    ///
    /// ie group elements of the form `{ \tau^i G }`
    pub g2s: Vec<G2Point>,
    /// The degree-0 term in the powers of tau G2 elements.
    pub g2_gen: G2Point,
}

impl OpeningKey {
    pub fn new(g1s: Vec<G1Point>, g2s: Vec<G2Point>) -> Self {
        // This assumes that the trusted setup contains more than 1 element.
        //
        // For all of our purposes and for any useful applications, this will be the case.
        let g2_gen = g2s[0];

        Self { g1s, g2s, g2_gen }
    }

    /// Commit to a polynomial in monomial form using the G2 group elements
    pub fn commit_g2(&self, polynomial: &[Scalar]) -> G2Projective {
        assert!(self.g2s.len() >= polynomial.len());
        g2_lincomb(&self.g2s[0..polynomial.len()], polynomial)
            .expect("number of g1 points is equal to the number of coefficients in the polynomial")
    }

    /// Commit to a polynomial in monomial form using the G1 group elements
    pub fn commit_g1(&self, polynomial: &[Scalar]) -> G1Projective {
        assert!(self.g1s.len() >= polynomial.len());
        g1_lincomb(&self.g1s[0..polynomial.len()], polynomial)
            .expect("number of g1 points is equal to the number of coefficients in the polynomial")
    }

    /// Returns the degree-0 element in the G2 powers of tau list
    pub fn g2_gen(&self) -> G2Point {
        self.g2_gen
    }
}
