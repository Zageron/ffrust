use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        return wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        };
    }
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

pub struct Note {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Note {
    pub fn new() -> Self {
        let mut vertices = Vec::new();

        for i in 0..32 {
            let angle: f32 = ((i as f32) / 10.0 * 360.0).to_radians();
            let x: f32 = angle.cos();
            let y: f32 = angle.sin();
            vertices.push(Vertex {
                position: [x, y, 0.0],
                color: [0.1, 0.4, 0.5],
            });
        }
        vertices.push(vertices[0]);

        let mut indices: Vec<u16> = Vec::new();

        for i in 0..vertices.len() {
            indices.push(i as u16);
        }

        Self { vertices, indices }
    }
}
