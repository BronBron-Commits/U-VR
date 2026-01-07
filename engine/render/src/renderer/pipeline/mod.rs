use crate::renderer::context::device::RenderDevice;

pub mod pipeline;
pub mod overlay_pipeline;

use pipeline::create_pipeline;
use overlay_pipeline::create_overlay_pipeline;

pub struct RenderPipelineBundle {
    pub main: wgpu::RenderPipeline,
    pub overlay: wgpu::RenderPipeline,
}

impl RenderPipelineBundle {
    pub fn new(
        device: &RenderDevice,
        config: &wgpu::SurfaceConfiguration,
        camera_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let main = create_pipeline(
            &device.device,
            config,
            camera_layout,
        );

        let overlay = create_overlay_pipeline(
            &device.device,
            config,
        );

        Self { main, overlay }
    }
}
