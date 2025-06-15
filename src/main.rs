mod vec2;
mod particle;
mod gravity;
mod render;
mod energy;

use crate::{energy::EnergyTracker, gravity::GravitySystem, particle::Particle, render::render_ascii};

fn main() {
  let mut particles: Vec<Particle> = Vec::new();
  particles.push(Particle::new(1.0, [0.0, 0.0], 0.05));
  particles.push(Particle::new(1.0, [3.0, 0.0], 0.05));
  particles.push(Particle::new(1.0, [1.5, 0.866], 0.05));

  let gravity: GravitySystem = GravitySystem::new(1.0);
  let dt: f32 = 0.001;
  let tracker = EnergyTracker::new(1.0);
  let initial_te = tracker.total_energy(&particles);

  for step in 0..5000 {
    gravity.apply(&mut particles);

    for particle in particles.iter_mut() {
      particle.integrate(dt);
    }

    let ke = tracker.total_kinetic(&particles);
    let pe = tracker.total_potential(&particles);
    let te = tracker.total_energy(&particles);

    println!("Step: {}", step);
    println!("Energy => KE: {:.5}, PE: {:.5}, Total: {:.5}", ke, pe, te);
    println!("Drift: {:.5}", te - initial_te);
    render_ascii(&particles, 40, 20, 0.1);
    std::thread::sleep(std::time::Duration::from_millis(5));
    println!();
  }
}
