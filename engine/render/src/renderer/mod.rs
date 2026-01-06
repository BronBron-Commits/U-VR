pub mod context;
pub mod frame;
pub mod pipeline;
pub mod scene;
pub mod objects;
pub mod resources;
pub mod uniforms;

use winit::window::Window;

pub struct Renderer {
    ctx: context::RenderContext,
    frame: frame::FrameRenderer,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let ctx = context::RenderContext::new(window).await;
        let frame = frame::FrameRenderer::new(&ctx);
        Self { ctx, frame }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.resize(width, height);
    }

    pub fn update(&mut self, _dt: f32, _input: glam::Vec3) {
        // intentionally empty for now
    }

    pub fn render(&mut self) {
        self.frame.render(&mut self.ctx);
    }
}
