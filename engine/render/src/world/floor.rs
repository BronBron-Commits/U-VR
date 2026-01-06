use wgpu::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Floor {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl Floor {
    pub fn new(device: &Device) -> Self {
        let size = 40;
        let tile = 1.0;
        let half = size as f32 * tile * 0.5;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for z in 0..size {
            for x in 0..size {
                let x0 = x as f32 * tile - half;
                let x1 = x0 + tile;
                let z0 = z as f32 * tile - half;
                let z1 = z0 + tile;

                let base = vertices.len() as u16;
                let c = if (x + z) % 2 == 0 { [0.25, 0.25, 0.3] } else { [0.15, 0.15, 0.2] };

                vertices.extend_from_slice(&[
                    Vertex { position: [x0, 0.0, z0], color: c },
                    Vertex { position: [x1, 0.0, z0], color: c },
                    Vertex { position: [x1, 0.0, z1], color: c },
                    Vertex { position: [x0, 0.0, z1], color: c },
                ]);

                indices.extend_from_slice(&[
                    base, base + 1, base + 2,
                    base, base + 2, base + 3,
                ]);
            }
        }

        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
