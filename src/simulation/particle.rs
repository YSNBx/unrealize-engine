use super::vec2::Vec2;

#[derive(Clone)]
pub struct Particle {
  pub mass: f64,
  pub position: Vec2,
  pub velocity: Vec2,
  pub acceleration: Vec2,
  pub radius: f64,
  pub static_body: bool,
}

impl Particle {
  pub fn new(mass: f64, position: [f64; 2], radius: f64) -> Self {
    Particle {
      mass,
      position: Vec2::new(position[0], position[1]),
      velocity: Vec2::zero(),
      acceleration: Vec2::zero(),
      radius,
      static_body: false,
    }
  }

  pub fn apply_force(&mut self, force: Vec2) {
    let acc = force.mul_scalar(1.0 / self.mass);
    self.acceleration = self.acceleration.add(acc);
  }

  pub fn integrate(&mut self, dt: f64) {
    self.velocity = self.velocity.add(self.acceleration.mul_scalar(dt));
    self.position = self.position.add(self.velocity.mul_scalar(dt));
    self.acceleration = Vec2::zero();
  }
}
