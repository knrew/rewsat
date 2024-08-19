use std::{collections::HashMap, hash::Hash, ops::Not};

use crate::dpll;

#[derive(Clone, Copy)]
pub struct Variable {
  id: usize,
  sign: bool,
}

impl Not for Variable {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self {
      id: self.id,
      sign: !self.sign,
    }
  }
}

#[derive(Debug)]
pub struct SATSolver<T> {
  name_to_id: HashMap<T, usize>,
  id_to_name: HashMap<usize, T>,
  num_variables: usize,
  cnf: dpll::CNF,
  model: HashMap<usize, bool>,
}

impl<T: Clone + Eq + Hash> SATSolver<T> {
  pub fn new() -> Self {
    Self {
      num_variables: 0,
      cnf: dpll::CNF::new(),
      name_to_id: HashMap::new(),
      id_to_name: HashMap::new(),
      model: HashMap::new(),
    }
  }

  pub fn solve(&mut self) -> bool {
    if let Some(model) = dpll::solve(self.num_variables, &self.cnf) {
      self.model = model;
      true
    } else {
      false
    }
  }

  pub fn get_model_value(&self, variable: &Variable) -> Option<bool> {
    self.get_model_value_from_id(&variable.id)
  }

  pub fn get_model_value_from_name(&self, name: &T) -> Option<bool> {
    let id = self.name_to_id[name];
    self.get_model_value_from_id(&id)
  }

  fn get_model_value_from_id(&self, id: &usize) -> Option<bool> {
    if let Some(v) = self.model.get(&id) {
      Some(*v)
    } else {
      None
    }
  }

  pub fn get_variable_name(&self, variable: &Variable) -> &T {
    &self.id_to_name[&variable.id]
  }

  pub fn variable(&mut self, name: T) -> Variable {
    if !self.name_to_id.contains_key(&name) {
      self.name_to_id.insert(name.clone(), self.num_variables);
      self.id_to_name.insert(self.num_variables, name.clone());
      self.num_variables += 1;
    }

    Variable {
      id: self.name_to_id[&name],
      sign: true,
    }
  }

  pub fn add_clause(&mut self, clause: &[Variable]) {
    if clause.is_empty() {
      return;
    }
    let clause = clause.iter().map(|v| (v.id, v.sign)).collect();
    self.cnf.push(clause);
  }

  pub fn add_clauses(&mut self, clauses: &[Vec<Variable>]) {
    for clause in clauses {
      self.add_clause(clause);
    }
  }
}
