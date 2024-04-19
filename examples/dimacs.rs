use std::{env, path::PathBuf};

use rewsat::dimacs_solver::solve_dimacs;

fn main() {
  println!("dimacs solver.");

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Error: not enough arguments.");
    return;
  }

  let dimacs_file = match PathBuf::from(&args[1]).canonicalize() {
    Ok(res) => res,
    Err(e) => {
      eprintln!("Error: {}", e);
      return;
    }
  };

  println!("dimacs file: {}", dimacs_file.to_string_lossy());

  match solve_dimacs(&dimacs_file) {
    Ok(result) => match result {
      Some(model) => {
        println!("SAT");
        let mut model = Vec::from_iter(model.iter());
        model.sort_by(|a, b| a.abs().cmp(&b.abs()));
        model.iter().for_each(|l| print!("{} ", l));
        println!("");
      }
      None => {
        println!("UNSAT");
      }
    },
    Err(e) => eprintln!("Error: {}", e),
  }
}
