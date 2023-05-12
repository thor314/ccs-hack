use ndarray::{Array, Array2}; // For matrix and vector operations
use std::fmt;

#[derive(Debug)]
pub struct CCSError {
    details: String
}

impl CCSError {
    fn new(msg: &str) -> CCSError {
        CCSError{details: msg.to_string()}
    }
}

impl fmt::Display for CCSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

pub struct CCS {
    m: usize,
    n: usize,
    N: usize,
    l: usize,
    t: usize,
    q: usize,
    d: usize,
    M: Vec<Array2<FiniteField>>,
    S: Vec<Vec<usize>>,
    c: Vec<FiniteField>,
}

pub struct CCSInstance {
    x: Array<FiniteField>,
}

pub struct CCSWitness {
    w: Array<FiniteField>,
}

impl CCS {
    pub fn is_satisfied_by(&self, instance: &CCSInstance, witness: &CCSWitness) -> Result<bool, CCSError> {
        if self.n < self.l {
            return Err(CCSError::new("n must be greater than l"));
        }

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
        let z = witness.w.clone().into_iter().chain(
            std::iter::once(FiniteField::one(&self.M[0][[0,0]].p)).chain(instance.x.clone().into_iter())
        ).collect::<Array<_, _>>();

        // Compute sum from i=0 to q-1 of (ci * sum for each j in Si of Mj * z)
        let result = (0..self.q).map(|i| {
            self.c[i] * self.S[i].iter().fold(FiniteField::zero(&self.M[0][[0,0]].p), |sum, &j| {
                sum + self.M[j].dot(&z)
            })
        }).sum::<FiniteField>();

        // Check if result is zero
        Ok(result.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use num_bigint::ToBigUint;

    #[test]
    fn test_ccs_satisfaction() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m1 = arr2(&[[FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m1];
        let s = vec![vec![0], vec![1]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()),
                     FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_ok());
    }

    #[test]
    fn test_ccs_invalid_n_l() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0];
        let s = vec![vec![0]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 2, t: 1, q: 1, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }

    #[test]
    fn test_ccs_invalid_m_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0];
        let s = vec![vec![0]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 1, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }

    #[test]
    fn test_ccs_invalid_s_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0];
        let s = vec![vec![0]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 1, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }

    #[test]
    fn test_ccs_invalid_c_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m0];
        let s = vec![vec![0], vec![1]];
        // Modify 'c' length
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];  // c's length is now 1, not 2.
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }
    #[test]
    fn test_ccs_invalid_M_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        // Modify 'M' length
        let m = vec![m0];  // M's length is now 1, not 2.
        let s = vec![vec![0], vec![1]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()), FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }

    #[test]
    fn test_ccs_invalid_x_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m0];
        let s = vec![vec![0], vec![1]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()), FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        // Modify 'x' length
        let x = Array::from(vec![]);  // x's length is now 0, not 1.
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }
    #[test]
    fn test_ccs_invalid_w_length() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m0];
        let s = vec![vec![0], vec![1]];
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()), FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        // Modify 'w' length
        let w = Array::from(vec![]);  // w's length is now 0, not 1 (n - l - 1 = 1 - 1 - 1 = -1).
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }
    #[test]
    fn test_ccs_invalid_S_cardinality() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m0];
        // Modify 'S' cardinality
        let s = vec![vec![0, 0], vec![1, 1]];  // Each multiset in S now has cardinality 2, not 1 (maximum allowed by 'd').
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()), FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }
    #[test]
    fn test_ccs_invalid_S_domain() {
        let m0 = arr2(&[[FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())]]);
        let m = vec![m0, m0];
        // Modify 'S' domain
        let s = vec![vec![0, 2], vec![1]];  // 2 is not in the domain {0, 1} (t - 1 = 2 - 1 = 1).
        let c = vec![FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap()), FiniteField::new(1.to_biguint().unwrap(), 5.to_biguint().unwrap())];
        let x = Array::from(vec![FiniteField::new(2.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let w = Array::from(vec![FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap())]);
        let ccs = CCS { m: 1, n: 1, N: 1, l: 1, t: 2, q: 2, d: 1, M: m, S: s, c: c };
        let instance = CCSInstance { x: x };
        let witness = CCSWitness { w: w };

        assert!(ccs.is_satisfied_by(&instance, &witness).is_err());
    }


}
