use crate::{
  simulation::{entity::Entity, vec2::Vec2},
  force::force::Force,
};

pub struct LinearPushForce {
  pub force: Vec2,
}

impl LinearPushForce {
  pub fn new(force: Vec2) -> Self {
    Self { force }
  }
}

impl Force for LinearPushForce {
  fn apply(&self, entities: &mut [Entity]) {
    for entity in entities.iter_mut() {
      if !entity.static_body {
        entity.apply_force(self.force);
      }
    }
  }
}
