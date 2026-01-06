use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TimeUniform {
    pub t: f32,
    pub _pad: [f32; 3],
}

impl TimeUniform {
    pub fn new(t: f32) -> Self {
        Self { t, _pad: [0.0; 3] }
    }
}
