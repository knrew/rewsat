use std::{collections::HashMap, hash::Hash, ops::Not};

use crate::{
  dpll::DPLL,
  expressions::{clause::Clause, literal::Literal, model::Model},
};

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

pub struct SATSolver<T> {
  name_to_id: HashMap<T, usize>,
  id_to_name: HashMap<usize, T>,
  num_variables: usize,
  clauses: Vec<Clause>,
  model: Model,
}

impl<T: Clone + Eq + Hash> SATSolver<T> {
  pub fn new() -> Self {
    Self {
      num_variables: 0,
      clauses: vec![],
      name_to_id: HashMap::new(),
      id_to_name: HashMap::new(),
      model: Model::new(0),
    }
  }

  pub fn solve(&mut self) -> bool {
    let solver = DPLL::new();
    if let Some(model) = solver.solve(self.num_variables, &self.clauses) {
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
    if let Some(id) = self.name_to_id.get(name) {
      self.get_model_value_from_id(&id)
    } else {
      None
    }
  }

  fn get_model_value_from_id(&self, id: &usize) -> Option<bool> {
    self.model.sign(*id)
  }

  pub fn get_variable_name(&self, variable: &Variable) -> Option<&T> {
    if let Some(name) = self.id_to_name.get(&variable.id) {
      Some(name)
    } else {
      None
    }
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
    let clause = Clause::from(
      &clause
        .iter()
        .map(|v| Literal::new(v.id, v.sign))
        .collect::<Vec<_>>(),
    );
    self.clauses.push(clause);
  }

  pub fn add_clauses(&mut self, clauses: &[Vec<Variable>]) {
    for clause in clauses {
      self.add_clause(clause);
    }
  }
}
