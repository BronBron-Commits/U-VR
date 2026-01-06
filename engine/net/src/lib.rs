#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct EntityId(pub u64);
