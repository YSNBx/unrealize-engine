mod vec2;
mod particle;
mod gravity;
mod render;

use crate::{gravity::GravitySystem, particle::Particle, render::render_ascii};

fn main() {
  let mut particles: Vec<Particle> = Vec::new();
  particles.push(Particle::new(1.0, [0.0, 0.0], 0.05));
  particles.push(Particle::new(1.0, [3.0, 0.0], 0.05));
  particles.push(Particle::new(1.0, [1.5, 0.866], 0.05));

  // const GRAVITY_CONSTANT: f64 = 6.67430e-11;

  let gravity: GravitySystem = GravitySystem::new(1.0);
  let dt: f32 = 0.001;

  for step in 0..5000 {
    gravity.apply(&mut particles);

    for particle in particles.iter_mut() {
      particle.integrate(dt);
    }

    println!("Step: {}", step);
    render_ascii(&particles, 40, 20, 0.1);
    std::thread::sleep(std::time::Duration::from_millis(5));
    println!();

    // println!("Step {}:", step);
    // for (i, p) in particles.iter().enumerate() {
    //   print!(
    //     "Particle {}: pos = ({:.3}, {:.3}), vel = ({:.3}, {:.3}) ",
    //     i, p.position.x, p.position.y, p.velocity.x, p.velocity.y,
    //   );
    // }
    // println!();
  }
}
