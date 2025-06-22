mod simulation;
mod render;

use render::render_pixels::run_render_loop;

fn main() {
  env_logger::init();
  run_render_loop();
}

