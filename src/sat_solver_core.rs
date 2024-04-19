use std::collections::HashSet;

pub fn solve(num_variables: usize, clauses: &[Vec<i32>]) -> Option<HashSet<i32>> {
  let mut clauses = Vec::from(clauses);
  let mut model = HashSet::new();

  let mut unassigned_variables = HashSet::from_iter(1..=num_variables as i32);

  // if !simpify(&mut clauses, &mut model) {
  //   return None;
  // }

  // model.iter().for_each(|literal| {
  //   unassigned_variables.remove(&literal.abs());
  // });

  let result = solve_impl(
    num_variables,
    &clauses,
    &mut model,
    &mut unassigned_variables,
  );

  if result {
    Some(model)
  } else {
    None
  }
}

fn solve_impl(
  num_variables: usize,
  clauses: &[Vec<i32>],
  model: &mut HashSet<i32>,
  unassigned_variables: &mut HashSet<i32>,
) -> bool {
  for clause in clauses.iter() {
    if exists_constant_false_clause(&model, &clause) {
      return false;
    }
  }

  if model.len() == num_variables {
    return true;
  }

  assert!(!unassigned_variables.is_empty());
  let variable = unassigned_variables.iter().nth(0).unwrap().clone();
  unassigned_variables.remove(&variable);

  for sign in [-1, 1] {
    model.insert(sign * variable);

    let result = solve_impl(num_variables, clauses, model, unassigned_variables);

    if result {
      return true;
    }

    model.remove(&(sign * variable));
  }

  unassigned_variables.insert(variable);

  false
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

    if model_size == model.len() {
      break;
    }

    for (clause_index, clause) in clauses.iter_mut().enumerate() {
      let mut literal_index = 0;
      while literal_index < clause.len() {
        if model.contains(&clause[literal_index]) {
          clause.clear();
          break;
        }

        if model.contains(&-clause[literal_index]) {
          clause.remove(clause_index);
          continue;
        }

        literal_index += 1;
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
