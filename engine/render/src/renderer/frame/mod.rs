pub mod render_pass;

use crate::renderer::context::RenderContext;

pub struct FrameRenderer;

impl FrameRenderer {
    pub fn new(_ctx: &RenderContext) -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &mut RenderContext) {
        render_pass::render_frame(ctx);
    }
}
