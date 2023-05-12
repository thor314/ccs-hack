// Import necessary libraries
pub mod ccs;
pub mod r1cs;
// pub mod air;
// pub mod plonkish;
pub mod finite_field;

/// convenience type for constant-sized l-dimensional array
pub type LArray<const l: usize> =
  ndarray::Array<finite_field::FiniteField, ndarray::Dim<[ndarray::Ix; l]>>;

// Multisets are represented as Vecs (placeholder)
pub type Multiset<T> = Vec<Vec<T>>;
