use glam::Mat4;

use super::RenderObject;

pub struct Floor {
    model: Mat4,
}

impl Floor {
    pub fn new() -> Self {
        Self {
            model: Mat4::IDENTITY,
        }
    }
}

impl RenderObject for Floor {
    fn model_matrix(&self) -> Mat4 {
        self.model
    }

    fn is_floor(&self) -> bool {
        true
    }
}
