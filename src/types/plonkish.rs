use ark_ff::Field;
use ark_poly::Polynomial;
use ndarray::{Array, Array2, IxDyn, Ix1, concatenate};


struct PlonkishStructure <F: Field, P: Polynomial<F>> {
    m: usize,
    n: usize,
    e: usize,
    t: usize,
    q: usize,
    d: usize,
    g: P,
    s: Array<F, Ix1>, // 1 dimensional for now
    constraints: Array2<Constraint<P::Point>>, // use P::Point in Constraint
}
#[derive(Clone)]
struct Constraint<T> {
    indices: Vec<T>,
}

impl<F: Field, P: Polynomial<F>> PlonkishStructure<F, P> {
    fn check_constraints(&self, x: &Array<F, Ix1>, w: &Array<F, Ix1>) -> bool {
        // Concatenate the witness, public input, and selectors
        // Todo: handle this error better
        let z = ndarray::stack(ndarray::Axis(0), &[w.view(), x.view(), self.s.view()]).unwrap();
        let len = z.len();
        let z = z.into_shape(len).unwrap();
        
        // Check each constraint
        for constraint in &self.constraints {
            // Iterate over each point in the constraint
            for point in &constraint.indices {
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
    use super::*;
    use ark_bls12_381::Fr;
    use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};
    use ark_ff::{Field, One, Zero, Fp};

    #[test]
    fn test_plonkish_satisfied() {
        // Define the polynomial g(x) = x by its coefficients [0, 1]
        let g: DensePolynomial<Fr> = DensePolynomial::from_coefficients_vec(vec![Fr::zero(), Fr::one()]);
        let s = Array::from(vec![Fp::one()]);
        let constraints = Array2::from_elem((1, 1), Constraint { indices: vec![Fp::zero()] });
        let structure = PlonkishStructure { m: 1, n: 1, e: 1, t: 1, q: 1, d: 1, g, s, constraints };

        let x = Array::from(vec![Fp::zero()]);
        let w = Array::from(vec![Fp::zero()]);

        assert!(structure.check_constraints(&x, &w));
    }
}
