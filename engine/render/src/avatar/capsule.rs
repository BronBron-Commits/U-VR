use glam::Vec3;

#[derive(Clone, Copy)]
pub struct CapsulePart {
    pub offset: Vec3,
    pub scale: Vec3,
}

pub struct CapsuleAvatar {
    pub parts: Vec<CapsulePart>,
}

impl CapsuleAvatar {
    pub fn humanoid() -> Self {
        let mut parts = Vec::new();

        // ===== BODY =====
        parts.push(CapsulePart {
            offset: Vec3::new(0.0, 0.9, 0.0),
            scale: Vec3::new(0.6, 1.2, 0.4),
        });

        // ===== HEAD =====
        parts.push(CapsulePart {
            offset: Vec3::new(0.0, 1.7, 0.0),
            scale: Vec3::new(0.45, 0.45, 0.45),
        });

        // ===== LOWER BODY =====
        parts.push(CapsulePart {
            offset: Vec3::new(0.0, 0.35, 0.0),
            scale: Vec3::new(0.5, 0.6, 0.35),
        });

        Self { parts }
    }
}
