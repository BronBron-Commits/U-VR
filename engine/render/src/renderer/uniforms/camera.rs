use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 8.0,
            yaw: 0.0,
            pitch: -0.6,
        }
    }

    pub fn handle_mouse(
        &mut self,
        mouse_dx: f32,
        mouse_dy: f32,
        scroll: f32,
        middle_mouse_held: bool,
    ) {
        // zoom always
        self.distance -= scroll * 0.5;
        self.distance = self.distance.clamp(2.0, 50.0);

        // orbit only when MMB held
        if middle_mouse_held {
            let sensitivity = 0.005;
            self.yaw += mouse_dx * sensitivity;
            self.pitch += mouse_dy * sensitivity;
            self.pitch = self.pitch.clamp(-1.5, 1.5);
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let dir = Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        );

        let pos = self.target - dir * self.distance;
        Mat4::look_at_rh(pos, self.target, Vec3::Y)
    }
}
