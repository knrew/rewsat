use std::collections::HashSet;

pub fn solve(num_variables: usize, clauses: &[Vec<i32>]) -> Option<HashSet<i32>> {
  // let mut clauses = Vec::from(clauses);
  // let mut model = HashSet::new();
  // if !simplify(&mut clauses, &mut model) {
  //   return None;
  // }
  // if solve_with_recursive(num_variables, &clauses, &mut model) {
  //   Some(model)
  // } else {
  //   None
  // }

  solve_norecursive(num_variables, &clauses)
}

#[allow(dead_code)]
fn solve_norecursive(num_variables: usize, clauses: &[Vec<i32>]) -> Option<HashSet<i32>> {
  let mut stack = Vec::from(vec![(HashSet::new(), Vec::from(clauses))]);

  while !stack.is_empty() {
    let (mut model, mut clauses) = stack.pop().unwrap();

    if !simplify(&mut clauses, &mut model) {
      continue;
    }

    if exists_constant_false_clauses(&model, &clauses) {
      continue;
    }

    if model.len() == num_variables {
      return Some(model);
    }

    let variable = select_variable(num_variables, &model);

    for sign in [-1, 1] {
      let mut model = model.clone();
      model.insert(sign * variable);
      stack.push((model, clauses.clone()));
    }
  }

  None
}

#[allow(dead_code)]
fn solve_recursive(num_variables: usize, clauses: &[Vec<i32>], model: &mut HashSet<i32>) -> bool {
  for clause in clauses.iter() {
    if exists_constant_false_clause(&model, &clause) {
      return false;
    }
  }

  if model.len() == num_variables {
    return true;
  }

  let variable = select_variable(num_variables, &model);

  for sign in [-1, 1] {
    model.insert(sign * variable);

    let result = solve_recursive(num_variables, clauses, model);

    if result {
      return true;
    }

    model.remove(&(sign * variable));
  }

  false
}

fn simplify(clauses: &mut Vec<Vec<i32>>, model: &mut HashSet<i32>) -> bool {
  loop {
    let model_size = model.len();

    for clause in clauses.iter() {
      if clause.len() == 1 {
        if model.contains(&-clause[0]) {
          return false;
        }
        model.insert(clause[0]);
      }
    }

    if model.len() == model_size {
      break;
    }

    clauses.retain(|clause| {
      for literal in clause.iter() {
        if model.contains(&literal) {
          return false;
        }
      }
      true
    });

    clauses
      .iter_mut()
      .for_each(|clause| clause.retain(|literal| !model.contains(&-literal)));

    if clauses.iter().any(|clause| clause.is_empty()) {
      return false;
    }
  }

  true
}

fn exists_constant_false_clause(model: &HashSet<i32>, clause: &[i32]) -> bool {
  for literal in clause.iter() {
    if !model.contains(&literal) && !model.contains(&-literal) {
      return false;
    }

    if model.contains(literal) {
      return false;
    }
  }

  true
}

fn exists_constant_false_clauses(model: &HashSet<i32>, clauses: &[Vec<i32>]) -> bool {
  for clause in clauses.iter() {
    if exists_constant_false_clause(&model, &clause) {
      return true;
    }
  }
  false
}

fn select_variable(num_variables: usize, model: &HashSet<i32>) -> i32 {
  for n in 1..=num_variables as i32 {
    if !model.contains(&n) && !model.contains(&-n) {
      return n;
    }
  }
  unreachable!()
}

#[test]
fn test_simplify1() {
  let mut model = HashSet::new();
  let mut clauses = vec![vec![1], vec![1, 2]];
  let result = simplify(&mut clauses, &mut model);
  assert!(result);
  assert!(clauses.is_empty());
  assert!(model.contains(&1));
  assert!(!model.contains(&2) && !model.contains(&-2));
}

#[test]
fn test_simplify2() {
  let mut clauses = vec![vec![1], vec![-1]];
  let res = simplify(&mut clauses, &mut HashSet::new());
  assert!(!res);
}
