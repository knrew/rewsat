use std::{error::Error, fmt, path::Path};

use crate::{dpll, utilities};

pub fn solve_dimacs<P: AsRef<Path>>(dimacs_file: P) -> Result<Option<dpll::Model>, Box<dyn Error>> {
  let (_, _, clauses) = parse_dimacs(dimacs_file)?;
  Ok(dpll::DPLL::solve(&clauses))
}

// return: (num_variables, num_clauses,  clauses)
fn parse_dimacs<P: AsRef<Path>>(
  dimacs_file: P,
) -> Result<(usize, usize, Vec<dpll::Clause>), Box<dyn Error>> {
  let mut has_read_header = false;
  let mut num_variables = 0;
  let mut num_clauses = 0;
  let mut clauses = vec![];

  let lines = utilities::read_file(dimacs_file)?;

  for line in lines.iter() {
    let words: Vec<_> = line
      .split_whitespace()
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .collect();

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

        clause.push(words[i].parse()?);
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

  Ok((num_variables, num_clauses, clauses))
}

#[derive(Clone, Debug)]
pub struct DimacsParseError;

impl fmt::Display for DimacsParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for DimacsParseError {}
