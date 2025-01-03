// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

//! This module provides the poly-commit

pub mod utils;

use crate::poly_commit::kzg::utils::{div, evaluate, interpolate, mul};
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use std::ops::Mul;

#[derive(Debug)]
pub struct KZG<E: Pairing> {
    pub degree: usize,
    pub crs_g1: Vec<E::G1>,
    pub crs_g2: Vec<E::G2>,
}

impl<E: Pairing> KZG<E> {
    pub fn new(g1: E::G1, g2: E::G2, degree: usize, secret: E::ScalarField) -> Self {
        let mut crs_g1 = vec![];
        let mut crs_g2 = vec![];
        for i in 0..degree + 1 {
            crs_g1.push(g1.mul(secret.pow([i as u64])));
            crs_g2.push(g2.mul(secret.pow([i as u64])));
        }

        Self {
            degree,
            crs_g1,
            crs_g2,
        }
    }

    pub fn commit(&self, poly: &[E::ScalarField]) -> E::G1 {
        let mut commitment = self.crs_g1.first().unwrap().mul(E::ScalarField::ZERO);
        for (i, p) in poly.iter().enumerate().take(self.degree + 1) {
            commitment += self.crs_g1[i] * p;
        }
        commitment
    }

    pub fn open(&self, poly: &[E::ScalarField], point: E::ScalarField) -> E::G1 {
        // evaluate the polynomial at point
        let value = evaluate(poly, point);

        // initialize denominator
        let denominator = [-point, E::ScalarField::ONE];

        // initialize numerator
        let first = poly[0] - value;
        let rest = &poly[1..];
        let temp: Vec<E::ScalarField> =
            std::iter::once(first).chain(rest.iter().cloned()).collect();
        let numerator: &[E::ScalarField] = &temp;

        // get quotient by dividing numerator by denominator
        let quotient = div(numerator, &denominator).unwrap();

        // calculate pi as proof (quotient multiplied by CRS)
        let mut pi = self.crs_g1.first().unwrap().mul(E::ScalarField::ZERO);
        for (i, &p) in quotient.iter().enumerate() {
            pi += self.crs_g1[i] * p;
        }

        // return pi
        pi
    }

    pub fn multi_open(&self, poly: &[E::ScalarField], points: &[E::ScalarField]) -> E::G1 {
        // denominator is a polynomial where all its root are points to be evaluated (zero poly)
        let mut zero_poly = vec![-points[0], E::ScalarField::ONE];
        for &point in points.iter().skip(1) {
            zero_poly = mul(&zero_poly, &[-point, E::ScalarField::ONE]);
        }

        // perform Lagrange interpolation on points
        let mut values = vec![];
        for point in points {
            values.push(evaluate(poly, *point));
        }
        let mut lagrange_poly = interpolate(points, &values).unwrap();
        lagrange_poly.resize(poly.len(), E::ScalarField::ZERO); // pad with zeros

        // numerator is the difference between the polynomial and the Lagrange interpolation
        let mut numerator = Vec::with_capacity(poly.len());
        for (coeff1, coeff2) in poly.iter().zip(lagrange_poly.as_slice()) {
            numerator.push(*coeff1 - coeff2);
        }

        // get quotient by dividing numerator by denominator
        let quotient = div(&numerator, &zero_poly).unwrap();

        // calculate pi as proof (quotient multiplied by CRS)
        let mut pi = self.crs_g1.first().unwrap().mul(E::ScalarField::ZERO);
        for (i, p) in quotient.iter().enumerate() {
            pi += self.crs_g1[i] * p;
        }

        // return pi
        pi
    }

    pub fn verify(
        &self,
        point: E::ScalarField,
        value: E::ScalarField,
        commitment: E::G1,
        pi: E::G1,
    ) -> bool {
        let lhs = E::pairing(
            pi,
            *self.crs_g2.get(1).unwrap() - self.crs_g2.first().unwrap().mul(point),
        );
        let rhs = E::pairing(
            commitment - self.crs_g1.first().unwrap().mul(value),
            self.crs_g2.first().unwrap(),
        );
        lhs == rhs
    }

    pub fn verify_multi(
        &self,
        points: &[E::ScalarField],
        values: &[E::ScalarField],
        commitment: E::G1,
        pi: E::G1,
    ) -> bool {
        // compute the zero polynomial
        let mut zero_poly = vec![-points[0], E::ScalarField::ONE];
        for &point in points.iter().skip(1) {
            zero_poly = mul(&zero_poly, &[-point, E::ScalarField::ONE]);
        }

        // compute commitment of zero polynomial in regards to crs_g2
        let mut zero_commitment = self.crs_g2.first().unwrap().mul(E::ScalarField::ZERO);
        for (i, &p) in zero_poly.iter().enumerate() {
            zero_commitment += self.crs_g2[i] * p;
        }

        // compute lagrange polynomial
        let lagrange_poly = interpolate(points, values).unwrap();

        // compute commitment of lagrange polynomial in regards to crs_g1
        let mut lagrange_commitment = self.crs_g1.first().unwrap().mul(E::ScalarField::ZERO);
        for (i, p) in lagrange_poly.iter().enumerate() {
            lagrange_commitment += self.crs_g1[i] * p;
        }

        let lhs = E::pairing(pi, zero_commitment);
        let rhs = E::pairing(
            commitment - lagrange_commitment,
            self.crs_g2.first().unwrap(),
        );
        lhs == rhs
    }
}
