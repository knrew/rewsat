use std::collections::HashSet;

pub type Variable = i32;
pub type Clause = Vec<Variable>;
pub type Model = HashSet<Variable>;

#[derive(Debug)]
pub struct DPLL;

impl DPLL {
  pub fn solve(clauses: &[Clause]) -> Option<Model> {
    if clauses
      .iter()
      .any(|clause| clause.iter().any(|literal| *literal == 0))
    {
      return None;
    }

    let num_variables = clauses
      .iter()
      .map(|clause| clause.iter().max_by(|a, b| a.abs().cmp(&b.abs())).unwrap())
      .max()
      .unwrap()
      .to_owned() as usize;

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

  let variable = select_variable(num_variables, &model).unwrap();

  for sign in [-1, 1] {
    let mut clauses = clauses.clone();
    clauses.push(vec![sign * variable]);

    if let Some(m) = solve_impl(num_variables, &clauses, &model) {
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

    if model.len() == previous_model_size {
      break;
    }

    clauses.retain(|clause| !clause.iter().any(|literal| model.contains(&literal)));

    clauses
      .iter_mut()
      .for_each(|clause| clause.retain(|literal| !model.contains(&-literal)));
  }

  (clauses, model)
}

fn select_variable(num_variables: usize, model: &Model) -> Option<Variable> {
  (1..=num_variables as Variable).find(|n| !model.contains(&n) && !model.contains(&-n))
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
