use rewsat::{
  dpll::Dpll,
  expressions::{clause::Clause, literal::Literal},
};

fn main() {
  let x = [
    Literal::new(0, true),
    Literal::new(1, true),
    Literal::new(2, true),
    Literal::new(3, true),
    Literal::new(4, true),
  ];

  let clauses = [
    Clause::from(&vec![x[0], !x[1]]),
    Clause::from(&vec![x[0], x[2], !x[3]]),
    Clause::from(&vec![!x[2], !x[4]]),
    Clause::from(&vec![!x[2], x[4]]),
    Clause::from(&vec![x[2], x[3]]),
  ];

  let mut solver = Dpll::new();

  if let Some(model) = solver.solve(5, &clauses) {
    println!("SAT");
    for i in 0..5 {
      println!("{}: {}", i, model.sign(i).unwrap());
    }
  } else {
    println!("UNSAT");
  }
}
