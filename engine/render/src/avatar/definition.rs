use glam::Vec3;

#[derive(Clone)]
pub struct AvatarDefinition {
    pub scale: f32,
    pub parts: Vec<AvatarPartDef>,
}

#[derive(Clone)]
pub struct AvatarPartDef {
    pub name: &'static str,
    pub local_offset: Vec3,
    pub size: Vec3,
}
