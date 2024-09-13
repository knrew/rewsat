use std::path::PathBuf;

use clap::{arg, command, value_parser};

use rewsat::dimacs;

fn main() {
  let matches = command!()
    .about("dimacs solver")
    .arg(
      arg!([dimacs_file]  "dimacs file")
        .value_parser(value_parser!(PathBuf))
        .required(true),
    )
    .get_matches();

  let dimacs_file = matches.get_one::<PathBuf>("dimacs_file").unwrap();
  let dimacs_file = dimacs_file
    .canonicalize()
    .unwrap_or_else(|_| panic!("not found: {:?}", dimacs_file));

  let mut dimacs = dimacs::Dimacs::from(&dimacs_file)
    .unwrap_or_else(|_| panic!("failed to parse dimacs file: {:?}", dimacs_file));

  if let Some(solution) = dimacs.solve() {
    println!("SAT");
    solution.iter().for_each(|e| print!("{} ", e));
    println!("");
  } else {
    println!("UNSAT");
  }
}
