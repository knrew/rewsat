use std::{
  env,
  error::Error,
  fmt::Display,
  path::{Path, PathBuf},
};

use rewsat::{sat_solver, utilities};

fn main() {
  println!("dimacs solver.");

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Error: not enough arguments.");
    return;
  }

  let sudoku_file = match PathBuf::from(&args[1]).canonicalize() {
    Ok(res) => res,
    Err(e) => {
      eprintln!("Error: {}", e);
      return;
    }
  };

  println!("sudoku file: {}", sudoku_file.to_string_lossy());

  let problem = match parse_sudoku(&sudoku_file) {
    Ok(res) => res,
    Err(e) => {
      eprintln!("Error: {}", e);
      return;
    }
  };

  println!("problem:");
  print_sudoku(&problem);

  println!("solving...");
  match solve_sudoku(&problem) {
    Some(res) => {
      println!("SOLVED");
      print_sudoku(&res);
    }
    None => println!("UNSOLVED"),
  }
}

// 4x4 or 9x9
fn solve_sudoku(problem: &[Vec<u8>]) -> Option<Vec<Vec<u8>>> {
  type SATSolver = sat_solver::SATSolver<(u8, u8, u8)>;
  type Variable = sat_solver::Variable<(u8, u8, u8)>;

  // variablesのうちどれかひとつのvariableだけがtrueであるような制約を追加する
  fn add_only_one_constrains(solver: &mut SATSolver, variables: &[Variable]) {
    // At Most One
    // (!x1 || !x2) && (!x2 || !x3) && ... && (!x8 || !x9)
    for i in 0..variables.len() - 1 {
      for j in i + 1..variables.len() {
        solver.add_clause(&[&variables[i].not(), &variables[j].not()])
      }
    }

    // At Least One
    // x1 || x2 || ... || x9
    solver.add_clause(&variables.iter().collect::<Vec<&Variable>>());
  }

  assert!(problem.len() == 4 || problem.len() == 9);
  let sudoku_size = problem.len() as u8;

  let mut solver = SATSolver::new();

  // solverに変数を追加(x001-x889)
  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      for c in 0..sudoku_size {
        solver.add_variable(&Variable::new(&(r, c, n)));
      }
    }
  }

  // 各マスには数字1-9のいずれかが入る
  for r in 0..sudoku_size {
    for c in 0..sudoku_size {
      let mut variables = vec![];
      for n in 1..=sudoku_size {
        variables.push(Variable::new(&(r, c, n)))
      }
      add_only_one_constrains(&mut solver, &variables);
    }
  }

  // 各行には1-9が1個ずつ入る
  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      let mut variables = vec![];
      for c in 0..sudoku_size {
        variables.push(Variable::new(&(r, c, n)));
      }
      add_only_one_constrains(&mut solver, &variables);
    }
  }

  // 各列には1-9が1個ずつ入る
  for n in 1..=sudoku_size {
    for c in 0..sudoku_size {
      let mut variables = vec![];
      for r in 0..sudoku_size {
        variables.push(Variable::new(&(r, c, n)));
      }
      add_only_one_constrains(&mut solver, &variables);
    }
  }

  // 各ブロックには1-9が1個ずつ入る
  {
    let block_size = if sudoku_size == 4 { 2 } else { 3 };
    for n in 1..=sudoku_size {
      for block_r in 0..block_size {
        for block_c in 0..block_size {
          let mut variables = vec![];
          for r in 0..block_size {
            for c in 0..block_size {
              variables.push(Variable::new(&(
                block_size * block_r + r,
                block_size * block_c + c,
                n,
              )));
            }
          }
          add_only_one_constrains(&mut solver, &variables);
        }
      }
    }
  }

  // 既に埋まっている数字を制約として追加
  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      for c in 0..sudoku_size {
        if problem[r as usize][c as usize] == 0 {
          continue;
        }
        if problem[r as usize][c as usize] == n {
          solver.add_clause(&[&Variable::new(&(r, c, n))]);
        } else {
          solver.add_clause(&[&Variable::new(&(r, c, n)).not()]);
        }
      }
    }
  }

  let model = match solver.solve() {
    Some(res) => res,
    None => return None,
  };

  let mut answer = Vec::from(problem);

  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      for c in 0..sudoku_size {
        if *model.get(&(r, c, n)).unwrap() {
          answer[r as usize][c as usize] = n;
        }
      }
    }
  }

  Some(answer)
}

fn parse_sudoku<P: AsRef<Path>>(sudoku_file: P) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
  let lines = utilities::read_file(sudoku_file)?;

  let sudoku_size = lines.len();
  assert!(sudoku_size == 4 || sudoku_size == 9);

  let mut sudoku = vec![vec![0; sudoku_size]; sudoku_size];

  for (r, line) in lines.iter().enumerate() {
    assert_eq!(line.len(), sudoku_size);
    for c in 0..sudoku_size {
      sudoku[r][c] = line.chars().nth(c).unwrap().to_digit(10).unwrap() as u8;
    }
  }

  Ok(sudoku)
}

fn print_sudoku<T: Display>(sudoku: &[Vec<T>]) {
  sudoku.iter().for_each(|line| {
    line.iter().for_each(|n| print!("{}", n));
    println!("");
  });
}
