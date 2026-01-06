use glam::{Mat4, Vec3};
use crate::renderer::mesh::Vertex;

pub struct Avatar {
    pub model: Mat4,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Avatar {
    pub fn new() -> Self {
        let s = 0.5;

        let vertices = vec![
            Vertex { position: [-s, 0.0, -s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [ s, 0.0, -s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [ s, 1.0, -s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [-s, 1.0, -s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [-s, 0.0,  s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [ s, 0.0,  s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [ s, 1.0,  s], color: [0.8, 0.2, 0.2] },
            Vertex { position: [-s, 1.0,  s], color: [0.8, 0.2, 0.2] },
        ];

        let indices = vec![
            0,1,2, 2,3,0,
            4,5,6, 6,7,4,
            0,4,7, 7,3,0,
            1,5,6, 6,2,1,
            3,2,6, 6,7,3,
            0,1,5, 5,4,0,
        ];

        let model = Mat4::from_translation(Vec3::ZERO);

        Self { model, vertices, indices }
    }
}
