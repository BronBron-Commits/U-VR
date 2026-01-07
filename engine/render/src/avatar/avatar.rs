use glam::Vec3;

pub struct Avatar {
    pub position: Vec3,
    pub yaw: f32,
    pub scale: Vec3,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            yaw: 0.0,
            scale: Vec3::ONE,
        }
    }
}
