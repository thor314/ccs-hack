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

    // computes z, and the r1cs product, checks if r1cs product reln is satisfied
    pub fn is_satisfied_by(
        &self,
        instance: &R1CSInstance<F>,
        witness: &R1CSWitness<F>,
      ) -> Result<bool, R1CSError> {

        // Enforce A, B, and C have the same dimensions
        if self.A.shape() != self.B.shape() || self.B.shape() != self.C.shape() {
          return Err(R1CSError::new("A, B, and C must have the same dimensions"));
        }
        
        // Enforce n < l
        if self.n < self.l {
          return Err(R1CSError::new("n must be greater than l"));
        }

        // check the shape of z
        if self.n != witness.w.len() + 1 + instance.x.len() {
            return Err(R1CSError::new("n must be equal to the length of w + 1 + the length of x"));
        }
      
        // Compute z = (w, 1, x)
        let one_value = ark_ff::One::one();
        let z = witness
          .w
          .clone()
          .into_iter()
          .chain(std::iter::once(one_value).chain(instance.x.clone().into_iter()))
          .collect::<Array<_, _>>();
      
        // Compute (A * z) * (B * z) - C * z
        let a_product = self.A.dot(&z);
        let b_product = self.B.dot(&z);
        let c_product = self.C.dot(&z);
      
        let lhs: Array<_, _> = a_product.iter().zip(b_product.iter()).map(|(a, b)| *a * *b).collect();
      
        let result = lhs - c_product;
      
        // Check if all entries are zero
        Ok(result.iter().all(|x| x.is_zero()))
      }
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

    // Compute (A * z) * (B * z) - C * z
    let dot = |v: &[F], w: &[F]| v.iter().zip(w.iter()).map(|(vi, wi)| *vi * *wi).sum();
    let Az: Vec<F> = self.A.iter().map(|row| dot(row, &z)).collect();
    let Bz: Vec<F> = self.A.iter().map(|row| dot(row, &z)).collect();
    let Cz: Vec<F> = self.A.iter().map(|row| dot(row, &z)).collect();

    Az.into_iter()
      .zip(Bz.into_iter())
      .zip(Cz.into_iter())
      .map(|((a, b), c)| a * b - c)
      .all(|x| x == F::zero())
    
  pub fn to_ccs(&self) -> CCS<F> {
    let (t, q, d) = (3, 2, 2);
    let multisets = vec![vec![0, 1], vec![2]];
    let constants: Vec<isize> = vec![1, -1];
    let matrices = vec![self.A.clone(), self.B.clone(), self.C.clone()];
    CCS::new(self.n, self.m, self.l, self.N, t, q, d, matrices, multisets, constants)
  }
}

#[cfg(test)]
mod tests {
  use ark_ff::{Fp, Fp2}; // arbitrary convenient quadratic extension field
  use ndarray::{arr1, arr2, ArrayBase}; // For creating 2D arrays
  use num_bigint::ToBigUint;

  use super::*;

  fn _setup(n: usize) -> (Array2<Fr>, Array2<Fr>, Array2<Fr>) {
    let a: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
    let b: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
    let c: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE);

    (a, b, c)
  }

  #[test]
  fn test_r1cs_satisfied() {
    let a: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE + Fr::ONE);
    let b: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE + Fr::ONE);
    let c: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE);

    let x = arr1(&[Fr::ONE + Fr::ONE]);
    let w = arr1(&[Fr::ONE + Fr::ONE]);

    let r1cs = R1CS { m: 1, n: 3, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).unwrap());
    }


  #[test]
  fn test_r1cs_not_satisfied() {
    let a: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE + Fr::ONE);
    let b: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE + Fr::ONE);
    let c: Array2<Fr> = Array2::from_elem((1, 3), Fr::ONE + Fr::ONE + Fr::ONE); // Here, c is different from the product of a and b, so the constraint should not be satisfied

    let x = arr1(&[Fr::ONE + Fr::ONE]);
    let w = arr1(&[Fr::ONE + Fr::ONE]);

    let r1cs = R1CS { m: 1, n: 3, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(!r1cs.is_satisfied_by(&instance, &witness).unwrap());

  }
}
