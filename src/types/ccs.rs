use std::fmt;

use ark_ff::PrimeField;
use ndarray::{Array, Array2};
use num_bigint::BigUint;
use thiserror::Error;

use super::{LArray, Multiset}; // For matrix and vector operations

#[derive(Debug, Error)]
pub enum CCSError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("Default error: {0}")]
  Default(String),
}

use num_traits::{Bounded, Unsigned};

// Defining the CCS structure
// todo: may want to move some of these usize parameters into type gen
#[derive(Debug)]
pub struct CCS<F: PrimeField, const l: usize, const n: usize, const t: usize>
where
  // enforce `n-l>0` at type level
  // ensure that `n-l` can be computed correctly
  [(); n - l]:,
  usize: Unsigned + Bounded,
{
  m: usize,
  // todo: n > l
  N: usize,
  t: usize,
  q: usize,
  d: usize,
  /// matrices $M_0,...,M_{t-1}\in \mathbb F^{m\times n}$ with at most $N=\Omega(\max(m,n))$
  /// non-zero entries in total
  M: [Array2<F>; t],
  /// a sequence of $q$ multisets $[S_0,...,S_{q-1}]$ where an element in each multiset is from the
  /// domain $[t-1]$, with cardinatily of each multiset at most $d$
  S: Multiset<usize>,
  /// a sequence of $q$ constants $[c_0,...,c_{q-1}]$ with $c_i\in \mathbb F$.
  c: Vec<F>,
}

/// A CCS witness consists of a vector $w\in $\mathbb F^{n-l-l}$.
#[derive(Debug)]
pub struct CCSWitness<F: PrimeField, const l: usize> {
  x: LArray<F, l>,
}

// impl<const l: usize> CCS<l> {
//   pub fn is_satisfied_by(&self, instance: &CCSInstance<l>, witness: &CCSWitness<l>) -> bool {
//     todo!();
//     // Implement the checks based on the equation (2) in the definition
//     // This will involve matrix-vector multiplication, Hadamard product, and summing over the
//     // multisets
//   }
// }
// /// A CCS instance consists of public input $x\in \mathbb F^l$.
#[derive(Debug)]
pub struct CCSInstance<F: PrimeField, const l: usize> {
  x: LArray<F, l>,
}

impl<F: PrimeField, const l: usize, const n: usize, const t: usize> CCS<F, l, n, t> where [(); n - l]:, usize: Unsigned + Bounded{
  pub fn is_satisfied_by(
    &self,
    instance: &CCSInstance<F, l>,
    witness: &CCSWitness<F, l>,
  ) -> Result<bool, CCSError> {
    // todo: validate at type level
    if self.M.len() != self.t {
      return Err(CCSError::new("M must have length t"));
    }

    if self.S.len() != self.q {
      return Err(CCSError::new("S must have length q"));
    }

    if self.c.len() != self.q {
      return Err(CCSError::new("c must have length q"));
    }

    // Compute z = (w, 1, x)
    let z = witness
      .w
      .clone()
      .into_iter()
      .chain(std::iter::once(F::one()).chain(instance.x.clone().into_iter()))
      .collect::<Array<_, _>>();

    // Compute sum from i=0 to q-1 of (ci * sum for each j in Si of Mj * z)
    let result = (0..self.q)
      .map(|i| self.c[i] * self.S[i].iter().fold(F::zero(), |sum, &j| sum + self.M[j].dot(&z)))
      .sum::<F>();

    // Check if result is zero
    Ok(result.is_zero())
  }
}

// #[cfg(test)]
// mod tests {
//   use ndarray::arr2;
//   use num_bigint::ToBigUint;

//   use super::*;

//   #[test]
//   fn test_ccs_satisfaction() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m1 = arr2(&[[F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m1];
//     let s = vec![vec![0], vec![1]];
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_ok());
//   }

//   #[test]
//   fn test_ccs_invalid_n_l() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0];
//     let s = vec![vec![0]];
//     let c = vec![F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 2, t: 1, q: 1, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }

//   #[test]
//   fn test_ccs_invalid_m_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0];
//     let s = vec![vec![0]];
//     let c = vec![F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 1, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }

//   #[test]
//   fn test_ccs_invalid_s_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0];
//     let s = vec![vec![0]];
//     let c = vec![F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 1, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }

//   #[test]
//   fn test_ccs_invalid_c_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m0];
//     let s = vec![vec![0], vec![1]];
//     // Modify 'c' length
//     let c = vec![F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]; // c's length is now 1, not 2.
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }
//   #[test]
//   fn test_ccs_invalid_M_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     // Modify 'M' length
//     let m = vec![m0]; // M's length is now 1, not 2.
//     let s = vec![vec![0], vec![1]];
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }

//   #[test]
//   fn test_ccs_invalid_x_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m0];
//     let s = vec![vec![0], vec![1]];
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     // Modify 'x' length
//     let x = Array::from(vec![]); // x's length is now 0, not 1.
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }
//   #[test]
//   fn test_ccs_invalid_w_length() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m0];
//     let s = vec![vec![0], vec![1]];
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     // Modify 'w' length
//     let w = Array::from(vec![]); // w's length is now 0, not 1 (n - l - 1 = 1 - 1 - 1 = -1).
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }
//   #[test]
//   fn test_ccs_invalid_S_cardinality() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m0];
//     // Modify 'S' cardinality
//     let s = vec![vec![0, 0], vec![1, 1]]; // Each multiset in S now has cardinality 2, not 1 (maximum allowed by 'd').
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }
//   #[test]
//   fn test_ccs_invalid_S_domain() {
//     let m0 = arr2(&[[F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
//     let m = vec![m0, m0];
//     // Modify 'S' domain
//     let s = vec![vec![0, 2], vec![1]]; // 2 is not in the domain {0, 1} (t - 1 = 2 - 1 = 1).
//     let c = vec![
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//       F::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
//     ];
//     let x = Array::from(vec![F::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let w = Array::from(vec![F::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
//     let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c };
//     let instance = CCSInstance { x };
//     let witness = CCSWitness { w };

//     assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
//   }
// }

// use super::{finite_field::F, LArray, Multiset};
