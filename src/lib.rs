#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod ccs;
pub mod r1cs;
#[cfg(test)]
mod tests;
pub(crate) mod utils;
// pub mod air;
pub mod plonkish;

pub use ccs::CCS;
// pub use plonkish::Plonkish;
pub use r1cs::R1CS;

/// convenience type for matrix type placeholder
pub type Matrix<F> = Vec<Vec<F>>;
// pub type nmMatrix<F, const n: usize, const m: usize> = [[F; m]; n];

/// Multisets are represented as Vecs (placeholder)
/// Multisets from domain $[t-1]$, with cardinality of each multiset at most $d$
pub type Multiset<T> = Vec<T>;
