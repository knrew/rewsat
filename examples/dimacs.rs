use std::{env, error::Error, fmt, path::PathBuf};

use rewsat::dimacs::solve_dimacs;

fn main() -> Result<(), Box<dyn Error>> {
  println!("dimacs solver.");

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    return Err(Box::new(NotEnoughArgumentsError));
  }

  let dimacs_file = PathBuf::from(&args[1]).canonicalize()?;

  println!("dimacs file: {}", dimacs_file.to_string_lossy());

  match solve_dimacs(&dimacs_file)? {
    Some(model) => {
      println!("SAT");
      let mut model = Vec::from_iter(model.iter());
      model.sort_by(|a, b| a.abs().cmp(&b.abs()));
      model.iter().for_each(|l| print!("{} ", l));
      println!("");
    }
    None => println!("UNSAT"),
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
