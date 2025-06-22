use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event::{ElementState, Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

use crate::simulation::{gravity::GravitySystem, particle::Particle, vec2::Vec2};
use crate::simulation::energy::{EnergyTracker};

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

  let mut particles = create_solar_system();

  let gravity = GravitySystem::new(6.6743, 3.0);
  let mut dt = 0.00694444;
  dt /= 3.0;

  let tracker = EnergyTracker::new(gravity.gravity_constant);
  let breakdowns = tracker.per_particle_energy(&particles);
  let initial_energy = tracker.total_energy(&particles);
  let mut particle_totals = Vec::new();
  println!("Initial System Energy = {:.6}", initial_energy);
  for (i, e) in breakdowns.iter().enumerate() {
    let total = e.total;
    particle_totals.push(total);
    println!("Initial Particle {}: Total = {:.6}", i, total); 
  }

  let mut paused = true;
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        *control_flow = ControlFlow::Exit;
      }

      Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
        pixels.resize_surface(size.width, size.height).unwrap();
      }

      Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
        if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
          if input.state == ElementState::Pressed {
            paused = !paused;
          }
        }
      }

      Event::RedrawRequested(_) => {
        if !paused {
          gravity.apply(&mut particles);

          for p in particles.iter_mut() {
            if !p.static_body {
              p.integrate(dt);
            }
          }

          let breakdowns = tracker.per_particle_energy(&particles);
          for (i, e) in breakdowns.iter().enumerate() {
            println!(
              "Particle {}: KE = {:.4}, PE = {:.4}, Total = {:.4}, Drift = {:.7}", 
              i, e.kinetic, e.potential, e.total, particle_totals[i] - e.total
            );
          }

          let total = tracker.total_energy(&particles);
          println!("Total system energy = {:.6}, Drift = {:.7}", total, initial_energy - total);
        }

        let frame_buffer = pixels.frame_mut();
        frame_buffer.fill(0);

        for p in &particles {
          let x = p.position.x.round() as i32;
          let y = p.position.y.round() as i32;
          if x >= 0 && y >= 0 && x < size.width as i32 && y < size.height as i32 {
            draw_particle(frame_buffer, size.width, size.height, x, y, 3, [0xff, 0xff, 0xff, 0xff]);
          }
        }
        pixels.render().unwrap();
      }

      Event::MainEventsCleared => {
        window.request_redraw();
      }

      _ => {}
    }
  });

  fn create_solar_system() -> Vec<Particle> {
    let mut sun = Particle::new(1_989_000.0, [400.0, 300.0], 5.0);
    sun.static_body = true;
    let mut bodies = vec![
      sun,
      Particle::new(0.33, [400.0, 250.0], 1.2),   // Mercury
      Particle::new(4.87, [400.0, 225.0], 1.5),   // Venus
      Particle::new(5.97, [400.0, 200.0], 1.8),   // Earth
      Particle::new(0.64, [400.0, 175.0], 1.6),   // Mars
      Particle::new(1898.0, [400.0, 120.0], 3.0), // Jupiter
      Particle::new(568.0, [400.0, 90.0], 2.8),   // Saturn
      Particle::new(86.8, [400.0, 70.0], 2.6),    // Uranus
      Particle::new(102.0, [400.0, 60.0], 2.6),   // Neptune
    ];

    for i in 1..bodies.len() {
      let r = (bodies[i].position.y - bodies[0].position.y).abs();
      let v = ((6.6743f64 * bodies[0].mass) / r).sqrt();
      bodies[i].velocity = Vec2::new(v, 0.0);
    }
    bodies
  }

  fn draw_particle(f_buf: &mut [u8], width: u32, height: u32, x: i32, y: i32, rad: i32, col: [u8; 4]) {
    for dy in -rad..=rad {
      for dx in -rad..=rad {
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
