use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

use rewsat;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let config = Config::new(&args)?;
  println!("{:?}", config);

  let dimacs = rewsat::DIMACS::parse_dimacs_file(&config.dimacs_file)?;
  println!("{:?}", dimacs);

  let mut solver = rewsat::SATSolver::new();

  let mut file = if config.enable_output_file {
    Some(fs::File::create(&config.output_file)?)
  } else {
    None
  };

  match solver.solve_dimacs(&dimacs) {
    Some(model) => {
      println!("SAT");
      match file.as_mut() {
        Some(f) => writeln!(f, "SAT")?,
        None => {}
      }

      for i in 1..solver.num_variables as i32 + 1 {
        print!("{} ", if model.contains(&i) { i } else { -i });
        match file.as_mut() {
          Some(f) => write!(f, "{} ", if model.contains(&i) { i } else { -i })?,
          None => {}
        }
      }
      println!("");
      match file.as_mut() {
        Some(f) => writeln!(f, "")?,
        None => {}
      }
    }
    None => {
      println!("UNSAT");
      match file.as_mut() {
        Some(f) => writeln!(f, "UNSAT")?,
        None => {}
      }

      return Ok(());
    }
  };

  Ok(())
}

#[derive(Debug)]
struct Config {
  dimacs_file: String,
  enable_output_file: bool,
  output_file: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    }

    let dimacs_file = args[1].clone();
    let enable_output_file = args.len() > 2;
    let output_file = if enable_output_file {
      args[2].clone()
    } else {
      String::from("")
    };

    Ok(Config {
      dimacs_file,
      enable_output_file,
      output_file,
    })
  }
}
