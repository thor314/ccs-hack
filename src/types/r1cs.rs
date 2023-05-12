use std::{fmt, marker::PhantomData};

use anyhow::{anyhow, Result};
use ndarray::{Array, Array2}; // For matrix and vector operations

use super::{finite_field::FiniteField, LArray};

// todo: validate n > l
pub struct R1CS<const l: usize, const n: usize> {
  m: usize,
  N: usize,
  // todo: SparseMatrix, not Array2
  A: Array2<FiniteField>,
  B: Array2<FiniteField>,
  C: Array2<FiniteField>,
}

pub struct R1CSWitness<const l: usize, const n: usize> {
  // todo; actual size n-l-1, fix type check
  w: LArray<n>,
}

pub struct R1CSInstance<const l: usize> {
  // Public input
  x: LArray<l>,
}

impl<const l: usize, const n: usize> R1CS<l, n> {
  pub fn is_satisfied_by(
    &self,
    instance: &R1CSInstance<l>,
    witness: &R1CSWitness<l, n>,
  ) -> Result<bool> {
    // Compute z = (w, 1, x)
    let z = witness
      .w
      .clone()
      .into_iter()
      .chain(
        std::iter::once(FiniteField::one(&self.A[[0, 0]].p)).chain(instance.x.clone().into_iter()),
      )
      .collect::<Array<_, _>>();

    // Compute (A * z) * (B * z) - C * z
    let lhs = self
      .A
      .dot(&z)
      .into_iter()
      .zip(self.B.dot(&z).into_iter())
      .map(|(a, b)| a * b)
      .collect::<Array<_, _>>();
    let rhs = self.C.dot(&z);
    let result = lhs - rhs;

    // Check if all entries are zero
    Ok(result.iter().all(|x| x.is_zero()))
  }
}

#[cfg(test)]
mod tests {
  use ndarray::arr2; // For creating 2D arrays
  use num_bigint::ToBigUint;

  use super::*;

  #[test]
  fn test_r1cs_satisfied() {
    let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).unwrap());
  }

  #[test]
  fn test_r1cs_not_satisfied() {
    let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(!r1cs.is_satisfied_by(&instance, &witness).unwrap());
  }

  #[test]
  fn test_r1cs_invalid_dimensions() {
    let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[
      FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap()),
      FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap()),
    ]]);
    let c = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
  }

  #[test]
  fn test_r1cs_invalid_n_l() {
    let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let b = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let c = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
    let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
    let r1cs = R1CS { m: 1, n: 1, N: 1, l: 2, A: a, B: b, C: c };
    let instance = R1CSInstance { x };
    let witness = R1CSWitness { w };

    assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
  }
}
