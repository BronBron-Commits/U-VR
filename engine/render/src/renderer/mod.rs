use glam::Vec3;
use winit::window::Window;

use crate::avatar::{CapsuleAvatar, CapsulePart};

use context::RenderContext;
use frame::FrameRenderer;

pub mod context;
pub mod frame;
pub mod pipeline;
pub mod resources;
pub mod uniforms;

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

    avatar: CapsuleAvatar,

    pub camera: OrbitCamera,
    world_props: Vec<Prop>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let ctx = RenderContext::new(window).await;
        let frame = FrameRenderer::new(&ctx);

        let mut world_props = Vec::new();

        for xi in -10..=10 {
            for zi in -10..=10 {
                let x = xi as i32;
                let z = zi as i32;

                if (x + z) % 7 == 0 {
                    let h = 1.0 + ((x * z).abs() % 4) as f32;

                    world_props.push(Prop {
                        position: Vec3::new(x as f32 * 2.0, h * 0.5, z as f32 * 2.0),
                        scale: Vec3::new(1.0, h, 1.0),
                    });
                }
            }
        }

        Self {
            ctx,
            frame,

            avatar_pos: Vec3::ZERO,
            avatar_yaw: 0.0,
            velocity: Vec3::ZERO,
            grounded: true,

            jump_count: 0,
            max_jumps: 2,

            avatar: CapsuleAvatar::humanoid(),

            camera: OrbitCamera::new(),
            world_props,
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
        // ===== BUILD A SINGLE SCENE LIST =====
        let mut scene_props = self.world_props.clone();

        for CapsulePart { offset, scale } in &self.avatar.parts {
            scene_props.push(Prop {
                position: self.avatar_pos + *offset,
                scale: *scale,
            });
        }

        // ===== SINGLE RENDER PASS =====
        self.frame.render(
            &mut self.ctx,
            &self.camera,
            Vec3::ZERO,
            self.avatar_yaw,
            &scene_props,
        );
    }
}
