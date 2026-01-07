use wgpu::*;

pub struct SkyboxPipeline {
    pub pipeline: RenderPipeline,
}

impl SkyboxPipeline {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("skybox_shader"),
            source: ShaderSource::Wgsl(include_str!("skybox.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("skybox_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("skybox_pipeline"),
            layout: Some(&layout),

            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },

            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),

            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },

            depth_stencil: None, // IMPORTANT
            multisample: MultisampleState::default(),
            multiview: None,
        });

        Self { pipeline }
    }
}
