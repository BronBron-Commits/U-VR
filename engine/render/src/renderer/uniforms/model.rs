use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModelUniform {
    pub model: [[f32; 4]; 4],
    pub is_floor: u32,
    pub _pad: [u32; 3],
}

impl ModelUniform {
    pub fn new(model: [[f32; 4]; 4], is_floor: bool) -> Self {
        Self {
            model,
            is_floor: if is_floor { 1 } else { 0 },
            _pad: [0; 3],
        }
    }
}
