use crate::{particle::Particle, vec2::Vec2};

pub struct GravitySystem {
  pub gravity_constant: f32,
}

impl GravitySystem {
  pub fn new(gravity_constant: f32) -> Self {
    GravitySystem { gravity_constant }
  }

  pub fn apply(&self, particles: &mut [Particle]) {
    let num_particles = particles.len();

    for i in 0..num_particles {
      for j in (i + 1)..num_particles {
        let (a, b) = {
          let (left, right) = particles.split_at_mut(j);
          (&mut left[i], &mut right[0])
        };

        let direction_vector: Vec2 = b.position.sub(a.position);
        let distance: f32 = direction_vector.length().max(0.01);
        let normalized_direction: Vec2 = direction_vector.normalize();

        let min_dist = a.radius + b.radius;
        if distance < min_dist {
          let penetration = min_dist - distance;
          let repulse_strength = penetration * 100.0;
          let repulse_force = normalized_direction.mul_scalar(repulse_strength);

          a.apply_force(repulse_force);
          b.apply_force(repulse_force.mul_scalar(-1.0));
        }

        let epsilon = 0.05;
        let softened_distance = (distance * distance + epsilon * epsilon).sqrt();
        let force_magnitude = self.gravity_constant * a.mass * b.mass / (softened_distance * softened_distance);
        let force: Vec2 = normalized_direction.mul_scalar(force_magnitude);

        a.apply_force(force);
        b.apply_force(force.mul_scalar(-1.0));
      }
    }
  }
}
