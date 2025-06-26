use crate::render::camera::Camera;
use crate::simulation::entity::Entity;
use crate::simulation::vec2::Vec2;
use crate::render::constants;

pub fn world_to_screen(pos: Vec2, center: Vec2, scale: f64, width: u32, height: u32) -> (i32, i32) {
  let x = ((pos.x - center.x) * scale + (width as f64 / 2.0)).round() as i32; 
  let y = ((pos.y - center.y) * scale + (height as f64 / 2.0)).round() as i32; 
  (x, y)
}

/// Draws an entity to the framebuffer.
///
/// # Parameters
/// - `f_buf`: The framebuffer buffer to draw to.
/// - `width`: Width of the framebuffer.
/// - `height`: Height of the framebuffer.
/// - `x`, `y`: Coordinates to draw at.
/// - `rad`: Radius of the entity.
/// - `col`: RGBA color.
///
/// # Example
/// ```
/// draw_entity(&mut buffer, 800, 600, 10, 10, 5, [255, 0, 0, 255]);
/// ```
pub fn draw_entity(f_buf: &mut [u8], width: u32, height: u32, x: i32, y: i32, rad: i32, col: [u8; 4]) {
  for dy in -rad..=rad {
    for dx in -rad..=rad {
      if dx * dx + dy * dy <= rad * rad {
        let px = x + dx;
        let py = y + dy;
        if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
          let idx = (py as usize * width as usize + px as usize) * 4;
          if idx + 3 < f_buf.len() {
            f_buf[idx..idx + 4].copy_from_slice(&col);
          }
        }
      } 
    }
  }
}

pub fn get_entity_color(index: usize) -> [u8; 4] {
  match index {
    0 => [255, 255, 0, 255],   // Sun - Yellow
    1 => [200, 200, 200, 255], // Mercury - Light Gray
    2 => [255, 165, 0, 255],   // Venus - Orange
    3 => [0, 0, 255, 255],     // Earth - Blue
    4 => [255, 0, 0, 255],     // Mars – Red
    5 => [255, 215, 0, 255],   // Jupiter – Gold
    6 => [210, 180, 140, 255], // Saturn – Tan
    7 => [0, 255, 255, 255],   // Uranus – Cyan
    8 => [0, 0, 128, 255],     // Neptune – Navy
    _ => [255, 255, 255, 255], // Fallback – White
  }
}

pub fn draw_orbit_circle(
  f_buf: &mut [u8],
  screen_w: u32,
  screen_h: u32,
  center_world: Vec2,
  radius_world: f64,
  camera_center: Vec2,
  scale: f64,
  color: [u8; 4],
) {
  let num_segments = 100;
  for i in 0..num_segments {
    let theta1 = (i as f64 / num_segments as f64) * std::f64::consts::TAU;
    let theta2 = ((i + 1) as f64 / num_segments as f64) * std::f64::consts::TAU;

    let p1 = Vec2::new(
      center_world.x + radius_world * theta1.cos(),
      center_world.y + radius_world * theta1.sin(),
    );
    let p2 = Vec2::new(
      center_world.x + radius_world * theta2.cos(),
      center_world.y + radius_world * theta2.sin(),
    );

    let (x1, y1) = world_to_screen(p1, camera_center, scale, screen_w, screen_h);
    let (x2, y2) = world_to_screen(p2, camera_center, scale, screen_w, screen_h);

    draw_line(f_buf, screen_w, screen_h, x1, y1, x2, y2, color);
  }
}

pub fn render_frame(frame_buffer: &mut [u8],
  entity: &mut [Entity],
  camera: &Camera,
  size: (u32, u32),
) {
  let (width, height) = size;
  frame_buffer.fill(0);

  for p in entity.iter_mut() {
    if !p.static_body {
      p.trail.push_back(p.position);
      if p.trail.len() > constants::MAX_TRAIL_LEN {
        p.trail.pop_front();
      }
    }
  }

  for p in entity.iter() {
    for (i, point) in p.trail.iter().enumerate() {
      let (x, y) = world_to_screen(*point, camera.center, camera.scale, width, height);
      let alpha = ((i as f32 / p.trail.len() as f32) * 255.0) as u8;
      draw_entity(frame_buffer, width, height, x, y, 1, [0x80, 0x80, 0x80, alpha]);
    }
  }

  for (_i, p) in entity.iter().enumerate().skip(1) {
    let sun_pos = entity[0].position;
    let orbit_radius = (p.position.sub(sun_pos)).vec_length();
    draw_orbit_circle(
      frame_buffer,
      width,
      height,
      sun_pos,
      orbit_radius,
      camera.center,
      camera.scale,
      [0x44, 0x44, 0x44, 0xff],
    );
  }

  for (i, p) in entity.iter().enumerate() {
    let (x, y) = world_to_screen(p.position, camera.center, camera.scale, width, height);
    if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
      draw_entity(frame_buffer, width, height, x, y, 7, get_entity_color(i));
    }
  }
}

fn draw_line(
  f_buf: &mut [u8],
  screen_w: u32,
  screen_h: u32,
  x0: i32,
  y0: i32,
  x1: i32,
  y1: i32,
  color: [u8; 4],
) {
  let dx = (x1 - x0).abs();
  let dy = -(y1 - y0).abs();
  let sx = if x0 < x1 { 1 } else { -1 };
  let sy = if y0 < y1 { 1 } else { -1 };
  let mut err = dx + dy;
  let mut x = x0;
  let mut y = y0;

  loop {
    if x >= 0 && y >= 0 && x < screen_w as i32 && y < screen_h as i32 {
      let idx = (y as usize * screen_w as usize + x as usize) * 4;
      if idx + 3 < f_buf.len() {
        f_buf[idx..idx + 4].copy_from_slice(&color);
      }
    }

    if x == x1 && y == y1 { break; }
    let e2 = 2 * err;
    if e2 >= dy {
      err += dy;
      x += sx;
    }
    if e2 <= dx {
      err += dx;
      y += sy;
    }
  }
}
