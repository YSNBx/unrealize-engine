use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

use crate::{
  render::{camera::Camera, draw, window, constants}, 
  simulation::{energy::EnergyTracker, vec2::Vec2, gravity::GravitySystem, particle::Particle},
  system::solar,
  logger::logger
};

pub fn run_render_loop() {
  let event_loop = EventLoop::new();
  let mut camera = Camera::new();
  let (window, mut pixels) = window::create_window(&event_loop);
  let mut size = window.inner_size();

  let mut particles = solar::create_solar_system();
  let gravity = GravitySystem::new(constants::GRAVITY_CONSTANT, constants::SOFTENING);

  let tracker = EnergyTracker::new(gravity.gravity_constant);
  let initial_energy = tracker.total_energy(&particles);
  let particle_totals = logger::log_initial_energy(&tracker, &particles);

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        *control_flow = ControlFlow::Exit;
      }

      Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
        size = new_size;
        pixels.resize_surface(size.width, size.height).unwrap();
        pixels.resize_buffer(size.width, size.height).unwrap();
      }

      Event::WindowEvent { event, .. } => {
        camera.handle_event(&event);
      }

      Event::RedrawRequested(_) => {
        advanced_integrate_step(&mut particles, &gravity);
        logger::log_drift(&particles, &tracker, &particle_totals);

        let total = tracker.total_energy(&particles);
        logger::log("Total system energy", total, initial_energy);

        draw::render_frame(pixels.frame_mut(), &mut particles, &camera, (size.width, size.height));
        pixels.render().unwrap();
      }

      Event::MainEventsCleared => {
        window.request_redraw();
      }
      _ => {}
    }
  });
}

fn advanced_integrate_step(particles: &mut [Particle], gravity: &GravitySystem) {
  gravity.apply(particles);
  let old_accels: Vec<Vec2> = particles.iter().map(|p| p.acceleration).collect();

  let mut particles_next = particles.to_vec();
  for (p, a0) in particles_next.iter_mut().zip(&old_accels) {
    if !p.static_body {
      p.position = p.position
        .add(p.velocity.mul_scalar(constants::DT))
        .add(a0.mul_scalar(0.5 * constants::DT * constants::DT));
    }
  }

  gravity.apply(&mut particles_next);
  let new_accels: Vec<Vec2> = particles.iter().map(|p| p.acceleration).collect();

  for ((p, a0), a1) in particles.iter_mut().zip(&old_accels).zip(&new_accels) {
    if !p.static_body {
      p.integrate(constants::DT, *a1);
    }
  }
}
