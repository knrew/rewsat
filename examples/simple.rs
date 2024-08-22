use rewsat::sat_solver::*;

fn main() {
  let mut solver = SATSolver::new();
  let a = solver.variable("a");
  let b = solver.variable("b");
  let c = solver.variable("c");
  let d = solver.variable("d");
  let e = solver.variable("e");
  let variables = [&a, &b, &c, &d, &e];

  // (a || !b) && (a || c || !d) && (!c || !e) && (!c || e) && (c || d)
  solver.add_clause(&[a, !b]);
  solver.add_clause(&[a, c, !d]);
  solver.add_clause(&[!c, !e]);
  solver.add_clause(&[!c, e]);
  solver.add_clause(&[c, d]);

  println!("(a || !b) && (a || c || !d) && (!c || !e) && (!c || e) && (c || d)");

  if solver.solve() {
    println!("SAT");
    for v in &variables {
      println!(
        "{}: {}",
        solver.get_variable_name(v).unwrap(),
        solver.get_model_value(&v).unwrap()
      );
    }
  } else {
    println!("UNSAT");
  }
}
