use std::time::{self, Duration};

use crate::expressions::{clause::Clause, literal::Literal, model::Model};

#[derive(Debug)]
pub struct DPLL {
  time: Duration,
}

impl DPLL {
  pub fn new() -> Self {
    Self {
      time: Duration::default(),
    }
  }

  pub fn solve(&mut self, num_variables: usize, clauses: &[Clause]) -> Option<Model> {
    let start = time::Instant::now();
    let result = solve_recursive(num_variables, &clauses, &Model::new(num_variables));
    self.time = time::Instant::now() - start;
    result
  }

  pub fn time(&self) -> Duration {
    self.time
  }
}

fn solve_recursive(num_variables: usize, clauses: &[Clause], model: &Model) -> Option<Model> {
  let (clauses, model) = if let Some(res) = simplify(clauses, model) {
    res
  } else {
    return None;
  };

  if let Some(variable) = select_variable(num_variables, &clauses, &model) {
    for sign in [true, false] {
      if let Some(clauses) = apply_unit_rule(&clauses, &model, variable, sign) {
        let mut model = model.clone();
        model.assign(variable, sign);
        if let Some(model) = solve_recursive(num_variables, &clauses, &model) {
          return Some(model);
        }
      }
    }
  }

  if clauses
    .iter()
    .all(|clause| clause.is_valid() && clause.has_deleted())
  {
    Some(model)
  } else {
    None
  }
}

/// 単リテラル規則を繰り返し適用しCNFを簡単にする
/// 充足不能であればNoneを返す
fn simplify(clauses: &[Clause], model: &Model) -> Option<(Vec<Clause>, Model)> {
  let mut clauses = clauses.to_vec();
  let mut model = model.clone();

  while let Some(literal) = find_unit(&clauses) {
    clauses = if let Some(clauses) =
      apply_unit_rule(&clauses, &model, literal.variable(), literal.sign())
    {
      clauses
    } else {
      return None;
    };
    model.assign(literal.variable(), literal.sign());
  }

  Some((clauses, model))
}

/// 単リテラル規則を適用する
/// 充足不能あればNoneを返す
/// (充足不能: 単リテラルがモデルと矛盾する or 規則適用した結果空節が生まれる)
fn apply_unit_rule(
  clauses: &[Clause],
  model: &Model,
  variable: usize,
  sign: bool,
) -> Option<Vec<Clause>> {
  if model.has_assigned(variable) && model.sign(variable).unwrap() != sign {
    return None;
  }

  let mut clauses = clauses.to_vec();

  for clause in clauses.iter_mut() {
    clause.assign(variable, sign);
    if !clause.is_valid() {
      return None;
    }
  }

  Some(clauses)
}

/// 単リテラル節を探し，あればそのリテラルを返す
fn find_unit(clauses: &[Clause]) -> Option<Literal> {
  if let Some(unit) = clauses
    .iter()
    .filter(|clause| clause.is_valid() && !clause.has_deleted())
    .find(|clause| clause.is_unit())
  {
    Some(unit[0])
  } else {
    None
  }
}

/// 節集合の中から未割り当ての変数を選択する
/// 未割り当ての変数がない場合None
fn select_variable(num_variables: usize, clauses: &[Clause], model: &Model) -> Option<usize> {
  if let Some(clause) = clauses
    .iter()
    .filter(|clause| clause.is_valid() && !clause.has_deleted())
    .nth(0)
  {
    Some(clause[0].variable())
  } else if let Some(i) = (0..num_variables).find(|&i| !model.has_assigned(i)) {
    Some(i)
  } else {
    None
  }
}
