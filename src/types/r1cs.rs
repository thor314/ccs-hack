use std::fmt;

use ark_ff::PrimeField;
use ndarray::{Array, Array2, IxDyn};
use num_bigint::BigUint; // For matrix and vector operations

// use crate::types::finite_field::FiniteField;

// Custom error for operations that are not allowed in R1CS
// todo: enum
#[derive(Debug)]
pub struct R1CSError {
  details: String,
}

impl R1CSError {
  fn new(msg: &str) -> R1CSError { R1CSError { details: msg.to_string() } }
}

impl fmt::Display for R1CSError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.details) }
}

pub struct R1CS<F: PrimeField> {
  m: usize,
  n: usize,
  N: usize,
  l: usize,
  A: Array2<F>,
  B: Array2<F>,
  C: Array2<F>,
}

pub struct R1CSInstance<F: PrimeField> {
  x: Array<F, IxDyn>,
}

pub struct R1CSWitness<F: PrimeField> {
  w: Array<F, IxDyn>,
}

impl<F: PrimeField> R1CS<F> {
  pub fn is_satisfied_by(
    &self,
    instance: &R1CSInstance<F>,
    witness: &R1CSWitness<F>,
  ) -> Result<bool, R1CSError> {
    if self.A.shape() != self.B.shape() || self.B.shape() != self.C.shape() {
      return Err(R1CSError::new("A, B, and C must have the same dimensions"));
    }

    if self.n < self.l {
      return Err(R1CSError::new("n must be greater than l"));
    }

    // Compute z = (w, 1, x)
    let one_value = F::new(BigUint::one(), self.A[[0, 0]].p.clone());
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

    let lhs: Array<_, _> = a_product.iter().zip(b_product.iter()).map(|(a, b)| a * b).collect();

    let result = lhs - c_product;

    // Check if all entries are zero
    Ok(result.iter().all(|x| x.is_zero()))
  }
}

#[cfg(test)]
mod tests {
  use ark_ff::Fp2; // arbitrary convenient quadratic extension field
  use ndarray::arr2; // For creating 2D arrays
  use num_bigint::ToBigUint;

  use super::*;

  #[test]
  fn test_r1cs_satisfied() {
    let a = arr2(&[[Fp2::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).unwrap());
  }

  #[test]
  fn test_r1cs_not_satisfied() {
    let a = arr2(&[[Fp2::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[Fp2::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(!r1cs.is_satisfied_by(&instance, &witness).unwrap());
  }

  #[test]
  fn test_r1cs_invalid_dimensions() {
    let a = arr2(&[[Fp2::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[
      Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap()),
      Fp2::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap()),
    ]]);
    let c = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
  }

  #[test]
  fn test_r1cs_invalid_n_l() {
    let a = arr2(&[[Fp2::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![Fp2::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 2, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
  }
}
