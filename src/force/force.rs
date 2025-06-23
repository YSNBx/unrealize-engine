use crate::simulation::entity::Entity;

pub trait Force {
  fn apply(&self, entities: &mut [Entity]);
}
