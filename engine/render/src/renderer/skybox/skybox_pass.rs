use wgpu::*;
use crate::renderer::skybox::skybox_pipeline::SkyboxPipeline;

pub fn draw_skybox<'a>(
    pass: &mut RenderPass<'a>,
    skybox: &'a SkyboxPipeline,
) {
    pass.set_pipeline(&skybox.pipeline);
    pass.draw(0..3, 0..1); // fullscreen triangle
}
