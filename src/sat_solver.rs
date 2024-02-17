use std::collections::HashMap;
use std::collections::HashSet;

use crate::dimacs;
use crate::sat_solver_core;

#[derive(Debug, Clone)]
pub struct Literal {
  pub name: String,
  pub sign: bool,
}

pub type Variable = Literal;

impl Literal {
  pub fn new(name: &str) -> Literal {
    Literal {
      name: name.to_string(),
      sign: true,
    }
  }

  pub fn not(&self) -> Literal {
    Literal {
      name: self.name.clone(),
      sign: !self.sign,
    }
  }
}

#[derive(Debug)]
pub struct SATSolver {
  pub num_variables: usize,
  pub clauses: Vec<Vec<i32>>,
  pub variable_to_id: HashMap<String, usize>,
  pub id_to_variable: HashMap<usize, String>,
}

impl SATSolver {
  pub fn new() -> SATSolver {
    SATSolver {
      num_variables: 0,
      clauses: vec![],
      variable_to_id: HashMap::new(),
      id_to_variable: HashMap::new(),
    }
  }

  pub fn solve(&self) -> Option<HashMap<String, bool>> {
    let result = match sat_solver_core::solve(self.num_variables, &self.clauses) {
      Some(result) => result,
      None => return None,
    };

    let mut model: HashMap<String, bool> = HashMap::new();

    for e in result.iter() {
      let key = self
        .id_to_variable
        .get(&(e.abs() as usize))
        .unwrap()
        .clone();
      let value = *e > 0;
      model.insert(key, value);
    }

    Some(model)
  }

  pub fn solve_dimacs(&mut self, dimacs: &dimacs::DIMACS) -> Option<HashSet<i32>> {
    self.num_variables = dimacs.num_variables;
    self.clauses = dimacs.clauses.clone();
    sat_solver_core::solve(self.num_variables, &self.clauses)
  }

  pub fn add_variable(&mut self, variable: &Literal) {
    self
      .variable_to_id
      .insert(variable.name.clone(), self.num_variables + 1);
    self
      .id_to_variable
      .insert(self.num_variables + 1, variable.name.clone());
    self.num_variables += 1;
  }

  pub fn add_clause(&mut self, clause: &[&Literal]) -> bool {
    let mut tmp: Vec<i32> = vec![];

    for e in clause.iter() {
      let literal = if e.sign { 1 } else { -1 }
        * match self.variable_to_id.get(&e.name) {
          Some(result) => *result as i32,
          None => return false,
        };

      tmp.push(literal);
    }

    self.clauses.push(tmp);

    true
  }
}
