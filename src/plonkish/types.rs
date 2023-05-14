use ark_ff::Field;
pub type UncheckedCopyConstaint = ((usize,usize),(usize,usize));
pub type UncheckedGateConstratint<F> = Vec<F>;
/// Each constraint $i$ is specified by a vector $T_i$ of len $t$, with
/// entries over [n+e-1]. $T_i$ is interpreted as specifying $t$ entries of a purported
/// satisfying assignment $z$ to feed to $g$.
#[derive(Debug, Clone)]
pub struct PlonkishGateConstraint<F: Field> {
  pub points: Vec<F>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PlonkishCopyConstraint {
  pub points: ((usize,usize),(usize,usize)),
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

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}
impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}