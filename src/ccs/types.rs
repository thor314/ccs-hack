use ark_ff::Field;

/// A CCS witness consists of a vector $w\in $\mathbb F^{n-l-l}$.
#[derive(Debug)]
pub struct CCSWitness<F: Field> {
  pub w: Vec<F>,
}

/// A CCS instance consists of public input $x\in \mathbb F^l$.
#[derive(Debug)]
pub struct CCSInstance<F: Field> {
  pub x: Vec<F>,
}

impl<F: Field> CCSWitness<F> {
  pub fn new(w: Vec<F>) -> Self {
    Self { w }
  }
}

impl<F: Field> CCSInstance<F> {
  pub fn new(x: Vec<F>) -> Self {
    Self { x }
  }
}
