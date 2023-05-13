use ark_ff::Field;

// convenience; todo; move to utils
pub(crate) fn dot<F: Field>(v: &[F], w: &[F]) -> F {
  v.iter().zip(w.iter()).map(|(&vi, &wi)| vi * wi).sum()
}

pub(crate) fn matrix_vector_prod<F: Field>(matrix: &[Vec<F>], vector: &[F]) -> Vec<F> {
  matrix.iter().map(|row| dot(row, vector)).collect::<Vec<F>>()
}

pub(crate) fn hadamard<F: Field>(v1: &[F], v2: &[F]) -> Vec<F> {
  v1.iter().zip(v2.iter()).map(|(&v1_i, &v2_i)| v1_i * v2_i).collect()
}

pub(crate) fn hadasum<F: Field>(v1: &[F], v2: &[F]) -> Vec<F> {
  v1.iter().zip(v2.iter()).map(|(&v1_i, &v2_i)| v1_i + v2_i).collect()
}
