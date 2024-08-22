use std::ops::Not;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal {
  variable: usize,
  sign: bool,
}

impl Literal {
  pub fn new(variable: usize, sign: bool) -> Self {
    Self { variable, sign }
  }

  pub fn variable(&self) -> usize {
    self.variable
  }

  pub fn sign(&self) -> bool {
    self.sign
  }
}

impl Not for Literal {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self {
      variable: self.variable,
      sign: !self.sign,
    }
  }
}
