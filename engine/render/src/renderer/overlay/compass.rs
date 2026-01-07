use wgpu::util::DeviceExt;
use crate::renderer::pipeline::overlay_pipeline::OverlayVertex;

pub struct Compass {
    buffer: wgpu::Buffer,
    count: u32,
}

impl Compass {
    pub fn new(device: &wgpu::Device) -> Self {
        let origin = [0.82, 0.82];
        let len = 0.10;

        let verts = [
            // X axis (Red)
            OverlayVertex { position: origin, color: [1.0, 0.0, 0.0] },
            OverlayVertex { position: [origin[0] + len, origin[1]], color: [1.0, 0.0, 0.0] },

            // Z axis (Blue)
            OverlayVertex { position: origin, color: [0.0, 0.0, 1.0] },
            OverlayVertex { position: [origin[0], origin[1] + len], color: [0.0, 0.0, 1.0] },
        ];

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("compass_buffer"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
            count: verts.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.count, 0..1);
    }
}
