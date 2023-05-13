use ark_ff::Field;
use thiserror::Error;

// Custom error for operations that are not allowed in R1CS
#[derive(Debug, Error)]
pub enum R1CSError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("Default error: {0}")]
  Default(String),
}

pub struct R1CSInstance<F: Field> {
  pub x: Vec<F>,
}

pub struct R1CSWitness<F: Field> {
  pub w: Vec<F>,
}
