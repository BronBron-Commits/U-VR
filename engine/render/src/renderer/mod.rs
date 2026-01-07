use glam::Vec3;
use winit::window::Window;

pub mod context;
pub mod frame;
pub mod pipeline;
pub mod resources;
pub mod uniforms;
pub mod skybox; // <-- ADD

use context::RenderContext;
use frame::FrameRenderer;
use uniforms::camera::OrbitCamera;

#[derive(Clone, Copy)]
pub struct Prop {
    pub position: Vec3,
    pub scale: Vec3,
}

pub struct Renderer {
    ctx: RenderContext,
    frame: FrameRenderer,

    avatar_pos: Vec3,
    avatar_yaw: f32,
    velocity: Vec3,
    grounded: bool,

    jump_count: u8,
    max_jumps: u8,

    pub camera: OrbitCamera,
    avatar_parts: Vec<Prop>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let ctx = RenderContext::new(window).await;
        let frame = FrameRenderer::new(&ctx);

        let avatar_parts = vec![
            Prop { position: Vec3::new(0.0, 0.9, 0.0), scale: Vec3::new(0.6, 0.9, 0.3) },
            Prop { position: Vec3::new(0.0, 1.6, 0.0), scale: Vec3::splat(0.35) },
            Prop { position: Vec3::new(-0.15, 0.3, 0.0), scale: Vec3::new(0.2, 0.6, 0.2) },
            Prop { position: Vec3::new(0.15, 0.3, 0.0), scale: Vec3::new(0.2, 0.6, 0.2) },
        ];

        Self {
            ctx,
            frame,
            avatar_pos: Vec3::ZERO,
            avatar_yaw: 0.0,
            velocity: Vec3::ZERO,
            grounded: true,
            jump_count: 0,
            max_jumps: 2,
            camera: OrbitCamera::new(),
            avatar_parts,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.resize(width, height);
    }

    pub fn update(&mut self, dt: f32, input: Vec3, jump: bool) {
        let speed = 4.0;

        let yaw = self.camera.yaw;
        let forward = Vec3::new(yaw.sin(), 0.0, yaw.cos());
        let right = Vec3::new(forward.z, 0.0, -forward.x);

        let mut move_dir = Vec3::ZERO;
        move_dir += forward * -input.z;
        move_dir += right * -input.x;

        if move_dir.length_squared() > 0.0001 {
            move_dir = move_dir.normalize();
            self.avatar_yaw = move_dir.x.atan2(move_dir.z);
            self.avatar_pos += move_dir * speed * dt;
        }

        if jump && self.jump_count < self.max_jumps {
            self.velocity.y = 5.0;
            self.grounded = false;
            self.jump_count += 1;
        }

        self.velocity.y -= 9.8 * dt;
        self.avatar_pos.y += self.velocity.y * dt;

        if self.avatar_pos.y <= 0.0 {
            self.avatar_pos.y = 0.0;
            self.velocity.y = 0.0;
            self.grounded = true;
            self.jump_count = 0;
        }

        self.camera.target = self.avatar_pos;
    }

    pub fn render(&mut self) {
        let avatar_world_props: Vec<Prop> = self
            .avatar_parts
            .iter()
            .map(|p| Prop {
                position: self.avatar_pos + p.position,
                scale: p.scale,
            })
            .collect();

        self.frame.render(
            &mut self.ctx,
            &self.camera,
            self.avatar_pos,
            self.avatar_yaw,
            &avatar_world_props,
        );
    }
}
