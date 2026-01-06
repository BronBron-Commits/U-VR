use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct PlayerUniform {
    pub position: [f32; 3],
    pub speed: f32,
}

impl PlayerUniform {
    pub fn new(position: [f32; 3], speed: f32) -> Self {
        Self { position, speed }
    }
}
