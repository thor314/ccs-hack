use std::fmt;

use ark_ff::{Field, Fp384};
use ndarray::{Array, Array2, IxDyn, Ix1};
use num_bigint::BigUint; // For matrix and vector operations
use ark_bls12_381::Fr;  // Fr is the prime field for the Bls12_381 curve

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

pub struct R1CS<F: Field> {
  m: usize,
  n: usize,
  N: usize,
  l: usize,
  A: Array2<F>,
  B: Array2<F>,
  C: Array2<F>,
}

pub struct R1CSInstance<F: Field> {
  x: Array<F, Ix1>,
}

  
pub struct R1CSWitness<F: Field> {
  w: Array<F, Ix1>,
}

impl<F: Field> R1CS<F> {
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
}

#[cfg(test)]
mod tests {
  use ark_ff::{Fp2, Fp}; // arbitrary convenient quadratic extension field
  use ndarray::{arr2, arr1}; // For creating 2D arrays
  use num_bigint::ToBigUint;

  use super::*;

    #[test]
    fn test_r1cs_satisfied() {
        let a: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
        let b: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
        let c: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE + Fr::ONE + Fr::ONE);

        let x = arr1(&[Fr::ONE + Fr::ONE]);
        let w = arr1(&[Fr::ONE + Fr::ONE]);
        

        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
        let instance = R1CSInstance { x };
        let witness = R1CSWitness { w };

        assert!(r1cs.is_satisfied_by(&instance, &witness).unwrap());
    }



    #[test]
    fn test_r1cs_not_satisfied() {
        let a: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
        let b: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
        let c: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE + Fr::ONE);  // Here, c is different from the product of a and b, so the constraint should not be satisfied

        let x = arr1(&[Fr::ONE + Fr::ONE]);
        let w = arr1(&[Fr::ONE + Fr::ONE]);

        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
        let instance = R1CSInstance { x };
        let witness = R1CSWitness { w };

        assert!(!r1cs.is_satisfied_by(&instance, &witness).unwrap());
    }


    #[test]
    fn test_r1cs_invalid_dimensions() {
        let a: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE);
        let b: Array2<Fr> = Array2::from_elem((1, 2), Fr::ONE + Fr::ONE); // Different dimensions
        let c: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE + Fr::ONE);

        let x = Array::from_elem((1, ), Fr::ONE + Fr::ONE);
        let w = Array::from_elem((1, ), Fr::ONE + Fr::ONE);

        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
        let instance = R1CSInstance { x };
        let witness = R1CSWitness { w };

        assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
    }


    #[test]
    fn test_r1cs_invalid_n_l() {
        let a: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE);
        let b: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE);
        let c: Array2<Fr> = Array2::from_elem((1, 1), Fr::ONE + Fr::ONE + Fr::ONE);
    
        let x = Array::from_elem((1, ), Fr::ONE + Fr::ONE);
        let w = Array::from_elem((1, ), Fr::ONE + Fr::ONE);
    
        // l value is set to 2 instead of 1
        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 2, A: a, B: b, C: c }; 
        let instance = R1CSInstance { x };
        let witness = R1CSWitness { w };
    
        assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
    }
    
    #[test]
    fn test_r1cs_higher_dimension() {
        let a: Array2<Fr> = Array2::from_elem((2, 2), Fr::ONE + Fr::ONE);
        let b: Array2<Fr> = Array2::from_elem((2, 2), Fr::ONE + Fr::ONE + Fr::ONE);
        let c: Array2<Fr> = Array2::from_elem((2, 2), Fr::ONE + Fr::ONE + Fr::ONE + Fr::ONE);
    
        let x = Array::from_elem((2, ), Fr::ONE + Fr::ONE);
        let w = Array::from_elem((2, ), Fr::ONE + Fr::ONE);
    
        let r1cs = R1CS { m: 2, n: 2, N: 2, l: 2, A: a, B: b, C: c };
        let instance = R1CSInstance { x };
        let witness = R1CSWitness { w };
    
        assert!(r1cs.is_satisfied_by(&instance, &witness).unwrap());
    }

}
