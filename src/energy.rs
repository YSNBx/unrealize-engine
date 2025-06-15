use crate::particle::Particle;

pub struct EnergyTracker {
  pub g: f32,
}

impl EnergyTracker {
  pub fn new(g: f32) -> Self {
    EnergyTracker { g }
  }

  pub fn total_kinetic(&self, particles: &[Particle]) -> f32 {
    particles.iter().map(|p| {
      0.5 * p.mass * (p.velocity.x * p.velocity.x + p.velocity.y * p.velocity.y)
    }).sum()
  }

  pub fn total_potential(&self, particles: &[Particle]) -> f32 {
    let mut potential = 0.0;
    let len = particles.len();

    for i in 0..len {
      for j in (i + 1)..len {
        let a = &particles[i];
        let b = &particles[j];
        let r = b.position.sub(a.position).length().max(0.01);
        potential += -self.g * a.mass * b.mass / r;
      }
    }
    potential
  }

  pub fn total_energy(&self, particles: &[Particle]) -> f32 {
    self.total_kinetic(particles) + self.total_potential(particles)
  }
}
