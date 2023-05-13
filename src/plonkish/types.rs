use ark_ff::Field;

#[derive(Clone)]
pub struct Constraint<F: Field> {
  pub points: Vec<F>,
}
