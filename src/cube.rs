use crate::vertex::Vertex;
use crate::mesh::Mesh;

pub fn cube_mesh() -> Mesh<'static> {
    let vertices = &[
        // Frontal (rojo)
        Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },

        // Trasera (verde)
        Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
    ];

    Mesh::new(Some("Cube"), vertices)
}
