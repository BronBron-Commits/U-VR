use glam::Vec3;

#[derive(Clone)]
pub struct AvatarPart {
    pub name: &'static str,
    pub offset: Vec3,
    pub size: Vec3,
}
