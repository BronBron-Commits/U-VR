use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 3.0, 6.0),
            target: Vec3::ZERO,
            aspect: width / height,
            fovy: 45.0_f32.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    pub fn follow(&mut self, target: Vec3) {
        self.target = target;
        self.position = target + Vec3::new(0.0, 3.0, 6.0);
    }

    pub fn uniform(&self) -> CameraUniform {
        let view = Mat4::look_at_rh(self.position, self.target, Vec3::Y);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
        }
    }
}
