use std::{borrow::Cow, fs::{self}, sync::Arc};

use wgpu::{Device, InstanceDescriptor, LoadOp, PipelineCompilationOptions, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp, VertexState};
use winit::window::Window;

pub struct WGPUState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: RenderPipeline,
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

        let pipeline = Self::create_pipeline(&device, "");

        Self {
            window: window_arc,
            surface,
            surface_config,
            device,
            queue,
            pipeline,
        }
    }

    fn create_pipeline(device: &Device, vertex_dir: &str) -> RenderPipeline
    {
        let shader_str = fs::read_to_string(vertex_dir).unwrap();
        let vertex_shader = device.create_shader_module(ShaderModuleDescriptor{
            label: Some("Vertex Shader"),
            source: ShaderSource::Wgsl(Cow::from(shader_str))
        });

        let vertex_state = VertexState{
            module: &vertex_shader,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions{
                constants: todo!(),
                zero_initialize_workgroup_memory: todo!(),
            },
            buffers: todo!(),
        };


        device.create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: None,
            vertex: vertex_state,
            primitive: todo!(),
            depth_stencil: todo!(),
            multisample: todo!(),
            fragment: todo!(),
            multiview: todo!(),
            cache: todo!(),
        })
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
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment{ 
                    view: &view, 
                    resolve_target: None, 
                    ops:wgpu::Operations {
                        load: LoadOp::Clear(wgpu::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: StoreOp::Store,
                    }, 
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        
        // 5. Enviar los comandos a la GPU y presentar el frame
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
}