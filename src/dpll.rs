use std::collections::HashSet;

pub type Variable = i32;
pub type Clause = Vec<Variable>;
pub type Model = HashSet<Variable>;

#[derive(Debug)]
pub struct DPLL;

impl DPLL {
  pub fn solve(num_variables: usize, clauses: &[Clause]) -> Option<Model> {
    assert!(!clauses
      .iter()
      .any(|clause| clause.iter().any(|literal| *literal == 0)));

    match solve_recursive(num_variables, &clauses, &Model::new()) {
      // match solve_norecursive(num_variables, &clauses, &Model::new()) {
      Some(model) => Some(
        (1..=num_variables as Variable)
          .map(|n| if model.contains(&-n) { -n } else { n })
          .collect(),
      ),
      None => None,
    }
  }
}

#[allow(dead_code)]
fn solve_norecursive(_num_variables: usize, clauses: &[Clause], model: &Model) -> Option<Model> {
  let mut stack = vec![(Vec::from(clauses), model.clone())];

  while !stack.is_empty() {
    let (clauses, model) = stack.pop().unwrap();

    let (clauses, model) = simplify(&clauses, &model);

    if has_empty_clause(&clauses) {
      continue;
    }

    if clauses.is_empty() {
      return Some(model);
    }

    let variable = select_variable(&clauses);

    for sign in [-1, 1] {
      let mut model = model.clone();
      model.insert(sign * variable);
      stack.push((clauses.clone(), model));
    }
  }

  None
}

#[allow(dead_code)]
fn solve_recursive(num_variables: usize, clauses: &[Clause], model: &Model) -> Option<Model> {
  let (clauses, model) = simplify(&clauses, &model);

  if has_empty_clause(&clauses) {
    return None;
  }

  if clauses.is_empty() {
    return Some(model);
  }

  let variable = select_variable(&clauses);

  for sign in [-1, 1] {
    let mut model = model.clone();
    model.insert(sign * variable);

    if let Some(m) = solve_recursive(num_variables, &clauses, &model) {
      return Some(m);
    }
  }

  None
}

// return: (clauses, model)
fn simplify(clauses: &[Clause], model: &Model) -> (Vec<Clause>, Model) {
  let mut clauses = Vec::from(clauses);
  let mut model = model.clone();

  while !has_empty_clause(&clauses) {
    let previous_model_size = model.len();

    for clause in clauses.iter() {
      if clause.len() == 1 && !model.contains(&-clause[0]) {
        model.insert(clause[0]);
      }
    }

    clauses.retain(|clause| !clause.iter().any(|literal| model.contains(&literal)));

    clauses
      .iter_mut()
      .for_each(|clause| clause.retain(|literal| !model.contains(&-literal)));

    if model.len() == previous_model_size {
      break;
    }
  }

  // clauses.sort_by(|a, b| a.len().cmp(&b.len()));

  (clauses, model)
}

fn select_variable(clauses: &[Clause]) -> Variable {
  assert!(!clauses.is_empty() && !clauses[0].is_empty());
  clauses[0][0].abs()
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
