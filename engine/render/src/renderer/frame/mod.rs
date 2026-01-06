pub mod render_pass;

use glam::Vec3;
use crate::renderer::context::RenderContext;
use crate::renderer::uniforms::camera::OrbitCamera;

pub struct FrameRenderer;

impl FrameRenderer {
    pub fn new(_ctx: &RenderContext) -> Self {
        Self
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderContext,
        camera: &OrbitCamera,
        avatar_pos: Vec3,
    ) {
        render_pass::render_frame(ctx, camera, avatar_pos);
    }
}
