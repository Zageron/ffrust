use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
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
    pub x: i32,
    pub y: i32,
    pub h: i32,
    pub w: i32,
}

impl Note {
    pub fn new(x: i32, y: i32, h: i32, w: i32) -> Self {
        Self { x, y, h, w }
    }

    pub fn uv_coord(self) -> (f32, f32, f32, f32) {
        (
            self.x as f32 / self.w as f32,
            self.y as f32 / self.h as f32,
            self.w as f32 / self.w as f32,
            self.h as f32 / self.h as f32,
        )
    }
}
