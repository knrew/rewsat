use std::collections::HashMap;

pub type CNF = Vec<Vec<(usize, bool)>>;
type Model = HashMap<usize, bool>;

/// 変数が$x_0, \dots, x_n$としてn個の変数の割り当てをさがす
/// n: num_variables
pub fn solve(num_variables: usize, cnf: &CNF) -> Option<Model> {
  if let Some(mut model) = solve_recursive(num_variables, &cnf, &HashMap::new()) {
    // 真偽どちらでも充足可能な変数は真にしておく
    for i in 0..num_variables {
      if !model.contains_key(&i) {
        model.insert(i, true);
      }
    }
    Some(model)
  } else {
    None
  }
}

fn solve_recursive(num_variables: usize, clauses: &CNF, model: &Model) -> Option<Model> {
  let (clauses, model) = if let Some(res) = simplify(clauses, model) {
    res
  } else {
    return None;
  };

  if let Some(literal) = select_literal(&clauses) {
    for sign in [false, true] {
      let variable = (literal.0, sign);
      if let Some(clauses) = apply_unit_rule(&clauses, &model, variable) {
        let mut model = model.clone();
        model.insert(variable.0, variable.1);
        if let Some(model) = solve_recursive(num_variables, &clauses, &model) {
          return Some(model);
        }
      }
    }
  }

  if clauses.is_empty() {
    Some(model)
  } else {
    None
  }
}

/// 単リテラル規則を繰り返し適用しCNFを簡単にする
/// 充足不能であればNoneを返す
fn simplify(clauses: &CNF, model: &Model) -> Option<(CNF, Model)> {
  let mut clauses = clauses.clone();
  let mut model = model.clone();

  while let Some(literal) = find_unit(&clauses) {
    clauses = if let Some(clauses) = apply_unit_rule(&clauses, &model, literal) {
      clauses
    } else {
      return None;
    };
    model.insert(literal.0, literal.1);
  }

  Some((clauses, model))
}

/// 単リテラル規則を適用する
/// 充足不能あればNoneを返す
/// (充足不能: 単リテラルがモデルと矛盾する or 規則適用した結果空節が生まれる)
fn apply_unit_rule(clauses: &CNF, model: &Model, unit_literal: (usize, bool)) -> Option<CNF> {
  if model.contains_key(&unit_literal.0) && model[&unit_literal.0] != unit_literal.1 {
    return None;
  }

  let mut clauses = clauses.clone();

  // 単リテラルLに対してLを含む節を消去する
  // (Lを含まないもののみを残す)
  clauses.retain(|clause| clause.iter().all(|&literal| literal != unit_literal));

  // 単リテラルLに対して各節から\lnot L を消去する
  // 消去した結果空節になれば充足不能
  for clause in clauses.iter_mut() {
    clause.retain(|&literal| literal != (unit_literal.0, !unit_literal.1));
    if clause.is_empty() {
      return None;
    }
  }

  Some(clauses)
}

/// 単リテラル節を探し，あればそのリテラルを返す
fn find_unit(clauses: &CNF) -> Option<(usize, bool)> {
  if let Some(unit) = clauses.iter().find(|clause| clause.len() == 1) {
    Some(unit[0])
  } else {
    None
  }
}

/// 節集合の中から未割り当ての変数を選択する
/// 未割り当ての変数がない場合None
fn select_literal(clauses: &CNF) -> Option<(usize, bool)> {
  if let Some(clause) = clauses.get(0) {
    debug_assert!(!clause.is_empty());
    Some(clause[0])
  } else {
    None
  }
}
