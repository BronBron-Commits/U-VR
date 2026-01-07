use glam::Vec3;
use super::definition::*;

pub fn load_default_avatar() -> AvatarDefinition {
    AvatarDefinition {
        scale: 1.0,
        parts: vec![
            AvatarPartDef {
                name: "head",
                local_offset: Vec3::new(0.0, 1.6, 0.0),
                size: Vec3::splat(0.4),
            },
            AvatarPartDef {
                name: "body",
                local_offset: Vec3::new(0.0, 1.0, 0.0),
                size: Vec3::new(0.5, 0.7, 0.3),
            },
            AvatarPartDef {
                name: "legs",
                local_offset: Vec3::new(0.0, 0.4, 0.0),
                size: Vec3::new(0.4, 0.8, 0.3),
            },
        ],
    }
}
