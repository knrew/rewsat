use std::ops::{Index, IndexMut};

use super::literal::Literal;

#[derive(Clone, Debug)]
pub struct Clause {
  literals: Vec<Literal>,
  is_valid: bool,    // 矛盾がないならtrue
  has_deleted: bool, // どんな割り当てでも真ならtrue
}

impl Clause {
  pub fn new() -> Self {
    Self {
      literals: vec![],
      is_valid: true,
      has_deleted: true,
    }
  }

  pub fn get(&self, index: usize) -> Option<&Literal> {
    self.literals.get(index)
  }

  pub fn get_mut(&mut self, index: usize) -> Option<&mut Literal> {
    self.literals.get_mut(index)
  }

  pub fn is_empty(&self) -> bool {
    self.literals.is_empty()
  }

  pub fn len(&self) -> usize {
    self.literals.len()
  }

  pub fn is_valid(&self) -> bool {
    self.is_valid
  }

  pub fn has_deleted(&self) -> bool {
    self.has_deleted
  }

  pub fn is_unit(&self) -> bool {
    self.literals.len() == 1
  }

  pub fn assign(&mut self, variable: usize, sign: bool) {
    if self.has_deleted || !self.is_valid {
      return;
    }

    // 単リテラルLに対してLを含む節を消去する
    // (Lを含まないもののみを残す)
    if self
      .literals
      .iter()
      .any(|&literal| literal.variable() == variable && literal.sign() == sign)
    {
      self.literals = vec![];
      self.has_deleted = true;
      return;
    }

    // 単リテラルLに対して各節から\lnot L を消去する
    // 消去した結果空節になれば充足不能
    self
      .literals
      .retain(|literal| (literal.variable(), literal.sign()) != (variable, !sign));

    if self.literals.is_empty() {
      self.literals = vec![];
      self.is_valid = false;
    }
  }
}

impl From<&Clause> for Clause {
  fn from(clause: &Clause) -> Self {
    clause.clone()
  }
}

impl From<&[Literal]> for Clause {
  fn from(literals: &[Literal]) -> Self {
    Self {
      literals: literals.to_vec(),
      is_valid: true,
      has_deleted: false,
    }
  }
}

impl From<&Vec<Literal>> for Clause {
  fn from(literals: &Vec<Literal>) -> Self {
    Self {
      literals: literals.clone(),
      is_valid: true,
      has_deleted: false,
    }
  }
}

impl Index<usize> for Clause {
  type Output = Literal;

  fn index(&self, index: usize) -> &Self::Output {
    &self.literals[index]
  }
}

impl IndexMut<usize> for Clause {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.literals[index]
  }
}
