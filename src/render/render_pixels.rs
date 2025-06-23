use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

use crate::{
  render::{camera::Camera, constants, draw, window}, 
  simulation::{energy::EnergyTracker, entity::Entity, vec2::Vec2},
  force::{newtonian_gravity, Force, linear_push::LinearPushForce}, 
  logger::logger, 
  system::solar
};

pub fn run_render_loop() {
  let event_loop = EventLoop::new();
  let mut camera = Camera::new();
  let (window, mut pixels) = window::create_window(&event_loop);
  let mut size = window.inner_size();

  let gravity = newtonian_gravity::NewtonianGravity::new(constants::GRAVITY_CONSTANT, constants::SOFTENING);
  let tracker = EnergyTracker::new(gravity.gravity_constant);

  let mut entities = solar::create_solar_system();
  let forces: Vec<Box<dyn Force>> = vec![
    Box::new(gravity),
    // Box::new(LinearPushForce::new(Vec2::new(5.0, 0.0))),
  ];

  let initial_energy = tracker.total_energy(&entities);
  let entities_totals = logger::log_initial_energy(&tracker, &entities);

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
        advanced_integrate_step(&mut entities, &forces);
        logger::log_drift(&entities, &tracker, &entities_totals);

        let total = tracker.total_energy(&entities);
        logger::log("Total system energy", total, initial_energy);

        draw::render_frame(pixels.frame_mut(), &mut entities, &camera, (size.width, size.height));
        pixels.render().unwrap();
      }

      Event::MainEventsCleared => {
        window.request_redraw();
      }
      _ => {}
    }
  });
}

fn advanced_integrate_step(entities: &mut [Entity], forces: &[Box<dyn Force>]) {
  apply_force(entities, forces);
  let old_accels: Vec<Vec2> = entities.iter().map(|p| p.acceleration).collect();

  let mut entities_next = entities.to_vec();
  for (p, a0) in entities_next.iter_mut().zip(&old_accels) {
    if !p.static_body {
      p.position = p.position
        .add(p.velocity.mul_scalar(constants::DT))
        .add(a0.mul_scalar(0.5 * constants::DT * constants::DT));
    }
  }

  apply_force(&mut entities_next, forces);
  let new_accels: Vec<Vec2> = entities.iter().map(|p| p.acceleration).collect();

  for ((p, a0), a1) in entities.iter_mut().zip(&old_accels).zip(&new_accels) {
    if !p.static_body {
      p.integrate(constants::DT, *a1);
    }
  }
}

fn apply_force(entities: &mut [Entity], forces: &[Box<dyn Force>]) {
  for force in forces {
    force.apply(entities);
  }
}
