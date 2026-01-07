pub mod render_pass;
pub mod overlay_pass;

use crate::renderer::context::RenderContext;
use crate::renderer::pipeline::RenderPipelineBundle;
use crate::renderer::skybox::skybox_pipeline::SkyboxPipeline;
use crate::renderer::uniforms::camera::OrbitCamera;
use crate::renderer::Prop;

pub struct FrameRenderer {
    pipelines: RenderPipelineBundle,
    skybox: SkyboxPipeline,
}

impl FrameRenderer {
    pub fn new(ctx: &RenderContext) -> Self {
        let pipelines = RenderPipelineBundle::new(
            &ctx.device,              // ✅ FIX: RenderDevice
            &ctx.surface.config,
            &ctx.camera_layout,
        );

        let skybox = SkyboxPipeline::new(
            &ctx.device.device,
            ctx.surface.config.format,
        );

        Self { pipelines, skybox }
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderContext,
        camera: &OrbitCamera,
        avatar_pos: glam::Vec3,
        avatar_yaw: f32,
        props: &[Prop],
    ) {
        render_pass::render_frame(
            ctx,
            camera,
            avatar_pos,
            avatar_yaw,
            props,
            &self.pipelines,
            &self.skybox,
        );
    }
}
