#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
  pub x: f64,
  pub y: f64,
}

impl Vec2 {
  pub fn new(x: f64, y: f64) -> Self {
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

  pub fn mul_scalar(&self, scalar: f64) -> Vec2 {
    Vec2::new(self.x * scalar, self.y * scalar)
  }

  pub fn vec_length(&self) -> f64 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn dot(&self, other: Vec2) -> f64 {
    self.x * other.x + self.y * other.y
  }

  pub fn normalize(&self) -> Vec2 {
    let len = self.vec_length();
    if len == 0.0 {
      Vec2::zero()
    } else {
      self.mul_scalar(1.0 / len)
    }
  }
}
