use std::fmt;

use anyhow::anyhow;
use ark_bls12_381::Fr;
use ark_ff::{Field, Fp384};
use ndarray::{Array, Array2, Ix1, IxDyn};
use num_bigint::BigUint;
use thiserror::Error;

use super::{ccs::CCS, Matrix}; // For matrix and vector operations // Fr is the prime field for the Bls12_381 curve

// Custom error for operations that are not allowed in R1CS
#[derive(Debug, Error)]
pub enum R1CSError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("Default error: {0}")]
  Default(String),
}

pub struct R1CS<F: Field> {
  /// matrix width
  n: usize,
  /// matrix height
  m: usize,
  l: usize,
  N: usize,
  A: Matrix<F>,
  B: Matrix<F>,
  C: Matrix<F>,
}

pub struct R1CSInstance<F: Field> {
  x: Vec<F>,
}

pub struct R1CSWitness<F: Field> {
  w: Vec<F>,
}

impl<F: Field> R1CS<F> {
  pub fn new(
    n: usize,
    m: usize,
    l: usize,
    N: usize,
    A: Matrix<F>,
    B: Matrix<F>,
    C: Matrix<F>,
  ) -> Self {
    assert!(n > l);
    for matrix in [&A, &B, &C] {
      assert!(matrix.len() == n);
      // lazybug: doesn't check if the matrices are jagged
      assert!(matrix[0].len() == m);
    }
    Self { m, n, l, N, A, B, C }
  }

  /// computes z, and the r1cs product, checks if r1cs product reln is satisfied
  pub fn is_satisfied_by(&self, instance: &R1CSInstance<F>, witness: &R1CSWitness<F>) -> bool {
    // Compute z = (w, 1, x)
    let z: Vec<F> = witness
      .w
      .clone()
      .into_iter()
      .chain(std::iter::once(F::one()).chain(instance.x.clone().into_iter()))
      .collect();

    // convenience; todo; move to utils
    let dot = |v: &[F], w: &[F]| v.iter().zip(w.iter()).map(|(vi, wi)| *vi * *wi).sum();
    let matrix_vector_prod =
      |matrix: &Vec<Vec<F>>, vector: &Vec<F>| matrix.iter().map(|row| dot(row, &z)).collect();

    let Az: Vec<F> = matrix_vector_prod(&self.A, &z);
    let Bz: Vec<F> = matrix_vector_prod(&self.B, &z);
    let Cz: Vec<F> = matrix_vector_prod(&self.C, &z);

    Az.into_iter()
      .zip(Bz.into_iter())
      .zip(Cz.into_iter())
      .map(|((a, b), c)| a * b - c)
      .all(|x| x == F::zero())
  }

  pub fn to_ccs(&self) -> CCS<F> {
    let (t, q, d) = (3, 2, 2);
    let multisets = vec![vec![0, 1], vec![2]];
    // hack: I'm not sure what -1 is supposed to be
    let constants: Vec<F> = vec![F::one(), -F::one()];
    let matrices = vec![self.A.clone(), self.B.clone(), self.C.clone()];
    CCS::new(self.n, self.m, self.l, self.N, t, q, d, matrices, multisets, constants)
  }
}
