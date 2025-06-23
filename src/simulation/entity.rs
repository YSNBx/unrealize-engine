use std::collections::VecDeque;

use super::vec2::Vec2;

#[derive(Clone)]
pub struct Entity {
  pub mass: f64,
  pub position: Vec2,
  pub velocity: Vec2,
  pub acceleration: Vec2,
  pub radius: f64,
  pub static_body: bool,
  pub trail: VecDeque<Vec2>,
}

impl Entity {
  pub fn new(mass: f64, position: [f64; 2], radius: f64) -> Self {
    Entity {
      mass,
      position: Vec2::new(position[0], position[1]),
      velocity: Vec2::zero(),
      acceleration: Vec2::zero(),
      radius,
      static_body: false,
      trail: VecDeque::with_capacity(100),
    }
  }

  pub fn apply_force(&mut self, force: Vec2) {
    let acc = force.mul_scalar(1.0 / self.mass);
    self.acceleration = self.acceleration.add(acc);
  }

  pub fn integrate(&mut self, dt: f64, new_accel: Vec2) {
    let current_accel = self.acceleration;

    self.position = self.position
      .add(self.velocity.mul_scalar(dt))
      .add(current_accel.add(new_accel).mul_scalar(0.5 * dt * dt));

    self.velocity = self.velocity.add(current_accel.add(new_accel).mul_scalar(0.5 * dt));
    self.acceleration = new_accel;
  }
}
