pub struct DepthTexture {
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
}

impl DepthTexture {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let format = wgpu::TextureFormat::Depth24Plus;

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { view, format }
    }
}
