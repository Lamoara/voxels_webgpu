use std::sync::Arc;

use wgpu::InstanceDescriptor;
use winit::window::Window;

pub struct WGPUState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl WGPUState {
    pub fn new(window_arc: Arc<Window>) -> Self {
        let size = window_arc.inner_size();
        let descriptor = InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&descriptor);

        let surface = instance.create_surface(&*window_arc).unwrap();
        let surface: wgpu::Surface<'static> =
            unsafe { std::mem::transmute(surface) };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )).unwrap();

        let surface_config = surface.get_default_config(&adapter, size.width, size.height)
            .expect("No default config in surface");
        surface.configure(&device, &surface_config);

        Self {
            window: window_arc,
            surface,
            surface_config,
            device,
            queue,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Obtener la textura actual (frame)
        let output = self.surface.get_current_texture()?;
        // 2. Crear la vista de la textura
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // 3. Crear un command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // 4. Iniciar un render pass (se usa un bloque para que render_pass se suelte al final)
        todo!();
        
        // 5. Enviar los comandos a la GPU y presentar el frame
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
}