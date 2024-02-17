use std::error::Error;

use rewsat;

fn main() -> Result<(), Box<dyn Error>> {
  let mut solver = rewsat::SATSolver::new();

  let a = rewsat::Variable::new("A");
  let b = rewsat::Variable::new("B");
  let c = rewsat::Variable::new("C");
  let d = rewsat::Variable::new("D");
  let e = rewsat::Variable::new("E");

  let variables = [&a, &b, &c, &d, &e];

  for v in variables.iter() {
    solver.add_variable(v);
  }

  solver.add_clause(&[&a, &b.not()]);
  solver.add_clause(&[&a, &c, &d.not()]);
  solver.add_clause(&[&c.not(), &e.not()]);
  solver.add_clause(&[&c.not(), &e]);
  solver.add_clause(&[&c, &d]);

  let model = match solver.solve() {
    Some(result) => result,
    None => {
      println!("UNSAT");
      return Ok(());
    }
  };

  for v in variables.iter() {
    println!("{}: {}", v.name, model.get(&v.name).unwrap());
  }

  Ok(())
}
