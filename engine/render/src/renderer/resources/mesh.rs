use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
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
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
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
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}

pub fn floor_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let v = vec![
        Vertex { position: [-5.0, 0.0, -5.0], color: [0.6, 0.6, 0.6] },
        Vertex { position: [ 5.0, 0.0, -5.0], color: [0.6, 0.6, 0.6] },
        Vertex { position: [ 5.0, 0.0,  5.0], color: [0.4, 0.4, 0.4] },
        Vertex { position: [-5.0, 0.0,  5.0], color: [0.4, 0.4, 0.4] },
    ];

    let i = vec![0, 1, 2, 2, 3, 0];
    (v, i)
}

pub fn cube_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let c = [0.2, 0.8, 0.3];

    let v = vec![
        // bottom
        Vertex { position: [-0.5, 0.0, -0.5], color: c },
        Vertex { position: [ 0.5, 0.0, -0.5], color: c },
        Vertex { position: [ 0.5, 0.0,  0.5], color: c },
        Vertex { position: [-0.5, 0.0,  0.5], color: c },
        // top
        Vertex { position: [-0.5, 1.0, -0.5], color: c },
        Vertex { position: [ 0.5, 1.0, -0.5], color: c },
        Vertex { position: [ 0.5, 1.0,  0.5], color: c },
        Vertex { position: [-0.5, 1.0,  0.5], color: c },
    ];

    let i = vec![
        0,1,2, 2,3,0, // bottom
        4,5,6, 6,7,4, // top
        0,1,5, 5,4,0, // front
        2,3,7, 7,6,2, // back
        1,2,6, 6,5,1, // right
        3,0,4, 4,7,3, // left
    ];

    (v, i)
}
