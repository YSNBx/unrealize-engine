use super::particle::Particle;

#[derive(Debug, Clone)]
pub struct EnergyBreakdown {
  pub kinetic: f64,
  pub potential: f64,
  pub total: f64,
}

pub struct EnergyTracker {
  pub g: f64,
}

impl EnergyTracker {
  pub fn new(g: f64) -> Self {
    EnergyTracker { g }
  }

  pub fn total_kinetic(&self, particles: &[Particle]) -> f64 {
    particles.iter().map(|p| {
      0.5 * p.mass * (p.velocity.x * p.velocity.x + p.velocity.y * p.velocity.y)
    }).sum()
  }

  pub fn total_potential(&self, particles: &[Particle]) -> f64 {
    let mut potential = 0.0;
    let len = particles.len();

    for i in 0..len {
      for j in (i + 1)..len {
        let a = &particles[i];
        let b = &particles[j];
        let r = b.position.sub(a.position).vec_length().max(0.01);
        potential += -self.g * a.mass * b.mass / r;
      }
    }
    potential
  }

  pub fn total_energy(&self, particles: &[Particle]) -> f64 {
    self.total_kinetic(particles) + self.total_potential(particles)
  }

  pub fn per_particle_energy(&self, particles: &[Particle]) -> Vec<EnergyBreakdown> {
    let mut breakdowns = Vec::with_capacity(particles.len());

    for (i, a) in particles.iter().enumerate() {
      let kinetic = 0.5 * a.mass * (a.velocity.x * a.velocity.x + a.velocity.y * a.velocity.y);

      let mut potential = 0.0;
      for (j, b) in particles.iter().enumerate() {
        if i != j {
          let r = b.position.sub(a.position).vec_length().max(0.01);
          potential += self.g * a.mass * b.mass / r;
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
