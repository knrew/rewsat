use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct DIMACS {
  pub num_variables: usize,
  pub clauses: Vec<Vec<i32>>,
}

impl DIMACS {
  pub fn parse_dimacs_file(file_name: &str) -> Result<DIMACS, &'static str> {
    let mut has_set_num_variables = false;
    let mut num_variables = 0usize;
    let mut num_clauses = 0usize;

    let mut clauses: Vec<Vec<i32>> = vec![];

    let strs = match read_dimacs_file(file_name) {
      Ok(result) => result,
      Err(_) => return Err("ParseError(cannot read file)"),
    };

    for str in strs.iter() {
      let words = str
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

      if has_set_num_variables {
        if words[words.len() - 1] != "0" {
          return Err("ParseError(invalid dimacs file)");
        }

        let mut clause: Vec<i32> = vec![];

        for i in 0..words.len() {
          if words[i] == "0" {
            if i == words.len() - 1 {
              break;
            } else {
              return Err("ParseError(invalid dimacs file)");
            }
          }

          match words[i].parse() {
            Ok(n) => clause.push(n),
            Err(_) => return Err("PaserError(invalid dimacs file)"),
          }
        }

        clauses.push(clause);
      } else {
        if words[0] == "c" {
          continue;
        }

        if words.len() == 4 && words[0] == "p" && words[1] == "cnf" {
          num_variables = match words[2].parse() {
            Ok(result) => result,
            Err(_) => return Err("ParseError(invalid dimacs file)"),
          };

          num_clauses = match words[3].parse() {
            Ok(result) => result,
            Err(_) => return Err("ParseError(invalid dimacs file)"),
          };

          has_set_num_variables = true;
        }
      }
    }

    if num_clauses != clauses.len() {
      return Err("ParseError(invalid dimacs file)");
    }

    Ok(DIMACS {
      num_variables,
      clauses,
    })
  }
}

fn read_dimacs_file(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let f = fs::File::open(file_name)?;

  let reader = BufReader::new(f);

  let mut result: Vec<String> = vec![];

  for l in reader.lines() {
    result.push(l?.trim().to_string());
  }

  Ok(result)
}
