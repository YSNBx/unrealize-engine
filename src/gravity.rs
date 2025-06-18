use crate::particle::Particle;

pub struct GravitySystem {
  pub gravity_constant: f32,
  pub softening: f32,
}

impl GravitySystem {
  pub fn new(gravity_constant: f32, softening: f32) -> Self {
    GravitySystem {
      gravity_constant,
      softening,
    }
  }

  pub fn apply(&self, particles: &mut [Particle]) {
    let num_particles = particles.len();

    for i in 0..num_particles {
      for j in (i + 1)..num_particles {
        let (a, b) = {
          let (left, right) = particles.split_at_mut(j);
          (&mut left[i], &mut right[0])
        };

        let delta_pos = b.position.sub(a.position);
        let distance = delta_pos.vec_length().max(1e-5);
        let collision_normal = delta_pos.normalize();

        let min_dist = a.radius + b.radius;

        if distance < min_dist {
          let v1n = a.velocity.dot(collision_normal);
          let v2n = b.velocity.dot(collision_normal);

          let m1 = a.mass;
          let m2 = b.mass;
          let v1n_prime = (v1n * (m1 - m2) + 2.0 * m2 * v2n) / (m1 + m2);
          let v2n_prime = (v2n * (m1 - m2) + 2.0 * m1 * v1n) / (m1 + m2);

          let delta_v1 = collision_normal.mul_scalar(v1n_prime - v1n);
          let delta_v2 = collision_normal.mul_scalar(v2n_prime - v2n);

          a.velocity = a.velocity.add(delta_v1);
          b.velocity = b.velocity.add(delta_v2);

          let penetration = min_dist - distance;
          let correction = collision_normal.mul_scalar(penetration / 2.0);
          a.position = a.position.sub(correction);
          b.position = b.position.add(correction);
        }

        let softened_distance = (distance * distance + self.softening * self.softening).sqrt();
        let force_magnitude = self.gravity_constant * a.mass * b.mass / (softened_distance * softened_distance);

        let gravity_force = collision_normal.mul_scalar(force_magnitude);

        a.apply_force(gravity_force);
        b.apply_force(gravity_force.mul_scalar(-1.0));
      }
    }
  }
}
