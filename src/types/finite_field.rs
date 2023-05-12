use std::{
  fmt,
  ops::{Add, Div, Mul, Sub},
};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

// Custom error for operations that are not allowed in a Finite Field
#[derive(Debug)]
pub struct FiniteFieldError {
  details: String,
}

impl FiniteFieldError {
  fn new(msg: &str) -> FiniteFieldError { FiniteFieldError { details: msg.to_string() } }
}

impl fmt::Display for FiniteFieldError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.details) }
}

#[derive(Debug, Clone)]
pub struct FiniteField {
    pub value: BigUint,
    pub p: BigUint,
}

impl FiniteField {
  pub fn new(value: BigUint, p: BigUint) -> Self { Self { value: value % &p, p } }

  pub fn inverse(&self) -> Result<Self, FiniteFieldError> {
    if self.value.is_zero() {
      Err(FiniteFieldError::new("Division by zero"))
    } else {
      // Extended Euclidean algorithm to find multiplicative inverse
      let mut u = self.value.clone();
      let mut v = self.p.clone();
      let mut a = BigUint::one();
      let mut c = BigUint::zero();
      let mut q;
      while !v.is_zero() {
        q = &u / &v;
        u = u - &q * &v;
        a = a - q * &c;
        std::mem::swap(&mut u, &mut v);
        std::mem::swap(&mut a, &mut c);
      }
      Ok(Self::new(a, self.p.clone()))
    }
  }
}

// Implement the required arithmetic operations
impl Add for FiniteField {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    if self.p != other.p {
      panic!("Cannot add elements with different moduli");
    }
    Self::new(self.value + other.value, self.p.clone())
  }
}

impl Sub for FiniteField {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    if self.p != other.p {
      panic!("Cannot subtract elements with different moduli");
    }
    Self::new(self.value + (self.p.clone() - other.value), self.p.clone())
  }
}

impl Mul for FiniteField {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    if self.p != other.p {
      panic!("Cannot multiply elements with different moduli");
    }
    Self::new(self.value * other.value, self.p.clone())
  }
}

impl Div for FiniteField {
  type Output = Result<Self, FiniteFieldError>;

  fn div(self, other: Self) -> Result<Self, FiniteFieldError> {
    if self.p != other.p {
      panic!("Cannot divide elements with different moduli");
    }
    Ok(self * other.inverse()?)
  }
}


#[cfg(test)]
mod tests {
  use num_bigint::ToBigUint;

  use super::*;

  #[test]
  fn test_add() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let b = FiniteField::new(4.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let result = a + b;
    assert_eq!(result.value, 2.to_biguint().unwrap());
  }

  #[test]
  fn test_sub() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let b = FiniteField::new(4.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let result = a - b;
    assert_eq!(result.value, 4.to_biguint().unwrap());
  }

  #[test]
  fn test_mul() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let b = FiniteField::new(4.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let result = a * b;
    assert_eq!(result.value, 2.to_biguint().unwrap());
  }

  #[test]
  fn test_div() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let b = FiniteField::new(4.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let result = a / b;
    assert_eq!(result.unwrap().value, 2.to_biguint().unwrap());
  }

  #[test]
  #[should_panic]
  fn test_div_by_zero() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let b = FiniteField::new(0.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let _ = a / b;
  }

  #[test]
  fn test_inverse() {
    let a = FiniteField::new(3.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let inverse = a.inverse();
    assert_eq!(inverse.unwrap().value, 2.to_biguint().unwrap());
  }

  #[test]
  #[should_panic]
  fn test_inverse_zero() {
    let a = FiniteField::new(0.to_biguint().unwrap(), 5.to_biguint().unwrap());
    let _ = a.inverse();
  }
}
