use std::{env, error::Error, fmt, path::PathBuf};

use rewsat::{self, dimacs};

fn main() -> Result<(), Box<dyn Error>> {
  // println!("dimacs solver.");

  let args = env::args().collect::<Vec<_>>();

  if args.len() < 2 {
    return Err(Box::new(NotEnoughArgumentsError));
  }

  let dimacs_file = PathBuf::from(&args[1]).canonicalize()?;
  // println!("dimacs file: {}", dimacs_file.to_string_lossy());

  let mut dimacs = dimacs::DIMACS::from(dimacs_file)?;

  if let Some(solution) = dimacs.solve() {
    println!("SAT");
    solution.iter().for_each(|e| print!("{} ", e));
    println!("");
  } else {
    println!("UNSAT");
  }

  Ok(())
}

#[derive(Clone, Debug)]
struct NotEnoughArgumentsError;

impl fmt::Display for NotEnoughArgumentsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for NotEnoughArgumentsError {}
