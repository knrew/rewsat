use std::{collections::HashMap, hash::Hash};

use crate::SATSolverCore;

#[derive(Clone, Debug)]
pub struct Variable<TName> {
  name: TName,
  sign: bool,
}

impl<TName: Clone> Variable<TName> {
  pub fn new(name: &TName) -> Self {
    Self {
      name: name.clone(),
      sign: true,
    }
  }

  pub fn not(&self) -> Self {
    Self {
      name: self.name.clone(),
      sign: !self.sign,
    }
  }

  pub fn name(&self) -> &TName {
    &self.name
  }

  pub fn sign(&self) -> bool {
    self.sign
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
  clauses: Vec<Vec<i32>>,
  name_to_id: HashMap<TName, i32>,
  id_to_name: HashMap<i32, TName>,
}

impl<TName: Clone + Eq + Hash> SATSolver<TName> {
  pub fn new() -> SATSolver<TName> {
    SATSolver {
      num_variables: 0,
      clauses: vec![],
      name_to_id: HashMap::new(),
      id_to_name: HashMap::new(),
    }
  }

  pub fn solve(&self) -> Option<HashMap<TName, bool>> {
    match SATSolverCore::solve(&self.clauses) {
      Some(res) => Some(
        res
          .iter()
          .map(|e| (self.id_to_name.get(&e.abs()).unwrap().clone(), *e > 0))
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

    self
      .name_to_id
      .insert(variable.name.clone(), self.num_variables as i32 + 1);
    self
      .id_to_name
      .insert(self.num_variables as i32 + 1, variable.name.clone());
    self.num_variables += 1;
  }

  pub fn add_clause(&mut self, clause: &[&Variable<TName>]) {
    let clause = clause
      .iter()
      .map(|literal| {
        (if literal.sign { 1 } else { -1 }) * self.name_to_id.get(&literal.name).unwrap().clone()
      })
      .collect();

    self.clauses.push(clause);
  }
}
