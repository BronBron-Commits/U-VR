use crate::renderer::mesh::Vertex;

pub fn generate() -> (Vec<Vertex>, Vec<u16>) {
    let s = 10.0;

    let vertices = vec![
        Vertex { position: [-s, 0.0, -s], color: [0.4, 0.4, 0.4] },
        Vertex { position: [ s, 0.0, -s], color: [0.4, 0.4, 0.4] },
        Vertex { position: [ s, 0.0,  s], color: [0.4, 0.4, 0.4] },
        Vertex { position: [-s, 0.0,  s], color: [0.4, 0.4, 0.4] },
    ];

    let indices = vec![0,1,2, 2,3,0];

    (vertices, indices)
}
