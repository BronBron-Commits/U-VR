use crate::renderer::resources::mesh::Vertex;

pub fn create_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    camera_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("main_shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("main_pipeline_layout"),
            bind_group_layouts: &[camera_layout],
            push_constant_ranges: &[],
        },
    );

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("main_pipeline"),
        layout: Some(&pipeline_layout),

        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::layout()],
        },

        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),

        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),

        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}
