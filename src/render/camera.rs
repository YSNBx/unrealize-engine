use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

use crate::{render::constants, simulation::vec2::Vec2};

pub struct Camera {
  pub dragging: bool,
  pub last_cursor_pos: Vec2,
  pub center: Vec2,
  pub scale: f64,
}

impl Camera {
  pub fn new() -> Self {
    Self {
      dragging: false,
      last_cursor_pos: Vec2::new(0.0, 0.0),
      center: Vec2::new(0.0, 0.0),
      scale: constants::INITIAL_SCALE,
    }
  }

  pub fn handle_event(&mut self, event: &WindowEvent) {
    match event {
      WindowEvent::MouseInput { state, button, .. } if *button == MouseButton::Left => {
        self.dragging = *state == ElementState::Pressed;
      }

      WindowEvent::CursorMoved { position, .. } => {
        let current = Vec2::new(position.x, position.y);
        if self.dragging {
          let delta = current.sub(self.last_cursor_pos).mul_scalar(1.0 / self.scale);
          self.center = self.center.sub(delta);
        }
        self.last_cursor_pos = current;
      }
      WindowEvent::MouseWheel { delta, .. } => {
        let zoom = match delta {
          MouseScrollDelta::LineDelta(_, y) => 1.1_f64.powf(*y as f64),
          MouseScrollDelta::PixelDelta(pos) => 1.1_f64.powf(pos.y / 100.0),
        };
        self.scale *= zoom;
      }
      _ => {}
    }
  }
}
