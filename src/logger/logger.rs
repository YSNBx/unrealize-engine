use crate::simulation::{energy::EnergyTracker, particle::Particle};

pub fn log(message: &str, total: f64, initial: f64) {
  println!("{} = {:.6}, Drift = {:.7}", message, total, initial - total); 
}

pub fn log_drift(
  particles: &[Particle],
  tracker: &EnergyTracker,
  initial_totals: &[f64],
) {
  let breakdowns = tracker.per_particle_energy(particles);
  for (i, e) in breakdowns.iter().enumerate() {
    println!(
      "Particle {}: KE = {:.4}, PE = {:.4}, Total = {:.4}, Drift = {:.7}", 
      i, e.kinetic, e.potential, e.total, initial_totals[i] - e.total
    );
  }
}

pub fn log_initial_energy(tracker: &EnergyTracker, particles: &[Particle]) -> Vec<f64> {
  let breakdowns = tracker.per_particle_energy(particles);
  let initial_total = tracker.total_energy(particles);

  println!("Initial System Energy = {:.6}", initial_total);

  let mut particle_totals = Vec::with_capacity(particles.len());
  for (i, e) in breakdowns.iter().enumerate() {
    println!("Initial Particle {}: Total = {:.6}", i, e.total);
    particle_totals.push(e.total);
  }

  particle_totals
}
