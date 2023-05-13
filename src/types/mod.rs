use ark_ff::PrimeField;
use nalgebra::{DMatrix, MatrixView};

// Import necessary libraries
pub mod ccs;
pub mod r1cs;
pub mod utils;
// pub mod air;
pub mod plonkish;
// pub mod finite_field;
/// convenience type for matrix type placeholder
pub type Matrix<F> = Vec<Vec<F>>;
// pub type nmMatrix<F, const n: usize, const m: usize> = [[F; m]; n];

/// Multisets are represented as Vecs (placeholder)
/// Multisets from domain $[t-1]$, with cardinality of each multiset at most $d$
pub type Multiset<T> = Vec<T>;
