use glam::{Mat4, Vec3};

use super::RenderObject;

pub struct Avatar {
    model: Mat4,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            model: Mat4::IDENTITY,
        }
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.model *= Mat4::from_translation(delta);
    }

    pub fn position(&self) -> Vec3 {
        self.model.w_axis.truncate()
    }
}

impl RenderObject for Avatar {
    fn model_matrix(&self) -> Mat4 {
        self.model
    }

    fn is_floor(&self) -> bool {
        false
    }
}
