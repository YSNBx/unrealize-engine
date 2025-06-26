use super::entity::Entity;

#[derive(Debug, Clone)]
pub struct EnergyBreakdown {
  pub kinetic: f64,
  pub potential: f64,
  pub total: f64,
}

pub struct EnergyTracker {
  pub gravity_constant: f64,
}

impl EnergyTracker {
  pub fn new(gravity_constant: f64) -> Self {
    EnergyTracker { gravity_constant }
  }

  pub fn total_kinetic(&self, entities: &[Entity]) -> f64 {
    entities.iter().map(|p| {
      0.5 * p.mass.unwrap() * (p.velocity.x * p.velocity.x + p.velocity.y * p.velocity.y)
    }).sum()
  }

  pub fn total_potential(&self, entities: &[Entity]) -> f64 {
    let mut potential = 0.0;
    let len = entities.len();

    for i in 0..len {
      for j in (i + 1)..len {
        let a = &entities[i];
        let b = &entities[j];
        let r = b.position.sub(a.position).vec_length().max(0.01);
        potential += -self.gravity_constant * a.mass.unwrap() * b.mass.unwrap() / r;
      }
    }
    potential
  }

  pub fn total_energy(&self, entities: &[Entity]) -> f64 {
    self.total_kinetic(entities) + self.total_potential(entities)
  }

  pub fn per_entity_energy(&self, entities: &[Entity]) -> Vec<EnergyBreakdown> {
    let mut breakdowns = Vec::with_capacity(entities.len());

    for (i, a) in entities.iter().enumerate() {
      let kinetic = 0.5 * a.mass.unwrap() * (a.velocity.x * a.velocity.x + a.velocity.y * a.velocity.y);

      let mut potential = 0.0;
      for (j, b) in entities.iter().enumerate() {
        if i != j {
          let r = b.position.sub(a.position).vec_length().max(0.01);
          potential += self.gravity_constant * a.mass.unwrap() * b.mass.unwrap() / r;
        }
      }
      potential *= 0.5;
      breakdowns.push(EnergyBreakdown {
        kinetic,
        potential,
        total: kinetic + potential,
      });
    }
    breakdowns
  }
}
