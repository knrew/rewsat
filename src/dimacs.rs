use std::{error::Error, fmt, path::Path};

use crate::{io, sat_solver::SATSolver};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DIMACS {
  num_variables: usize,
  num_clauses: usize,
  clauses: Vec<Vec<(usize, bool)>>,
}

#[allow(dead_code)]
impl DIMACS {
  pub fn new() -> Self {
    Self {
      num_variables: 0,
      num_clauses: 0,
      clauses: vec![],
    }
  }

  /// parse dimacs file
  pub fn from<P: AsRef<Path>>(dimacs_file: P) -> Result<DIMACS, Box<dyn Error>> {
    let mut has_read_header = false;
    let mut num_variables = 0;
    let mut num_clauses = 0;
    let mut clauses = vec![];

    for line in &io::read_file(dimacs_file)? {
      let words = line
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

      if has_read_header {
        if words[words.len() - 1] != "0" {
          return Err(Box::new(DimacsParseError));
        }

        let mut clause = vec![];

        for i in 0..words.len() {
          if words[i] == "0" {
            if i == words.len() - 1 {
              break;
            } else {
              return Err(Box::new(DimacsParseError));
            }
          }

          let num = words[i].parse::<i64>()?;
          clause.push(if num < 0 {
            (num.abs() as usize, false)
          } else {
            (num as usize, true)
          });
        }

        clauses.push(clause);
      } else {
        if words[0].chars().nth(0).unwrap() == 'c' {
          continue;
        }

        if words.len() == 4 && words[0] == "p" && words[1] == "cnf" {
          num_variables = words[2].parse()?;
          num_clauses = words[3].parse()?;
          has_read_header = true;
        } else {
          return Err(Box::new(DimacsParseError));
        }
      }
    }

    if !has_read_header || num_clauses != clauses.len() {
      return Err(Box::new(DimacsParseError));
    }

    Ok(DIMACS {
      num_variables,
      num_clauses,
      clauses,
    })
  }

  pub fn solve(&mut self) -> Option<Vec<i64>> {
    let mut solver = SATSolver::new();
    for i in 1..=self.num_variables {
      let _ = solver.variable(i);
    }

    for clause in &self.clauses {
      let clause = clause
        .iter()
        .map(|&(i, sign)| {
          if sign {
            solver.variable(i)
          } else {
            !solver.variable(i)
          }
        })
        .collect::<Vec<_>>();
      solver.add_clause(&clause);
    }

    if !solver.solve() {
      return None;
    }

    let mut solution = (1..=self.num_variables)
      .map(|i| {
        if solver.get_model_value_from_name(&i).unwrap() {
          i as i64
        } else {
          -(i as i64)
        }
      })
      .collect::<Vec<_>>();

    solution.push(0);

    Some(solution)
  }
}

#[derive(Clone, Debug)]
pub struct DimacsParseError;

impl fmt::Display for DimacsParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for DimacsParseError {}
