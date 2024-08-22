#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruthAssign {
  Flase,
  True,
  Unassigned,
}

#[derive(Clone, Debug)]
pub struct Model {
  value: Vec<TruthAssign>,
}

impl Model {
  pub fn new(num_variables: usize) -> Self {
    Self {
      value: vec![TruthAssign::Unassigned; num_variables],
    }
  }

  pub fn has_assigned(&self, index: usize) -> bool {
    if let Some(&v) = self.value.get(index) {
      v != TruthAssign::Unassigned
    } else {
      false
    }
  }

  pub fn sign(&self, index: usize) -> Option<bool> {
    if let Some(v) = self.value.get(index) {
      match v {
        TruthAssign::Flase => Some(false),
        TruthAssign::True => Some(true),
        TruthAssign::Unassigned => None,
      }
    } else {
      None
    }
  }

  pub fn assign(&mut self, index: usize, sign: bool) -> bool {
    if let Some(v) = self.value.get_mut(index) {
      *v = if sign {
        TruthAssign::True
      } else {
        TruthAssign::Flase
      };
      true
    } else {
      false
    }
  }
}
