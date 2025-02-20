use wgpu::VertexAttribute;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}


impl Vertex {
    pub const ATTRIBUTES: [VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // Posición en el slot 0
        1 => Float32x3  // Color en el slot 1
    ];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}