use wgpu::Device;

pub struct BindGroupLayouts {
    pub camera: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &Device) -> Self {
        let camera = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self { camera }
    }
}
