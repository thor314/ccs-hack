use std::collections::HashSet;

use ark_ff::Field;
use ark_poly::Polynomial;

use self::types::{PlonkishInstance, PlonkishWitness, UncheckedCopyConstaint, UncheckedGateConstratint, PlonkishGateConstraint, PlonkishCopyConstraint};

pub mod types;

pub struct PlonkishStructure<F: Field, P: Polynomial<F, Point = F>> {
  n:           usize,
  m:           usize,
  l:           usize,
  t:           usize,
  q:           usize,
  d:           usize,
  e:           usize,
  // todo: not entirely clear how to express this
  // Srinath's comment, via Shumo: "Uh, use a matrix"
  // Thor's comment: hrgnngngngggg, something like this? https://github.com/arkworks-rs/sumcheck/blob/master/src/ml_sumcheck/data_structures.rs#L33
  /// A multivariate polynomial $g$ in $t$ variables, where $g$ is a sum of $q$ monomials, with
  /// each monomial of degree at most $d$.
  g:           P,
  /// A vector of constants, over $\mathbb F^e$.
  selectors:   Vec<F>,
  /// A set of $m$ constraints. Each constraint is specified via  vector $T_i$ of length $t$ over
  /// [n+e-1].
  gate_constraints: Vec<PlonkishGateConstraint<F>>,
  copy_constraints: Vec<PlonkishCopyConstraint>,
}

impl<F: Field, P: Polynomial<F, Point = F>> PlonkishStructure<F, P> {
  pub fn new(
    m: usize, // numbers of rows
    n: usize, // number of columns
    l: usize, // size of public input (collums)
    e: usize, // non selector collumns 
    t: usize, // Number of variables
    q: usize, // number of terms in plonk equation
    d: usize, // Max degree
    g: P,
    selectors: Vec<F>,
    gate_constraints: Vec<UncheckedGateConstratint<F>>,
    copy_constaints: Vec<UncheckedCopyConstaint>,
  ) -> Self {
    // todo: assert g has t vars
    // todo assert g is a sum of q monomials
    assert!(g.degree() <= d);
    assert!(selectors.len() == e);
    assert!(gate_constraints.len() == m);
    let constraint_point_max = F::from((n + e - 1) as u64);
    let gate_constraints: Vec<PlonkishGateConstraint<F>> =
      gate_constraints.into_iter().map(|c| Self::new_gate_constraint(c, t, constraint_point_max)).collect();

    let hashSet_copy_constraints: HashSet<_> = copy_constaints.into_iter().map(|c| Self::new_copy_constraint(c, m, n )).collect();
    let copy_constraints: Vec<PlonkishCopyConstraint> = hashSet_copy_constraints.into_iter().collect();
    // get this ^ into a vec of copy constraints
    Self { m, n, l, e, t, q, d, g, selectors, gate_constraints, copy_constraints }
  }

  /// A Plonkish structure-instance (S,w) is satisfied by a Plonkish witness $w$ if:
  /// for all $i\in [m-1], g(z[T_i[1]],...,z[T_i[t]])=0$
  /// where $z=(w,x,s)\in\mathbb F^{n+e}$.
  pub fn is_satisfied_by(&self, x: &PlonkishInstance<F>, w: &PlonkishWitness<F>) -> bool {
    let z = [w.w.clone(), x.x.clone(), self.selectors.clone()].concat();

    // for all i in [m-1]
    (0..self.m)
      .map(
        |i| F::zero(), /* temp
                        * self.g.evaluate(&self.constraints[i].points) */
      )
      .all(|eval_i| eval_i == F::zero())
  }

  // todo: panic -> err
  pub fn new_gate_constraint(t_i: Vec<F>, t: usize, point_max: F) -> PlonkishGateConstraint<F> {
    assert_eq!(t_i.len(), t);
    for &point in t_i.iter() {
      assert!(point <= point_max);
    }
    PlonkishGateConstraint { points: t_i }
  }
  pub fn new_copy_constraint(copy_constraint: UncheckedCopyConstaint ,rows: usize, columns: usize) -> PlonkishCopyConstraint {

    assert!(copy_constraint.0.0 < columns);
    assert!(copy_constraint.1.0 < columns);
    assert!(copy_constraint.0.1 < rows);
    assert!(copy_constraint.1.1 < rows);
    PlonkishCopyConstraint { points: copy_constraint }
  }

  pub fn new_instance(x: Vec<F>, n: usize, l: usize) -> PlonkishInstance<F> {
    assert_eq!(x.len(), l);
    PlonkishInstance { x }
  }

  pub fn new_witness(w: Vec<F>, n: usize, l: usize) -> PlonkishWitness<F> {
    assert_eq!(w.len(), n - l);
    PlonkishWitness { w }
  }
}
