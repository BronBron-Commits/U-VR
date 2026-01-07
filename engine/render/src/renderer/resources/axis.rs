use crate::renderer::resources::mesh::Vertex;

pub fn axis_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let len = 0.6;

    let vertices = vec![
        // X axis (red)
        Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [len, 0.0, 0.0], color: [1.0, 0.0, 0.0] },

        // Y axis (green)
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.0, len, 0.0], color: [0.0, 1.0, 0.0] },

        // Z axis (blue)
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [0.0, 0.0, len], color: [0.0, 0.0, 1.0] },
    ];

    let indices = vec![0,1, 2,3, 4,5];
    (vertices, indices)
}
