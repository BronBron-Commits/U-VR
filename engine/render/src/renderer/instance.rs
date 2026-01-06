use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceUniform {
    pub model: [[f32; 4]; 4],
}
