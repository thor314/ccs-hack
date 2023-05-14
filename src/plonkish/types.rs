use ark_ff::Field;

/// Each constraint $i$ is specified by a vector $T_i$ of len $t$, with
/// entries over [n+e-1]. $T_i$ is interpreted as specifying $t$ entries of a purported
/// satisfying assignment $z$ to feed to $g$.
#[derive(Clone)]
pub struct Constraint<F: Field> {
  pub points: Vec<F>,
}
