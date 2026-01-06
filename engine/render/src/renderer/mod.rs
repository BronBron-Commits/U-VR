pub mod context;
pub mod frame;
pub mod pipeline;
pub mod resources;
pub mod uniforms;

use glam::Vec3;
use winit::window::Window;

use context::RenderContext;
use frame::FrameRenderer;

pub struct Renderer {
    ctx: RenderContext,
    frame: FrameRenderer,
    avatar_pos: Vec3,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let ctx = RenderContext::new(window).await;
        let frame = FrameRenderer::new(&ctx);

        Self {
            ctx,
            frame,
            avatar_pos: Vec3::ZERO,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.resize(width, height);
    }

    pub fn update(&mut self, dt: f32, input: Vec3) {
        let speed = 4.0;
        let dir = input.normalize_or_zero();
        self.avatar_pos += dir * speed * dt;
    }

    pub fn render(&mut self) {
        self.frame.render(&mut self.ctx, self.avatar_pos);
    }
}