use ark_ff::Field;

/// Each constraint $i$ is specified by a vector $T_i$ of len $t$, with
/// entries over [n+e-1]. $T_i$ is interpreted as specifying $t$ entries of a purported
/// satisfying assignment $z$ to feed to $g$.
#[derive(Debug, Clone)]
pub struct PlonkishConstraint<F: Field> {
  pub points: Vec<F>,
}

#[derive(Debug, Clone)]
pub struct PlonkishInstance<F: Field> {
  /// w in %\mathbb F^{l}$
  pub x: Vec<F>,
}

#[derive(Debug, Clone)]
pub struct PlonkishWitness<F: Field> {
  /// w in %\mathbb F^{n-l}$
  pub w: Vec<F>,
}
