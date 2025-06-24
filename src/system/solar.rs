use crate::{render::constants, simulation::{entity::Entity, vec2::Vec2}};

pub fn create_solar_system() -> Vec<Entity> {
  let sun = Entity::new(Some(1_989_000.0), [0.0, 0.0], 5.0);
  // sun.static_body = true;
  let mut bodies = vec![
    sun,
    Entity::new(Some(0.33), [0.0, 40.0], 1.2),      // Mercury
    Entity::new(Some(4.87), [0.0, 70.0], 1.5),      // Venus
    Entity::new(Some(5.97), [0.0, 100.0], 1.8),     // Earth
    Entity::new(Some(0.64), [0.0, 150.0], 1.6),     // Mars
    Entity::new(Some(1898.0), [0.0, 520.0], 3.0),   // Jupiter
    Entity::new(Some(568.0), [0.0, 960.0], 2.8),    // Saturn
    Entity::new(Some(86.8), [0.0, 1910.0], 2.6),    // Uranus
    Entity::new(Some(102.0), [0.0, 3000.0], 2.6),   // Neptune
  ];

  let msg = "Central body must have a mass!";
  for i in 1..bodies.len() {
    let r = (bodies[i].position.y - bodies[0].position.y).abs();
    let v = ((constants::GRAVITY_CONSTANT * bodies[0].mass.expect(msg)) / r).sqrt();
    bodies[i].velocity = Vec2::new(v, 0.0);
  }
  bodies
}
