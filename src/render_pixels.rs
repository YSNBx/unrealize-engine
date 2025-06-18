use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

use crate::{gravity::GravitySystem, particle::Particle};

pub fn run_render_loop() {
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Unrealize Engine")
    .with_inner_size(LogicalSize::new(800.0, 600.0))
    .build(&event_loop)
    .unwrap();

  let size = window.inner_size();
  let surface = SurfaceTexture::new(size.width, size.height, &window);
  let mut pixels = Pixels::new(size.width, size.height, surface).unwrap();

  let mut particles = vec![
    Particle::new(1.0, [100.0, 100.0], 0.2),
    Particle::new(1.0, [200.0, 100.0], 0.2),
    Particle::new(1.0, [150.0, 173.2], 0.2),
  ];  

  let gravity = GravitySystem::new(1000.0, 5.0);
  let dt = 0.00694444;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        *control_flow = ControlFlow::Exit;
      }

      Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
        pixels.resize_surface(size.width, size.height).unwrap();
      }

      Event::RedrawRequested(_) => {
        // simulation step
        gravity.apply(&mut particles);
        for p in particles.iter_mut() {
          p.integrate(dt);
        }

        let frame = pixels.frame_mut();
        frame.fill(0); // clear to black

        for p in &particles {
          let x = p.position.x.round() as i32;
          let y = p.position.y.round() as i32;

          if x >= 0 && y >= 0 && x < size.width as i32 && y < size.height as i32 {
            let idx = (y as usize * size.width as usize + x as usize) * 4;
            if idx + 3 < frame.len() {
              let radius = 2; // particle "radius" in pixels
              for dy in -radius..=radius {
                for dx in -radius..=radius {
                  let px = x + dx;
                  let py = y + dy;
                  if px >= 0 && py >= 0 && px < size.width as i32 && py < size.height as i32 {
                    let idx = (py as usize * size.width as usize + px as usize) * 4;
                    if idx + 3 < frame.len() {
                      frame[idx..idx + 4].copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                    }
                  }
                }
              }            }
          }
        }

        pixels.render().unwrap();
      }

      Event::MainEventsCleared => {
        window.request_redraw(); // triggers RedrawRequested
      }

      _ => {}
    }
  });
}

