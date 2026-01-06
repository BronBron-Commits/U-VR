use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("vertex_buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("index_buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}

/* =========================================================
   GRID FLOOR (LINE LIST)
   ========================================================= */

pub fn floor_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let size: i32 = 20;
    let spacing = 1.0;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut i: u16 = 0;

    let grid_color = [0.4, 0.4, 0.4];

    // lines parallel to Z
    for x in -size..=size {
        let x = x as f32 * spacing;

        vertices.push(Vertex {
            position: [x, 0.0, -size as f32 * spacing],
            color: grid_color,
        });
        vertices.push(Vertex {
            position: [x, 0.0, size as f32 * spacing],
            color: grid_color,
        });

        indices.push(i);
        indices.push(i + 1);
        i += 2;
    }

    // lines parallel to X
    for z in -size..=size {
        let z = z as f32 * spacing;

        vertices.push(Vertex {
            position: [-size as f32 * spacing, 0.0, z],
            color: grid_color,
        });
        vertices.push(Vertex {
            position: [size as f32 * spacing, 0.0, z],
            color: grid_color,
        });

        indices.push(i);
        indices.push(i + 1);
        i += 2;
    }

    (vertices, indices)
}

/* =========================================================
   CUBE (TRIANGLES)
   ========================================================= */

pub fn cube_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let c = [0.8, 0.2, 0.2];

    let vertices = vec![
        Vertex { position: [-0.5, -0.5, -0.5], color: c },
        Vertex { position: [ 0.5, -0.5, -0.5], color: c },
        Vertex { position: [ 0.5,  0.5, -0.5], color: c },
        Vertex { position: [-0.5,  0.5, -0.5], color: c },
        Vertex { position: [-0.5, -0.5,  0.5], color: c },
        Vertex { position: [ 0.5, -0.5,  0.5], color: c },
        Vertex { position: [ 0.5,  0.5,  0.5], color: c },
        Vertex { position: [-0.5,  0.5,  0.5], color: c },
    ];

    let indices: Vec<u16> = vec![
        0, 1, 2, 2, 3, 0,
        4, 5, 6, 6, 7, 4,
        0, 4, 7, 7, 3, 0,
        1, 5, 6, 6, 2, 1,
        3, 2, 6, 6, 7, 3,
        0, 1, 5, 5, 4, 0,
    ];

    (vertices, indices)
}
