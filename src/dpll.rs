use std::collections::HashSet;

#[derive(Debug)]
pub struct DPLL;

type Clause = Vec<i32>;
type Model = HashSet<i32>;

impl DPLL {
  pub fn solve(num_variables: usize, clauses: &[Clause]) -> Option<Model> {
    solve_impl(num_variables, &clauses, &Model::new())
  }
}

fn solve_impl(num_variables: usize, clauses: &[Clause], model: &Model) -> Option<Model> {
  let (clauses, model) = simplify(&clauses, &model);

  if has_empty_clause(&clauses) {
    return None;
  }

  if clauses.is_empty() && model.len() == num_variables {
    return Some(model.clone());
  }

  let variable = select_variable(num_variables, &model);

  for sign in [-1, 1] {
    let mut clauses = clauses.clone();
    clauses.push(vec![sign * variable]);
    match solve_impl(num_variables, &clauses, &model) {
      Some(m) => return Some(m),
      None => {}
    }
  }

  None
}

// return: (clauses, model)
fn simplify(clauses: &[Clause], model: &Model) -> (Vec<Clause>, Model) {
  let mut clauses = Vec::from(clauses);
  let mut model = model.clone();

  while !has_empty_clause(&clauses) {
    let model_size = model.len();

    for clause in clauses.iter() {
      if clause.len() == 1 {
        if !model.contains(&-clause[0]) {
          model.insert(clause[0]);
        }
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
  }

  (clauses, model)
}

fn select_variable(num_variables: usize, model: &Model) -> i32 {
  for n in 1..=num_variables as i32 {
    if !model.contains(&n) && !model.contains(&-n) {
      return n;
    }
  }
  unreachable!()
}

fn has_empty_clause(clauses: &[Clause]) -> bool {
  clauses.iter().any(|clause| clause.is_empty())
}

#[test]
fn test_simplify1() {
  let clauses = vec![vec![1], vec![1, 2]];
  let (clauses, model) = simplify(&clauses, &HashSet::new());
  assert!(model.contains(&1));
  assert!(clauses.is_empty());
}
