use std::{borrow::Cow, collections::HashMap, fs::{self}, sync::Arc};

use wgpu::{util::DeviceExt, Buffer, BufferUsages, Device, FragmentState, InstanceDescriptor, LoadOp, MultisampleState, PipelineCompilationOptions, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp, SurfaceConfiguration, VertexState};
use winit::window::Window;

use crate::{shader_config::ShaderConfig, vertex::Vertex};

pub struct WGPUState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}


const VERTICES: &[Vertex] = &[
    Vertex { position: [ -1.0,  1.0, 0.0], color: [1.0, 0.0, 0.0] }, // Arriba (Rojo)
    Vertex { position: [-1.0, -1.0, 0.0], color: [0.0, 1.0, 0.0] }, // Izquierda (Verde)
    Vertex { position: [ 1.0, -1.0, 0.0], color: [0.0, 0.0, 1.0] }, // Derecha (Azul)
    Vertex { position: [ -1.0,  1.0, 0.0], color: [1.0, 0.0, 0.0] }, // Arriba (Rojo)
    Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 1.0, 0.0] }, // Izquierda (Verde)
    Vertex { position: [ 1.0, -1.0, 0.0], color: [0.0, 0.0, 1.0] }, // Derecha (Azul)
];


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
                required_features: wgpu::Features::CONSERVATIVE_RASTERIZATION,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )).unwrap();

        let surface_config = surface.get_default_config(&adapter, size.width, size.height)
            .expect("No default config in surface");
        surface.configure(&device, &surface_config);

        let vertex_config = ShaderConfig{
            path: r"shaders\vertex.wgsl",
            label: r"Vertex Shader",
            entry_point: r"main",
            constants: HashMap::new(),
            zero_initialize_workgrouo_memory: true,
        };

        let fragment_config = ShaderConfig{
            path: r"shaders\fragment.wgsl",
            label: r"Fragment Shader",
            entry_point: r"main",
            constants: HashMap::new(),
            zero_initialize_workgrouo_memory: false,
        };

        let pipeline = Self::create_pipeline(&device, &surface_config, &vertex_config, &fragment_config);
        let vertex_buffer = Self::create_vertex_buffer(&device, &queue);

        Self {
            window: window_arc,
            surface,
            surface_config,
            device,
            queue,
            pipeline,
            vertex_buffer,
        }
    }

    fn create_pipeline(device: &Device, surface: &SurfaceConfiguration, vertex_config: &ShaderConfig, fragment_config: &ShaderConfig) -> RenderPipeline
    {
        let shader_str = fs::read_to_string(vertex_config.path).unwrap();
        let vertex_shader = device.create_shader_module(ShaderModuleDescriptor{
            label: Some(vertex_config.label),
            source: ShaderSource::Wgsl(Cow::from(shader_str))
        });

        let vertex_state = VertexState{
            module: &vertex_shader,
            entry_point: Some(vertex_config.entry_point),
            compilation_options: PipelineCompilationOptions{
                constants: &vertex_config.constants,
                zero_initialize_workgroup_memory: vertex_config.zero_initialize_workgrouo_memory,
            },
            buffers: &[Vertex::layout()],
        };

        let shader_str = fs::read_to_string(fragment_config.path).unwrap();
        let fragment_shader = device.create_shader_module(ShaderModuleDescriptor{
            label: Some(fragment_config.label),
            source: ShaderSource::Wgsl(Cow::from(shader_str))
        });

        let fragment_state = FragmentState{
            module: &fragment_shader,
            entry_point: Some(fragment_config.entry_point),
            compilation_options: PipelineCompilationOptions{
                constants: &fragment_config.constants,
                zero_initialize_workgroup_memory: fragment_config.zero_initialize_workgrouo_memory,
            },
            targets: &[Some(wgpu::ColorTargetState {
                format: surface.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };


        device.create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: None,
            vertex: vertex_state,
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(fragment_state),
            multiview: None,
            cache: None,
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
                        load: LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: StoreOp::Store,
                    }, 
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
        
        // 5. Enviar los comandos a la GPU y presentar el frame
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }

    fn create_vertex_buffer(device: &Device, queue: &Queue) -> Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        vertex_buffer
    }

    
}