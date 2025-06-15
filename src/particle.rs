use crate::vec2::Vec2;

pub struct Particle {
  pub mass: f32,
  pub position: Vec2,
  pub velocity: Vec2,
  pub acceleration: Vec2,
}

impl Particle {
  pub fn new(mass: f32, position: [f32; 2]) -> Self {
    Particle {
      mass,
      position: Vec2::new(position[0], position[1]),
      velocity: Vec2::zero(),
      acceleration: Vec2::zero(),
    }
  }

  pub fn apply_force(&mut self, force: Vec2) {
    let acc = force.mul_scalar(1.0 / self.mass);
    self.acceleration = self.acceleration.add(acc);
  }

  pub fn integrate(&mut self, dt: f32) {
    self.velocity = self.velocity.add(self.acceleration.mul_scalar(dt));
    self.position = self.position.add(self.velocity.mul_scalar(dt));
    self.acceleration = Vec2::zero();
  }
}
