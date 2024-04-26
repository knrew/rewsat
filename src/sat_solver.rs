use std::{collections::HashMap, hash::Hash};

use crate::dpll;

#[derive(Clone, Debug)]
pub struct Variable<TName> {
  name: TName,
  sign: bool,
}

impl<TName> Variable<TName> {
  pub fn name(&self) -> &TName {
    &self.name
  }

  pub fn sign(&self) -> bool {
    self.sign
  }
}

impl<TName: Clone> Variable<TName> {
  pub fn new(name: &TName) -> Self {
    Self {
      name: name.to_owned(),
      sign: true,
    }
  }

  pub fn not(&self) -> Self {
    Self {
      name: self.name.to_owned(),
      sign: !self.sign,
    }
  }
}

impl From<&str> for Variable<String> {
  fn from(name: &str) -> Self {
    Self {
      name: String::from(name),
      sign: true,
    }
  }
}

pub struct SATSolver<TName> {
  num_variables: usize,
  clauses: Vec<dpll::Clause>,
  name_to_id: HashMap<TName, dpll::Variable>,
  id_to_name: HashMap<dpll::Variable, TName>,
}

impl<TName: Clone + Eq + Hash> SATSolver<TName> {
  pub fn new() -> Self {
    Self {
      num_variables: 0,
      clauses: vec![],
      name_to_id: HashMap::new(),
      id_to_name: HashMap::new(),
    }
  }

  pub fn solve(&self) -> Option<HashMap<TName, bool>> {
    match dpll::DPLL::solve(&self.clauses) {
      Some(res) => Some(
        res
          .iter()
          .map(|e| (self.id_to_name.get(&e.abs()).unwrap().to_owned(), *e > 0))
          .collect(),
      ),
      None => None,
    }
  }

  pub fn clear(&mut self) {
    *self = Self::new();
  }

  pub fn add_variable(&mut self, variable: &Variable<TName>) {
    if self.name_to_id.contains_key(&variable.name) {
      return;
    }

    self.num_variables += 1;
    self.name_to_id.insert(
      variable.name.to_owned(),
      self.num_variables as dpll::Variable,
    );
    self.id_to_name.insert(
      self.num_variables as dpll::Variable,
      variable.name.to_owned(),
    );
  }

  pub fn add_clause(&mut self, clause: &[&Variable<TName>]) {
    let clause = clause
      .iter()
      .map(|literal| {
        (if literal.sign { 1 } else { -1 }) * self.name_to_id.get(&literal.name).unwrap().to_owned()
      })
      .collect();

    self.clauses.push(clause);
  }
}
