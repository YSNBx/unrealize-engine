mod vec2;
mod particle;
mod gravity;
mod energy;
mod render_pixels; // 2D renderer

use crate::render_pixels::run_render_loop;

fn main() {
    env_logger::init(); // optional logging

    run_render_loop();
}

