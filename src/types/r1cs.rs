use ndarray::{Array, Array2, IxDyn}; // For matrix and vector operations
use std::fmt;
use crate::types::finite_field::FiniteField;
// Custom error for operations that are not allowed in R1CS
#[derive(Debug)]
pub struct R1CSError {
    details: String
}

impl R1CSError {
    fn new(msg: &str) -> R1CSError {
        R1CSError{details: msg.to_string()}
    }
}

impl fmt::Display for R1CSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

pub struct R1CS {
    m: usize,
    n: usize,
    N: usize,
    l: usize,
    A: Array2<FiniteField>,
    B: Array2<FiniteField>,
    C: Array2<FiniteField>,
}

pub struct R1CSInstance {
    x: Array<FiniteField, IxDyn>,
}

pub struct R1CSWitness {
    w: Array<FiniteField, IxDyn>,
}

impl R1CS {
    pub fn is_satisfied_by(&self, instance: &R1CSInstance, witness: &R1CSWitness) -> Result<bool, R1CSError> {
        if self.A.shape() != self.B.shape() || self.B.shape() != self.C.shape() {
            return Err(R1CSError::new("A, B, and C must have the same dimensions"));
        }

        if self.n < self.l {
            return Err(R1CSError::new("n must be greater than l"));
        }

        // Compute z = (w, 1, x)
        let z = witness.w.clone().into_iter().chain(
            std::iter::once(FiniteField::one(&self.A[[0,0]].p)).chain(instance.x.clone().into_iter())
        ).collect::<Array<_, _>>();

        // Compute (A * z) * (B * z) - C * z
        let lhs = self.A.dot(&z).into_iter().zip(self.B.dot(&z).into_iter()).map(|(a, b)| a * b).collect::<Array<_, _>>();
        let rhs = self.C.dot(&z);
        let result = lhs - rhs;

        // Check if all entries are zero
        Ok(result.iter().all(|x| x.is_zero()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2; // For creating 2D arrays
    use num_bigint::ToBigUint;

    #[test]
    fn test_r1cs_satisfied() {
        let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let b = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let c = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
        let instance = R1CSInstance { x: x };
        let witness = R1CSWitness { w: w };

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
        let instance = R1CSInstance { x: x };
        let witness = R1CSWitness { w: w };

        assert!(!r1cs.is_satisfied_by(&instance, &witness).unwrap());
    }

    #[test]
    fn test_r1cs_invalid_dimensions() {
        let a = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let b = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap()),
                         FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let c = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let r1cs = R1CS { m: 1, n: 1, N: 1, l: 1, A: a, B: b, C: c };
        let instance = R1CSInstance { x: x };
        let witness = R1CSWitness { w: w };

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
        let instance = R1CSInstance { x: x };
        let witness = R1CSWitness { w: w };

        assert!(r1cs.is_satisfied_by(&instance, &witness).is_err());
    }
}
