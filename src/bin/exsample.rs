use std::error::Error;

use rewsat::sat_solver::{SATSolver, Variable};

fn main() -> Result<(), Box<dyn Error>> {
  let mut solver = SATSolver::new();

  let a = Variable::new(&"a");
  let b = Variable::new(&"b");
  let c = Variable::new(&"c");
  let d = Variable::new(&"d");
  let e = Variable::new(&"e");

  let variables = [&a, &b, &c, &d, &e];

  variables.iter().for_each(|v| solver.add_variable(v));

  solver.add_clause(&[&a, &b.not()]);
  solver.add_clause(&[&a, &c, &d.not()]);
  solver.add_clause(&[&c.not(), &e.not()]);
  solver.add_clause(&[&c.not(), &e.not()]);
  solver.add_clause(&[&c, &d]);

  match solver.solve() {
    Some(model) => {
      println!("SAT");
      variables
        .iter()
        .for_each(|v| println!("{}: {}", v.name, model.get(&v.name).unwrap()));
    }
    None => {
      println!("UNSAT");
    }
  }

  Ok(())
}
