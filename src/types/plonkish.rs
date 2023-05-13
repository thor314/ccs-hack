use ark_ff::Field;
use ark_poly::Polynomial;
use ndarray::{Array, Array2, IxDyn, Ix1, concatenate, Ix2};


pub struct PlonkishStructure<F: Field, P: Polynomial<F, Point = F>> {
    m: usize,
    n: usize,
    e: usize,
    t: usize,
    q: usize,
    d: usize,
    g: P,
    s: Array<F, Ix1>,
    constraints: Array<Constraint<F>, Ix2>,
}

#[derive(Clone)]
pub struct Constraint<F: Field> {
    points: Vec<F>,
}

impl<F: Field, P: Polynomial<F, Point = F>> PlonkishStructure<F, P> {
    pub fn new(m: usize, n: usize, e: usize, t: usize, q: usize, d: usize, g: P, s: Array<F, Ix1>, constraints: Array<Constraint<F>, Ix2>) -> Self {
        PlonkishStructure { m, n, e, t, q, d, g, s, constraints }
    }
    fn check_constraints(&self, x: &Array<F, Ix1>, w: &Array<F, Ix1>) -> bool {
        // Concatenate the witness, public input, and selectors
        let z = ndarray::stack(ndarray::Axis(0), &[w.view(), x.view(), self.s.view()]).unwrap();
        let len = z.len();
        let z = z.into_shape(len).unwrap();

        // Check each constraint
        for constraint in self.constraints.iter() {
            // Iterate over each point in the constraint
            for &point in &constraint.points {
                // Constraint is satisfied if g applied to this point is zero
                if self.g.evaluate(&P::Point::from(point)) != F::zero() {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};
    use ark_ff::{Field, One, Zero, Fp};

    #[test]
    fn test_plonkish_satisfied() {
        // Define the polynomial g(x) = x by its coefficients [0, 1]
        let g: DensePolynomial<Fr> = DensePolynomial::from_coefficients_vec(vec![Fr::zero(), Fr::one()]);
        let s = Array::from(vec![Fp::one()]);
        let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });
        let structure = PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 1, g, s, constraints };

        let x = Array::from(vec![Fp::zero()]);
        let w = Array::from(vec![Fp::zero()]);

        assert!(structure.check_constraints(&x, &w));
    }

    #[test]
    fn test_non_trivial_polynomial() {
        // g(x) = x^2 + 1
        let g: DensePolynomial<Fr> = DensePolynomial::from_coefficients_vec(vec![Fr::one(), Fr::zero(), Fr::one()]);

        // Create a selector array filled with ones
        let s = Array::ones(1);

        // Create an array of constraints. For this test, we will just use one constraint that requires g(x) = 0, which is not possible for g(x) = x^2 + 1
        let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });
        let structure = PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, s, constraints };

        // Test that the constraints are not satisfied with a witness of [0]
        assert!(!structure.check_constraints(&Array::zeros(1), &Array::zeros(1)));

        // Test that the constraints are not satisfied with a witness of [1]
        assert!(!structure.check_constraints(&Array::ones(1), &Array::ones(1)));
    }

    #[test]
    fn test_unsatisfied_constraints() {
           // Define g(x) = x^2 + 1
    let g: DensePolynomial<Fr> = DensePolynomial::from_coefficients_vec(vec![Fr::one(), Fr::zero(), Fr::one()]);

    // Create a selector array filled with ones
    let s = Array::ones(1);

    // Create an array of constraints. For this test, we will just use one constraint that requires g(x) = 0
    // This is impossible for g(x) = x^2 + 1, so this constraint will never be satisfied
    let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::zero()] });

    // Create the Plonkish structure
    let structure = PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, s, constraints };

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
        let g: DensePolynomial<Fr> = DensePolynomial::from_coefficients_vec(vec![Fr::from(2u32), Fr::one()]);
    
        // Create a selector array filled with non-zero values (2 in this case)
        let s = Array::from_elem(1, Fr::from(2u32));
    
        // Create an array of constraints that requires g(x) = 0
        let constraints = Array2::from_elem((1, 1), Constraint { points: vec![Fr::from(2u32)] });
    
        // Create the Plonkish structure
        let structure = PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 2, g, s, constraints };
    
        // Test that the constraints are satisfied with a witness of [2]
        assert!(structure.check_constraints(&Array::from_elem(1, Fr::from(2u32)), &Array::from_elem(1, Fr::from(2u32))));
    
        // Test that the constraints are not satisfied with a witness of [1]
        assert!(!structure.check_constraints(&Array::ones(1), &Array::ones(1)));
    
        // Test that the constraints are not satisfied with a witness of [0]
        assert!(!structure.check_constraints(&Array::zeros(1), &Array::zeros(1)));
    
        // Test that the constraints are not satisfied with a random witness
        let witness = Array::from_elem(1, Fr::from(3u32));
        assert!(!structure.check_constraints(&witness, &witness));
    }
    
}
