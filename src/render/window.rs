use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::{Window, WindowBuilder}};

pub fn create_window(event_loop: &EventLoop<()>) -> (Window, Pixels) {
  let window = WindowBuilder::new()
    .with_title("Unrealize Engine")
    .with_inner_size(LogicalSize::new(800.0, 600.0))
    .with_resizable(true)
    .build(event_loop)
    .unwrap();

  let size = window.inner_size();
  let surface = SurfaceTexture::new(size.width, size.height, &window);
  let pixels = Pixels::new(size.width, size.height, surface).unwrap();

  (window, pixels)
}
