use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::wgpu_state::WGPUState;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    state: Option<WGPUState>,
}


impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Voxels wgpu");
        self.window = Some(Arc::new(event_loop.create_window(attributes).unwrap()));
        
        let arc = self.window.as_mut().expect("Window not initiated").clone();
        self.state = Some(WGPUState::new(arc));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }

        self.state.as_mut().expect("State not generated").render().unwrap();
    }
}