use app::App;
use winit::event_loop::EventLoop;

mod wgpu_state;
mod app;
mod shader_config;
mod mesh;
mod vertex;
mod cube;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
