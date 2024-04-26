use std::{
  env,
  error::Error,
  fmt,
  path::{Path, PathBuf},
};

use rewsat::sat_solver;

fn main() -> Result<(), Box<dyn Error>> {
  println!("sudoku solver.");

  let args: Vec<_> = env::args().collect();

  if args.len() < 2 {
    return Err(Box::new(NotEnoughArgumentsError));
  }

  let sudoku_file = PathBuf::from(&args[1]).canonicalize()?;
  println!("sudoku file: {}", sudoku_file.to_string_lossy());

  let problem = parse_sudoku(&sudoku_file)?;
  println!("problem:");
  print_sudoku(&problem);

  println!("solving...");
  match solve_sudoku(&problem) {
    Some(answer) => {
      println!("SOLVED");
      print_sudoku(&answer);
    }
    None => println!("UNSOLVABLE"),
  }

  Ok(())
}

type Sudoku = Vec<Vec<u8>>;

// 4x4 or 9x9
fn solve_sudoku(problem: &Sudoku) -> Option<Sudoku> {
  assert!(problem.len() == 4 || problem.len() == 9);

  type SATSolver = sat_solver::SATSolver<(u8, u8, u8)>;
  type Variable = sat_solver::Variable<(u8, u8, u8)>;

  // variablesのうちどれかひとつのvariableだけがtrueであるような制約を追加する
  fn add_only_one_constraints(solver: &mut SATSolver, variables: &[Variable]) {
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

  let sudoku_size = problem.len() as u8;

  let mut solver = SATSolver::new();

  // solverに変数を設定(x001-x889)
  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      for c in 0..sudoku_size {
        solver.add_variable(&Variable::new(&(r, c, n)));
      }
    }
  }

  // 制約「各マスには数字1-9のいずれかが入る」を追加
  for r in 0..sudoku_size {
    for c in 0..sudoku_size {
      let variables: Vec<_> = (1..=sudoku_size)
        .map(|n| Variable::new(&(r, c, n)))
        .collect();
      add_only_one_constraints(&mut solver, &variables);
    }
  }

  // 制約「各行には1-9が1個ずつ入る」を追加
  for n in 1..=sudoku_size {
    for r in 0..sudoku_size {
      let variables: Vec<_> = (0..sudoku_size)
        .map(|c| Variable::new(&(r, c, n)))
        .collect();
      add_only_one_constraints(&mut solver, &variables);
    }
  }

  // 制約「各列には1-9が1個ずつ入る」を追加
  for n in 1..=sudoku_size {
    for c in 0..sudoku_size {
      let variables: Vec<_> = (0..sudoku_size)
        .map(|r| Variable::new(&(r, c, n)))
        .collect();
      add_only_one_constraints(&mut solver, &variables);
    }
  }

  // 制約「各ブロックには1-9が1個ずつ入る」を追加
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
          add_only_one_constraints(&mut solver, &variables);
        }
      }
    }
  }

  // 問題で与えられているマスを制約として追加
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

  let answer = (0..sudoku_size)
    .map(|r| {
      (0..sudoku_size)
        .map(|c| {
          (1..=sudoku_size)
            .find(|n| *model.get(&(r, c, *n)).unwrap())
            .unwrap()
        })
        .collect()
    })
    .collect();

  Some(answer)
}

fn parse_sudoku<P: AsRef<Path>>(sudoku_file: P) -> Result<Sudoku, Box<dyn Error>> {
  use rewsat::utilities;

  let lines = utilities::read_file(sudoku_file)?;

  let sudoku_size = lines.len();

  if sudoku_size != 4 && sudoku_size != 9 {
    return Err(Box::new(ParseSudokuError));
  }

  let mut sudoku = vec![vec![0; sudoku_size]; sudoku_size];

  for (r, line) in lines.iter().enumerate() {
    if line.len() != sudoku_size {
      return Err(Box::new(ParseSudokuError));
    }

    for c in 0..sudoku_size {
      sudoku[r][c] = match line.chars().nth(c).unwrap().to_digit(10) {
        Some(n) => n as u8,
        None => return Err(Box::new(ParseSudokuError)),
      };
    }
  }

  Ok(sudoku)
}

fn print_sudoku(sudoku: &Sudoku) {
  sudoku.iter().for_each(|line| {
    line.iter().for_each(|n| print!("{}", n));
    println!("");
  });
}

#[derive(Clone, Debug)]
struct NotEnoughArgumentsError;

impl fmt::Display for NotEnoughArgumentsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for NotEnoughArgumentsError {}

#[derive(Clone, Debug)]
struct ParseSudokuError;

impl fmt::Display for ParseSudokuError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for ParseSudokuError {}
