use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MaterialUniform {
    pub color: [f32; 4],
}

pub struct Material {
    pub uniform: MaterialUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        color: [f32; 4],
    ) -> Self {
        let uniform = MaterialUniform { color };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("material_buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("material_bind_group"),
        });

        Self {
            uniform,
            buffer,
            bind_group,
        }
    }
}
