pub struct EntityId(pub u64);

pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

pub struct Renderable {
    pub mesh: u32,
    pub material: u32,
}

pub struct Script {
    pub script_handle: u32,
}

// Core ECS and world logic will go here.
