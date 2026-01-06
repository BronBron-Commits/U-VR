use glam::Vec3;
use winit::window::Window;

use context::RenderContext;
use frame::FrameRenderer;

pub mod context;
pub mod frame;
pub mod pipeline;
pub mod resources;
pub mod uniforms;

use uniforms::camera::OrbitCamera;

pub struct Renderer {
    ctx: RenderContext,
    frame: FrameRenderer,

    avatar_pos: Vec3,
    velocity: Vec3,
    grounded: bool,

    pub camera: OrbitCamera,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let ctx = RenderContext::new(window).await;
        let frame = FrameRenderer::new(&ctx);

        Self {
            ctx,
            frame,
            avatar_pos: Vec3::ZERO,
            velocity: Vec3::ZERO,
            grounded: true,
            camera: OrbitCamera::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.resize(width, height);
    }

    pub fn update(&mut self, dt: f32, input: Vec3, jump: bool) {
        let speed = 4.0;
        let dir = Vec3::new(-input.x, 0.0, -input.z).normalize_or_zero();
        self.avatar_pos += dir * speed * dt;

        if jump && self.grounded {
            self.velocity.y = 5.0;
            self.grounded = false;
        }

        self.velocity.y -= 9.8 * dt;
        self.avatar_pos.y += self.velocity.y * dt;

        if self.avatar_pos.y <= 0.0 {
            self.avatar_pos.y = 0.0;
            self.velocity.y = 0.0;
            self.grounded = true;
        }

        self.camera.target = self.avatar_pos;
    }

    pub fn render(&mut self) {
        self.frame.render(&mut self.ctx, &self.camera, self.avatar_pos);
    }
}
