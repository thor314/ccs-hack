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
