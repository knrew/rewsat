use std::collections::HashSet;

pub fn solve(num_variables: usize, clauses: &Vec<Vec<i32>>) -> Option<HashSet<i32>> {
  let num_variables = num_variables;
  let mut clauses = clauses.clone();

  let mut model = HashSet::new();
  let mut unassigned_variables = HashSet::new();

  for i in 1..num_variables as i32 + 1 {
    unassigned_variables.insert(i);
  }

  if !simpify(&mut clauses, &mut model) {
    return None;
  }

  for l in model.iter() {
    unassigned_variables.insert(l.abs());
  }

  if !solve_impl(
    num_variables,
    &clauses,
    &mut model,
    &mut unassigned_variables,
  ) {
    return None;
  }

  Some(model)
}

fn solve_impl(
  num_variables: usize,
  clauses: &Vec<Vec<i32>>,
  model: &mut HashSet<i32>,
  unassigned_variables: &mut HashSet<i32>,
) -> bool {
  for clause in clauses.iter() {
    if exists_constant_false_clause(clause, model) {
      return false;
    }
  }

  if model.len() == num_variables {
    return true;
  }

  let variable = select_variable(unassigned_variables);
  unassigned_variables.remove(&variable);

  for sign in [1, -1] {
    model.insert(sign * variable);

    if solve_impl(num_variables, clauses, model, unassigned_variables) {
      return true;
    }

    model.remove(&(sign * variable));
  }

  unassigned_variables.insert(variable);

  return false;
}

fn simpify(clauses: &mut Vec<Vec<i32>>, model: &mut HashSet<i32>) -> bool {
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

    for (i, clause) in clauses.iter_mut().enumerate() {
      let mut index = 0;
      while index < clause.len() {
        if model.contains(&clause[index]) {
          clause.clear();
          break;
        }

        if model.contains(&-clause[index]) {
          clause.remove(i);
          continue;
        }

        index += 1;
      }
    }
  }

  let mut index = 0;
  while index < clauses.len() {
    if clauses[index].is_empty() {
      clauses.remove(index);
      continue;
    }

    index += 1;
  }

  return true;
}

fn exists_constant_false_clause(clause: &Vec<i32>, model: &HashSet<i32>) -> bool {
  for literal in clause.iter() {
    if !model.contains(literal) && !model.contains(&-literal) {
      return false;
    }

    if model.contains(literal) {
      return false;
    }
  }

  return true;
}

fn select_variable(variables: &HashSet<i32>) -> i32 {
  for n in variables.iter() {
    return *n;
  }
  return 0;
}
