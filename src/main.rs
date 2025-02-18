use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let attributes = Window::default_attributes().with_title("Voxels wgpu");
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
