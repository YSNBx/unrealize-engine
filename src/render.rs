use crate::particle::Particle;

pub fn render_ascii(particles: &[Particle], width: usize, height: usize, scale: f32) {
  let mut grid = vec![vec!['.'; width]; height];

  for (i, p) in particles.iter().enumerate() {
    let x = (p.position.x / scale).round() as isize;
    let y = (p.position.y / scale).round() as isize;

    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
      let gx = x as usize;
      let gy = height - 1 - (y as usize);
      grid[gy][gx] = match i {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        _ => '*',
      };
    }
  }

  for row in &grid {
    let line: String = row.iter().collect();
    println!("{}", line);
  }
}
