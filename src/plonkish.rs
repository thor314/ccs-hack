use ark_ff::Field;
use ark_poly::Polynomial;

pub struct PlonkishStructure<F: Field, P: Polynomial<F, Point = F>> {
  n:           usize,
  m:           usize,
  l:           usize,
  t:           usize,
  q:           usize,
  d:           usize,
  e:           usize,
  // todo: not entirely clear how to express this
  /// A multivariate polynomial $g$ in $t$ variables, where $g$ is a sum of $q$ monomials, with
  /// each monomial of degree at most $d$.
  g:           P,
  /// A vector of constants, over $\mathbb F^e$.
  selectors:   Vec<F>,
  /// A set of $m$ constraints. Each constraint $i$ is specified by a vector $T_i$ of len $t$, with
  /// entries over [n+e-1]. $T_i$ is interpreted as specifying $t$ entries of a purported
  /// satisfying assignment $z$ to feed to $g$.
  constraints: Vec<Constraint<F>>,
}

#[derive(Clone)]
pub struct Constraint<F: Field> {
  points: Vec<F>,
}

impl<F: Field, P: Polynomial<F, Point = F>> PlonkishStructure<F, P> {
  pub fn new(
    m: usize,
    n: usize,
    e: usize,
    t: usize,
    q: usize,
    d: usize,
    g: P,
    selectors: Vec<F>,
    constraints: Vec<Constraint<F>>,
  ) -> Self {
    assert!(g.degree() <= d);
    assert!(selectors.len() == e);
    Self { m, n, e, t, q, d, g, selectors, constraints }
  }

  fn check_constraints(&self, x: &Array<F, Ix1>, w: &Array<F, Ix1>) -> bool {
    // Concatenate the witness, public input, and selectors
    let z = ndarray::stack(ndarray::Axis(0), &[w.view(), x.view(), self.selectors.view()]).unwrap();
    let len = z.len();
    let z = z.into_shape(len).unwrap();

    // Check each constraint
    for constraint in self.constraints.iter() {
      // Iterate over each point in the constraint
      for &point in &constraint.points {
        // Constraint is satisfied if g applied to this point is zero
        if self.g.evaluate(&point) != F::zero() {
          return false;
        }
      }
    }
    true
  }
}
#[cfg(test)]
mod tests {
  use ark_bls12_381::Fr;
  use ark_ff::{Field, Fp, One, Zero};
  use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};

  use super::*;

  #[test]
  fn test_plonkish_satisfied() {
    // Define the polynomial g(x) = x by its coefficients [0, 1]
    let g: DensePolynomial<Fr> =
      DensePolynomial::from_coefficients_vec(vec![Fr::zero(), Fr::one()]);
    let s = Array::from(vec![Fp::one()]);
    let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });
    let structure =
      PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 1, g, selectors: s, constraints };

    let x = Array::from(vec![Fp::zero()]);
    let w = Array::from(vec![Fp::zero()]);

    assert!(structure.check_constraints(&x, &w));
  }

  #[test]
  fn test_non_trivial_polynomial() {
    // g(x) = x^2 + 1
    let g: DensePolynomial<Fr> =
      DensePolynomial::from_coefficients_vec(vec![Fr::one(), Fr::zero(), Fr::one()]);

    // Create a selector array filled with ones
    let s = Array::ones(1);

    // Create an array of constraints. For this test, we will just use one constraint that requires
    // g(x) = 0, which is not possible for g(x) = x^2 + 1
    let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });
    let structure =
      PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, selectors: s, constraints };

    // Test that the constraints are not satisfied with a witness of [0]
    assert!(!structure.check_constraints(&Array::zeros(1), &Array::zeros(1)));

    // Test that the constraints are not satisfied with a witness of [1]
    assert!(!structure.check_constraints(&Array::ones(1), &Array::ones(1)));
  }

  #[test]
  fn test_unsatisfied_constraints() {
    // Define g(x) = x^2 + 1
    let g: DensePolynomial<Fr> =
      DensePolynomial::from_coefficients_vec(vec![Fr::one(), Fr::zero(), Fr::one()]);

    // Create a selector array filled with ones
    let s = Array::ones(1);

    // Create an array of constraints. For this test, we will just use one constraint that requires
    // g(x) = 0 This is impossible for g(x) = x^2 + 1, so this constraint will never be
    // satisfied
    let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });

    // Create the Plonkish structure
    let structure =
      PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, selectors: s, constraints };

    // Test that the constraints are not satisfied with a witness of [1]
    assert!(!structure.check_constraints(&Array::ones(1), &Array::ones(1)));

    // Test that the constraints are not satisfied with a witness of [0]
    assert!(!structure.check_constraints(&Array::zeros(1), &Array::zeros(1)));

    // Test that the constraints are not satisfied with a random witness
    let witness = Array::from_elem(1, Fr::from(3u32));
    assert!(!structure.check_constraints(&witness, &witness));
  }

  #[test]
  fn test_non_zero_selectors() {
    // Define g(x) = x - 2
    let g: DensePolynomial<Fr> =
      DensePolynomial::from_coefficients_vec(vec![Fr::from(2u32), Fr::one()]);

    // Create a selector array filled with non-zero values (2 in this case)
    let s = Array::from_elem(1, Fr::from(2u32));

    // Create an array of constraints that requires g(x) = 0
    let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::from(2u32)] });

    // Create the Plonkish structure
    let structure =
      PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, selectors: s, constraints };

    // Test that the constraints are satisfied with a witness of [2]
    assert!(structure.check_constraints(
      &Array::from_elem(1, Fr::from(2u32)),
      &Array::from_elem(1, Fr::from(2u32))
    ));

    // Test that the constraints are not satisfied with a witness of [1]
    assert!(!structure.check_constraints(&Array::ones(1), &Array::ones(1)));

    // Test that the constraints are not satisfied with a witness of [0]
    assert!(!structure.check_constraints(&Array::zeros(1), &Array::zeros(1)));

    // Test that the constraints are not satisfied with a random witness
    let witness = Array::from_elem(1, Fr::from(3u32));
    assert!(!structure.check_constraints(&witness, &witness));
  }
}
