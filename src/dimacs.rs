use std::{collections::HashSet, error::Error, fmt, path::Path};

use crate::{sat_solver_core, utilities};

pub fn solve_dimacs<P: AsRef<Path>>(
  dimacs_file: P,
) -> Result<Option<HashSet<i32>>, Box<dyn Error>> {
  let (num_variables, clauses) = parse_dimacs(dimacs_file)?;
  Ok(sat_solver_core::solve(num_variables, &clauses))
}

// return: (num_variables, clauses)
fn parse_dimacs<P: AsRef<Path>>(dimacs_file: P) -> Result<(usize, Vec<Vec<i32>>), Box<dyn Error>> {
  let mut has_set_num_variables = false;
  let mut num_variables = 0;
  let mut num_clauses = 0;
  let mut clauses = vec![];

  let lines = utilities::read_file(dimacs_file)?;
  
    for line in lines.iter() {
    let words = line
      .split_whitespace()
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .collect::<Vec<_>>();

    if has_set_num_variables {
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

        clause.push(words[i].parse()?);
      }

      clauses.push(clause);
    } else {
      if words[0] == "c" {
        continue;
      }

      if words.len() == 4 && words[0] == "p" && words[1] == "cnf" {
        num_variables = words[2].parse()?;

        num_clauses = words[3].parse()?;

        has_set_num_variables = true;
      }
    }
  }

  if num_clauses != clauses.len() {
    return Err(Box::new(DimacsParseError));
  }

  Ok((num_variables, clauses))
}

#[derive(Clone, Debug)]
pub struct DimacsParseError;

impl fmt::Display for DimacsParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for DimacsParseError {}
