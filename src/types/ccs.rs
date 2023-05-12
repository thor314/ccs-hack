use ndarray::{Array, Array2};

use super::finite_field::FiniteField;

// Multisets are represented as Vecs (placeholder)
type Multiset<T> = Vec<Vec<T>>;

// Defining the CCS structure
// todo: may want to move some of these usize parameters into type gen
#[derive(Debug)]
pub struct CCS<const l: usize> {
  m: usize,
  // todo: n > l
  n: usize,
  N: usize,
  t: usize,
  q: usize,
  d: usize,
  /// matrices $M_0,...,M_{t-1}\in \mathbb F^{m\times n}$ with at most $N=\Omega(\max(m,n))$
  /// non-zero entries in total
  M: Vec<Array2<FiniteField>>,
  /// a sequence of $q$ multisets $[S_0,...,S_{q-1}]$ where an element in each multiset is from the
  /// domain $[t-1]$, with cardinatily of each multiset at most $d$
  S: Multiset<usize>,
  /// a sequence of $q$ constants $[c_0,...,c_{q-1}]$ with $c_i\in \mathbb F$.
  c: Vec<FiniteField>,
}

/// A CCS instance consists of public input $x\in \mathbb F^l$.
#[derive(Debug)]
pub struct CCSInstance<const l: usize> {
  x: Array<FiniteField, ndarray::Dim<[ndarray::Ix; l]>>,
}

/// A CCS witness consists of a vector $w\in $\mathbb F^{n-l-l}$.
#[derive(Debug)]
pub struct CCSWitness<const l: usize> {
  x: Array<FiniteField, ndarray::Dim<[ndarray::Ix; l]>>,
}

impl<const l: usize> CCS<l> {
  pub fn is_satisfied_by(&self, instance: &CCSInstance<l>, witness: &CCSWitness<l>) -> bool {
    todo!();
    // Implement the checks based on the equation (2) in the definition
    // This will involve matrix-vector multiplication, Hadamard product, and summing over the
    // multisets
  }
}
