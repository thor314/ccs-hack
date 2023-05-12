// Import necessary libraries
use ndarray::{Array, Array2}; // For matrix and vector operations
use num_bigint::BigUint; // For finite field operations
pub mod ccs; // For the CCS structure
pub mod r1cs; // For the R1CS structure
// We need to define our own Finite Field type.
#[derive(Debug, Clone)]
pub struct FiniteField {
    p: BigUint,
}
