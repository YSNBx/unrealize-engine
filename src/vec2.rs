#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn new(x: f32, y: f32) -> Self {
    Vec2 { x, y }
  }

  pub fn zero() -> Self {
    Vec2 { x: 0.0, y: 0.0 }
  }

  pub fn add(&self, other: Vec2) -> Vec2 {
    Vec2::new(self.x + other.x, self.y + other.y)
  }

  pub fn sub(&self, other: Vec2) -> Vec2 {
    Vec2::new(self.x - other.x, self.y - other.y)
  }

  pub fn mul_scalar(&self, scalar: f32) -> Vec2 {
    Vec2::new(self.x * scalar, self.y * scalar)
  }

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn normalize(&self) -> Vec2 {
    let len = self.length();
    if len == 0.0 {
      Vec2::zero()
    } else {
      self.mul_scalar(1.0 / len)
    }
  }
}

