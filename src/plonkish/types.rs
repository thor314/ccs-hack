use ark_ff::Field;
pub type UncheckedCopyConstaint = (Point, Point);
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
  pub points: (Point, Point),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}
impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match self.x.cmp(&other.x) {
      std::cmp::Ordering::Equal => self.y.partial_cmp(&other.y),
      std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
      std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
    }
  }
}
